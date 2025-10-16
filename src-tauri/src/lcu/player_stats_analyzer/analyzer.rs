use crate::lcu::types::{AnalysisChampionStats, MatchPerformance, PlayerMatchStats};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// 分析上下文
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    /// 当前队列ID（用于过滤相关对局）
    pub current_queue_id: Option<i32>,
    /// 是否只分析排位赛（420=单双排，440=灵活组排）
    pub ranked_only: bool,
}

impl Default for AnalysisContext {
    fn default() -> Self {
        Self {
            current_queue_id: None,
            ranked_only: false,
        }
    }
}

impl AnalysisContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_queue_id(mut self, queue_id: i32) -> Self {
        self.current_queue_id = Some(queue_id);
        self
    }

    #[allow(dead_code)]
    pub fn ranked_only(mut self) -> Self {
        self.ranked_only = true;
        self
    }
}

/// 通用玩家战绩分析器
///
/// 输入：原始对局数据 (JSON)
/// 输出：完整的 PlayerMatchStats（包含所有计算好的字段）
pub fn analyze_player_stats(
    games: &[Value],
    puuid: &str,
    context: AnalysisContext,
) -> PlayerMatchStats {
    // 获取今天零点的时间戳（毫秒）
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let current_ms = since_epoch.as_millis() as i64;
    // 计算今天零点（UTC时区）
    let today_start_ms = (current_ms / 86400000) * 86400000;

    // 过滤相关对局
    let relevant_games: Vec<&Value> = games
        .iter()
        .filter(|game| {
            let queue_id = game["queueId"].as_i64().unwrap_or(0) as i32;

            // 根据上下文过滤
            if let Some(current_queue) = context.current_queue_id {
                // 精确匹配队列ID
                queue_id == current_queue
            } else if context.ranked_only {
                // 只统计排位赛（无具体队列时使用）
                queue_id == 420 || queue_id == 440
            } else {
                true // 无限制，显示所有对局
            }
        })
        .collect();

    let total_games = relevant_games.len() as u32;
    if total_games == 0 {
        return PlayerMatchStats::default();
    }

    // 初始化统计变量
    let mut wins = 0u32;
    let mut today_games = 0u32;
    let mut today_wins = 0u32;
    let mut total_kills = 0.0;
    let mut total_deaths = 0.0;
    let mut total_assists = 0.0;
    let mut total_duration_secs = 0.0;
    let mut total_damage_to_champs = 0.0;
    let mut total_vision_score = 0.0;
    let mut total_cs = 0.0;
    let mut favorite_champions_map: HashMap<i32, (u32, u32)> = HashMap::new();
    let mut recent_performance = Vec::new();

    // 遍历所有对局
    for game in &relevant_games {
        // 找到当前玩家的参与者信息
        if let Some(participant_id) = game["participantIdentities"]
            .as_array()
            .and_then(|ids| {
                ids.iter()
                    .find(|id| id["player"]["puuid"].as_str() == Some(puuid))
            })
            .and_then(|id| id["participantId"].as_i64())
        {
            if let Some(participant) = game["participants"]
                .as_array()
                .and_then(|parts| {
                    parts
                        .iter()
                        .find(|part| part["participantId"].as_i64() == Some(participant_id))
                })
            {
                let stats = &participant["stats"];
                let win = stats["win"].as_bool().unwrap_or(false);
                let kills = stats["kills"].as_i64().unwrap_or(0) as f64;
                let deaths = stats["deaths"].as_i64().unwrap_or(0) as f64;
                let assists = stats["assists"].as_i64().unwrap_or(0) as f64;
                let game_duration = game["gameDuration"].as_i64().unwrap_or(0) as f64;

                // 基础统计
                if win {
                    wins += 1;
                }
                total_kills += kills;
                total_deaths += deaths;
                total_assists += assists;
                total_duration_secs += game_duration;

                // 衍生指标数据
                total_damage_to_champs += stats["totalDamageDealtToChampions"]
                    .as_i64()
                    .unwrap_or(0) as f64;
                total_vision_score += stats["visionScore"].as_i64().unwrap_or(0) as f64;
                total_cs += (stats["totalMinionsKilled"].as_i64().unwrap_or(0)
                    + stats["neutralMinionsKilled"].as_i64().unwrap_or(0))
                    as f64;

                // 今日统计
                if let Some(creation_ms) = game["gameCreation"].as_i64() {
                    // 如果对局创建时间在今天零点之后，则算作今日对局
                    if creation_ms >= today_start_ms {
                        today_games += 1;
                        if win {
                            today_wins += 1;
                        }
                    }
                }

                // 常用英雄统计
                if let Some(champion_id) = participant["championId"].as_i64() {
                    let entry = favorite_champions_map
                        .entry(champion_id as i32)
                        .or_insert((0, 0));
                    entry.0 += 1;
                    if win {
                        entry.1 += 1;
                    }
                }

                // 最近战绩
                recent_performance.push(MatchPerformance {
                    game_id: game["gameId"].as_u64(),
                    win,
                    champion_id: participant["championId"].as_i64().unwrap_or(0) as i32,
                    champion_name: String::new(), // 会在外部填充
                    kills: kills as i32,
                    deaths: deaths as i32,
                    assists: assists as i32,
                    kda: if deaths > 0.0 {
                        (kills + assists) / deaths
                    } else {
                        kills + assists
                    },
                    game_duration: Some(game_duration as i32),
                    game_creation: game["gameCreation"].as_i64(),
                    queue_id: game["queueId"].as_i64(),
                    game_mode: game["gameMode"].as_str().map(String::from),
                });
            }
        }
    }

    // 计算平均值和衍生指标
    let total_duration_mins = if total_duration_secs > 0.0 {
        total_duration_secs / 60.0
    } else {
        1.0 // 避免除零
    };

    let avg_kills = if total_games > 0 {
        total_kills / total_games as f64
    } else {
        0.0
    };
    let avg_deaths = if total_games > 0 {
        total_deaths / total_games as f64
    } else {
        0.0
    };
    let avg_assists = if total_games > 0 {
        total_assists / total_games as f64
    } else {
        0.0
    };
    let avg_kda = if total_deaths > 0.0 {
        (total_kills + total_assists) / total_deaths
    } else {
        total_kills + total_assists
    };

    let dpm = total_damage_to_champs / total_duration_mins;
    let cspm = total_cs / total_duration_mins;
    let vspm = total_vision_score / total_duration_mins;

    // 处理常用英雄
    let mut favorite_champions: Vec<AnalysisChampionStats> = favorite_champions_map
        .into_iter()
        .map(|(champion_id, (games, wins))| AnalysisChampionStats {
            champion_id,
            champion_name: String::new(), // 会在外部填充
            games,
            wins,
            win_rate: if games > 0 {
                (wins as f64 / games as f64) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    // 按游戏场次排序
    favorite_champions.sort_by(|a, b| b.games.cmp(&a.games));

    // 构建结果（注意：traits 将由 traits_analyzer 填充）
    PlayerMatchStats {
        total_games,
        wins,
        losses: total_games - wins,
        win_rate: if total_games > 0 {
            (wins as f64 / total_games as f64) * 100.0
        } else {
            0.0
        },
        avg_kills,
        avg_deaths,
        avg_assists,
        avg_kda,
        today_games,
        today_wins,
        dpm,
        cspm,
        vspm,
        traits: Vec::new(), // 由 traits_analyzer 填充
        favorite_champions,
        recent_performance,
    }
}

