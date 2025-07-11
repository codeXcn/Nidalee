use crate::lcu::request::lcu_get;
use crate::lcu::types::{ChampionStats, MatchStatistics, RecentGame, TeamStats};
use reqwest::Client;
use serde_json::{json, Value};

/// 获取当前玩家历史战绩统计（自动认证、统一请求、日志耗时）
pub async fn get_match_history(client: &Client) -> Result<MatchStatistics, String> {
    println!("\n🔍 ===== 开始获取我的战绩 =====");

    // 第1步：获取当前召唤师信息来得到PUUID
    println!("\n📍 第1步：获取当前召唤师信息");
    let summoner_data: Value = lcu_get(client, "/lol-summoner/v1/current-summoner").await?;
    let puuid = summoner_data
        .get("puuid")
        .and_then(|p| p.as_str())
        .ok_or_else(|| "未找到PUUID".to_string())?;
    println!("🆔 提取到的PUUID: {}", puuid);

    // 第2步：使用PUUID获取对局列表
    println!("\n📍 第2步：使用PUUID获取对局列表");
    let match_list_url = format!("/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex=20", puuid);
    let match_list_data: Value = lcu_get(client, &match_list_url).await?;

    // 第3步：直接分析对局列表数据
    println!("\n📍 第3步：分析对局列表数据");
    let statistics = analyze_match_list_data(match_list_data, puuid)?;

    println!("\n✅ ===== 我的战绩查询完成 =====");
    println!("📊 最终统计结果:");
    println!("   - 总对局: {}", statistics.total_games);
    println!("   - 胜场: {}", statistics.wins);
    println!("   - 负场: {}", statistics.losses);
    println!("   - 胜率: {:.1}%", statistics.win_rate);
    println!("   - 平均KDA: {:.2}", statistics.avg_kda);
    println!("   - 最近对局数: {}", statistics.recent_performance.len());

    Ok(statistics)
}

/// 获取游戏详细信息
pub async fn get_game_detail(client: &Client, game_id: u64) -> Result<Value, String> {
    println!("\n🔍 ===== 获取游戏详细信息 =====");
    println!("🎮 游戏ID: {}", game_id);

    let url = format!("/lol-match-history/v1/games/{}", game_id);
    let game_data: Value = lcu_get(client, &url).await?;

    // 处理游戏数据
    let mut blue_team_stats = TeamStats::default();
    let mut red_team_stats = TeamStats::default();

    let mut max_damage = 0;
    let mut best_player_champion_id = 0;
    let mut max_tank = 0;
    let mut max_tank_champion_id = 0;
    let mut max_streak = 0;
    let mut max_streak_champion_id = 0;

    // 处理参与者数据
    let participants = if let Some(participants_data) =
        game_data.get("participants").and_then(|p| p.as_array())
    {
        participants_data
            .iter()
            .map(|p| {
                let stats = &p["stats"];
                let champion_id = p.get("championId").and_then(|id| id.as_i64()).unwrap_or(0) as i32;
                let team_id = p.get("teamId").and_then(|id| id.as_i64()).unwrap_or(0) as i32;
                let kills = stats.get("kills").and_then(|k| k.as_i64()).unwrap_or(0) as i32;
                let deaths = stats.get("deaths").and_then(|d| d.as_i64()).unwrap_or(0) as i32;
                let assists = stats.get("assists").and_then(|a| a.as_i64()).unwrap_or(0) as i32;
                let damage = stats.get("totalDamageDealtToChampions").and_then(|d| d.as_i64()).unwrap_or(0) as i32;
                let damage_taken = stats.get("totalDamageTaken").and_then(|d| d.as_i64()).unwrap_or(0) as i32;
                let gold = stats.get("goldEarned").and_then(|g| g.as_i64()).unwrap_or(0) as i32;
                let vision = stats.get("visionScore").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let level = stats.get("champLevel").and_then(|l| l.as_i64()).unwrap_or(0) as i32;
                let streak = stats.get("largestKillingSpree").and_then(|s| s.as_i64()).unwrap_or(0) as i32;

                // 更新队伍统计
                if team_id == 100 {
                    blue_team_stats.kills += kills;
                    blue_team_stats.gold_earned += gold;
                    blue_team_stats.total_damage_dealt_to_champions += damage;
                    blue_team_stats.vision_score += vision;
                } else {
                    red_team_stats.kills += kills;
                    red_team_stats.gold_earned += gold;
                    red_team_stats.total_damage_dealt_to_champions += damage;
                    red_team_stats.vision_score += vision;
                }

                // 更新最高数据
                if damage > max_damage {
                    max_damage = damage;
                    best_player_champion_id = champion_id;
                }
                if damage_taken > max_tank {
                    max_tank = damage_taken;
                    max_tank_champion_id = champion_id;
                }
                if streak > max_streak {
                    max_streak = streak;
                    max_streak_champion_id = champion_id;
                }

                json!({
                    "participantId": p.get("participantId").and_then(|id| id.as_i64()).unwrap_or(0) as i32,
                    "championId": champion_id,
                    "teamId": team_id,
                    "rankTier": None::<String>,
                    "stats": {
                        "kills": kills,
                        "deaths": deaths,
                        "assists": assists,
                        "champLevel": level,
                        "goldEarned": gold,
                        "totalDamageDealtToChampions": damage,
                        "totalDamageTaken": damage_taken,
                        "visionScore": vision,
                        "item0": stats.get("item0").and_then(|i| i.as_i64()).map(|i| i as i32),
                        "item1": stats.get("item1").and_then(|i| i.as_i64()).map(|i| i as i32),
                        "item2": stats.get("item2").and_then(|i| i.as_i64()).map(|i| i as i32),
                        "item3": stats.get("item3").and_then(|i| i.as_i64()).map(|i| i as i32),
                        "item4": stats.get("item4").and_then(|i| i.as_i64()).map(|i| i as i32),
                        "item5": stats.get("item5").and_then(|i| i.as_i64()).map(|i| i as i32),
                        "item6": stats.get("item6").and_then(|i| i.as_i64()).map(|i| i as i32),
                    },
                    "score": None::<i32>,
                })
            })
            .collect::<Vec<Value>>()
    } else {
        Vec::new()
    };

    let participant_identities = game_data
        .get("participantIdentities")
        .cloned()
        .unwrap_or(Value::Null);

    // 处理队伍数据
    let teams = if let Some(teams_data) = game_data.get("teams").and_then(|t| t.as_array()) {
        teams_data
            .iter()
            .map(|t| {
                json!({
                    "teamId": t.get("teamId").and_then(|id| id.as_i64()).unwrap_or(0) as i32,
                    "win": t.get("win").and_then(|w| w.as_str()).unwrap_or("").to_string(),
                    "bans": t.get("bans")
                        .and_then(|b| b.as_array())
                        .map(|bans| {
                            bans.iter()
                                .map(|ban| {
                                    json!({
                                        "championId": ban.get("championId").and_then(|id| id.as_i64()).unwrap_or(0) as i32,
                                        "pickTurn": ban.get("pickTurn").and_then(|t| t.as_i64()).unwrap_or(0) as i32,
                                    })
                                })
                                .collect::<Vec<Value>>()
                        })
                        .unwrap_or_default(),
                    "baronKills": t.get("baronKills").and_then(|k| k.as_i64()).unwrap_or(0) as i32,
                    "dragonKills": t.get("dragonKills").and_then(|k| k.as_i64()).unwrap_or(0) as i32,
                    "towerKills": t.get("towerKills").and_then(|k| k.as_i64()).unwrap_or(0) as i32,
                    "inhibitorKills": t.get("inhibitorKills").and_then(|k| k.as_i64()).unwrap_or(0) as i32,
                })
            })
            .collect::<Vec<Value>>()
    } else {
        Vec::new()
    };

    Ok(json!({
        "gameId": game_id,
        "gameDuration": game_data.get("gameDuration").and_then(|d| d.as_i64()).unwrap_or(0) as i32,
        "gameCreation": game_data.get("gameCreation").and_then(|c| c.as_i64()).unwrap_or(0),
        "gameMode": game_data.get("gameMode").and_then(|m| m.as_str()).unwrap_or("").to_string(),
        "gameType": game_data.get("gameType").and_then(|t| t.as_str()).unwrap_or("").to_string(),
        "gameVersion": game_data.get("gameVersion").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        "mapId": game_data.get("mapId").and_then(|m| m.as_i64()).unwrap_or(0) as i32,
        "queueId": game_data.get("queueId").and_then(|q| q.as_i64()).unwrap_or(0) as i32,
        "teams": teams,
        "participants": participants,
        "blueTeamStats": {
            "kills": blue_team_stats.kills,
            "gold_earned": blue_team_stats.gold_earned,
            "total_damage_dealt_to_champions": blue_team_stats.total_damage_dealt_to_champions,
            "vision_score": blue_team_stats.vision_score,
        },
        "redTeamStats": {
            "kills": red_team_stats.kills,
            "gold_earned": red_team_stats.gold_earned,
            "total_damage_dealt_to_champions": red_team_stats.total_damage_dealt_to_champions,
            "vision_score": red_team_stats.vision_score,
        },
        "bestPlayerChampionId": best_player_champion_id,
        "maxDamage": max_damage,
        "maxTankChampionId": max_tank_champion_id,
        "maxTank": max_tank,
        "maxStreakChampionId": max_streak_champion_id,
        "maxStreak": max_streak,
        "participantIdentities": participant_identities,
    }))
}

/// 获取指定召唤师最近几场简单战绩
pub async fn get_recent_matches_by_summoner_id(
    client: &Client,
    puuid: &str,
    count: usize,
) -> Result<MatchStatistics, String> {
    let url = format!(
        "/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex={}",
        puuid, count
    );
    let match_list_data: Value = lcu_get(client, &url).await?;
    log::info!("match_list_data (查询到的战绩): {:#}", match_list_data);    // 第3步：直接分析对局列表数据
    let statistics = analyze_match_list_data(match_list_data, puuid)?;
    Ok(statistics)
}

fn analyze_match_list_data(
    match_list_data: Value,
    current_puuid: &str,
) -> Result<MatchStatistics, String> {
    println!("📊 开始分析对局列表数据");
    println!("👤 目标玩家PUUID: {}", current_puuid);

    let empty_games = vec![];
    let games = match_list_data
        .get("games")
        .and_then(|games_obj| games_obj.get("games"))
        .and_then(|g| g.as_array())
        .unwrap_or(&empty_games);

    println!("📊 找到 {} 场对局记录", games.len());

    let mut total_games = 0;
    let mut wins = 0;
    let mut total_kills = 0;
    let mut total_deaths = 0;
    let mut total_assists = 0;
    let mut champion_stats = std::collections::HashMap::new();
    let mut recent_performance = Vec::new();

    // 只分析前20场游戏
    let games_to_analyze = games.iter().take(20);

    for (index, game) in games_to_analyze.enumerate() {
        println!("\n🎮 分析第 {} 场游戏", index + 1);
        total_games += 1;

        // 查找当前玩家的参与者信息
        if let Some(participant_identities) = game
            .get("participantIdentities")
            .and_then(|pi| pi.as_array())
        {
            // 在participantIdentities中找到匹配PUUID的玩家
            let current_identity = participant_identities.iter().find(|identity| {
                let player_puuid = identity
                    .get("player")
                    .and_then(|player| player.get("puuid"))
                    .and_then(|puuid| puuid.as_str());
                player_puuid == Some(current_puuid)
            });

            if let Some(identity) = current_identity {
                let participant_id = identity
                    .get("participantId")
                    .and_then(|id| id.as_u64())
                    .unwrap_or(0);

                // 在participants中找到对应participantId的参与者
                if let Some(participants) = game.get("participants").and_then(|p| p.as_array()) {
                    let current_participant = participants.iter().find(|p| {
                        let p_id = p
                            .get("participantId")
                            .and_then(|id| id.as_u64())
                            .unwrap_or(0);
                        p_id == participant_id
                    });

                    if let Some(participant) = current_participant {
                        let stats = &participant["stats"];
                        let champion_id = participant
                            .get("championId")
                            .and_then(|id| id.as_u64())
                            .unwrap_or(0);
                        let is_win = stats.get("win").and_then(|w| w.as_bool()).unwrap_or(false);
                        let kills = stats.get("kills").and_then(|k| k.as_i64()).unwrap_or(0) as i32;
                        let deaths =
                            stats.get("deaths").and_then(|d| d.as_i64()).unwrap_or(0) as i32;
                        let assists =
                            stats.get("assists").and_then(|a| a.as_i64()).unwrap_or(0) as i32;

                        println!("🏆 英雄: {}", champion_id);
                        println!("🎯 结果: {}", if is_win { "胜利" } else { "失败" });
                        println!("⚔️  KDA: {}/{}/{}", kills, deaths, assists);

                        if is_win {
                            wins += 1;
                        }

                        total_kills += kills;
                        total_deaths += deaths;
                        total_assists += assists;

                        // 统计英雄数据
                        let entry = champion_stats
                            .entry(champion_id.to_string())
                            .or_insert((0, 0));
                        entry.0 += 1; // 游戏数
                        if is_win {
                            entry.1 += 1; // 胜场数
                        }

                        // 添加到最近游戏
                        let penta_kills = stats
                            .get("pentaKills")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0) as i32;
                        let quadra_kills = stats
                            .get("quadraKills")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0) as i32;
                        let performance_rating = get_performance_rating(
                            kills,
                            deaths,
                            assists,
                            penta_kills,
                            quadra_kills,
                        );
                        recent_performance.push(RecentGame {
                            game_id: game.get("gameId").and_then(|id| id.as_u64()).unwrap_or(0),
                            champion_id: champion_id as i32,
                            queue_id: game.get("queueId").and_then(|id| id.as_i64()).unwrap_or(0),
                            game_mode: game
                                .get("gameMode")
                                .and_then(|gm| gm.as_str())
                                .unwrap_or("Unknown")
                                .to_string(),
                            win: is_win,
                            kills,
                            deaths,
                            assists,
                            game_duration: game
                                .get("gameDuration")
                                .and_then(|gd| gd.as_i64())
                                .unwrap_or(0) as i32,
                            game_creation: game
                                .get("gameCreation")
                                .and_then(|gc| gc.as_i64())
                                .unwrap_or(0),
                            performance_rating: performance_rating.clone(),
                        });
                    }
                }
            }
        }
    }

    // 计算统计数据
    let win_rate = if total_games > 0 {
        (wins as f32 / total_games as f32) * 100.0
    } else {
        0.0
    };

    let avg_kills = if total_games > 0 {
        total_kills as f32 / total_games as f32
    } else {
        0.0
    };

    let avg_deaths = if total_games > 0 {
        total_deaths as f32 / total_games as f32
    } else {
        0.0
    };

    let avg_assists = if total_games > 0 {
        total_assists as f32 / total_games as f32
    } else {
        0.0
    };

    let avg_kda = (avg_kills + avg_assists) / avg_deaths.max(1.0);

    // 转换英雄统计
    let mut favorite_champions: Vec<ChampionStats> = champion_stats
        .into_iter()
        .map(|(name, (games, wins))| ChampionStats {
            champion_id: name.parse::<i32>().unwrap(),
            games_played: games,
            wins,
            win_rate: if games > 0 {
                (wins as f32 / games as f32) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    // 按游戏数排序
    favorite_champions.sort_by(|a, b| b.games_played.cmp(&a.games_played));
    favorite_champions.truncate(6); // 只保留前6个

    Ok(MatchStatistics {
        total_games,
        wins,
        losses: total_games - wins,
        win_rate,
        avg_kills,
        avg_deaths,
        avg_assists,
        avg_kda,
        favorite_champions,
        recent_performance,
    })
}

fn get_performance_rating(
    kills: i32,
    deaths: i32,
    assists: i32,
    penta_kills: i32,
    quadra_kills: i32,
) -> String {
    let kda = (kills + assists) as f32 / deaths.max(1) as f32;
    if penta_kills > 0 {
        return "五杀超神！".to_string();
    }
    if quadra_kills > 0 {
        return "四杀爆发！".to_string();
    }
    if kda >= 8.0 {
        return "超神表现！".to_string();
    }
    if kda >= 5.0 {
        return "表现亮眼".to_string();
    }
    if kda >= 3.0 {
        return "发挥不错".to_string();
    }
    if kda >= 1.5 {
        return "发挥一般".to_string();
    }
    "需要加油".to_string()
}
