// WebSocket 事件处理器 - 将 WebSocket 事件转换为应用事件
use crate::lcu::types::{ChampSelectSession, MatchmakingState, LobbyInfo, SummonerInfo};
use crate::lcu::summoner::service::get_summoner_by_id;
use crate::lcu::analysis_data;  // 使用模块化的 analysis_data
use serde_json::Value;
use tauri::{AppHandle, Emitter};
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::Client;

// 事件数据缓存，用于避免重复发送相同数据
#[derive(Default)]
struct EventCache {
    gameflow_phase: Option<String>,
    gameflow_session: Option<String>, // 存储 session 的 JSON 字符串用于比较
    champ_select_session: Option<ChampSelectSession>,
    matchmaking_state: Option<MatchmakingState>,
    lobby_info: Option<LobbyInfo>,
    // 🔥 战绩数据缓存（key: display_name, value: PlayerMatchStats）
    match_stats_cache: std::collections::HashMap<String, crate::lcu::types::PlayerMatchStats>,
    // 🔥 分析数据缓存（这才是我们要缓存的！）
    team_analysis_data: Option<crate::lcu::types::TeamAnalysisData>,
}

pub struct WsEventHandler {
    app: AppHandle,
    cache: Arc<RwLock<EventCache>>,
    client: Client, // HTTP 客户端用于获取召唤师信息
}

impl WsEventHandler {
    pub fn new(app: AppHandle) -> Self {
        // 创建 HTTP 客户端
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            app,
            cache: Arc::new(RwLock::new(EventCache::default())),
            client,
        }
    }

    /// 🔥 获取当前缓存的分析数据（这才是我们真正要缓存的！）
    pub async fn get_cached_team_analysis_data(&self) -> Option<crate::lcu::types::TeamAnalysisData> {
        let cache = self.cache.read().await;
        cache.team_analysis_data.clone()
    }

    /// 🔍 获取缓存状态信息（用于调试）
    pub async fn get_cache_debug_info(&self) -> (bool, bool, usize) {
        let cache = self.cache.read().await;
        (
            cache.champ_select_session.is_some(),
            cache.team_analysis_data.is_some(),
            cache.match_stats_cache.len()
        )
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
        // Create 和 Update 都表示有数据
        if event_type == "Create" || event_type == "Update" {
            if let Some(phase) = data.as_str() {
                let mut cache = self.cache.write().await;
                if cache.gameflow_phase.as_ref() != Some(&phase.to_string()) {
                    log::info!("[WS-Event] 游戏阶段变化 ({}): {}", event_type, phase);

                    // 🔥 如果离开选人阶段（进入游戏或返回大厅），清理战绩缓存和 Session 缓存
                    if phase != "ChampSelect" && phase != "ReadyCheck" && phase != "Matchmaking" {
                        if !cache.match_stats_cache.is_empty() {
                            log::info!("[WS-Event] 🗑️ 阶段切换到 {}，清理战绩缓存 (共 {} 条)", phase, cache.match_stats_cache.len());
                            cache.match_stats_cache.clear();
                        }
                        if cache.team_analysis_data.is_some() {
                            log::info!("[WS-Event] 🗑️ 阶段切换到 {}，清理分析数据缓存", phase);
                            cache.team_analysis_data = None;
                        }
                    }

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
        // Create 和 Update 都表示有数据
        if event_type == "Create" || event_type == "Update" {
            // 序列化 session 数据用于比较
            let session_json = serde_json::to_string(data).unwrap_or_default();

            let mut cache = self.cache.write().await;

            // 检查 session 是否发生变化
            let session_changed = cache.gameflow_session.as_ref() != Some(&session_json);

            if session_changed {
                // 更新缓存
                cache.gameflow_session = Some(session_json);

                // 发送完整的 gameflow session 数据到前端
                drop(cache); // 释放锁，避免在 emit 时持有锁
                let _ = self.app.emit("gameflow-session-changed", data);

                log::debug!("[WS-Event] Gameflow Session 已更新并发送 ({})", event_type);
            } else {
                drop(cache);
                log::trace!("[WS-Event] Gameflow Session 无变化，跳过发送");
            }

            // 注意：不在这里发送 gameflow-phase-change 事件
            // 因为 /lol-gameflow/v1/gameflow-phase 会单独触发 handle_gameflow_phase_change
            // 这样避免重复发送阶段变化事件

            // 调试模式下打印详细信息
            #[cfg(debug_assertions)]
            {
                log::debug!("[WS-Event] Gameflow Session 更新:");
                log::debug!("  Phase: {:?}", data["phase"]);
                log::debug!("  Map: {:?}", data["map"]["name"]);
                log::debug!("  GameData: {:?}", data["gameData"]["queue"]);
            }
        } else if event_type == "Delete" {
            log::info!("[WS-Event] 游戏会话清除");

            let mut cache = self.cache.write().await;
            cache.gameflow_session = None;
            drop(cache);

            // 只发送 session 删除事件
            // phase 删除由 handle_gameflow_phase_change 处理
            let _ = self.app.emit("gameflow-session-changed", &None::<Value>);
        }
        Ok(())
    }

    async fn handle_champ_select_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        log::info!("[WS-Event] 🔔 收到 champ-select 事件，类型: {}", event_type);

        // Create 和 Update 都表示有数据
        if event_type == "Create" || event_type == "Update" {
            // 🔍 先打印原始 WebSocket 收到的数据
            if let Ok(pretty_json) = serde_json::to_string_pretty(data) {
                log::info!("[WS-Event] 📋 WebSocket 收到的原始 Session 数据:");
                log::info!("{}", pretty_json);
            }

            // 🔥 新方案：调用服务层生成完整的分析数据（使用缓存）
            let mut cache = self.cache.write().await;
            match analysis_data::service::build_team_analysis_from_session(
                data,
                &self.client,
                &mut cache.match_stats_cache  // 传递战绩缓存
            ).await {
                Ok(analysis_data) => {
                    log::info!("[WS-Event] ✅ 分析数据生成成功");
                    log::debug!("[WS-Event] 我方玩家数: {}, 敌方玩家数: {}",
                        analysis_data.my_team.len(), analysis_data.enemy_team.len());
                    log::info!("[WS-Event] 📊 当前缓存的战绩数: {}", cache.match_stats_cache.len());

                    // 🔥 缓存分析数据（这才是我们要缓存的！）
                    cache.team_analysis_data = Some(analysis_data.clone());
                    log::info!("[WS-Event] 💾 TeamAnalysisData 已缓存");

                    // 发送完整的分析数据到前端
                    drop(cache); // 提前释放锁
                    let _ = self.app.emit("team-analysis-data", &analysis_data);
                }
                Err(e) => {
                    log::error!("[WS-Event] ❌ 生成分析数据失败: {}", e);
                    log::error!("[WS-Event] 错误详情: {:?}", e);
                    log::error!("[WS-Event] 错误来源: {}", e.source().map(|s| s.to_string()).unwrap_or_else(|| "无".to_string()));

                    // 降级：发送原始 champ-select-session 数据
                    log::warn!("[WS-Event] 🔄 尝试降级方案：发送原始会话数据");
                    match serde_json::from_value::<ChampSelectSession>(data.clone()) {
                        Ok(mut session) => {
                            log::debug!("[WS-Event] ✅ 降级成功，发送原始会话数据");
                            self.enrich_champ_select_session(&mut session).await;
                            let _ = self.app.emit("champ-select-session-changed", &Some(session));
                        }
                        Err(parse_err) => {
                            log::error!("[WS-Event] ❌ 降级方案也失败，解析选人会话数据失败: {}", parse_err);
                            log::error!("[WS-Event] Session 数据预览: {}",
                                serde_json::to_string(data).unwrap_or_else(|_| "无法序列化".to_string())
                                    .chars().take(200).collect::<String>());
                        }
                    }
                }
            }
        } else if event_type == "Delete" {
            log::info!("[WS-Event] 🗑️ 选人会话清除");

            // 🔥 清理缓存
            let mut cache = self.cache.write().await;
            cache.champ_select_session = None;
            cache.team_analysis_data = None;
            log::info!("[WS-Event] 🗑️ 分析数据缓存已清除");
            drop(cache);

            // 发送空的分析数据
            let _ = self.app.emit("team-analysis-data", &None::<crate::lcu::types::TeamAnalysisData>);
        }
        Ok(())
    }

    async fn handle_lobby_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        // Create 和 Update 都表示有数据
        if event_type == "Create" || event_type == "Update" {
            // 解析为 LobbyInfo
            if let Ok(lobby) = serde_json::from_value::<LobbyInfo>(data.clone()) {
                let mut cache = self.cache.write().await;
                if cache.lobby_info.as_ref() != Some(&lobby) {
                    log::info!("[WS-Event] 大厅信息更新 ({})", event_type);
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
        // Create 和 Update 都表示有数据
        if event_type == "Create" || event_type == "Update" {
            // 尝试解析匹配状态数据
            if let Ok(matchmaking_state) = serde_json::from_value::<MatchmakingState>(data.clone()) {
                let mut cache = self.cache.write().await;
                if cache.matchmaking_state.as_ref() != Some(&matchmaking_state) {
                    log::info!("[WS-Event] 匹配状态更新 ({}): {:?}", event_type, matchmaking_state.search_state);
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

    /// Enrich 选人会话 - 填充召唤师信息
    async fn enrich_champ_select_session(&self, session: &mut ChampSelectSession) {
        // 收集所有需要查询的 summoner_id
        let mut all_ids = vec![];
        for p in session.my_team.iter().chain(session.their_team.iter()) {
            if let Some(sid) = &p.summoner_id {
                if sid != "0" && !all_ids.contains(sid) {
                    all_ids.push(sid.clone());
                }
            }
        }

        // 批量查询召唤师信息
        let mut info_map = std::collections::HashMap::new();
        for sid in &all_ids {
            if let Ok(id) = sid.parse::<u64>() {
                match get_summoner_by_id(&self.client, id).await {
                    Ok(info) => {
                        info_map.insert(sid.clone(), info);
                    }
                    Err(e) => {
                        log::debug!("[WS-Event] 获取召唤师信息失败 (ID: {}): {}", sid, e);
                    }
                }
            }
        }

        // 补全 my_team
        for p in session.my_team.iter_mut() {
            Self::enrich_player(p, &info_map);
        }

        // 补全 their_team
        for p in session.their_team.iter_mut() {
            Self::enrich_player(p, &info_map);
        }
    }

    /// Enrich 单个玩家信息
    fn enrich_player(
        player: &mut crate::lcu::types::ChampSelectPlayer,
        info_map: &std::collections::HashMap<String, SummonerInfo>,
    ) {
        if let Some(sid) = &player.summoner_id {
            if sid == "0" {
                // 人机
                player.display_name = Some("电脑玩家".to_string());
                player.tag_line = None;
                player.profile_icon_id = None;
                player.tier = None;
            } else if let Some(info) = info_map.get(sid) {
                // 真人玩家：优先用 game_name + tag_line
                let display_name =
                    if let (Some(game_name), Some(tag_line)) = (&info.game_name, &info.tag_line) {
                        format!("{}#{}", game_name, tag_line)
                    } else {
                        info.display_name.clone()
                    };
                player.display_name = Some(display_name);
                player.tag_line = info.tag_line.clone();
                player.profile_icon_id = Some(info.profile_icon_id);
                player.tier = info.solo_rank_tier.clone();
            }
        }
    }
}
