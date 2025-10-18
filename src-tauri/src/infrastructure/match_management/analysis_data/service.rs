use crate::lcu::matches::service::{get_recent_matches_by_puuid, get_recent_matches_by_puuid_with_perspective};
use crate::infrastructure::data_services::summoner::service::get_summoner_by_id;
use crate::infrastructure::data_services::summoner::service::get_summoners_by_names;
use crate::lcu::types::{PlayerAnalysisData, PlayerMatchStats, TeamAnalysisData, AdvicePerspective};

/// 从 ChampSelect 会话构建完整的分析数据
///
/// 核心业务逻辑：
/// 1. 解析选人会话，提取玩家基础信息
/// 2. 批量获取召唤师详细信息（enrichment）
/// 3. 批量获取战绩数据（使用缓存优化，避免重复请求）
/// 4. 生成完整的 PlayerAnalysisData
pub async fn build_team_analysis_from_session(
    session: &serde_json::Value,
    http_client: &reqwest::Client,
    match_stats_cache: &mut std::collections::HashMap<String, PlayerMatchStats>,
) -> Result<TeamAnalysisData, Box<dyn std::error::Error + Send + Sync>> {
    let local_player_cell_id = session["localPlayerCellId"].as_i64().unwrap_or(0) as i32;
    let queue_id = session["queueId"].as_i64().unwrap_or(0);
    let is_custom_game = session["isCustomGame"].as_bool().unwrap_or(false);

    log::info!(
        target: "analysis_data::service",
        "Building team analysis data: localPlayerCellId={}, queueId={}, isCustom={}",
        local_player_cell_id,
        queue_id,
        is_custom_game
    );

    if is_custom_game {
        log::debug!(
            target: "analysis_data::service",
            "Custom game detected, some players may be bots"
        );
    }

    // 解析我方队伍
    let mut my_team_players = Vec::new();
    if let Some(my_team) = session["myTeam"].as_array() {
        log::debug!(
            target: "analysis_data::service",
            "Parsing my team, player count: {}",
            my_team.len()
        );
        for (idx, player) in my_team.iter().enumerate() {
            match parse_player_from_session(player, local_player_cell_id).await {
                Ok(mut player_data) => {
                    log::debug!(
                        target: "analysis_data::service",
                        "Parsed my team player[{}]: displayName='{}', isBot={}, cellId={}",
                        idx,
                        player_data.display_name,
                        player_data.is_bot,
                        player_data.cell_id
                    );

                    match enrich_player_data(&mut player_data, player, http_client).await {
                        Ok(_) => {
                            log::debug!(
                                target: "analysis_data::service",
                                "Enriched my team player[{}]: tier={:?}",
                                idx,
                                player_data.tier
                            );
                            my_team_players.push(player_data);
                        }
                        Err(e) => {
                            log::warn!(
                                target: "analysis_data::service",
                                "Failed to enrich my team player[{}]: {}, continuing with basic data",
                                idx,
                                e
                            );
                            my_team_players.push(player_data);
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        target: "analysis_data::service",
                        "Failed to parse my team player[{}]: {}, skipping",
                        idx,
                        e
                    );
                    continue;
                }
            }
        }
    } else {
        log::warn!(
            target: "analysis_data::service",
            "Session missing 'myTeam' array"
        );
    }

    // 解析敌方队伍
    let mut enemy_team_players = Vec::new();
    if let Some(their_team) = session["theirTeam"].as_array() {
        log::debug!(
            target: "analysis_data::service",
            "Parsing enemy team, player count: {}",
            their_team.len()
        );
        for (idx, player) in their_team.iter().enumerate() {
            match parse_player_from_session(player, local_player_cell_id).await {
                Ok(mut player_data) => {
                    log::debug!(
                        target: "analysis_data::service",
                        "Parsed enemy team player[{}]: displayName='{}', isBot={}, cellId={}",
                        idx,
                        player_data.display_name,
                        player_data.is_bot,
                        player_data.cell_id
                    );

                    match enrich_player_data(&mut player_data, player, http_client).await {
                        Ok(_) => {
                            log::debug!(
                                target: "analysis_data::service",
                                "Enriched enemy team player[{}]: tier={:?}",
                                idx,
                                player_data.tier
                            );
                            enemy_team_players.push(player_data);
                        }
                        Err(e) => {
                            log::warn!(
                                target: "analysis_data::service",
                                "Failed to enrich enemy team player[{}]: {}, continuing with basic data",
                                idx,
                                e
                            );
                            enemy_team_players.push(player_data);
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        target: "analysis_data::service",
                        "Failed to parse enemy team player[{}]: {}, skipping",
                        idx,
                        e
                    );
                    continue;
                }
            }
        }
    } else {
        log::warn!(
            target: "analysis_data::service",
            "Session missing 'theirTeam' array"
        );
    }

    // ⭐ 分别为我方和敌方获取战绩，使用不同的建议视角
    
    // 1. 获取我方队友战绩（使用 Collaboration 视角）
    let my_team_real_players: Vec<_> = my_team_players
        .iter_mut()
        .filter(|p| !p.is_bot && !p.display_name.is_empty() && p.display_name != "未知召唤师")
        .collect();

    if !my_team_real_players.is_empty() {
        log::info!(
            target: "analysis_data::service",
            "Fetching match stats for {} ally players with Collaboration perspective",
            my_team_real_players.len()
        );
        match fetch_players_match_stats_with_perspective(
            my_team_real_players,
            http_client,
            queue_id,
            match_stats_cache,
            AdvicePerspective::Collaboration,  // ⭐ 队友视角
            local_player_cell_id,
        ).await {
            Ok(count) => log::info!("Ally team stats: {} fetched", count),
            Err(e) => log::warn!("Failed to fetch ally stats: {}", e),
        }
    }

    // 2. 获取敌方战绩（使用 Targeting 视角）
    let enemy_real_players: Vec<_> = enemy_team_players
        .iter_mut()
        .filter(|p| !p.is_bot && !p.display_name.is_empty() && p.display_name != "未知召唤师")
        .collect();

    if !enemy_real_players.is_empty() {
        log::info!(
            target: "analysis_data::service",
            "Fetching match stats for {} enemy players with Targeting perspective",
            enemy_real_players.len()
        );
        match fetch_players_match_stats_with_perspective(
            enemy_real_players,
            http_client,
            queue_id,
            match_stats_cache,
            AdvicePerspective::Targeting,  // ⭐ 针对敌人视角
            local_player_cell_id,
        ).await {
            Ok(count) => log::info!("Enemy team stats: {} fetched", count),
            Err(e) => log::warn!("Failed to fetch enemy stats: {}", e),
        }
    }

    // 提取 actions、bans、timer
    let actions = session
        .get("actions")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    let bans = session.get("bans").and_then(|v| serde_json::from_value(v.clone()).ok());

    let timer = session
        .get("timer")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    Ok(TeamAnalysisData {
        my_team: my_team_players,
        enemy_team: enemy_team_players,
        local_player_cell_id,
        game_phase: "ChampSelect".to_string(),
        queue_id,
        is_custom_game,
        actions,
        bans,
        timer,
    })
}

/// 从选人会话的玩家数据中解析基础信息
async fn parse_player_from_session(
    player: &serde_json::Value,
    local_cell_id: i32,
) -> Result<PlayerAnalysisData, Box<dyn std::error::Error + Send + Sync>> {
    let cell_id = player["cellId"].as_i64().unwrap_or(0) as i32;
    let display_name = player["displayName"].as_str().unwrap_or("").to_string();
    // 兼容 LCU 返回的 summonerId 可能为 string 或 number
    let summoner_id = if let Some(s) = player["summonerId"].as_str() {
        Some(s.to_string())
    } else if let Some(n) = player["summonerId"].as_i64() {
        Some(n.to_string())
    } else {
        None
    };

    // 机器人判断逻辑
    // 1. summonerId 为 0 表示机器人
    // 2. gameName 为空字符串且隐藏名称
    // 3. puuid 为空字符串
    // 注意：敌方玩家在选人阶段 puuid 可能为空，这是正常的，会在后续获取战绩时修正
    let summoner_id_num = player["summonerId"].as_i64().unwrap_or(0);
    let game_name = player["gameName"].as_str().unwrap_or("");
    let puuid = player["puuid"].as_str().unwrap_or("");
    let name_visibility = player["nameVisibilityType"].as_str().unwrap_or("");

    let is_bot = summoner_id_num == 0 || (game_name.is_empty() && name_visibility == "HIDDEN") || puuid.is_empty();

    log::debug!(
        target: "analysis_data::service",
        "Parsed player: cellId={}, displayName='{}', gameName='{}', tagLine='{}', summonerId={}, isBot={}, puuid='{}'",
        cell_id,
        display_name,
        game_name,
        player["tagLine"].as_str().unwrap_or(""),
        summoner_id_num,
        is_bot,
        puuid
    );

    Ok(PlayerAnalysisData {
        cell_id,
        display_name,
        summoner_id,
        puuid: player["puuid"].as_str().map(|s| s.to_string()),
        is_local: cell_id == local_cell_id,
        is_bot,

        champion_id: player["championId"].as_i64().map(|id| id as i32),
        champion_name: player["championName"].as_str().map(|s| s.to_string()),
        champion_pick_intent: player["championPickIntent"].as_i64().map(|id| id as i32),
        position: player["assignedPosition"].as_str().map(|s| s.to_string()),

        tier: player["tier"].as_str().map(|s| s.to_string()),
        profile_icon_id: player["profileIconId"].as_i64().map(|id| id as i32),
        tag_line: player["tagLine"].as_str().map(|s| s.to_string()),
        spell1_id: player["spell1Id"].as_i64(),
        spell2_id: player["spell2Id"].as_i64(),

        match_stats: None, // 后续填充
    })
}

/// 从 LCU API 获取并填充玩家的召唤师信息（displayName, tier等）
async fn enrich_player_data(
    player_data: &mut PlayerAnalysisData,
    raw_player: &serde_json::Value,
    http_client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 优先从 raw_player 中提取 displayName
    // ChampSelect session 中通常有 gameName + tagLine 组合
    let game_name = raw_player["gameName"].as_str();
    let tag_line = raw_player["tagLine"].as_str();

    // 如果有 gameName，构建完整的显示名称
    if let Some(name) = game_name {
        if !name.is_empty() {
            if let Some(tag) = tag_line {
                if !tag.is_empty() {
                    player_data.display_name = format!("{}#{}", name, tag);
                } else {
                    player_data.display_name = name.to_string();
                }
            } else {
                player_data.display_name = name.to_string();
            }
            player_data.tag_line = tag_line.map(|s| s.to_string());
            log::debug!(
                target: "analysis_data::service",
                "Extracted display name from session: {}",
                player_data.display_name
            );
        }
    }

    // 如果已经有完整信息（显示名称且段位），直接返回
    if !player_data.display_name.is_empty() && player_data.display_name != "未知召唤师" && player_data.tier.is_some()
    {
        log::debug!(
            target: "analysis_data::service",
            "Player already has complete info, skipping API query"
        );
        return Ok(());
    }

    // 降级方案：通过 summonerId 查询（兼容 string 或 number）
    let summoner_id_u64 = if let Some(s) = raw_player["summonerId"].as_str() {
        s.parse().unwrap_or(0)
    } else if let Some(n) = raw_player["summonerId"].as_u64() {
        n
    } else {
        0
    };
    if summoner_id_u64 > 0 {
        match get_summoner_by_id(http_client, summoner_id_u64).await {
            Ok(summoner_info) => {
                if !summoner_info.display_name.trim().is_empty() {
                    player_data.display_name = summoner_info.display_name;
                }
                player_data.profile_icon_id = Some(summoner_info.profile_icon_id as i32);
                player_data.tag_line = summoner_info.tag_line;
                player_data.tier = summoner_info.solo_rank_tier;
                log::debug!(
                    target: "analysis_data::service",
                    "Fetched summoner info by ID {}: displayName='{}', tier={:?}",
                    summoner_id_u64,
                    player_data.display_name,
                    player_data.tier
                );
            }
            Err(e) => {
                log::warn!(
                    target: "analysis_data::service",
                    "Failed to fetch summoner info for ID {}: {}",
                    summoner_id_u64,
                    e
                );
            }
        }
    }

    Ok(())
}

/// 批量获取所有真实玩家的战绩数据（支持指定建议视角）⭐
///
/// 参数：
/// - perspective: 建议视角（Collaboration for队友，Targeting for敌人）
/// - local_cell_id: 本地玩家cellId，用于判断是否是自己
async fn fetch_players_match_stats_with_perspective(
    mut players: Vec<&mut PlayerAnalysisData>,
    http_client: &reqwest::Client,
    queue_id: i64,
    match_stats_cache: &mut std::collections::HashMap<String, PlayerMatchStats>,
    perspective: AdvicePerspective,
    local_cell_id: i32,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    println!("🎯 批量获取玩家战绩（视角：{:?}）", perspective);
    
    fetch_all_players_match_stats_internal(
        players,
        http_client,
        queue_id,
        match_stats_cache,
        Some(perspective),
        local_cell_id,
    )
    .await
}

/// 批量获取所有真实玩家的战绩数据（内部函数）
async fn fetch_all_players_match_stats(
    mut players: Vec<&mut PlayerAnalysisData>,
    http_client: &reqwest::Client,
    queue_id: i64,
    match_stats_cache: &mut std::collections::HashMap<String, PlayerMatchStats>,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    fetch_all_players_match_stats_internal(
        players,
        http_client,
        queue_id,
        match_stats_cache,
        None,  // 默认SelfImprovement
        -1,    // 不关心本地玩家
    )
    .await
}

/// 批量获取所有真实玩家的战绩数据（内部实现）
async fn fetch_all_players_match_stats_internal(
    mut players: Vec<&mut PlayerAnalysisData>,
    http_client: &reqwest::Client,
    queue_id: i64,
    match_stats_cache: &mut std::collections::HashMap<String, PlayerMatchStats>,
    perspective: Option<AdvicePerspective>,
    local_cell_id: i32,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    // 第一步：先从缓存恢复已有的战绩数据
    let mut cached_count = 0;
    let mut need_fetch_indices = Vec::new();

    for (idx, player) in players.iter_mut().enumerate() {
        if player.display_name.is_empty() || player.display_name == "未知召唤师" {
            continue;
        }
        if let Some(cached_stats) = match_stats_cache.get(&player.display_name) {
            log::debug!(
                target: "analysis_data::service",
                "Using cached stats for player '{}'",
                player.display_name
            );
            player.match_stats = Some(cached_stats.clone());

            if player.is_bot {
                player.is_bot = false;
            }

            cached_count += 1;
        } else {
            log::debug!(
                target: "analysis_data::service",
                "Player '{}' needs match stats fetch",
                player.display_name
            );
            need_fetch_indices.push(idx);
        }
    }

    log::info!(
        target: "analysis_data::service",
        "Match stats cache: {}/{} hit, {} need fetch",
        cached_count,
        players.len(),
        need_fetch_indices.len()
    );

    // 第二步：只为没有缓存的玩家获取战绩
    if need_fetch_indices.is_empty() {
        log::debug!(
            target: "analysis_data::service",
            "All match stats from cache, skipping network request"
        );
        return Ok(cached_count);
    }

    let player_names: Vec<String> = need_fetch_indices
        .iter()
        .map(|&idx| players[idx].display_name.clone())
        .collect();

    log::debug!(
        target: "analysis_data::service",
        "Batch querying summoners: {:?}",
        player_names
    );

    // 使用现有的批量获取召唤师信息功能
    let summoners = match get_summoners_by_names(http_client, player_names.clone()).await {
        Ok(summoners) => summoners,
        Err(e) => {
            log::error!(
                target: "analysis_data::service",
                "Batch summoner query failed: {}",
                e
            );
            return Err(e.into());
        }
    };

    log::debug!(
        target: "analysis_data::service",
        "Fetched {} summoner infos",
        summoners.len()
    );

    // 第三步：为需要获取战绩的玩家匹配召唤师信息并获取战绩
    for &idx in need_fetch_indices.iter() {
        let player = &mut players[idx];

        let summoner = summoners.iter().find(|s| {
            // LCU返回的displayName可能是空的，需要用gameName#tagLine拼接
            let summoner_full_name = if let (Some(game_name), Some(tag_line)) = (&s.game_name, &s.tag_line) {
                format!("{}#{}", game_name, tag_line)
            } else {
                s.display_name.clone()
            };

            summoner_full_name.to_lowercase() == player.display_name.to_lowercase()
        });

        match summoner {
            Some(summoner_info) => {
                log::debug!(
                    target: "analysis_data::service",
                    "Fetching recent matches for player '{}'",
                    player.display_name
                );

                // ⭐ 根据视角调用不同的函数
                let player_stats_result = if let Some(persp) = perspective {
                    // 确定是否是自己
                    let is_self = player.cell_id == local_cell_id;
                    let final_perspective = if is_self {
                        AdvicePerspective::SelfImprovement  // 自己永远是自我提升
                    } else {
                        persp  // 队友或敌人使用传入的视角
                    };
                    
                    get_recent_matches_by_puuid_with_perspective(
                        http_client,
                        &summoner_info.puuid,
                        20,
                        Some(queue_id as i32),
                        final_perspective,
                        Some(player.display_name.clone()),
                    )
                    .await
                } else {
                    get_recent_matches_by_puuid(http_client, &summoner_info.puuid, 20, Some(queue_id as i32)).await
                };

                match player_stats_result {
                    Ok(player_stats) => {
                        // 注意：get_recent_matches_by_puuid 已经返回完整的 PlayerMatchStats
                        // 包含了 traits, today_games 等所有增强字段
                        // 在排位模式下，会自动过滤只显示排位战绩
                        // ⭐ 现在还包含了正确视角的 advice

                        match_stats_cache.insert(player.display_name.clone(), player_stats.clone());
                        log::debug!(
                            target: "analysis_data::service",
                            "Cached match stats for player '{}'",
                            player.display_name
                        );

                        player.match_stats = Some(player_stats);
                    }
                    Err(e) => {
                        log::warn!(
                            target: "analysis_data::service",
                            "Failed to fetch match stats for player '{}': {}, skipping",
                            player.display_name,
                            e
                        );
                        player.match_stats = None;
                    }
                }
            }
            None => {
                log::warn!(
                    target: "analysis_data::service",
                    "Summoner info not found for player '{}', skipping",
                    player.display_name
                );
                player.match_stats = None;
            }
        }
    }

    Ok(cached_count)
}

// 注意：convert_match_statistics_to_player_stats 函数已被移除
// 原因：PlayerMatchStats 已经在 get_recent_matches_by_puuid 中由通用分析器完整计算
// 包含所有增强字段（traits, today_games, dpm, cspm, vspm 等）
