use crate::lcu::matches::service::get_recent_matches_by_puuid;
use crate::lcu::summoner::service::get_summoner_by_id;
use crate::lcu::summoner::service::get_summoners_by_names;
use crate::lcu::types::{
    AnalysisChampionStats, MatchPerformance, PlayerAnalysisData, PlayerMatchStats, TeamAnalysisData,
};

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

    // 批量获取真实玩家的战绩数据
    let all_real_players: Vec<_> = my_team_players
        .iter_mut()
        .chain(enemy_team_players.iter_mut())
        .filter(|p| !p.is_bot && !p.display_name.is_empty() && p.display_name != "未知召唤师")
        .collect();

    log::info!(
        target: "analysis_data::service",
        "Fetching match stats for {} real players (cache size: {})",
        all_real_players.len(),
        match_stats_cache.len()
    );

    if !all_real_players.is_empty() {
        match fetch_all_players_match_stats(all_real_players, http_client, queue_id, match_stats_cache).await {
            Ok(cached_count) => {
                log::info!(
                    target: "analysis_data::service",
                    "Match stats fetch completed: {} from cache",
                    cached_count,
                );
            }
            Err(e) => {
                log::warn!(
                    target: "analysis_data::service",
                    "Failed to fetch some match stats: {}, continuing",
                    e
                );
            }
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
    let summoner_id_num = player["summonerId"].as_i64().unwrap_or(0);
    let game_name = player["gameName"].as_str().unwrap_or("");
    let puuid = player["puuid"].as_str().unwrap_or("");
    let name_visibility = player["nameVisibilityType"].as_str().unwrap_or("");

    let is_bot = summoner_id_num == 0 || (game_name.is_empty() && name_visibility == "HIDDEN") || puuid.is_empty();

    log::debug!(
        target: "analysis_data::service",
        "Parsed player: cellId={}, displayName='{}', isBot={}, puuid='{}'",
        cell_id,
        display_name,
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
        spell1_id: player["spell1Id"].as_i64().map(|id| id as i32),
        spell2_id: player["spell2Id"].as_i64().map(|id| id as i32),

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

/// 批量获取所有真实玩家的战绩数据
async fn fetch_all_players_match_stats(
    mut players: Vec<&mut PlayerAnalysisData>,
    http_client: &reqwest::Client,
    queue_id: i64,
    match_stats_cache: &mut std::collections::HashMap<String, PlayerMatchStats>,
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

                match get_recent_matches_by_puuid(http_client, &summoner_info.puuid, 20).await {
                    Ok(match_stats) => {
                        let player_stats =
                            convert_match_statistics_to_player_stats(match_stats, &player.display_name, queue_id);

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

/// 将 LCU API 的 MatchStatistics 转换为我们的 PlayerMatchStats
pub fn convert_match_statistics_to_player_stats(
    lcu_stats: crate::lcu::types::MatchStatistics,
    player_name: &str,
    current_queue_id: i64,
) -> PlayerMatchStats {
    // 排位模式过滤：只显示排位战绩
    let is_ranked = current_queue_id == 420 || current_queue_id == 440;

    let filtered_performance: Vec<_> = if is_ranked {
        log::debug!(
            target: "analysis_data::service",
            "Ranked mode detected, filtering ranked games only (420/440)"
        );
        lcu_stats
            .recent_performance
            .into_iter()
            .filter(|game| game.queue_id == 420 || game.queue_id == 440)
            .collect()
    } else {
        log::debug!(
            target: "analysis_data::service",
            "Non-ranked mode, showing all game history"
        );
        lcu_stats.recent_performance
    };

    let total_games = filtered_performance.len() as u32;
    let wins = filtered_performance.iter().filter(|game| game.win).count() as u32;
    let losses = total_games - wins;
    let win_rate = if total_games > 0 {
        (wins as f64 / total_games as f64) * 100.0
    } else {
        0.0
    };

    // 转换 recent_performance
    let recent_performance: Vec<MatchPerformance> = filtered_performance
        .into_iter()
        .map(|game| {
            let kda = if game.deaths > 0 {
                (game.kills + game.assists) as f64 / game.deaths as f64
            } else {
                (game.kills + game.assists) as f64
            };

            MatchPerformance {
                game_id: Some(game.game_id),
                win: game.win,
                champion_id: game.champion_id,
                champion_name: format!("Champion_{}", game.champion_id),
                kills: game.kills,
                deaths: game.deaths,
                assists: game.assists,
                kda,
                game_duration: Some(game.game_duration),
                queue_id: Some(game.queue_id),
                game_mode: Some(game.game_mode),
            }
        })
        .collect();

    // 转换 favorite_champions
    let favorite_champions: Vec<AnalysisChampionStats> = lcu_stats
        .favorite_champions
        .into_iter()
        .map(|champ| AnalysisChampionStats {
            champion_id: champ.champion_id,
            champion_name: format!("Champion_{}", champ.champion_id),
            games: champ.games_played as u32,
            wins: champ.wins as u32,
            win_rate: champ.win_rate as f64,
        })
        .collect();

    // 🔥 重新计算平均KDA（基于过滤后的数据）
    let avg_kills = if total_games > 0 {
        recent_performance.iter().map(|g| g.kills as f64).sum::<f64>() / total_games as f64
    } else {
        0.0
    };
    let avg_deaths = if total_games > 0 {
        recent_performance.iter().map(|g| g.deaths as f64).sum::<f64>() / total_games as f64
    } else {
        0.0
    };
    let avg_assists = if total_games > 0 {
        recent_performance.iter().map(|g| g.assists as f64).sum::<f64>() / total_games as f64
    } else {
        0.0
    };
    let avg_kda = if avg_deaths > 0.0 {
        (avg_kills + avg_assists) / avg_deaths
    } else {
        avg_kills + avg_assists
    };

    log::debug!(
        target: "analysis_data::service",
        "Player '{}' stats: {}W/{}L ({:.1}% WR), KDA: {:.1}/{:.1}/{:.1}",
        player_name,
        wins,
        losses,
        win_rate,
        avg_kills,
        avg_deaths,
        avg_assists
    );

    PlayerMatchStats {
        total_games,
        wins,
        losses,
        win_rate,
        avg_kills,
        avg_deaths,
        avg_assists,
        avg_kda,
        favorite_champions,
        recent_performance,
    }
}
