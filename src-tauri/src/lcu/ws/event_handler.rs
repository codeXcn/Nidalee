// WebSocket 事件处理器 - 将 WebSocket 事件转换为应用事件
use crate::lcu::types::{ChampSelectSession, MatchmakingState, LobbyInfo};
use serde_json::Value;
use tauri::{AppHandle, Emitter};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

// 事件数据缓存，用于避免重复发送相同数据
#[derive(Default)]
struct EventCache {
    gameflow_phase: Option<String>,
    champ_select_session: Option<ChampSelectSession>,
    matchmaking_state: Option<MatchmakingState>,
    lobby_info: Option<LobbyInfo>,
    last_champ_select_emit: Option<Instant>,
}

pub struct WsEventHandler {
    app: AppHandle,
    cache: Arc<RwLock<EventCache>>,
}

impl WsEventHandler {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            cache: Arc::new(RwLock::new(EventCache::default())),
        }
    }

    /// 处理 WebSocket 事件并转换为应用事件
    pub async fn handle_event(&self, event_data: &str) -> Result<(), String> {
        // 检查是否为空数据
        if event_data.trim().is_empty() {
            return Err("收到空数据".to_string());
        }

        let data: Value = serde_json::from_str(event_data)
            .map_err(|e| format!("解析 WebSocket 事件失败 (数据长度: {}, 前100字符: '{}'): {}",
                event_data.len(),
                if event_data.len() > 100 { &event_data[..100] } else { event_data },
                e))?;

        // 检查是否是 WAMP 格式的事件
        if let Some(event_array) = data.as_array() {
            if event_array.len() >= 3 {
                let message_type = event_array[0].as_u64();
                let event_name = event_array[1].as_str();
                let payload = &event_array[2];

                if message_type == Some(8) && event_name == Some("OnJsonApiEvent") {
                    // 只处理我们关心的关键事件
                    if self.is_important_event(payload) {
                        self.handle_json_api_event(payload).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// 判断是否是重要的事件（需要处理的事件）
    fn is_important_event(&self, payload: &Value) -> bool {
        let uri = payload["uri"].as_str().unwrap_or("");

        // 只处理这些关键事件
        matches!(uri,
            "/lol-gameflow/v1/gameflow-phase" |
            "/lol-champ-select/v1/session" |
            "/lol-lobby/v2/lobby" |
            "/lol-matchmaking/v1/search" |
            "/lol-gameflow/v1/session"
        )
    }

    async fn handle_json_api_event(&self, payload: &Value) -> Result<(), String> {
        let uri = payload["uri"].as_str().unwrap_or("");
        let event_type = payload["eventType"].as_str().unwrap_or("");
        let data = &payload["data"];

        match uri {
            "/lol-gameflow/v1/gameflow-phase" => {
                self.handle_gameflow_phase_change(data, event_type).await?;
            }
            "/lol-gameflow/v1/session" => {
                self.handle_gameflow_session_change(data, event_type).await?;
            }
            "/lol-champ-select/v1/session" => {
                self.handle_champ_select_change(data, event_type).await?;
            }
            "/lol-lobby/v2/lobby" => {
                self.handle_lobby_change(data, event_type).await?;
            }
            "/lol-matchmaking/v1/search" => {
                self.handle_matchmaking_change(data, event_type).await?;
            }
            _ => {
                // 其他事件，记录但不处理
                log::trace!("[WS-Event] 未处理的事件: {}", uri);
            }
        }

        Ok(())
    }

    async fn handle_gameflow_phase_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        if event_type == "Update" {
            if let Some(phase) = data.as_str() {
                let mut cache = self.cache.write().await;
                if cache.gameflow_phase.as_ref() != Some(&phase.to_string()) {
                    log::info!("[WS-Event] 游戏阶段变化: {}", phase);
                    cache.gameflow_phase = Some(phase.to_string());
                    let _ = self.app.emit("gameflow-phase-change", &Some(phase.to_string()));
                } else {
                    log::debug!("[WS-Event] 游戏阶段无变化，跳过发送: {}", phase);
                }
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.gameflow_phase.is_some() {
                log::info!("[WS-Event] 游戏阶段清除");
                cache.gameflow_phase = None;
                let _ = self.app.emit("gameflow-phase-change", &None::<String>);
            }
        }
        Ok(())
    }

    async fn handle_gameflow_session_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        if event_type == "Update" {
            if let Some(phase) = data["phase"].as_str() {
                let mut cache = self.cache.write().await;
                if cache.gameflow_phase.as_ref() != Some(&phase.to_string()) {
                    log::info!("[WS-Event] 游戏会话阶段变化: {}", phase);
                    cache.gameflow_phase = Some(phase.to_string());
                    let _ = self.app.emit("gameflow-phase-change", &Some(phase.to_string()));
                } else {
                    log::debug!("[WS-Event] 游戏会话阶段无变化，跳过发送: {}", phase);
                }
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.gameflow_phase.is_some() {
                log::info!("[WS-Event] 游戏会话清除");
                cache.gameflow_phase = None;
                let _ = self.app.emit("gameflow-phase-change", &None::<String>);
            }
        }
        Ok(())
    }

    async fn handle_champ_select_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        if event_type == "Update" {
            // 尝试解析选人会话数据
            if let Ok(session) = serde_json::from_value::<ChampSelectSession>(data.clone()) {
                let mut cache = self.cache.write().await;
                let now = Instant::now();
                let min_interval = Duration::from_millis(100);
                let allow_emit = match cache.last_champ_select_emit {
                    Some(last) => now.duration_since(last) >= min_interval,
                    None => true,
                };

                if cache.champ_select_session.as_ref() != Some(&session) {
                    if allow_emit {
                        log::info!("[WS-Event] 选人会话更新(节流后)");
                        cache.champ_select_session = Some(session.clone());
                        cache.last_champ_select_emit = Some(now);
                        let _ = self.app.emit("champ-select-session-changed", &Some(session));
                    } else {
                        log::debug!("[WS-Event] 选人会话更新过于频繁，节流跳过");
                    }
                } else {
                    log::debug!("[WS-Event] 选人会话无变化，跳过发送");
                }
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.champ_select_session.is_some() {
                log::info!("[WS-Event] 选人会话清除");
                cache.champ_select_session = None;
                cache.last_champ_select_emit = None;
                let _ = self.app.emit("champ-select-session-changed", &None::<ChampSelectSession>);
            }
        }
        Ok(())
    }

    async fn handle_lobby_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        if event_type == "Update" {
            // 解析为 LobbyInfo
            if let Ok(lobby) = serde_json::from_value::<LobbyInfo>(data.clone()) {
                let mut cache = self.cache.write().await;
                if cache.lobby_info.as_ref() != Some(&lobby) {
                    log::info!("[WS-Event] 大厅信息更新");
                    cache.lobby_info = Some(lobby.clone());
                    let _ = self.app.emit("lobby-change", &Some(lobby));
                } else {
                    log::debug!("[WS-Event] 大厅信息无变化，跳过发送");
                }
            } else {
                log::debug!("[WS-Event] 大厅信息解析失败，跳过");
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.lobby_info.is_some() {
                log::info!("[WS-Event] 离开大厅");
                cache.lobby_info = None;
                let _ = self.app.emit("lobby-change", &None::<LobbyInfo>);
            }
        }
        Ok(())
    }

    async fn handle_matchmaking_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        if event_type == "Update" {
            // 尝试解析匹配状态数据
            if let Ok(matchmaking_state) = serde_json::from_value::<MatchmakingState>(data.clone()) {
                let mut cache = self.cache.write().await;
                if cache.matchmaking_state.as_ref() != Some(&matchmaking_state) {
                    log::info!("[WS-Event] 匹配状态更新: {:?}", matchmaking_state.search_state);
                    cache.matchmaking_state = Some(matchmaking_state.clone());
                    let _ = self.app.emit("matchmaking-state-changed", matchmaking_state);
                } else {
                    log::debug!("[WS-Event] 匹配状态无变化，跳过发送: {:?}", matchmaking_state.search_state);
                }
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.matchmaking_state.is_some() {
                log::info!("[WS-Event] 匹配状态清除");
                cache.matchmaking_state = None;
                let _ = self.app.emit("matchmaking-state-changed", &None::<MatchmakingState>);
            }
        }
        Ok(())
    }
}
