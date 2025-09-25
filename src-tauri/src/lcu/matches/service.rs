use crate::lcu::request::{lcu_get, lcu_request_json};
use crate::lcu::types::{
    ChampionStats, GameDetail, MatchStatistics, ParticipantInfo, ParticipantStats, RecentGame,
    TeamInfo, TeamStats,
};
use reqwest::{Client, Method};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// 以下内容为原 match_history.rs 全部内容，粘贴至此
// 其余内容保持不变，全部迁移

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiGameData {
    game_id: u64,
    game_duration: i32,
    game_creation: i64,
    game_mode: String,
    game_type: String,
    game_version: String,
    map_id: i32,
    queue_id: i32,
    teams: Vec<TeamInfo>,
    participants: Vec<ApiParticipant>,
    participant_identities: Vec<ApiParticipantIdentity>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiParticipant {
    participant_id: i32,
    #[serde(default)]
    team_id: Option<i32>,
    #[serde(default)]
    champion_id: Option<i32>,
    stats: ApiParticipantStats,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct ApiParticipantStats {
    #[serde(default)]
    kills: Option<i32>,
    #[serde(default)]
    deaths: Option<i32>,
    #[serde(default)]
    assists: Option<i32>,
    #[serde(default)]
    champ_level: Option<i32>,
    #[serde(default)]
    gold_earned: Option<i32>,
    #[serde(default)]
    total_damage_dealt_to_champions: Option<i32>,
    #[serde(default)]
    total_damage_taken: Option<i32>,
    #[serde(default)]
    vision_score: Option<i32>,
    #[serde(default)]
    item0: Option<i32>,
    #[serde(default)]
    item1: Option<i32>,
    #[serde(default)]
    item2: Option<i32>,
    #[serde(default)]
    item3: Option<i32>,
    #[serde(default)]
    item4: Option<i32>,
    #[serde(default)]
    item5: Option<i32>,
    #[serde(default)]
    item6: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ApiParticipantIdentity {
    participant_id: i32,
    player: ApiPlayer,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ApiPlayer {
    #[serde(default)]
    summoner_name: Option<String>,
    #[serde(default)]
    game_name: Option<String>,
    #[serde(default)]
    tag_line: Option<String>,
    profile_icon: i64,
}

/// 获取当前玩家历史战绩统计（自动认证、统一请求、日志耗时）
pub async fn get_match_history(client: &Client, end_count: usize) -> Result<MatchStatistics, String> {
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
    let safe_end = if end_count == 0 { 20 } else { end_count.min(100) };
    // LCU API 的 endIndex 是包含的，所以需要减1
    let actual_end_index = if safe_end > 0 { safe_end - 1 } else { 0 };
    println!("🔢 请求的对局数量: end_count={}, safe_end={}, actual_end_index={}", end_count, safe_end, actual_end_index);
    let match_list_url = format!(
        "/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex={}",
        puuid, actual_end_index
    );
    println!("🌐 请求URL: {}", match_list_url);
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

pub async fn get_game_detail_logic(client: &Client, game_id: u64) -> Result<GameDetail, String> {
    let path = format!("/lol-match-history/v1/games/{}", game_id);
    let api_game_data: ApiGameData = lcu_request_json(client, Method::GET, &path, None)
        .await
        .map_err(|e| format!("获取游戏详细信息失败: {}", e))?;

    let mut blue_team_stats = TeamStats::default();
    let mut red_team_stats = TeamStats::default();
    let mut max_damage = 0;
    let mut best_player_champion_id = 0;
    let mut max_tank = 0;
    let mut max_tank_champion_id = 0;
    let mut max_streak = 0;
    let mut max_streak_champion_id = 0;

    let player_map: HashMap<i32, ApiPlayer> = api_game_data
        .participant_identities
        .into_iter()
        .map(|p| (p.participant_id, p.player))
        .collect();

    let mut participants: Vec<ParticipantInfo> =
        Vec::with_capacity(api_game_data.participants.len());
    for p in api_game_data.participants {
        let stats = p.stats; // No need to clone anymore
        let kills = stats.kills.unwrap_or(0);
        let deaths = stats.deaths.unwrap_or(0);
        let assists = stats.assists.unwrap_or(0);
        let damage = stats.total_damage_dealt_to_champions.unwrap_or(0);
        let damage_taken = stats.total_damage_taken.unwrap_or(0);
        let gold = stats.gold_earned.unwrap_or(0);
        let vision = stats.vision_score.unwrap_or(0);
        let champ_level = stats.champ_level.unwrap_or(0);
        let champion_id = p.champion_id.unwrap_or(0);
        let team_id = p.team_id.unwrap_or(0);

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

        if damage > max_damage {
            max_damage = damage;
            best_player_champion_id = champion_id;
        }

        if damage_taken > max_tank {
            max_tank = damage_taken;
            max_tank_champion_id = champion_id;
        }

        let player_identity = player_map.get(&p.participant_id);
        let summoner_name = player_identity.map_or_else(String::new, |pi| {
            if let (Some(name), Some(tag)) = (&pi.game_name, &pi.tag_line) {
                if !name.is_empty() && !tag.is_empty() {
                    return format!("{}#{}", name, tag);
                }
            }
            if let Some(s_name) = &pi.summoner_name {
                return s_name.clone();
            }
            String::from("未知玩家")
        });
        let profile_icon_id = player_identity.map_or(0, |pi| pi.profile_icon);

        participants.push(ParticipantInfo {
            participant_id: p.participant_id,
            champion_id,
            summoner_name,
            profile_icon_id,
            team_id,
            rank_tier: None,
            score: None,
            stats: ParticipantStats {
                kills,
                deaths,
                assists,
                champ_level,
                gold_earned: gold,
                total_damage_dealt_to_champions: damage,
                total_damage_taken: damage_taken,
                vision_score: vision,
                item0: stats.item0,
                item1: stats.item1,
                item2: stats.item2,
                item3: stats.item3,
                item4: stats.item4,
                item5: stats.item5,
                item6: stats.item6,
            },
        });
    }

    Ok(GameDetail {
        game_id: api_game_data.game_id,
        game_duration: api_game_data.game_duration,
        game_creation: api_game_data.game_creation,
        game_mode: api_game_data.game_mode,
        game_type: api_game_data.game_type,
        game_version: api_game_data.game_version,
        map_id: api_game_data.map_id,
        queue_id: api_game_data.queue_id,
        teams: api_game_data.teams,
        participants,
        blue_team_stats,
        red_team_stats,
        best_player_champion_id,
        max_damage,
        max_tank_champion_id,
        max_tank,
        max_streak_champion_id,
        max_streak,
    })
}

/// 获取指定召唤师最近几场简单战绩
pub async fn get_recent_matches_by_puuid(
    client: &Client,
    puuid: &str,
    count: usize,
) -> Result<MatchStatistics, String> {
    let url = format!(
        "/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex={}",
        puuid, count
    );
    let match_list_data: Value = lcu_get(client, &url).await?;
    log::info!("match_list_data (查询到的战绩): {:#}", match_list_data); // 第3步：直接分析对局列表数据
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

    // 分析所有获取到的游戏
    let games_to_analyze = games.iter();

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
