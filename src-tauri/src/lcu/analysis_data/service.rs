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

    log::info!("[AnalysisService] 🚀 开始构建完整分析数据...");
    log::info!("[AnalysisService] 本地玩家 cellId: {}", local_player_cell_id);

    // 🔍 检查游戏类型
    let queue_id = session["queueId"].as_i64().unwrap_or(0);
    let is_custom_game = session["isCustomGame"].as_bool().unwrap_or(false);
    log::info!(
        "[AnalysisService] 🎮 游戏类型: queueId={}, isCustomGame={}",
        queue_id,
        is_custom_game
    );

    if is_custom_game {
        log::warn!("[AnalysisService] ⚠️ 这是自定义游戏（人机模式），部分玩家可能是机器人");
    }

    // 🔍 打印原始 session 数据（美化输出）
    if let Ok(pretty_json) = serde_json::to_string_pretty(session) {
        log::info!("[AnalysisService] 📋 原始 ChampSelect Session:");
        log::info!("{}", pretty_json);
    }

    // 解析我方队伍
    let mut my_team_players = Vec::new();
    if let Some(my_team) = session["myTeam"].as_array() {
        log::info!("[AnalysisService] 解析我方队伍，玩家数: {}", my_team.len());
        for (idx, player) in my_team.iter().enumerate() {
            log::info!("[AnalysisService] 🔍 处理我方玩家[{}]", idx);
            // 打印原始玩家数据
            if let Ok(player_json) = serde_json::to_string_pretty(player) {
                log::info!("[AnalysisService] 原始玩家数据:\n{}", player_json);
            }

            match parse_player_from_session(player, local_player_cell_id).await {
                Ok(mut player_data) => {
                    log::info!(
                        "[AnalysisService] 解析后基础数据: displayName='{}', isBot={}, puuid={:?}",
                        player_data.display_name,
                        player_data.is_bot,
                        player_data.puuid
                    );

                    match enrich_player_data(&mut player_data, player, http_client).await {
                        Ok(_) => {
                            log::info!(
                                "[AnalysisService] Enrich 后: displayName='{}', tagLine={:?}",
                                player_data.display_name,
                                player_data.tag_line
                            );
                            my_team_players.push(player_data);
                        }
                        Err(e) => {
                            log::error!("[AnalysisService] ❌ Enrich 我方玩家[{}] 失败: {}", idx, e);
                            // 继续处理其他玩家，不中断整个流程
                            my_team_players.push(player_data);
                        }
                    }
                }
                Err(e) => {
                    log::error!("[AnalysisService] ❌ 解析我方玩家[{}] 失败: {}", idx, e);
                    // 跳过这个玩家，继续处理其他的
                    continue;
                }
            }
        }
    } else {
        log::warn!("[AnalysisService] ⚠️ Session 中没有 myTeam 数组！");
    }

    // 解析敌方队伍
    let mut enemy_team_players = Vec::new();
    if let Some(their_team) = session["theirTeam"].as_array() {
        log::info!("[AnalysisService] 解析敌方队伍，玩家数: {}", their_team.len());
        for (idx, player) in their_team.iter().enumerate() {
            log::info!("[AnalysisService] 🔍 处理敌方玩家[{}]", idx);
            // 打印原始玩家数据
            if let Ok(player_json) = serde_json::to_string_pretty(player) {
                log::info!("[AnalysisService] 原始玩家数据:\n{}", player_json);
            }

            match parse_player_from_session(player, local_player_cell_id).await {
                Ok(mut player_data) => {
                    log::info!(
                        "[AnalysisService] 解析后基础数据: displayName='{}', isBot={}, puuid={:?}",
                        player_data.display_name,
                        player_data.is_bot,
                        player_data.puuid
                    );

                    match enrich_player_data(&mut player_data, player, http_client).await {
                        Ok(_) => {
                            log::info!(
                                "[AnalysisService] Enrich 后: displayName='{}', tagLine={:?}",
                                player_data.display_name,
                                player_data.tag_line
                            );
                            enemy_team_players.push(player_data);
                        }
                        Err(e) => {
                            log::error!("[AnalysisService] ❌ Enrich 敌方玩家[{}] 失败: {}", idx, e);
                            // 继续处理其他玩家，不中断整个流程
                            enemy_team_players.push(player_data);
                        }
                    }
                }
                Err(e) => {
                    log::error!("[AnalysisService] ❌ 解析敌方玩家[{}] 失败: {}", idx, e);
                    // 跳过这个玩家，继续处理其他的
                    continue;
                }
            }
        }
    } else {
        log::warn!("[AnalysisService] ⚠️ Session 中没有 theirTeam 数组！");
    }

    // 🔍 调试：打印所有玩家的状态（在借用之前）
    log::debug!("[AnalysisService] 我方玩家详情:");
    for (i, player) in my_team_players.iter().enumerate() {
        log::debug!(
            "  [{}] displayName='{}', isBot={}, puuid={:?}",
            i,
            player.display_name,
            player.is_bot,
            player.puuid
        );
    }
    log::debug!("[AnalysisService] 敌方玩家详情:");
    for (i, player) in enemy_team_players.iter().enumerate() {
        log::debug!(
            "  [{}] displayName='{}', isBot={}, puuid={:?}",
            i,
            player.display_name,
            player.is_bot,
            player.puuid
        );
    }

    // 🔥 核心：批量获取真实玩家的战绩数据
    let all_real_players: Vec<_> = my_team_players
        .iter_mut()
        .chain(enemy_team_players.iter_mut())
        .filter(|p| !p.is_bot && !p.display_name.is_empty() && p.display_name != "未知召唤师")
        .collect();

    log::info!("[AnalysisService] 真实玩家数量: {}", all_real_players.len());
    log::info!("[AnalysisService] 📊 当前缓存中的战绩数: {}", match_stats_cache.len());

    if !all_real_players.is_empty() {
        match fetch_all_players_match_stats(all_real_players, http_client, queue_id, match_stats_cache).await {
            Ok(cached_count) => {
                log::info!("[AnalysisService] ✅ 战绩处理完成，其中 {} 个来自缓存", cached_count);
            }
            Err(e) => {
                log::warn!("[AnalysisService] ⚠️ 战绩获取部分失败: {}", e);
                // 继续执行，不阻塞整体流程
            }
        }
    }

    // 🔥 提取 actions、bans、timer
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

    // 🔥 改进机器人判断逻辑
    // 1. summonerId 为 0 表示机器人
    // 2. gameName 为空字符串且隐藏名称
    // 3. puuid 为空字符串
    let summoner_id_num = player["summonerId"].as_i64().unwrap_or(0);
    let game_name = player["gameName"].as_str().unwrap_or("");
    let puuid = player["puuid"].as_str().unwrap_or("");
    let name_visibility = player["nameVisibilityType"].as_str().unwrap_or("");

    let is_bot = summoner_id_num == 0 || (game_name.is_empty() && name_visibility == "HIDDEN") || puuid.is_empty();

    if is_bot {
        log::info!("[AnalysisService] [Bot] cellId={}, puuid='{}'", cell_id, puuid);
    } else {
        log::info!("[AnalysisService] [Player] cellId={}, summonerId={:?}, puuid={:?}, displayName='{}', gameName='{}', tagLine='{}', visibility='{}'",
            cell_id, summoner_id, puuid, display_name, game_name, player["tagLine"].as_str().unwrap_or(""), name_visibility);
    }

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
    log::info!("[AnalysisService] 🔧 开始 Enrich 玩家数据");

    // 🔥 关键修复：优先从 raw_player 中提取 displayName
    // ChampSelect session 中通常有 gameName + tagLine 组合
    let game_name = raw_player["gameName"].as_str();
    let tag_line = raw_player["tagLine"].as_str();

    log::info!(
        "[AnalysisService] 检查字段: gameName={:?}, tagLine={:?}",
        game_name,
        tag_line
    );

    // 记录是否从 session 获取了显示名称
    let mut has_display_name_from_session = false;

    // 如果有 gameName，构建完整的显示名称
    if let Some(name) = game_name {
        if !name.is_empty() {
            // 🔥 额外检查：确保 gameName 不是空字符串
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
            has_display_name_from_session = true;
            log::info!(
                "[AnalysisService] ✅ 从 session 中提取到显示名称: {}",
                player_data.display_name
            );
        } else {
            log::warn!("[AnalysisService] ⚠️ gameName 存在但为空字符串");
        }
    } else {
        log::warn!("[AnalysisService] ⚠️ gameName 字段不存在");
    }

    // 如果已经有 displayName 但没有段位信息，仍需要通过 API 获取段位
    if has_display_name_from_session && player_data.tier.is_none() {
        log::info!("[AnalysisService] 🔍 已有显示名称，但需要获取段位信息...");
        // 继续执行下面的 API 查询逻辑以获取段位
    }

    // 如果已经有完整信息（显示名称且段位），直接返回
    if !player_data.display_name.is_empty() && player_data.display_name != "未知召唤师" && player_data.tier.is_some()
    {
        log::info!("[AnalysisService] ✅ 已有完整信息，跳过 API 查询");
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
                log::info!("[AnalysisService] [SummonerInfo] 查询结果: {:?}", summoner_info);
                // 填充 enriched 数据
                if !summoner_info.display_name.trim().is_empty() {
                    player_data.display_name = summoner_info.display_name;
                }
                player_data.profile_icon_id = Some(summoner_info.profile_icon_id as i32);
                player_data.tag_line = summoner_info.tag_line;
                // 🔥 修复：填充段位信息
                player_data.tier = summoner_info.solo_rank_tier;
                log::debug!(
                    "[AnalysisService] ✅ 通过 summonerId 查询到显示名称: {}, 段位: {:?}",
                    player_data.display_name,
                    player_data.tier
                );
            }
            Err(e) => {
                log::warn!("[AnalysisService] ⚠️ 获取召唤师信息失败 ({}): {}", summoner_id_u64, e);
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
    log::info!(
        "[AnalysisService] 🎯 开始批量获取战绩，玩家数: {}，队列ID: {}",
        players.len(),
        queue_id
    );

    // 🔥 第一步：先从缓存恢复已有的战绩数据
    let mut cached_count = 0;
    let mut need_fetch_indices = Vec::new();

    for (idx, player) in players.iter_mut().enumerate() {
        if player.display_name.is_empty() || player.display_name == "未知召唤师" {
            continue;
        }
        if let Some(cached_stats) = match_stats_cache.get(&player.display_name) {
            log::info!("[AnalysisService] ♻️ 从缓存恢复 {} 的战绩", player.display_name);
            player.match_stats = Some(cached_stats.clone());
            cached_count += 1;
        } else {
            log::info!("[AnalysisService] 🆕 {} 需要获取战绩", player.display_name);
            need_fetch_indices.push(idx);
        }
        // 只查战绩，不再兜底查段位，段位应在 enrich_player_data 阶段已补全
    }

    log::info!(
        "[AnalysisService] 📊 缓存命中: {}/{}, 需要获取: {}",
        cached_count,
        players.len(),
        need_fetch_indices.len()
    );

    // 🔥 第二步：只为没有缓存的玩家获取战绩
    if need_fetch_indices.is_empty() {
        log::info!("[AnalysisService] ✅ 所有战绩均来自缓存，跳过网络请求");
        return Ok(cached_count);
    }

    let player_names: Vec<String> = need_fetch_indices
        .iter()
        .map(|&idx| players[idx].display_name.clone())
        .collect();

    log::info!(
        "[AnalysisService] 批量查询召唤师 (共{}个): {:?}",
        player_names.len(),
        player_names
    );

    // 🚀 使用现有的批量获取召唤师信息功能
    let summoners = match get_summoners_by_names(http_client, player_names.clone()).await {
        Ok(summoners) => summoners,
        Err(e) => {
            log::error!("[AnalysisService] ❌ 批量获取召唤师信息失败: {}", e);
            return Err(e.into());
        }
    };

    log::info!("[AnalysisService] ✅ 成功获取到 {} 个召唤师信息", summoners.len());

    // 🔍 打印返回的召唤师详情
    for (i, summoner) in summoners.iter().enumerate() {
        log::info!(
            "  [{}] LCU返回: displayName='{}', gameName={:?}, tagLine={:?}, puuid={}",
            i,
            summoner.display_name,
            summoner.game_name,
            summoner.tag_line,
            summoner.puuid
        );
    }

    // 🔥 第三步：为需要获取战绩的玩家匹配召唤师信息并获取战绩
    for &idx in need_fetch_indices.iter() {
        let player = &mut players[idx];
        // 查找对应的召唤师信息
        log::info!("[AnalysisService] 🔍 尝试匹配玩家: '{}'", player.display_name);

        let summoner = summoners.iter().find(|s| {
            // 🔥 修复：LCU返回的displayName可能是空的，需要用gameName#tagLine拼接
            let summoner_full_name = if let (Some(game_name), Some(tag_line)) = (&s.game_name, &s.tag_line) {
                format!("{}#{}", game_name, tag_line)
            } else {
                s.display_name.clone()
            };

            let matches = summoner_full_name.to_lowercase() == player.display_name.to_lowercase();
            log::info!(
                "    比较: '{}' (拼接后) vs '{}' -> {}",
                summoner_full_name,
                player.display_name,
                matches
            );
            matches
        });

        match summoner {
            Some(summoner_info) => {
                log::info!(
                    "[AnalysisService] ✅ 找到召唤师 {} 信息，获取战绩...",
                    player.display_name
                );

                // 获取战绩
                match get_recent_matches_by_puuid(http_client, &summoner_info.puuid, 20).await {
                    Ok(match_stats) => {
                        let player_stats =
                            convert_match_statistics_to_player_stats(match_stats, &player.display_name, queue_id);

                        // 🔥 保存到缓存
                        match_stats_cache.insert(player.display_name.clone(), player_stats.clone());
                        log::info!("[AnalysisService] 💾 {} 的战绩已缓存", player.display_name);

                        player.match_stats = Some(player_stats);
                    }
                    Err(e) => {
                        log::warn!(
                            "[AnalysisService] ⚠️ 获取 {} 战绩失败: {}, 跳过",
                            player.display_name,
                            e
                        );
                        player.match_stats = None; // 🔥 不生成假数据，保持为 None
                    }
                }
            }
            None => {
                log::warn!("[AnalysisService] ⚠️ 未找到召唤师 {} 的信息，跳过", player.display_name);
                player.match_stats = None; // 🔥 不生成假数据，保持为 None
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
    // 🎯 排位模式过滤：只显示排位战绩
    let is_ranked = current_queue_id == 420 || current_queue_id == 440;

    let filtered_performance: Vec<_> = if is_ranked {
        log::info!("[AnalysisService] 🏆 排位模式检测到，只显示排位战绩 (420/440)");
        lcu_stats
            .recent_performance
            .into_iter()
            .filter(|game| game.queue_id == 420 || game.queue_id == 440)
            .collect()
    } else {
        log::info!("[AnalysisService] 🎲 非排位模式，显示所有战绩");
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
        "[AnalysisService] 📊 {} (过滤后) 统计: {}胜{}负 胜率{:.1}%, 平均KDA: {:.1}/{:.1}/{:.1}",
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
