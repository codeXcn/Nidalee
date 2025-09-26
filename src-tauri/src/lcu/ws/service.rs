use base64::{engine::general_purpose, Engine};
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::OnceCell;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::client::IntoClientRequest};
use crate::lcu::auth::service::ensure_valid_auth_info;
use tauri::Emitter;

static WS_RUNNING: AtomicBool = AtomicBool::new(false);
static WS_TASK: OnceCell<tokio::task::JoinHandle<()>> = OnceCell::new();
static WS_SENDER: OnceCell<Arc<Mutex<Option<tokio_tungstenite::tungstenite::protocol::WebSocket<tokio_tungstenite::MaybeTlsStream<TcpStream>>>>>> = OnceCell::new();

pub async fn start_ws(app: tauri::AppHandle) {
    if WS_RUNNING.swap(true, Ordering::SeqCst) { return; }

    let handle = tokio::spawn(async move {
        let Some(auth) = ensure_valid_auth_info() else {
            log::warn!("[LCU-WS] 未获取到有效的 AuthInfo，WS 未启动");
            WS_RUNNING.store(false, Ordering::SeqCst);
            return;
        };
        let url = format!("wss://127.0.0.1:{}/", auth.app_port);
        let auth = format!("riot:{}", auth.remoting_auth_token);
        let auth_b64 = general_purpose::STANDARD.encode(auth.as_bytes());

        let mut req = url.into_client_request().expect("请求构建失败");
        req.headers_mut().insert("Authorization", format!("Basic {}", auth_b64).parse().unwrap());

        let config = tokio_tungstenite::Connector::NativeTls(native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true) // 忽略本地自签证书
            .build()
            .unwrap());

        let connect_res = connect_async_tls_with_config(req, None, false, Some(config)).await;
        let Ok((mut ws_stream, _resp)) = connect_res else {
            log::error!("LCU WS 连接失败: {:?}", connect_res.err());
            WS_RUNNING.store(false, Ordering::SeqCst);
            return;
        };

        // 订阅常用主题示例
        let subs = vec![
            "/lol-gameflow/v1/gameflow-phase",
            "/lol-lobby/v2/lobby",
            "/lol-champ-select/v1/session",
            "/lol-matchmaking/v1/search",
        ];
        for path in subs {
            let msg = json!(["subscribe", path]).to_string();
            if let Err(e) = ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await {
                log::warn!("订阅 {} 失败: {}", path, e);
            }
        }

        // 简单接收循环：转发到 tauri 事件总线
        while WS_RUNNING.load(Ordering::SeqCst) {
            match ws_stream.next().await {
                Some(Ok(tokio_tungstenite::tungstenite::Message::Text(text))) => {
                    // 事件格式: [eventType, path, payload]
                    let _ = app.emit("lcu-ws", text.clone());
                    log::trace!("[LCU-WS] {}", text);
                }
                Some(Ok(_)) => {}
                Some(Err(e)) => {
                    log::error!("WS 错误: {}", e);
                    break;
                }
                None => break,
            }
        }

        WS_RUNNING.store(false, Ordering::SeqCst);
    });

    let _ = WS_TASK.set(handle);
}

pub async fn stop_ws() {
    WS_RUNNING.store(false, Ordering::SeqCst);
}
