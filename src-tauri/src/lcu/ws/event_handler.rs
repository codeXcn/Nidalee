// WebSocket 事件处理器 - 将 WebSocket 事件转换为应用事件
use crate::lcu::analysis_data; // 使用模块化的 analysis_data
use crate::lcu::summoner::service::get_summoner_by_id;
use crate::lcu::types::{ChampSelectSession, LobbyInfo, MatchmakingState, SummonerInfo};
use crate::lcu::champion_data; // 引入英雄数据模块
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

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
    pub async fn get_cached_team_analysis_data(
        &self,
    ) -> Option<crate::lcu::types::TeamAnalysisData> {
        let cache = self.cache.read().await;
        cache.team_analysis_data.clone()
    }

    /// 🔍 获取缓存状态信息（用于调试）
    pub async fn get_cache_debug_info(&self) -> (bool, bool, usize) {
        let cache = self.cache.read().await;
        (
            cache.champ_select_session.is_some(),
            cache.team_analysis_data.is_some(),
            cache.match_stats_cache.len(),
        )
    }

    /// 处理 WebSocket 事件并转换为应用事件
    pub async fn handle_event(&self, event_data: &str) -> Result<(), String> {
        // 检查是否为空数据
        if event_data.trim().is_empty() {
            return Err("收到空数据".to_string());
        }

        let data: Value = serde_json::from_str(event_data).map_err(|e| {
            format!(
                "解析 WebSocket 事件失败 (数据长度: {}, 前100字符: '{}'): {}",
                event_data.len(),
                if event_data.len() > 100 {
                    &event_data[..100]
                } else {
                    event_data
                },
                e
            )
        })?;

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
        matches!(
            uri,
            "/lol-gameflow/v1/gameflow-phase"
                | "/lol-champ-select/v1/session"
                | "/lol-lobby/v2/lobby"
                | "/lol-matchmaking/v1/search"
                | "/lol-gameflow/v1/session"
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
                self.handle_gameflow_session_change(data, event_type)
                    .await?;
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

    async fn handle_gameflow_phase_change(
        &self,
        data: &Value,
        event_type: &str,
    ) -> Result<(), String> {
        log::info!("[WS-Event] 游戏阶段变化 ({}): {}", event_type, data);
        if event_type == "Create" || event_type == "Update" {
            if let Some(phase) = data.as_str() {
                let mut cache = self.cache.write().await;
                if cache.gameflow_phase.as_ref() != Some(&phase.to_string()) {

                    // 🔥 核心修改：当进入 InProgress 阶段时，不再清理缓存，而是触发回填逻辑
                    if phase == "InProgress" {
                        if cache.team_analysis_data.is_some() {
                            log::info!("[WS-Event] 🚀 InProgress 阶段，触发敌方数据回填");
                            // 克隆所需的字段以满足 'static 生命周期要求
                            let app_clone = self.app.clone();
                            let cache_clone = Arc::clone(&self.cache);
                            let client_clone = self.client.clone();

                            tokio::spawn(async move {
                                // 在新任务中重新构建 handler（避免生命周期问题）
                                let temp_handler = WsEventHandler {
                                    app: app_clone,
                                    cache: cache_clone,
                                    client: client_clone,
                                };

                                if let Err(e) = temp_handler.backfill_enemy_team_data().await {
                                    log::error!("[WS-Event-Backfill] ❌ 敌方数据回填失败: {}", e);
                                }
                            });
                        }
                    } else if phase != "ChampSelect"
                        && phase != "ReadyCheck"
                        && phase != "Matchmaking"
                        && phase != "GameStart"
                    {
                        // 对于其他阶段切换（如返回大厅），清理所有缓存
                        if !cache.match_stats_cache.is_empty() {
                            log::info!("[WS-Event] 🗑️ 阶段切换到 {}，清理战绩缓存", phase);
                            cache.match_stats_cache.clear();
                        }
                        if cache.team_analysis_data.is_some() {
                            log::info!("[WS-Event] 🗑️ 阶段切换到 {}，清理分析数据缓存", phase);
                            cache.team_analysis_data = None;
                        }
                    }

                    cache.gameflow_phase = Some(phase.to_string());
                    let _ = self
                        .app
                        .emit("gameflow-phase-change", &Some(phase.to_string()));
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


    /// 🔥 新增：在 InProgress 阶段回填敌方队伍的详细战绩
    async fn backfill_enemy_team_data(&self) -> Result<(), String> {
        log::info!("[WS-Event-Backfill] 🚀 开始执行回填任务...");

        // 1. 获取 LiveClient 中的完整玩家列表
        // 增加重试逻辑，因为 LiveClient API 可能在游戏刚开始时还未完全就绪
        let live_players = {
            let mut attempts = 0;
            let max_attempts = 30; // 增加到30次，总共等待约1分钟
            loop {
                attempts += 1;
                match crate::lcu::liveclient::service::get_live_player_list().await {
                    Ok(players) if !players.is_empty() => {
                        log::info!("[WS-Event-Backfill] ✅ 成功获取 LiveClient 玩家列表");
                        break players;
                    }
                    Ok(_) => {
                        if attempts >= max_attempts {
                            return Err(
                                "多次尝试后仍无法获取 LiveClient 玩家列表: 返回了空玩家列表 (游戏加载中)".to_string()
                            );
                        }
                        log::warn!("[WS-Event-Backfill] ⚠️ LiveClient 返回空列表 (游戏加载中)，尝试 {}/{}...", attempts, max_attempts);
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                    Err(e) => {
                        if attempts >= max_attempts {
                            return Err(format!(
                                "多次尝试后仍无法获取 LiveClient 玩家列表: {}",
                                e
                            ));
                        }
                        log::warn!("[WS-Event-Backfill] ⚠️ 获取 LiveClient 玩家列表失败 (尝试 {}/{})，2秒后重试: {}", attempts, max_attempts, e);
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                }
            }
        };

        log::info!(
            "[WS-Event-Backfill] ✅ 成功获取 LiveClient 玩家列表，共 {} 人",
            live_players.len()
        );

        // 2. 获取缓存的分析数据
        let mut cache = self.cache.write().await;
        let team_analysis = match cache.team_analysis_data.as_mut() {
            Some(data) => data,
            None => {
                log::warn!("[WS-Event-Backfill] ⚠️ 缓存中没有 TeamAnalysisData，无法执行回填");
                return Ok(());
            }
        };

        // 3. 识别敌方玩家（CHAOS队伍）
        let enemy_live_players: Vec<_> = live_players
            .into_iter()
            .filter(|p| p.team == "CHAOS" && !p.is_bot && !p.summoner_name.is_empty())
            .collect();

        if enemy_live_players.is_empty() {
            log::info!("[WS-Event-Backfill] ✅ LiveClient 数据中没有需要处理的敌方玩家");
            return Ok(());
        }

        log::info!(
            "[WS-Event-Backfill] 🎯 找到 {} 名真实的敌方玩家，开始回填数据...",
            enemy_live_players.len()
        );

        // 4. 批量获取召唤师信息
        let player_names: Vec<String> = enemy_live_players
            .iter()
            .map(|p| p.summoner_name.clone())
            .collect();

        let summoners_info = match crate::lcu::summoner::service::get_summoners_by_names(
            &self.client,
            player_names.clone(),
        )
        .await
        {
            Ok(info) => {
                log::info!(
                    "[WS-Event-Backfill] ✅ 成功获取 {} 名敌方召唤师的详细信息",
                    info.len()
                );
                info
            }
            Err(e) => {
                log::error!(
                    "[WS-Event-Backfill] ❌ 批量获取敌方召唤师信息失败: {}",
                    e
                );
                return Ok(()); // 不中断流程，返回 OK
            }
        };

        // 5. 遍历 LiveClient 的敌方玩家，更新 team_analysis.enemy_team
        // 为了避免可变借用冲突，我们先收集需要缓存的战绩数据
        let mut stats_to_cache = Vec::new();

        for live_player in enemy_live_players {
            // 5.1 通过中文名查找英雄ID
            let champion_id = champion_data::get_champion_id_by_name(&live_player.champion_name);
            if champion_id.is_none() {
                log::warn!(
                    "[WS-Event-Backfill] ⚠️ 无法找到英雄 '{}' 的ID，跳过该玩家",
                    live_player.champion_name
                );
                continue;
            }
            let champion_id = champion_id.unwrap();

            // 5.2 在 team_analysis.enemy_team 中查找对应的玩家
            // 匹配规则：通过 championId 或 displayName
            let enemy_player = team_analysis.enemy_team.iter_mut().find(|p| {
                // 优先通过英雄ID匹配（最可靠）
                if let Some(p_champ_id) = p.champion_id {
                    if p_champ_id == champion_id {
                        return true;
                    }
                }
                // 备选：通过召唤师名称匹配
                p.display_name.to_lowercase() == live_player.summoner_name.to_lowercase()
            });

            let enemy_player = match enemy_player {
                Some(player) => player,
                None => {
                    log::warn!(
                        "[WS-Event-Backfill] ⚠️ 在缓存的敌方队伍中找不到玩家 '{}' (英雄: {})，跳过",
                        live_player.summoner_name,
                        live_player.champion_name
                    );
                    continue;
                }
            };

            // 5.3 更新玩家基础信息
            enemy_player.display_name = live_player.summoner_name.clone();
            enemy_player.champion_id = Some(champion_id);
            enemy_player.champion_name = Some(live_player.champion_name.clone());

            // 5.4 查找对应的召唤师信息
            let summoner_info = summoners_info.iter().find(|s| {
                let full_name = if let (Some(game_name), Some(tag_line)) =
                    (&s.game_name, &s.tag_line)
                {
                    format!("{}#{}", game_name, tag_line)
                } else {
                    s.display_name.clone()
                };
                full_name.to_lowercase() == live_player.summoner_name.to_lowercase()
            });

            if let Some(info) = summoner_info {
                // 5.5 更新段位、头像等信息
                enemy_player.puuid = Some(info.puuid.clone());
                enemy_player.tier = info.solo_rank_tier.clone();
                enemy_player.profile_icon_id = Some(info.profile_icon_id as i32);
                enemy_player.tag_line = info.tag_line.clone();

                // 5.6 获取战绩数据
                match crate::lcu::matches::service::get_recent_matches_by_puuid(
                    &self.client,
                    &info.puuid,
                    20,
                )
                .await
                {
                    Ok(match_stats) => {
                        let queue_id = team_analysis.queue_id;
                        let player_stats =
                            analysis_data::service::convert_match_statistics_to_player_stats(
                                match_stats,
                                &live_player.summoner_name,
                                queue_id,
                            );

                        // 先保存到临时列表，稍后批量插入缓存
                        stats_to_cache.push((live_player.summoner_name.clone(), player_stats.clone()));

                        enemy_player.match_stats = Some(player_stats);
                        log::info!(
                            "[WS-Event-Backfill] ✅ 成功回填玩家 '{}' 的完整数据",
                            live_player.summoner_name
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "[WS-Event-Backfill] ⚠️ 获取玩家 '{}' 战绩失败: {}",
                            live_player.summoner_name,
                            e
                        );
                    }
                }
            } else {
                log::warn!(
                    "[WS-Event-Backfill] ⚠️ 未找到召唤师 '{}' 的详细信息",
                    live_player.summoner_name
                );
            }
        }

        // 5.7 克隆更新后的数据（在释放 team_analysis 借用之前）
        let updated_data = team_analysis.clone();

        // 5.8 批量插入战绩缓存（现在可以安全地借用 cache 了）
        for (summoner_name, stats) in stats_to_cache {
            cache.match_stats_cache.insert(summoner_name, stats);
        }

        // 释放锁
        drop(cache);

        // 6. 发送更新后的完整数据到前端
        log::info!("[WS-Event-Backfill] 📡 发送已回填的完整对局分析数据到前端");
        let _ = self.app.emit("team-analysis-data", &updated_data);
        log::info!("[WS-Event-Backfill] ✅ 回填任务完成");

        Ok(())
    }

    async fn handle_gameflow_session_change(
        &self,
        data: &Value,
        event_type: &str,
    ) -> Result<(), String> {
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

    async fn handle_champ_select_change(
        &self,
        data: &Value,
        event_type: &str,
    ) -> Result<(), String> {
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
                &mut cache.match_stats_cache, // 传递战绩缓存
            )
            .await
            {
                Ok(analysis_data) => {
                    log::info!("[WS-Event] ✅ 分析数据生成成功");
                    log::debug!(
                        "[WS-Event] 我方玩家数: {}, 敌方玩家数: {}",
                        analysis_data.my_team.len(),
                        analysis_data.enemy_team.len()
                    );
                    log::info!(
                        "[WS-Event] 📊 当前缓存的战绩数: {}",
                        cache.match_stats_cache.len()
                    );

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
                    log::error!(
                        "[WS-Event] 错误来源: {}",
                        e.source()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "无".to_string())
                    );

                    // 降级：发送原始 champ-select-session 数据
                    log::warn!("[WS-Event] 🔄 尝试降级方案：发送原始会话数据");
                    match serde_json::from_value::<ChampSelectSession>(data.clone()) {
                        Ok(mut session) => {
                            log::debug!("[WS-Event] ✅ 降级成功，发送原始会话数据");
                            self.enrich_champ_select_session(&mut session).await;
                            let _ = self
                                .app
                                .emit("champ-select-session-changed", &Some(session));
                        }
                        Err(parse_err) => {
                            log::error!(
                                "[WS-Event] ❌ 降级方案也失败，解析选人会话数据失败: {}",
                                parse_err
                            );
                            log::error!(
                                "[WS-Event] Session 数据预览: {}",
                                serde_json::to_string(data)
                                    .unwrap_or_else(|_| "无法序列化".to_string())
                                    .chars()
                                    .take(200)
                                    .collect::<String>()
                            );
                        }
                    }
                }
            }
        } else if event_type == "Delete" {
            log::info!("[WS-Event] 🗑️ 选人会话清除，但保留分析数据用于回填");

            // 🔥 清理缓存
            let mut cache = self.cache.write().await;
            cache.champ_select_session = None;
            // 移除此行: cache.team_analysis_data = None;
            // 理由: 当阶段变为 InProgress 时，需要此数据来回填敌方战绩。
            // 清理工作由 handle_gameflow_phase_change 在游戏完全结束后负责。
            drop(cache);

            // 注意：不再向前端发送空的分析数据，因为我们期望 InProgress 阶段能使用它
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

    async fn handle_matchmaking_change(
        &self,
        data: &Value,
        event_type: &str,
    ) -> Result<(), String> {
        // Create 和 Update 都表示有数据
        if event_type == "Create" || event_type == "Update" {
            // 尝试解析匹配状态数据
            if let Ok(matchmaking_state) = serde_json::from_value::<MatchmakingState>(data.clone())
            {
                let mut cache = self.cache.write().await;
                if cache.matchmaking_state.as_ref() != Some(&matchmaking_state) {
                    log::info!(
                        "[WS-Event] 匹配状态更新 ({}): {:?}",
                        event_type,
                        matchmaking_state.search_state
                    );
                    cache.matchmaking_state = Some(matchmaking_state.clone());
                    let _ = self
                        .app
                        .emit("matchmaking-state-changed", matchmaking_state);
                } else {
                    log::debug!(
                        "[WS-Event] 匹配状态无变化，跳过发送: {:?}",
                        matchmaking_state.search_state
                    );
                }
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.matchmaking_state.is_some() {
                log::info!("[WS-Event] 匹配状态清除");
                cache.matchmaking_state = None;
                let _ = self
                    .app
                    .emit("matchmaking-state-changed", &None::<MatchmakingState>);
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
