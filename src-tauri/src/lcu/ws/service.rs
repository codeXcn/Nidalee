use base64::{engine::general_purpose, Engine};
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::OnceCell;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::collections::HashSet;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::client::IntoClientRequest};
use crate::lcu::auth::service::ensure_valid_auth_info;
use crate::lcu::ws::event_handler::WsEventHandler;
use tauri::Emitter;

static WS_RUNNING: AtomicBool = AtomicBool::new(false);
static WS_TASK: OnceCell<tokio::task::JoinHandle<()>> = OnceCell::new();
static WS_SENDER: OnceCell<Arc<Mutex<Option<tokio_tungstenite::tungstenite::protocol::WebSocket<tokio_tungstenite::MaybeTlsStream<TcpStream>>>>>> = OnceCell::new();

// 辅助函数：确保订阅某个路径（幂等）
async fn ensure_subscribed(
    ws_stream: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
    path: &str,
    subscribed: &mut std::collections::HashSet<String>,
) {
    use tokio_tungstenite::tungstenite::Message;
    if !subscribed.contains(path) {
        let msg = json!([5, "OnJsonApiEvent", path]).to_string();
        match ws_stream.send(Message::Text(msg)).await {
            Ok(_) => {
                subscribed.insert(path.to_string());
                log::info!("[LCU-WS] 成功订阅: {}", path);
            }
            Err(e) => log::warn!("[LCU-WS] 订阅 {} 失败: {}", path, e),
        }
    }
}

// 当 WS 长时间无事件时，做一次 HTTP 回退拉取并发送规范化事件
// phase_hint: 当前已知阶段的提示，用于裁剪需要拉取的端点，减少无意义的 404
async fn fallback_fetch_and_emit(app: &tauri::AppHandle, phase_hint: Option<&str>) {
    // 构造一个短超时的 HTTP 客户端
    let client = match reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            log::warn!("[LCU-WS] 回退 HTTP 客户端创建失败: {}", e);
            return;
        }
    };

    // 先尝试获取最新阶段；仅在成功时发送正向数据，避免因为临时失败清空前端状态
    let mut latest_phase: Option<String> = None;
    if let Ok(phase) = crate::lcu::gameflow::service::get_gameflow_phase(&client).await {
        let _ = app.emit("gameflow-phase-change", &Some(phase.clone()));
        latest_phase = Some(phase);
    }

    // 根据阶段决定是否获取其他数据，减少不必要的 404
    let effective_phase = latest_phase.as_deref().or(phase_hint);
    match effective_phase {
        Some("Lobby") | Some("Matchmaking") | Some("None") => {
            if let Ok(lobby) = crate::lcu::lobby::service::get_lobby_info(&client).await {
                let _ = app.emit("lobby-change", &Some(lobby));
            }
            if let Ok(state) = crate::lcu::matchmaking::service::get_matchmaking_state(&client).await {
                let _ = app.emit("matchmaking-state-changed", state);
            }
        }
        Some("ChampSelect") => {
            if let Ok(session) = crate::lcu::champ_select::service::get_champ_select_session(&client).await {
                let _ = app.emit("champ-select-session-changed", &session);
            }
        }
        _ => {
            // 其他阶段（如 None/InProgress）仅同步阶段即可
        }
    }
}

pub async fn start_ws(app: tauri::AppHandle) {
    if WS_RUNNING.swap(true, Ordering::SeqCst) { return; }

    let handle = tokio::spawn(async move {
        // 主循环：持续尝试连接和重连
        while WS_RUNNING.load(Ordering::SeqCst) {
            // 等待认证信息可用
            let mut retry_count = 0;
            let max_retries = 10;

            while retry_count < max_retries && WS_RUNNING.load(Ordering::SeqCst) {
                if let Some(auth) = ensure_valid_auth_info() {
                    log::info!("[LCU-WS] 获取到认证信息，尝试连接...");
                    if let Err(e) = connect_and_run_ws(&app, &auth).await {
                        log::warn!("[LCU-WS] 连接失败: {}, 将在3秒后重试", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    } else {
                        log::info!("[LCU-WS] 连接正常结束，将在3秒后重连");
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    }
                } else {
                    retry_count += 1;
                    log::debug!("[LCU-WS] 等待认证信息... ({}/{})", retry_count, max_retries);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }

            if retry_count >= max_retries {
                log::warn!("[LCU-WS] 多次尝试后仍无法获取认证信息，将在5秒后重试");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }

        log::info!("[LCU-WS] 主循环已退出");
        WS_RUNNING.store(false, Ordering::SeqCst);
    });

    let _ = WS_TASK.set(handle);
}

async fn connect_and_run_ws(app: &tauri::AppHandle, auth: &crate::lcu::types::LcuAuthInfo) -> Result<(), String> {
    let url = format!("wss://127.0.0.1:{}/", auth.app_port);
    let auth_string = format!("riot:{}", auth.remoting_auth_token);
    let auth_b64 = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let mut req = url.into_client_request().map_err(|e| format!("请求构建失败: {}", e))?;
    req.headers_mut().insert("Authorization", format!("Basic {}", auth_b64).parse().unwrap());

    let config = tokio_tungstenite::Connector::NativeTls(native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(true) // 忽略本地自签证书
        .build()
        .map_err(|e| format!("TLS配置失败: {}", e))?);

    let connect_res = connect_async_tls_with_config(req, None, false, Some(config)).await;
    let Ok((mut ws_stream, _resp)) = connect_res else {
        return Err(format!("LCU WS 连接失败: {:?}", connect_res.err()));
    };

    log::info!("[LCU-WS] WebSocket 连接成功");

    // 动态订阅集合：初始仅订阅基础阶段主题，其他按阶段按需订阅
    let mut subscribed: HashSet<String> = HashSet::new();
    let mut current_phase: Option<String> = None;

    // 基础订阅：始终订阅阶段相关
    ensure_subscribed(&mut ws_stream, "/lol-gameflow/v1/gameflow-phase", &mut subscribed).await;
    ensure_subscribed(&mut ws_stream, "/lol-gameflow/v1/session", &mut subscribed).await;

    // 创建事件处理器
    let event_handler = WsEventHandler::new(app.clone());

    // 接收循环：处理 WebSocket 事件
    while WS_RUNNING.load(Ordering::SeqCst) {
        // 使用 select 机制：既响应 WS 消息，也在空闲时做一次回退刷新，不主动断线
        tokio::select! {
            match_item = ws_stream.next() => {
                match match_item {
            Some(Ok(tokio_tungstenite::tungstenite::Message::Text(text))) => {
                // 先尝试解析事件以获取更多调试信息
                let event_info = if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(event_array) = data.as_array() {
                        if event_array.len() >= 3 {
                            if let (Some(8), Some("OnJsonApiEvent"), Some(payload)) = (
                                event_array[0].as_u64(),
                                event_array[1].as_str(),
                                event_array[2].as_object()
                            ) {
                                // 动态订阅：根据阶段按需补充订阅主题
                                if let Some(uri) = payload.get("uri").and_then(|v| v.as_str()) {
                                    // 如果是阶段事件，尝试解析新阶段
                                    if uri == "/lol-gameflow/v1/gameflow-phase" {
                                        if let Some(phase_str) = payload.get("data").and_then(|v| v.as_str()) {
                                            if current_phase.as_deref() != Some(phase_str) {
                                                log::info!("[LCU-WS] 阶段变化: {:?} -> {} (动态订阅) ", current_phase, phase_str);
                                                current_phase = Some(phase_str.to_string());
                                                match phase_str {
                                                    "Lobby" | "Matchmaking" | "None" => {
                                                        ensure_subscribed(&mut ws_stream, "/lol-lobby/v2/lobby", &mut subscribed).await;
                                                        ensure_subscribed(&mut ws_stream, "/lol-matchmaking/v1/search", &mut subscribed).await;
                                                    }
                                                    "ChampSelect" => {
                                                        ensure_subscribed(&mut ws_stream, "/lol-champ-select/v1/session", &mut subscribed).await;
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                    format!("URI: {}", uri)
                                } else {
                                    "未知事件类型".to_string()
                                }
                            } else {
                                format!("非API事件: {:?}", event_array)
                            }
                        } else {
                            format!("事件数组长度不足: {}", event_array.len())
                        }
                    } else {
                        "非数组事件".to_string()
                    }
                } else {
                    format!("原始文本: {}", if text.len() > 100 { &text[..100] } else { &text })
                };

                // 使用事件处理器处理事件
                if let Err(e) = event_handler.handle_event(&text).await {
                    log::warn!("[LCU-WS] 事件处理失败 [{}]: {}", event_info, e);
                }

                // 只发送重要事件到前端（减少噪音）
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(event_array) = data.as_array() {
                        if event_array.len() >= 3 {
                            if let (Some(8), Some("OnJsonApiEvent"), Some(payload)) = (
                                event_array[0].as_u64(),
                                event_array[1].as_str(),
                                event_array[2].as_object()
                            ) {
                                if let Some(uri) = payload.get("uri").and_then(|v| v.as_str()) {
                                    // 只发送重要事件
                                    if matches!(uri,
                                        "/lol-gameflow/v1/gameflow-phase" |
                                        "/lol-gameflow/v1/session" |
                                        "/lol-champ-select/v1/session" |
                                        "/lol-lobby/v2/lobby" |
                                        "/lol-matchmaking/v1/search"
                                    ) {
                                        let _ = app.emit("lcu-ws", text.clone());
                                        log::debug!("[LCU-WS] 重要事件: {}", uri);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(Ok(_)) => {}
            Some(Err(e)) => {
                log::error!("WS 错误: {}", e);
                return Err(format!("WebSocket 错误: {}", e));
            }
            None => {
                log::warn!("[LCU-WS] 连接被服务器关闭");
                return Err("连接被服务器关闭".to_string());
            }
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                // 空闲 10s：做一次 HTTP 回退刷新，不断开 WS
                log::debug!("[LCU-WS] 10s 内无 WS 事件，执行一次 HTTP 回退拉取以对齐状态");
                fallback_fetch_and_emit(app, current_phase.as_deref()).await;
                // 下次循环继续等待 WS 事件
            }
        }
    }

    Ok(())
}

pub async fn stop_ws() {
    WS_RUNNING.store(false, Ordering::SeqCst);
}
