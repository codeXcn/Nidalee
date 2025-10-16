use crate::lcu::player_stats_analyzer::{analyze_player_stats, analyze_traits, AnalysisContext};
use crate::lcu::request::{lcu_get, lcu_request_json};
use crate::lcu::types::{
    GameDetail, ParticipantInfo, ParticipantStats, PlayerMatchStats, TeamInfo, TeamStats,
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
///
/// # 参数
/// - `client`: HTTP 客户端
/// - `end_count`: 获取对局数量
/// - `queue_id`: 可选的队列ID过滤（如 420=单排, 440=灵活排, 450=大乱斗）
pub async fn get_match_history(
    client: &Client,
    end_count: usize,
    queue_id: Option<i32>,
) -> Result<PlayerMatchStats, String> {
    println!("\n🔍 ===== 开始获取我的战绩 =====");
    if let Some(qid) = queue_id {
        println!("🎯 队列过滤: queueId={}", qid);
    }

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
    println!(
        "🔢 请求的对局数量: end_count={}, safe_end={}, actual_end_index={}",
        end_count, safe_end, actual_end_index
    );
    let match_list_url = format!(
        "/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex={}",
        puuid, actual_end_index
    );
    println!("🌐 请求URL: {}", match_list_url);
    let match_list_data: Value = lcu_get(client, &match_list_url).await?;

    // 第3步：直接分析对局列表数据
    println!("\n📍 第3步：分析对局列表数据");
    let statistics = analyze_match_list_data(match_list_data, puuid, queue_id)?;

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
    let max_streak = 0;
    let max_streak_champion_id = 0;

    let player_map: HashMap<i32, ApiPlayer> = api_game_data
        .participant_identities
        .into_iter()
        .map(|p| (p.participant_id, p.player))
        .collect();

    let mut participants: Vec<ParticipantInfo> = Vec::with_capacity(api_game_data.participants.len());
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
///
/// # 参数
/// - `client`: HTTP 客户端
/// - `puuid`: 玩家 PUUID
/// - `count`: 获取对局数量
/// - `queue_id`: 可选的队列ID，用于过滤（如 420=单排, 440=灵活排）
pub async fn get_recent_matches_by_puuid(
    client: &Client,
    puuid: &str,
    count: usize,
    queue_id: Option<i32>,
) -> Result<PlayerMatchStats, String> {
    let url = format!(
        "/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex={}",
        puuid, count
    );
    let match_list_data: Value = lcu_get(client, &url).await?;
    // log::info!("match_list_data (查询到的战绩): {:#}", match_list_data);
    // 第3步：直接分析对局列表数据
    let statistics = analyze_match_list_data(match_list_data, puuid, queue_id)?;
    Ok(statistics)
}

fn analyze_match_list_data(
    match_list_data: Value,
    current_puuid: &str,
    queue_id: Option<i32>,
) -> Result<PlayerMatchStats, String> {
    println!("📊 开始分析对局列表数据 (使用通用分析器)");
    println!("👤 目标玩家PUUID: {}", current_puuid);

    let empty_games = vec![];
    let games = match_list_data
        .get("games")
        .and_then(|games_obj| games_obj.get("games"))
        .and_then(|g| g.as_array())
        .unwrap_or(&empty_games);

    println!("📊 找到 {} 场对局记录", games.len());

    // ===使用通用分析器进行数据计算===
    let mut context = AnalysisContext::new();

    // 根据队列ID设置分析上下文（精确匹配）
    if let Some(qid) = queue_id {
        context = context.with_queue_id(qid);
        println!("🎯 队列过滤: 精确匹配 queueId={}", qid);
    } else {
        println!("🎯 队列过滤: 无过滤，显示所有对局");
    }

    let mut player_stats = analyze_player_stats(games, current_puuid, context);

    // 添加特征分析
    let traits = analyze_traits(&player_stats);
    player_stats.traits = traits;

    println!("✅ 分析完成: 总对局={}, 胜场={}, 胜率={:.1}%",
        player_stats.total_games, player_stats.wins, player_stats.win_rate);
    println!("📊 今日对局: {}/{}", player_stats.today_wins, player_stats.today_games);
    println!("🏷️  识别特征: {}个", player_stats.traits.len());

    Ok(player_stats)
}
