// Handles converting raw WebSocket messages into application-specific events.
use crate::lcu::analysis_data;
use crate::lcu::champion_data;
use crate::lcu::summoner::service::get_summoner_by_id;
use crate::lcu::summoner_spells;
use crate::lcu::types::{ChampSelectSession, LobbyInfo, MatchmakingState, SummonerInfo};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

// Caches event data to avoid sending redundant information.
#[derive(Default)]
struct EventCache {
    gameflow_phase: Option<String>,
    gameflow_session: Option<String>, // Stores the session JSON string for comparison.
    champ_select_session: Option<ChampSelectSession>,
    matchmaking_state: Option<MatchmakingState>,
    lobby_info: Option<LobbyInfo>,
    // Cache for match statistics, keyed by summoner display name.
    match_stats_cache: std::collections::HashMap<String, crate::lcu::types::PlayerMatchStats>,
    // Cache for team analysis data.
    team_analysis_data: Option<crate::lcu::types::TeamAnalysisData>,
}

pub struct WsEventHandler {
    app: AppHandle,
    cache: Arc<RwLock<EventCache>>,
    client: Client, // HTTP client for fetching summoner information.
}

impl WsEventHandler {
    pub fn new(app: AppHandle) -> Self {
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

    /// Gets the cached team analysis data.
    pub async fn get_cached_team_analysis_data(&self) -> Option<crate::lcu::types::TeamAnalysisData> {
        let cache = self.cache.read().await;
        cache.team_analysis_data.clone()
    }

    /// Handles a raw WebSocket event message.
    pub async fn handle_event(&self, event_data: &str) -> Result<(), String> {
        if event_data.trim().is_empty() {
            return Err("Received empty event data".to_string());
        }

        let data: Value = serde_json::from_str(event_data).map_err(|e| {
            format!(
                "Failed to parse WebSocket event (length: {}, first 100 chars: '{}'): {}",
                event_data.len(),
                event_data.chars().take(100).collect::<String>(),
                e
            )
        })?;

        // Check for the standard WAMP event format: [8, "OnJsonApiEvent", payload]
        if let Some(event_array) = data.as_array() {
            if event_array.len() >= 3 {
                let message_type = event_array[0].as_u64();
                let event_name = event_array[1].as_str();
                let payload = &event_array[2];

                if message_type == Some(8) && event_name == Some("OnJsonApiEvent") {
                    // Process only the events we are interested in to reduce noise.
                    if self.is_important_event(payload) {
                        self.handle_json_api_event(payload).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Checks if the event is one of the critical events that need to be processed.
    fn is_important_event(&self, payload: &Value) -> bool {
        let uri = payload["uri"].as_str().unwrap_or("");

        matches!(
            uri,
            "/lol-gameflow/v1/gameflow-phase"
                | "/lol-champ-select/v1/session"
                | "/lol-lobby/v2/lobby"
                | "/lol-matchmaking/v1/search"
                | "/lol-gameflow/v1/session"
                | "/lol-summoner/v1/current-summoner"
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
            "/lol-summoner/v1/current-summoner" => {
                self.handle_current_summoner_change(data, event_type).await?;
            }
            "/lol-matchmaking/v1/search" => {
                self.handle_matchmaking_change(data, event_type).await?;
            }
            _ => {
                // Other events are logged at trace level but not processed.
                log::trace!("[ws-event] Unhandled event URI: {}", uri);
            }
        }

        Ok(())
    }

    async fn handle_gameflow_phase_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        log::info!("[ws-event] Gameflow phase change ({}) received: {}", event_type, data);
        if event_type == "Create" || event_type == "Update" {
            if let Some(phase) = data.as_str() {
                let mut cache = self.cache.write().await;
                if cache.gameflow_phase.as_ref() != Some(&phase.to_string()) {
                    // When entering 'InProgress', trigger backfill or build logic.
                    if phase == "InProgress" {
                        let has_cache = cache.team_analysis_data.is_some();

                        // Clone required fields to satisfy the 'static lifetime for the new task.
                        let app_clone = self.app.clone();
                        let cache_clone = Arc::clone(&self.cache);
                        let client_clone = self.client.clone();

                        tokio::spawn(async move {
                            // Reconstruct the handler in the new task to avoid lifetime issues.
                            let temp_handler = WsEventHandler {
                                app: app_clone,
                                cache: cache_clone,
                                client: client_clone,
                            };

                            if has_cache {
                                // 🔥 有缓存数据（正常流程：选人→游戏），只回填敌方队伍
                                log::info!("[ws-event] Cache available, starting enemy team backfill...");
                                if let Err(e) = temp_handler.backfill_enemy_team_data().await {
                                    log::error!("[ws-event-backfill] Failed to backfill enemy team data: {}", e);
                                }
                            } else {
                                // 🔥 没有缓存数据（应用重启或游戏中打开），主动构建完整数据
                                log::info!("[ws-event] No cache, building team data from scratch...");
                                if let Err(e) = temp_handler.build_team_data_from_scratch().await {
                                    log::error!("[ws-event-scratch] Failed to build team data from scratch: {}", e);
                                }
                            }
                        });
                    } else if phase == "Lobby"
                        || phase == "None"
                        || phase == "EndOfGame"
                        || phase == "WaitingForStats"
                        || phase == "PreEndOfGame"
                        || phase == "Terminated"
                    {
                        // ✅ 优化：游戏结束或返回大厅时，立即清空缓存释放内存
                        // 每把游戏的缓存独立，防止长期运行导致内存累积
                        let cache_size = cache.match_stats_cache.len();
                        if cache_size > 0 {
                            log::info!(
                                target: "ws::event_handler",
                                "Phase '{}': clearing match stats cache ({} players, ~{}KB released)",
                                phase,
                                cache_size,
                                cache_size * 10
                            );
                            cache.match_stats_cache.clear();
                        }
                        if cache.team_analysis_data.is_some() {
                            log::info!(
                                target: "ws::event_handler",
                                "Phase '{}': clearing team analysis data",
                                phase
                            );
                            cache.team_analysis_data = None;
                        }
                    }

                    cache.gameflow_phase = Some(phase.to_string());
                    let _ = self.app.emit("gameflow-phase-change", &Some(phase.to_string()));
                } else {
                    log::debug!("[ws-event] Gameflow phase unchanged, skipping broadcast: {}", phase);
                }
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.gameflow_phase.is_some() {
                log::info!("[ws-event] Gameflow phase cleared.");
                cache.gameflow_phase = None;
                let _ = self.app.emit("gameflow-phase-change", &None::<String>);
            }
        }
        Ok(())
    }

    /// Backfills detailed match history for the enemy team during the 'InProgress' phase.
    async fn backfill_enemy_team_data(&self) -> Result<(), String> {
        log::info!("[ws-event-backfill] Starting backfill task...");

        // 1. Fetch the full player list from the LiveClient API.
        // Retry logic is necessary as the LiveClient API may not be ready immediately at game start.
        let live_players = {
            let mut attempts = 0;
            let max_attempts = 30; // Increased to 30 attempts, total wait time ~1 minute.
            loop {
                attempts += 1;
                match crate::lcu::liveclient::service::get_live_player_list().await {
                    Ok(players) if !players.is_empty() => {
                        log::info!("[ws-event-backfill] Successfully fetched player list from LiveClient.");
                        break players;
                    }
                    Ok(_) => {
                        if attempts >= max_attempts {
                            return Err(
                                "Failed to get player list from LiveClient after multiple attempts: returned an empty list (game loading?).".to_string(),
                            );
                        }
                        log::warn!(
                            "[ws-event-backfill] LiveClient returned an empty list (game loading?), attempt {}/{}...",
                            attempts,
                            max_attempts
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                    Err(e) => {
                        if attempts >= max_attempts {
                            return Err(format!(
                                "Failed to get player list from LiveClient after {} attempts: {}",
                                max_attempts, e
                            ));
                        }
                        log::warn!(
                            "[ws-event-backfill] Failed to fetch LiveClient player list (attempt {}/{}), retrying in 2s: {}",
                            attempts,
                            max_attempts,
                            e
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                }
            }
        };

        log::info!(
            "[ws-event-backfill] Found {} players in LiveClient data.",
            live_players.len()
        );

        // 2. Get the cached analysis data.
        let mut cache = self.cache.write().await;
        let team_analysis = match cache.team_analysis_data.as_mut() {
            Some(data) => data,
            None => {
                log::warn!("[ws-event-backfill] No TeamAnalysisData in cache, cannot perform backfill.");
                return Ok(());
            }
        };

        // 3. Identify enemy players (CHAOS team).
        let enemy_live_players: Vec<_> = live_players
            .into_iter()
            .filter(|p| p.team == "CHAOS" && !p.is_bot && !p.summoner_name.is_empty())
            .collect();

        if enemy_live_players.is_empty() {
            log::info!("[ws-event-backfill] No enemy players found in LiveClient data to process.");
            return Ok(());
        }

        log::info!(
            "[ws-event-backfill] Found {} real enemy players, starting data backfill...",
            enemy_live_players.len()
        );

        // 4. Batch fetch summoner info.
        let player_names: Vec<String> = enemy_live_players.iter().map(|p| p.summoner_name.clone()).collect();

        let summoners_info =
            match crate::lcu::summoner::service::get_summoners_by_names(&self.client, player_names.clone()).await {
                Ok(info) => {
                    log::info!(
                        "[ws-event-backfill] Successfully fetched details for {} enemy summoners.",
                        info.len()
                    );
                    info
                }
                Err(e) => {
                    log::error!(
                    "[ws-event-backfill] Batch fetch for enemy summoner info failed: {}. Proceeding without this data.",
                    e
                );
                    return Ok(()); // Do not interrupt the flow, just log the error.
                }
            };

        // 5. Iterate through LiveClient enemy players to update team_analysis.enemy_team.
        // We collect stats to be cached separately to avoid mutable borrow conflicts.
        let mut stats_to_cache = Vec::new();

        for live_player in enemy_live_players {
            // 5.1 Find champion ID by name.
            let champion_id = champion_data::get_champion_id_by_name(&live_player.champion_name);
            if champion_id.is_none() {
                log::warn!(
                    "[ws-event-backfill] Could not find champion ID for '{}', skipping player.",
                    live_player.champion_name
                );
                continue;
            }
            let champion_id = champion_id.unwrap();

            // 5.2 Find the corresponding player in team_analysis.enemy_team.
            // Match by championId (most reliable) or displayName as a fallback.
            let enemy_player = team_analysis.enemy_team.iter_mut().find(|p| {
                if let Some(p_champ_id) = p.champion_id {
                    if p_champ_id == champion_id {
                        return true;
                    }
                }
                p.display_name.to_lowercase() == live_player.summoner_name.to_lowercase()
            });

            let enemy_player = match enemy_player {
                Some(player) => player,
                None => {
                    log::warn!(
                        "[ws-event-backfill] Could not find player '{}' (champion: {}) in cached enemy team, skipping.",
                        live_player.summoner_name,
                        live_player.champion_name
                    );
                    continue;
                }
            };

            // 5.3 Update basic player info.
            enemy_player.display_name = live_player.summoner_name.clone();
            enemy_player.champion_id = Some(champion_id);
            enemy_player.champion_name = Some(live_player.champion_name.clone());

            // 5.3.1 解析并转换召唤师技能（从中文名转为 ID）
            if let Some(spells) = live_player.summoner_spells.as_object() {
                // 技能1
                if let Some(spell_one) = spells.get("summonerSpellOne") {
                    if let Some(spell_name) = spell_one.get("displayName").and_then(|v| v.as_str()) {
                        if let Some(spell_id) = summoner_spells::get_spell_id_by_name(spell_name) {
                            enemy_player.spell1_id = Some(spell_id);
                            log::debug!(
                                "[ws-event-backfill] 转换召唤师技能1: '{}' -> ID {}",
                                spell_name,
                                spell_id
                            );
                        }
                    }
                }

                // 技能2
                if let Some(spell_two) = spells.get("summonerSpellTwo") {
                    if let Some(spell_name) = spell_two.get("displayName").and_then(|v| v.as_str()) {
                        if let Some(spell_id) = summoner_spells::get_spell_id_by_name(spell_name) {
                            enemy_player.spell2_id = Some(spell_id);
                            log::debug!(
                                "[ws-event-backfill] 转换召唤师技能2: '{}' -> ID {}",
                                spell_name,
                                spell_id
                            );
                        }
                    }
                }
            }

            // 5.4 Find the corresponding summoner info.
            let summoner_info = summoners_info.iter().find(|s| {
                let full_name = if let (Some(game_name), Some(tag_line)) = (&s.game_name, &s.tag_line) {
                    format!("{}#{}", game_name, tag_line)
                } else {
                    s.display_name.clone()
                };
                full_name.to_lowercase() == live_player.summoner_name.to_lowercase()
            });

            if let Some(info) = summoner_info {
                // 5.5 Update rank, icon, etc.
                enemy_player.puuid = Some(info.puuid.clone());
                enemy_player.tier = info.solo_rank_tier.clone();
                enemy_player.profile_icon_id = Some(info.profile_icon_id as i32);
                enemy_player.tag_line = info.tag_line.clone();

                // 5.6 Fetch recent matches.
                let queue_id = Some(team_analysis.queue_id);
                match crate::lcu::matches::service::get_recent_matches_by_puuid(&self.client, &info.puuid, 20, queue_id).await {
                    Ok(player_stats) => {
                        // 注意：get_recent_matches_by_puuid 已经返回完整的 PlayerMatchStats
                        // 包含所有增强字段（traits, today_games, dpm, cspm, vspm 等）
                        // 在排位模式下会自动过滤只显示排位战绩

                        // Defer caching to avoid borrow conflicts.
                        stats_to_cache.push((live_player.summoner_name.clone(), player_stats.clone()));

                        enemy_player.match_stats = Some(player_stats);

                        if enemy_player.is_bot {
                            enemy_player.is_bot = false;
                        }

                        log::info!(
                            "[ws-event-backfill] Successfully backfilled full data for player '{}'.",
                            live_player.summoner_name
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "[ws-event-backfill] Failed to get match history for player '{}': {}",
                            live_player.summoner_name,
                            e
                        );
                    }
                }
            } else {
                log::warn!(
                    "[ws-event-backfill] Could not find detailed summoner info for '{}'.",
                    live_player.summoner_name
                );
            }
        }

        // Clone the updated data before releasing the lock on team_analysis.
        let updated_data = team_analysis.clone();

        // Batch insert match stats into the cache now that the borrow on team_analysis is over.
        for (summoner_name, stats) in stats_to_cache {
            cache.match_stats_cache.insert(summoner_name, stats);
        }

        drop(cache);

        // 6. Emit the updated data to the frontend.
        log::info!("[ws-event-backfill] Emitting complete, backfilled analysis data to frontend.");
        let _ = self.app.emit("team-analysis-data", &updated_data);
        log::info!("[ws-event-backfill] Backfill task completed.");

        Ok(())
    }

    /// 从头构建队伍分析数据（应用重启后没有缓存时使用）
    /// 同时处理我方和敌方队伍的数据
    async fn build_team_data_from_scratch(&self) -> Result<(), String> {
        log::info!(
            target: "ws::event_handler",
            "Building team data from LiveClient (app restart recovery)"
        );

        // 1. 获取 LiveClient 玩家列表（带重试）
        let live_players = {
            let mut attempts = 0;
            let max_attempts = 30;
            loop {
                attempts += 1;
                match crate::lcu::liveclient::service::get_live_player_list().await {
                    Ok(players) if !players.is_empty() => {
                        log::debug!(
                            target: "ws::event_handler",
                            "Fetched {} players from LiveClient",
                            players.len()
                        );
                        break players;
                    }
                    Ok(_) => {
                        if attempts >= max_attempts {
                            return Err(
                                "LiveClient returned empty list after max retries (game still loading?)".to_string()
                            );
                        }
                        log::debug!(
                            target: "ws::event_handler",
                            "LiveClient empty, retrying {}/{}",
                            attempts,
                            max_attempts
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                    Err(e) => {
                        if attempts >= max_attempts {
                            return Err(format!(
                                "Failed to get LiveClient data after {} attempts: {}",
                                max_attempts, e
                            ));
                        }
                        log::warn!(
                            target: "ws::event_handler",
                            "LiveClient fetch failed (attempt {}/{}): {}, retrying",
                            attempts,
                            max_attempts,
                            e
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                }
            }
        };

        log::debug!(
            target: "ws::event_handler",
            "Found {} players in LiveClient data",
            live_players.len()
        );

        // 2. 分离我方（ORDER）和敌方（CHAOS）队伍
        let my_team_players: Vec<_> = live_players
            .iter()
            .filter(|p| p.team == "ORDER" && !p.is_bot && !p.summoner_name.is_empty())
            .collect();

        let enemy_team_players: Vec<_> = live_players
            .iter()
            .filter(|p| p.team == "CHAOS" && !p.is_bot && !p.summoner_name.is_empty())
            .collect();

        log::debug!(
            target: "ws::event_handler",
            "Team split: {} my team, {} enemy team",
            my_team_players.len(),
            enemy_team_players.len()
        );

        // 3. 收集所有玩家名称用于批量获取召唤师信息
        let all_player_names: Vec<String> = live_players
            .iter()
            .filter(|p| !p.is_bot && !p.summoner_name.is_empty())
            .map(|p| p.summoner_name.clone())
            .collect();

        let summoners_info =
            match crate::lcu::summoner::service::get_summoners_by_names(&self.client, all_player_names.clone()).await {
                Ok(info) => {
                    log::debug!(
                        target: "ws::event_handler",
                        "Fetched {} summoner details",
                        info.len()
                    );
                    info
                }
                Err(e) => {
                    log::error!(
                        target: "ws::event_handler",
                        "Batch fetch for summoner info failed: {}",
                        e
                    );
                    return Err(format!("Failed to fetch summoner info: {}", e));
                }
            };

        // 4. 先获取游戏信息（需要用于战绩过滤）
        let (queue_id, is_custom_game) = match crate::lcu::gameflow::service::get_gameflow_session(&self.client).await {
            Ok(session) => {
                let queue_id = session["gameData"]["queue"]["id"].as_i64().unwrap_or(420);
                let is_custom = session["gameData"]["isCustomGame"].as_bool().unwrap_or(false);
                log::debug!(
                    target: "ws::event_handler",
                    "Got game info from API: queue_id={}, is_custom={}",
                    queue_id,
                    is_custom
                );
                (queue_id, is_custom)
            }
            Err(e) => {
                log::warn!(
                    target: "ws::event_handler",
                    "Failed to get gameflow session: {}, using defaults",
                    e
                );
                (420, false)
            }
        };

        // 5. 构建我方队伍数据
        let mut my_team_data = Vec::new();
        let mut stats_to_cache = Vec::new();

        for (idx, live_player) in my_team_players.iter().enumerate() {
            match self
                .build_player_data(live_player, idx as i32, &summoners_info, &mut stats_to_cache, queue_id)
                .await
            {
                Ok(player_data) => my_team_data.push(player_data),
                Err(e) => {
                    log::warn!(
                        target: "ws::event_handler",
                        "Failed to build data for my team player '{}': {}",
                        live_player.summoner_name,
                        e
                    );
                }
            }
        }

        // 6. 构建敌方队伍数据
        let mut enemy_team_data = Vec::new();

        for (idx, live_player) in enemy_team_players.iter().enumerate() {
            match self
                .build_player_data(
                    live_player,
                    (idx + 100) as i32,
                    &summoners_info,
                    &mut stats_to_cache,
                    queue_id,
                )
                .await
            {
                Ok(player_data) => enemy_team_data.push(player_data),
                Err(e) => {
                    log::warn!(
                        target: "ws::event_handler",
                        "Failed to build data for enemy team player '{}': {}",
                        live_player.summoner_name,
                        e
                    );
                }
            }
        }

        // 7. 识别本地玩家并设置 is_local 标志
        let local_player_cell_id = match crate::lcu::summoner::service::get_current_summoner(&self.client).await {
            Ok(summoner) => {
                // 构建完整召唤师名称（GameName#TagLine 或 DisplayName）
                let local_name = if let (Some(game_name), Some(tag_line)) = (&summoner.game_name, &summoner.tag_line) {
                    format!("{}#{}", game_name, tag_line)
                } else {
                    summoner.display_name.clone()
                };

                // 在我方队伍中查找本地玩家并设置 is_local = true
                my_team_data
                    .iter_mut()
                    .find(|p| p.display_name.to_lowercase() == local_name.to_lowercase())
                    .map(|p| {
                        p.is_local = true;
                        log::debug!(
                            target: "ws::event_handler",
                            "Found local player '{}' at cell_id={}",
                            p.display_name,
                            p.cell_id
                        );
                        p.cell_id
                    })
                    .unwrap_or_else(|| {
                        log::warn!(
                            target: "ws::event_handler",
                            "Could not find local player '{}', using default cell_id=0",
                            local_name
                        );
                        0
                    })
            }
            Err(e) => {
                log::warn!(
                    target: "ws::event_handler",
                    "Failed to get current summoner: {}, using default cell_id=0",
                    e
                );
                0
            }
        };

        // 8. 创建 TeamAnalysisData
        let team_analysis_data = crate::lcu::types::TeamAnalysisData {
            my_team: my_team_data,
            enemy_team: enemy_team_data,
            local_player_cell_id,
            game_phase: "InProgress".to_string(),
            queue_id,
            is_custom_game,
            actions: None,
            bans: None,
            timer: None,
        };

        // 9. 缓存数据
        let mut cache = self.cache.write().await;

        // 批量插入战绩缓存
        for (summoner_name, stats) in stats_to_cache {
            cache.match_stats_cache.insert(summoner_name, stats);
        }

        cache.team_analysis_data = Some(team_analysis_data.clone());
        log::debug!(
            target: "ws::event_handler",
            "TeamAnalysisData cached, match_stats_cache size: {}",
            cache.match_stats_cache.len()
        );

        drop(cache);

        // 10. 发送到前端
        let _ = self.app.emit("team-analysis-data", &team_analysis_data);
        log::info!(
            target: "ws::event_handler",
            "Build from scratch completed: my_team={}, enemy_team={}",
            team_analysis_data.my_team.len(),
            team_analysis_data.enemy_team.len()
        );

        Ok(())
    }

    /// 辅助方法：从 LiveClient 玩家数据构建 PlayerAnalysisData
    async fn build_player_data(
        &self,
        live_player: &crate::lcu::types::LiveClientPlayer,
        cell_id: i32,
        summoners_info: &[crate::lcu::types::SummonerInfo],
        stats_to_cache: &mut Vec<(String, crate::lcu::types::PlayerMatchStats)>,
        queue_id: i64,
    ) -> Result<crate::lcu::types::PlayerAnalysisData, String> {
        // 获取英雄 ID
        let champion_id = champion_data::get_champion_id_by_name(&live_player.champion_name)
            .ok_or_else(|| format!("Could not find champion ID for '{}'", live_player.champion_name))?;

        // 查找召唤师信息
        let summoner_info = summoners_info.iter().find(|s| {
            let full_name = if let (Some(game_name), Some(tag_line)) = (&s.game_name, &s.tag_line) {
                format!("{}#{}", game_name, tag_line)
            } else {
                s.display_name.clone()
            };
            full_name.to_lowercase() == live_player.summoner_name.to_lowercase()
        });

        let mut player_data = crate::lcu::types::PlayerAnalysisData {
            cell_id,
            display_name: live_player.summoner_name.clone(),
            summoner_id: None,
            puuid: None,
            is_local: false,
            is_bot: false,
            champion_id: Some(champion_id),
            champion_name: Some(live_player.champion_name.clone()),
            champion_pick_intent: None,
            position: None,
            tier: None,
            profile_icon_id: None,
            tag_line: None,
            spell1_id: None,
            spell2_id: None,
            match_stats: None,
        };

        // 解析召唤师技能
        if let Some(spells) = live_player.summoner_spells.as_object() {
            if let Some(spell_one) = spells.get("summonerSpellOne") {
                if let Some(spell_name) = spell_one.get("displayName").and_then(|v| v.as_str()) {
                    if let Some(spell_id) = summoner_spells::get_spell_id_by_name(spell_name) {
                        player_data.spell1_id = Some(spell_id);
                    }
                }
            }

            if let Some(spell_two) = spells.get("summonerSpellTwo") {
                if let Some(spell_name) = spell_two.get("displayName").and_then(|v| v.as_str()) {
                    if let Some(spell_id) = summoner_spells::get_spell_id_by_name(spell_name) {
                        player_data.spell2_id = Some(spell_id);
                    }
                }
            }
        }

        // 填充召唤师详细信息
        if let Some(info) = summoner_info {
            player_data.puuid = Some(info.puuid.clone());
            player_data.tier = info.solo_rank_tier.clone();
            player_data.profile_icon_id = Some(info.profile_icon_id as i32);
            player_data.tag_line = info.tag_line.clone();

            // 获取战绩数据
            match crate::lcu::matches::service::get_recent_matches_by_puuid(&self.client, &info.puuid, 20, Some(queue_id)).await {
                Ok(player_stats) => {
                    // 注意：get_recent_matches_by_puuid 已经返回完整的 PlayerMatchStats
                    // 在排位模式下会自动过滤只显示排位战绩

                    stats_to_cache.push((live_player.summoner_name.clone(), player_stats.clone()));
                    player_data.match_stats = Some(player_stats);

                    log::debug!(
                        target: "ws::event_handler",
                        "Fetched match data for player '{}'",
                        live_player.summoner_name
                    );
                }
                Err(e) => {
                    log::warn!(
                        target: "ws::event_handler",
                        "Failed to get match history for player '{}': {}",
                        live_player.summoner_name,
                        e
                    );
                }
            }
        } else {
            log::warn!(
                target: "ws::event_handler",
                "Could not find detailed summoner info for '{}'",
                live_player.summoner_name
            );
        }

        Ok(player_data)
    }

    async fn handle_gameflow_session_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        // Create and Update events both contain data.
        if event_type == "Create" || event_type == "Update" {
            // Serialize session data to a string for comparison.
            let session_json = serde_json::to_string(data).unwrap_or_default();

            let mut cache = self.cache.write().await;

            let session_changed = cache.gameflow_session.as_ref() != Some(&session_json);

            if session_changed {
                cache.gameflow_session = Some(session_json);

                // Drop the lock before emitting to avoid holding it during the emit.
                drop(cache);
                let _ = self.app.emit("gameflow-session-changed", data);

                log::debug!("[ws-event] Gameflow session updated and broadcast ({}).", event_type);
            } else {
                drop(cache);
                log::trace!("[ws-event] Gameflow session unchanged, skipping broadcast.");
            }

            // Note: We don't emit a gameflow-phase-change event here because
            // /lol-gameflow/v1/gameflow-phase triggers its own handler. This prevents duplicate events.

            #[cfg(debug_assertions)]
            {
                log::debug!("[ws-event] Gameflow Session Details:");
                log::debug!("  Phase: {:?}", data["phase"]);
                log::debug!("  Map: {:?}", data["map"]["name"]);
                log::debug!("  GameData: {:?}", data["gameData"]["queue"]);
            }
        } else if event_type == "Delete" {
            log::info!("[ws-event] Gameflow session cleared.");

            let mut cache = self.cache.write().await;
            cache.gameflow_session = None;
            drop(cache);

            // Only emit the session deletion event.
            // Phase deletion is handled by handle_gameflow_phase_change.
            let _ = self.app.emit("gameflow-session-changed", &None::<Value>);
        }
        Ok(())
    }

    async fn handle_champ_select_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        log::info!("[ws-event] Champ select event received, type: {}", event_type);

        if event_type == "Create" || event_type == "Update" {
            // Step 1: Immediately send raw session data for fast auto-pick response.
            log::info!("[ws-event] Sending raw champ-select-session-changed event (immediate)");
            let _ = self.app.emit("champ-select-session-changed", data);

            // Step 2: Asynchronously generate full analysis data with match stats.
            let app = self.app.clone();
            let client = self.client.clone();
            let cache_clone = self.cache.clone();
            let data_clone = data.clone();

            tokio::spawn(async move {
                let mut cache = cache_clone.write().await;
                match analysis_data::service::build_team_analysis_from_session(
                    &data_clone,
                    &client,
                    &mut cache.match_stats_cache,
                )
                .await
                {
                    Ok(enriched_data) => {
                        log::info!("[ws-event] Successfully generated enriched team analysis data (with match stats).");
                        log::debug!(
                            "[ws-event] My team size: {}, Enemy team size: {}",
                            enriched_data.my_team.len(),
                            enriched_data.enemy_team.len()
                        );
                        log::info!(
                            "[ws-event] Current cached match stats count: {}",
                            cache.match_stats_cache.len()
                        );

                        // Cache the enriched analysis data.
                        cache.team_analysis_data = Some(enriched_data.clone());
                        log::info!("[ws-event] Enriched TeamAnalysisData has been cached.");

                        // Drop the lock before emitting.
                        drop(cache);

                        // Send enriched data (this will update the UI with match stats).
                        let _ = app.emit("team-analysis-data", &enriched_data);
                    }
                    Err(e) => {
                        log::error!("[ws-event] Failed to generate enriched team analysis data: {}", e);
                        if let Some(source) = e.source() {
                            log::error!("[ws-event] Caused by: {}", source);
                        }
                        log::warn!("[ws-event] Session data already sent, match stats will be unavailable");
                    }
                }
            });
        } else if event_type == "Delete" {
            log::info!("[ws-event] Champ select session cleared, but preserving analysis data for backfill.");

            let mut cache = self.cache.write().await;
            cache.champ_select_session = None;
            // The team_analysis_data is intentionally not cleared here.
            // It is needed for the backfill process when the game phase changes to 'InProgress'.
            // Cleanup is handled by handle_gameflow_phase_change after the game ends.
            drop(cache);

            // Send session deletion event to frontend.
            let _ = self.app.emit("champ-select-session-changed", &None::<Value>);
        }
        Ok(())
    }

    async fn handle_lobby_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        log::info!("[ws-event] Lobby info event received, data: {}", data);
        if event_type == "Create" || event_type == "Update" {
            if let Ok(lobby) = serde_json::from_value::<LobbyInfo>(data.clone()) {
                let mut cache = self.cache.write().await;
                if cache.lobby_info.as_ref() != Some(&lobby) {
                    log::info!("[ws-event] Lobby info updated ({}).", event_type);
                    cache.lobby_info = Some(lobby.clone());
                    let _ = self.app.emit("lobby-change", &Some(lobby));
                } else {
                    log::debug!("[ws-event] Lobby info unchanged, skipping broadcast.");
                }
            } else {
                log::warn!("[ws-event] Failed to parse lobby info, skipping.");
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.lobby_info.is_some() {
                log::info!("[ws-event] Left lobby.");
                cache.lobby_info = None;
                let _ = self.app.emit("lobby-change", &None::<LobbyInfo>);
            }
        }
        Ok(())
    }

    async fn handle_current_summoner_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        log::info!(
            target: "ws::event_handler",
            "🧪 [EXPERIMENTAL] Received current-summoner event, type: {}, testing if LCU supports this subscription",
            event_type
        );

        if event_type == "Create" || event_type == "Update" {
            // 尝试解析召唤师信息
            if let Ok(summoner) = serde_json::from_value::<SummonerInfo>(data.clone()) {
                log::info!(
                    target: "ws::event_handler",
                    "✅ [EXPERIMENTAL] LCU supports current-summoner subscription! Summoner: {}, Level: {}",
                    summoner.display_name,
                    summoner.summoner_level
                );

                // 发送到前端
                let _ = self.app.emit("summoner-change", &Some(summoner));
            } else {
                log::warn!(
                    target: "ws::event_handler",
                    "⚠️ [EXPERIMENTAL] Failed to parse summoner data from WebSocket event"
                );
            }
        } else if event_type == "Delete" {
            log::info!(
                target: "ws::event_handler",
                "🧪 [EXPERIMENTAL] Summoner data cleared"
            );
            let _ = self.app.emit("summoner-change", &None::<SummonerInfo>);
        }

        Ok(())
    }

    async fn handle_matchmaking_change(&self, data: &Value, event_type: &str) -> Result<(), String> {
        if event_type == "Create" || event_type == "Update" {
            if let Ok(matchmaking_state) = serde_json::from_value::<MatchmakingState>(data.clone()) {
                let mut cache = self.cache.write().await;
                if cache.matchmaking_state.as_ref() != Some(&matchmaking_state) {
                    log::info!(
                        "[ws-event] Matchmaking state updated ({}) to: {:?}",
                        event_type,
                        matchmaking_state.search_state
                    );
                    cache.matchmaking_state = Some(matchmaking_state.clone());
                    let _ = self.app.emit("matchmaking-state-changed", matchmaking_state);
                } else {
                    log::debug!(
                        "[ws-event] Matchmaking state unchanged, skipping broadcast: {:?}",
                        matchmaking_state.search_state
                    );
                }
            }
        } else if event_type == "Delete" {
            let mut cache = self.cache.write().await;
            if cache.matchmaking_state.is_some() {
                log::info!("[ws-event] Matchmaking state cleared.");
                cache.matchmaking_state = None;
                let _ = self.app.emit("matchmaking-state-changed", &None::<MatchmakingState>);
            }
        }
        Ok(())
    }

    /// Enriches the champ select session with full summoner details.
    async fn enrich_champ_select_session(&self, session: &mut ChampSelectSession) {
        // Collect all unique, non-bot summoner IDs that need to be fetched.
        let mut all_ids = vec![];
        for p in session.my_team.iter().chain(session.their_team.iter()) {
            if let Some(sid) = &p.summoner_id {
                if sid != "0" && !all_ids.contains(sid) {
                    all_ids.push(sid.clone());
                }
            }
        }

        // Batch fetch summoner information.
        let mut info_map = std::collections::HashMap::new();
        for sid in &all_ids {
            if let Ok(id) = sid.parse::<u64>() {
                match get_summoner_by_id(&self.client, id).await {
                    Ok(info) => {
                        info_map.insert(sid.clone(), info);
                    }
                    Err(e) => {
                        log::debug!("[ws-event] Failed to get summoner info for ID {}: {}", sid, e);
                    }
                }
            }
        }

        // Enrich my_team.
        for p in session.my_team.iter_mut() {
            Self::enrich_player(p, &info_map);
        }

        // Enrich their_team.
        for p in session.their_team.iter_mut() {
            Self::enrich_player(p, &info_map);
        }
    }

    /// Enriches a single player's information using the fetched data.
    fn enrich_player(
        player: &mut crate::lcu::types::ChampSelectPlayer,
        info_map: &std::collections::HashMap<String, SummonerInfo>,
    ) {
        if let Some(sid) = &player.summoner_id {
            if sid == "0" {
                // Bot player
                player.display_name = Some("Bot".to_string());
                player.tag_line = None;
                player.profile_icon_id = None;
                player.tier = None;
            } else if let Some(info) = info_map.get(sid) {
                // Real player: prefer game_name + tag_line for the display name.
                let display_name = if let (Some(game_name), Some(tag_line)) = (&info.game_name, &info.tag_line) {
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
