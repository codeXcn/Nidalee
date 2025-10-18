use crate::domains::analysis::{
    analyze_advanced_traits, analyze_distribution_traits, analyze_player_stats,
    analyze_role_based_traits, analyze_timeline_traits, analyze_traits, analyze_win_loss_pattern,
    identify_main_role, identify_player_roles, optimize_traits, parse_games,
    AnalysisContext, AnalysisStrategy,
};
use crate::domains::tactical_advice::generate_advice;
use crate::shared::utils::{lcu_get, lcu_request_json};
use crate::lcu::types::{
    AdvicePerspective, GameAdvice, GameDetail, ParticipantInfo, ParticipantStats, PlayerMatchStats, TeamInfo, TeamStats,
};
use reqwest::{Client, Method};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

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
/// - `perspective`: 建议视角（默认SelfImprovement）
/// - `target_name`: 目标玩家名称（用于建议措辞）
pub async fn get_recent_matches_by_puuid(
    client: &Client,
    puuid: &str,
    count: usize,
    queue_id: Option<i32>,
) -> Result<PlayerMatchStats, String> {
    get_recent_matches_by_puuid_with_perspective(
        client,
        puuid,
        count,
        queue_id,
        AdvicePerspective::SelfImprovement,
        None,
    )
    .await
}

/// 获取指定召唤师最近几场战绩（支持指定建议视角）
///
/// 用于 TeamAnalysis 时可以指定不同的视角
pub async fn get_recent_matches_by_puuid_with_perspective(
    client: &Client,
    puuid: &str,
    count: usize,
    queue_id: Option<i32>,
    perspective: AdvicePerspective,
    target_name: Option<String>,
) -> Result<PlayerMatchStats, String> {
    let url = format!(
        "/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex={}",
        puuid, count
    );
    let match_list_data: Value = lcu_get(client, &url).await?;
    // 第3步：直接分析对局列表数据，传入视角参数
    let statistics = analyze_match_list_data_with_perspective(
        match_list_data,
        puuid,
        queue_id,
        perspective,
        target_name,
    )?;
    Ok(statistics)
}

fn analyze_match_list_data(
    match_list_data: Value,
    current_puuid: &str,
    queue_id: Option<i32>,
) -> Result<PlayerMatchStats, String> {
    // 默认使用 SelfImprovement 视角（用于Dashboard）
    analyze_match_list_data_with_perspective(
        match_list_data,
        current_puuid,
        queue_id,
        AdvicePerspective::SelfImprovement,
        None,
    )
}

/// 分析对局列表数据（支持指定建议视角）⭐
///
/// 用于 TeamAnalysis 时可以为敌方生成 Targeting，为队友生成 Collaboration
fn analyze_match_list_data_with_perspective(
    match_list_data: Value,
    current_puuid: &str,
    queue_id: Option<i32>,
    advice_perspective: AdvicePerspective,
    target_player_name: Option<String>,
) -> Result<PlayerMatchStats, String> {
    println!("📊 开始分析对局列表数据 (使用优化架构: Parser + Strategy)");
    println!("👤 目标玩家PUUID: {}", current_puuid);

    let empty_games = vec![];
    let games = match_list_data
        .get("games")
        .and_then(|games_obj| games_obj.get("games"))
        .and_then(|g| g.as_array())
        .unwrap_or(&empty_games);

    println!("📊 找到 {} 场对局记录", games.len());

    // === 第1步: Parser 层 - 解析原始数据为统一格式 ===
    let parsed_games = parse_games(games, current_puuid);
    println!("✅ Parser: 解析了 {} 场对局数据", parsed_games.len());

    // === 第2步: Strategy 层 - 根据队列选择分析策略 ===
    let strategy = if let Some(qid) = queue_id {
        let strategy = AnalysisStrategy::from_queue_id(qid as i64);
        println!("🎯 Strategy: {} (queueId={})", strategy.description(), qid);
        strategy
    } else {
        let strategy = AnalysisStrategy::from_games(&parsed_games);
        println!("🎯 Strategy: {} (自动推断)", strategy.description());
        strategy
    };
    println!("📊 分析方案: {}", strategy.analysis_layers());

    // === 第3步: Analyzer 层 - 使用解析后的数据进行统计计算 ===
    let mut context = AnalysisContext::new();

    // 根据队列ID设置分析上下文（精确匹配）
    if let Some(qid) = queue_id {
        context = context.with_queue_id(qid);
    }

    let mut player_stats = analyze_player_stats(&parsed_games, current_puuid, context);

    // === 第4步: 多层次特征分析（根据策略决定分析深度）===
    let mut traits = Vec::new();

    // 第1层：基础特征（所有模式都执行）
    traits.extend(analyze_traits(&player_stats));

    // 第2-5层：深度分析（仅排位模式）
    if strategy.enable_advanced_analysis() {
        // 第2层：深度特征（参团率、伤害占比、稳定性、趋势）
        traits.extend(analyze_advanced_traits(&player_stats, games, current_puuid));
    }

    if strategy.enable_role_analysis() {
        // 第3层：位置特征（基于位置的专项分析）
        let role_stats_map = identify_player_roles(games, current_puuid);
        traits.extend(analyze_role_based_traits(&player_stats, &role_stats_map));
    }

    if strategy.enable_distribution_analysis() {
        // 第4层：分布特征（高光时刻、崩盘场次、稳定性）
        traits.extend(analyze_distribution_traits(&player_stats.recent_performance));

        // ⭐ 第4.5层：时间线特征（对线压制、稳定发育、爆发成长）NEW
        let main_role = identify_main_role(&parsed_games);
        traits.extend(analyze_timeline_traits(&parsed_games, &main_role));
        println!("✅ 时间线分析：识别对线期和发育曲线特征");
    }

    // 第5层：胜负模式（所有模式都执行，但其他模式会简化）
    traits.extend(analyze_win_loss_pattern(&player_stats.recent_performance));

    // === 第5步: 智能去重优化 ===
    // 根据策略限制特征数量
    let max_traits = strategy.max_traits();
    let mut optimized_traits = optimize_traits(traits);
    optimized_traits.truncate(max_traits);
    player_stats.traits = optimized_traits;

    // === 第6步: 生成智能建议（仅排位模式）⭐ v3.0 ===
    if matches!(strategy, AnalysisStrategy::Ranked) {
        let main_role = identify_main_role(&parsed_games);
        player_stats.advice = generate_advice(
            &player_stats,
            &parsed_games,
            &main_role,
            advice_perspective,  // 使用传入的视角 ⭐
            target_player_name,  // 使用传入的目标名称 ⭐
            &strategy,
        );
        println!("💡 建议生成：共 {} 条建议（视角：{:?}）", 
            player_stats.advice.len(), advice_perspective);
    }

    println!("✅ 分析完成 ({:?}):", strategy);
    println!("   总对局={}, 胜场={}, 胜率={:.1}%",
        player_stats.total_games, player_stats.wins, player_stats.win_rate);
    println!("   今日对局: {}/{}", player_stats.today_wins, player_stats.today_games);
    println!("   识别特征: {}个 (限制{}个)", player_stats.traits.len(), max_traits);
    println!("   智能建议: {}条", player_stats.advice.len());

    Ok(player_stats)
}

/// 获取指定玩家的战术建议
///
/// 用于针对敌人或协作队友生成建议
pub async fn get_player_tactical_advice(
    client: &Client,
    summoner_name: String,
    perspective: AdvicePerspective,
    target_role: Option<String>,
) -> Result<Vec<GameAdvice>, String> {
    println!("🎯 开始获取玩家战术建议");
    println!("   召唤师: {}", summoner_name);
    println!("   视角: {:?}", perspective);

    // 1. 根据召唤师名称获取PUUID
    let summoner_url = format!(
        "/lol-summoner/v1/summoners?name={}",
        utf8_percent_encode(&summoner_name, NON_ALPHANUMERIC)
    );
    
    let summoner_response: Value = lcu_request_json(&client, Method::GET, &summoner_url, None)
        .await
        .map_err(|e| format!("获取召唤师信息失败: {}", e))?;
    
    let puuid = summoner_response["puuid"]
        .as_str()
        .ok_or("无法获取玩家PUUID")?
        .to_string();
    
    println!("✅ 获取到PUUID: {}", puuid);

    // 2. 获取玩家战绩（最近20场排位）
    let match_history_url = format!(
        "/lol-match-history/v1/products/lol/{}/matches?begIndex=0&endIndex=20",
        puuid
    );
    
    let match_list_response: Value = lcu_request_json(&client, Method::GET, &match_history_url, None)
        .await
        .map_err(|e| format!("获取战绩失败: {}", e))?;
    
    let match_list_data = match_list_response["games"]["games"]
        .as_array()
        .ok_or("无法解析战绩数据")?;
    
    println!("✅ 获取到{}场对局", match_list_data.len());

    if match_list_data.is_empty() {
        return Ok(Vec::new());
    }

    // 3. 解析战绩数据
    let parsed_games = parse_games(match_list_data, &puuid);
    println!("✅ 解析完成：{} 场对局", parsed_games.len());

    // 4. 确定分析策略（默认使用排位策略，因为我们需要深度分析）
    let strategy = AnalysisStrategy::Ranked;

    // 5. 分析玩家数据
    let context = AnalysisContext::new().with_queue_id(420); // 排位模式
    let player_stats = analyze_player_stats(&parsed_games, &puuid, context);
    
    // 6. 识别主要位置
    let main_role = target_role.unwrap_or_else(|| identify_main_role(&parsed_games));
    println!("✅ 主要位置: {}", main_role);

    // 7. 生成建议（使用指定的视角）
    let advice = generate_advice(
        &player_stats,
        &parsed_games,
        &main_role,
        perspective,
        Some(summoner_name.clone()),  // 传递目标玩家名称
        &strategy,
    );

    println!("💡 生成建议：共 {} 条", advice.len());

    Ok(advice)
}
