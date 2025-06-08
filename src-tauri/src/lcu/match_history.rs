use base64::{engine::general_purpose, Engine as _};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::lcu::auth::get_lcu_auth_info;
use crate::lcu::types::{LcuAuthInfo, GameDetail, TeamInfo, BanInfo, ParticipantInfo, ParticipantStats, TeamStats};

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchStatistics {
    pub total_games: i32,
    pub wins: i32,
    pub losses: i32,
    pub win_rate: f32,
    pub avg_kills: f32,
    pub avg_deaths: f32,
    pub avg_assists: f32,
    pub avg_kda: f32,
    pub favorite_champions: Vec<ChampionStats>,
    pub recent_performance: Vec<RecentGame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampionStats {
    pub champion_name: String,
    pub games_played: i32,
    pub wins: i32,
    pub win_rate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentGame {
    pub game_id: u64,
    pub champion_name: String,
    pub game_mode: String,
    pub win: bool,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub game_duration: i32,
    pub game_creation: i64,
}

pub async fn get_match_history() -> Result<MatchStatistics, String> {
    println!("\n🔍 ===== 开始获取我的战绩 =====");

    let auth_info = get_lcu_auth_info()?;
    println!("🔐 认证信息获取成功:");
    println!("   - 端口: {}", auth_info.app_port);
    println!("   - Token前缀: {}...", &auth_info.remoting_auth_token[0..8.min(auth_info.remoting_auth_token.len())]);

    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());
    println!("   - Base64认证: {}...", &base64_auth[0..20.min(base64_auth.len())]);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // 第1步：获取当前召唤师信息来得到PUUID
    println!("\n📍 第1步：获取当前召唤师信息");
    let summoner_url = format!("https://127.0.0.1:{}/lol-summoner/v1/current-summoner", auth_info.app_port);
    println!("🌐 请求URL: {}", summoner_url);

    let summoner_response = client
        .get(&summoner_url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| {
            println!("❌ 获取召唤师信息失败: {}", e);
            format!("获取召唤师信息失败: {}", e)
        })?;

    println!("📊 召唤师信息响应状态: {}", summoner_response.status());

    let summoner_data: Value = summoner_response
        .json()
        .await
        .map_err(|e| {
            println!("❌ 解析召唤师信息失败: {}", e);
            format!("解析召唤师信息失败: {}", e)
        })?;

    let puuid = summoner_data
        .get("puuid")
        .and_then(|p| p.as_str())
        .ok_or_else(|| {
            println!("❌ 未找到PUUID");
            "未找到PUUID".to_string()
        })?;

    println!("🆔 提取到的PUUID: {}", puuid);

    // 第2步：使用PUUID获取对局列表
    println!("\n📍 第2步：使用PUUID获取对局列表");
    let match_list_url = format!("https://127.0.0.1:{}/lol-match-history/v1/products/lol/{}/matches", auth_info.app_port, puuid);
    println!("🌐 请求URL: {}", match_list_url);

    let match_list_response = client
        .get(&match_list_url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| {
            println!("❌ 获取对局列表失败: {}", e);
            format!("获取对局列表失败: {}", e)
        })?;

    println!("📊 对局列表响应状态: {}", match_list_response.status());

    let match_list_data: Value = match_list_response
        .json()
        .await
        .map_err(|e| {
            println!("❌ 解析对局列表失败: {}", e);
            format!("解析对局列表失败: {}", e)
        })?;

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

pub async fn get_game_detail(game_id: u64) -> Result<Value, String> {
    println!("\n🔍 ===== 获取游戏详细信息 =====");
    println!("🎮 游戏ID: {}", game_id);

    let auth_info = get_lcu_auth_info()?;
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    // 获取游戏详细信息
    let game_detail_url = format!("https://127.0.0.1:{}/lol-match-history/v1/games/{}", auth_info.app_port, game_id);
    println!("🌐 请求URL: {}", game_detail_url);

    let response = client
        .get(&game_detail_url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("获取游戏详情失败: {}", e))?;

    println!("📊 响应状态: {}", response.status());

    let game_data: Value = response
        .json()
        .await
        .map_err(|e| format!("解析游戏详情JSON失败: {}", e))?;

    // 处理游戏数据
    let mut blue_team_stats = TeamStats {
        kills: 0,
        gold_earned: 0,
        total_damage_dealt_to_champions: 0,
        vision_score: 0,
    };

    let mut red_team_stats = TeamStats {
        kills: 0,
        gold_earned: 0,
        total_damage_dealt_to_champions: 0,
        vision_score: 0,
    };

    let mut max_damage = 0;
    let mut best_player_champion_id = 0;
    let mut max_tank = 0;
    let mut max_tank_champion_id = 0;
    let mut max_streak = 0;
    let mut max_streak_champion_id = 0;

    // 处理参与者数据
    let participants = if let Some(participants_data) = game_data.get("participants").and_then(|p| p.as_array()) {
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
                    "championName": get_champion_name(champion_id as u64),
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
       let participant_identities = game_data.get("participantIdentities").cloned().unwrap_or(Value::Null);
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

fn analyze_match_list_data(match_list_data: Value, current_puuid: &str) -> Result<MatchStatistics, String> {
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

    // 只分析前10场游戏
    let games_to_analyze = games.iter().take(10);

    for (index, game) in games_to_analyze.enumerate() {
        println!("\n🎮 分析第 {} 场游戏", index + 1);
        total_games += 1;

        // 查找当前玩家的参与者信息
        if let Some(participant_identities) = game.get("participantIdentities").and_then(|pi| pi.as_array()) {
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
                        let p_id = p.get("participantId").and_then(|id| id.as_u64()).unwrap_or(0);
                        p_id == participant_id
                    });

                    if let Some(participant) = current_participant {
                        let stats = &participant["stats"];
                        let champion_id = participant
                            .get("championId")
                            .and_then(|id| id.as_u64())
                            .unwrap_or(0);
                        let champion_name = get_champion_name(champion_id);
                        let is_win = stats.get("win").and_then(|w| w.as_bool()).unwrap_or(false);
                        let kills = stats.get("kills").and_then(|k| k.as_i64()).unwrap_or(0) as i32;
                        let deaths = stats.get("deaths").and_then(|d| d.as_i64()).unwrap_or(0) as i32;
                        let assists = stats.get("assists").and_then(|a| a.as_i64()).unwrap_or(0) as i32;

                        println!("🏆 英雄: {}", champion_name);
                        println!("🎯 结果: {}", if is_win { "胜利" } else { "失败" });
                        println!("⚔️  KDA: {}/{}/{}", kills, deaths, assists);

                        if is_win {
                            wins += 1;
                        }

                        total_kills += kills;
                        total_deaths += deaths;
                        total_assists += assists;

                        // 统计英雄数据
                        let entry = champion_stats.entry(champion_name.clone()).or_insert((0, 0));
                        entry.0 += 1; // 游戏数
                        if is_win {
                            entry.1 += 1; // 胜场数
                        }

                        // 添加到最近游戏
                        recent_performance.push(RecentGame {
                            game_id: game
                                .get("gameId")
                                .and_then(|id| id.as_u64())
                                .unwrap_or(0),
                            champion_name,
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

    let avg_kda = if avg_deaths > 0.0 {
        (avg_kills + avg_assists) / avg_deaths
    } else {
        avg_kills + avg_assists
    };

    // 转换英雄统计
    let mut favorite_champions: Vec<ChampionStats> = champion_stats
        .into_iter()
        .map(|(name, (games, wins))| ChampionStats {
            champion_name: name,
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
    favorite_champions.truncate(5); // 只保留前5个

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

// 英雄ID到名称的映射（部分常见英雄）
fn get_champion_name(champion_id: u64) -> String {
    match champion_id {
        1 => "安妮".to_string(),
        2 => "奥拉夫".to_string(),
        3 => "加里奥".to_string(),
        4 => "卡牌大师".to_string(),
        5 => "赵信".to_string(),
        6 => "厄加特".to_string(),
        7 => "乐芙兰".to_string(),
        8 => "弗拉基米尔".to_string(),
        9 => "费德提克".to_string(),
        10 => "凯尔".to_string(),
        11 => "易大师".to_string(),
        12 => "牛头酋长".to_string(),
        13 => "瑞兹".to_string(),
        14 => "塞恩".to_string(),
        15 => "希维尔".to_string(),
        16 => "索拉卡".to_string(),
        17 => "提莫".to_string(),
        18 => "崔斯塔娜".to_string(),
        19 => "沃里克".to_string(),
        20 => "努努和威朗普".to_string(),
        21 => "赏金猎人".to_string(),
        22 => "艾希".to_string(),
        23 => "崔斯塔娜".to_string(),
        24 => "贾克斯".to_string(),
        25 => "莫甘娜".to_string(),
        26 => "时光守护者".to_string(),
        27 => "辛吉德".to_string(),
        28 => "伊芙琳".to_string(),
        29 => "图奇".to_string(),
        30 => "卡尔萨斯".to_string(),
        31 => "虚空恐惧".to_string(),
        32 => "木乃伊".to_string(),
        33 => "拉莫斯".to_string(),
        34 => "冰晶凤凰".to_string(),
        35 => "恶魔小丑".to_string(),
        36 => "祖安狂人".to_string(),
        37 => "琴女".to_string(),
        38 => "虚空行者".to_string(),
        39 => "刀锋舞者".to_string(),
        40 => "风暴之怒".to_string(),
        41 => "海盗船长".to_string(),
        42 => "英勇投弹手".to_string(),
        43 => "天启者".to_string(),
        44 => "瓦洛兰之盾".to_string(),
        45 => "邪恶小法师".to_string(),
        48 => "巨魔之王".to_string(),
        50 => "破败之王".to_string(),
        51 => "皮城女警".to_string(),
        53 => "蒸汽机器人".to_string(),
        54 => "熔岩巨兽".to_string(),
        55 => "不祥之刃".to_string(),
        56 => "虚空遁地兽".to_string(),
        57 => "扭曲树精".to_string(),
        58 => "荒漠屠夫".to_string(),
        59 => "德玛西亚皇子".to_string(),
        60 => "蜘蛛女皇".to_string(),
        61 => "发条魔灵".to_string(),
        62 => "齐天大圣".to_string(),
        63 => "复仇焰魂".to_string(),
        64 => "盲僧".to_string(),
        67 => "暗夜猎手".to_string(),
        68 => "机械公敌".to_string(),
        69 => "魅惑魔女".to_string(),
        72 => "水晶先锋".to_string(),
        74 => "大发明家".to_string(),
        75 => "沙漠死神".to_string(),
        76 => "狂野女猎手".to_string(),
        77 => "兽灵行者".to_string(),
        78 => "圣锤之毅".to_string(),
        79 => "酒桶".to_string(),
        80 => "不屈之枪".to_string(),
        81 => "探险家".to_string(),
        82 => "铁铠冥魂".to_string(),
        83 => "牧魂人".to_string(),
        84 => "离群之刺".to_string(),
        85 => "狂电之心".to_string(),
        86 => "德玛西亚之力".to_string(),
        89 => "曙光女神".to_string(),
        90 => "虚空先知".to_string(),
        91 => "刀锋之影".to_string(),
        92 => "放逐之刃".to_string(),
        96 => "深渊巨口".to_string(),
        98 => "暮光之眼".to_string(),
        99 => "光辉女郎".to_string(),
        101 => "远古巫灵".to_string(),
        102 => "龙血武姬".to_string(),
        103 => "九尾妖狐".to_string(),
        104 => "法外狂徒".to_string(),
        105 => "潮汐海灵".to_string(),
        106 => "雷霆咆哮".to_string(),
        107 => "傲之追猎者".to_string(),
        110 => "惩戒之箭".to_string(),
        111 => "深海泰坦".to_string(),
        112 => "机械先驱".to_string(),
        113 => "北地之怒".to_string(),
        114 => "无双剑姬".to_string(),
        115 => "爆破鬼才".to_string(),
        117 => "仙灵女巫".to_string(),
        119 => "荣耀行刑官".to_string(),
        120 => "战争之影".to_string(),
        121 => "虚空掠夺者".to_string(),
        122 => "蛮族之王".to_string(),
        126 => "未来守护者".to_string(),
        127 => "冰霜女巫".to_string(),
        131 => "皎月女神".to_string(),
        133 => "德玛西亚之翼".to_string(),
        134 => "暗黑元首".to_string(),
        136 => "铸星龙王".to_string(),
        141 => "影流之镰".to_string(),
        142 => "暮光星灵".to_string(),
        143 => "荆棘之兴".to_string(),
        145 => "虚空之女".to_string(),
        147 => "星籁歌姬".to_string(),
        150 => "迷失之牙".to_string(),
        154 => "生化魔人".to_string(),
        157 => "疾风剑豪".to_string(),
        161 => "虚空之眼".to_string(),
        163 => "岩雀".to_string(),
        164 => "青钢影".to_string(),
        166 => "影哨".to_string(),
        200 => "圣枪游侠".to_string(),
        201 => "弗雷尔卓德之心".to_string(),
        202 => "戏命师".to_string(),
        203 => "永猎双子".to_string(),
        222 => "暴走萝莉".to_string(),
        223 => "河流之王".to_string(),
        234 => "破败之王".to_string(),
        235 => "涤魂圣枪".to_string(),
        236 => "圣枪游侠".to_string(),
        238 => "影流之主".to_string(),
        240 => "暴怒骑士".to_string(),
        245 => "时间刺客".to_string(),
        246 => "元素女皇".to_string(),
        254 => "皮城执法官".to_string(),
        266 => "暗裔剑魔".to_string(),
        267 => "唤潮鲛姬".to_string(),
        268 => "沙漠皇帝".to_string(),
        350 => "魔法猫咪".to_string(),
        360 => "沙漠玫瑰".to_string(),
        412 => "魂锁典狱长".to_string(),
        420 => "海兽祭司".to_string(),
        421 => "虚空遁地兽".to_string(),
        427 => "翠神".to_string(),
        429 => "复仇之矛".to_string(),
        432 => "星界游神".to_string(),
        518 => "万花通灵".to_string(),
        523 => "残月之肃".to_string(),
        526 => "山隐之焰".to_string(),
        555 => "血港鬼影".to_string(),
        711 => "愁云使者".to_string(),
        777 => "封魔剑魂".to_string(),
        875 => "腕豪".to_string(),
        876 => "含羞蓓蕾".to_string(),
        887 => "灵罗娃娃".to_string(),
        888 => "祖安花火".to_string(),
        895 => "不羁之悦".to_string(),
        897 => "K'sante".to_string(),
        901 => "永恒梦魇".to_string(),
        902 => "明烛".to_string(),
        950 => "百裂冥犬".to_string(),
        _ => format!("英雄_{}", champion_id),
    }
}

