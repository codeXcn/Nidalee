// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use serde::{Deserialize, Serialize};
use reqwest::{Client, header};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use anyhow::Result;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use lazy_static;

#[derive(Debug, Serialize, Deserialize)]
pub struct LcpConfig {
    token: String,
    port: String,
}

#[derive(Debug, Clone)]
pub struct LcpClient {
    config: LcpConfig,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameflowPhase {
    phase: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampSelectSession {
    actions: Vec<Vec<ChampSelectAction>>,
    my_team: Vec<ChampSelectPlayer>,
    their_team: Vec<ChampSelectPlayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampSelectAction {
    #[serde(rename = "actorCellId")]
    actor_cell_id: i32,
    #[serde(rename = "championId")]
    champion_id: i32,
    completed: bool,
    id: i32,
    #[serde(rename = "type")]
    action_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampSelectPlayer {
    #[serde(rename = "cellId")]
    cell_id: i32,
    #[serde(rename = "championId")]
    champion_id: i32,
    #[serde(rename = "summonerId")]
    summoner_id: i64,
    #[serde(rename = "assignedPosition")]
    assigned_position: String,
}

impl LcpClient {
    pub fn new(config: LcpConfig) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    fn get_base_url(&self) -> String {
        format!("https://127.0.0.1:{}", self.config.port)
    }

    fn get_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        let auth = format!("riot:{}", self.config.token);
        let auth_header = format!("Basic {}", BASE64.encode(auth));
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_header).unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers
    }

    pub async fn get_gameflow_phase(&self) -> Result<GameflowPhase> {
        let url = format!("{}/lol-gameflow/v1/gameflow-phase", self.get_base_url());
        let resp = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await?;
        
        let phase = resp.json().await?;
        Ok(phase)
    }

    pub async fn accept_matchmaking(&self) -> Result<()> {
        let url = format!("{}/lol-matchmaking/v1/ready-check/accept", self.get_base_url());
        self.client
            .post(&url)
            .headers(self.get_headers())
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_champ_select_session(&self) -> Result<ChampSelectSession> {
        let url = format!("{}/lol-champ-select/v1/session", self.get_base_url());
        let resp = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await?;
        
        let session = resp.json().await?;
        Ok(session)
    }

    pub async fn select_champion(&self, action_id: i32, champion_id: i32) -> Result<()> {
        let url = format!("{}/lol-champ-select/v1/session/actions/{}", self.get_base_url(), action_id);
        self.client
            .patch(&url)
            .headers(self.get_headers())
            .json(&serde_json::json!({
                "championId": champion_id,
                "completed": true
            }))
            .send()
            .await?;
        Ok(())
    }

    /// 获取玩家统计信息
    pub async fn get_player_stats(&self, summoner_id: i64) -> Result<PlayerStats> {
        // 获取玩家近期比赛记录
        let matches = self.get_recent_matches(summoner_id).await?;
        
        // 计算统计数据
        let mut stats = PlayerStats {
            total_games: matches.len() as i32,
            wins: 0,
            kda: 0.0,
            main_positions: Vec::new(),
            main_champions: Vec::new(),
            performance_score: 0,
        };

        // 位置和英雄使用统计
        let mut position_counts: HashMap<String, i32> = HashMap::new();
        let mut champion_counts: HashMap<i32, i32> = HashMap::new();
        let mut total_kills = 0;
        let mut total_deaths = 0;
        let mut total_assists = 0;

        for game in matches {
            // 统计胜场
            if game.stats.win {
                stats.wins += 1;
            }

            // 统计 KDA
            total_kills += game.stats.kills;
            total_deaths += game.stats.deaths;
            total_assists += game.stats.assists;

            // 统计位置
            *position_counts.entry(game.position.clone()).or_default() += 1;

            // 统计英雄
            *champion_counts.entry(game.champion_id).or_default() += 1;
        }

        // 计算 KDA
        stats.kda = if total_deaths > 0 {
            (total_kills as f32 + total_assists as f32) / total_deaths as f32
        } else {
            (total_kills + total_assists) as f32
        };

        // 获取主要位置
        stats.main_positions = position_counts.into_iter()
            .filter(|(_, count)| *count >= 3)
            .map(|(pos, _)| pos)
            .collect();

        // 获取主要英雄
        stats.main_champions = champion_counts.into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(champ, _)| champ)
            .collect();

        // 计算表现评分
        stats.performance_score = calculate_performance_score(
            stats.wins as f32 / stats.total_games as f32,
            stats.kda,
            &matches
        );

        Ok(stats)
    }

    /// 获取符文配置建议
    pub async fn get_rune_suggestion(&self, champion_id: i32) -> Result<RunePage> {
        // 从第三方 API 获取推荐符文配置
        let url = format!("https://api.example.com/runes/{}", champion_id);
        let rune_data = self.client
            .get(&url)
            .send()
            .await?
            .json::<RunePage>()
            .await?;

        Ok(rune_data)
    }

    /// 分析对局
    pub async fn analyze_match(&self, session: &ChampSelectSession) -> Result<MatchAnalysis> {
        let mut analysis = MatchAnalysis {
            team_score: 0,
            composition_score: 0,
            lane_advantages: HashMap::new(),
            teamfight_score: 0,
            suggested_tactics: Vec::new(),
        };

        // 获取所有玩家的统计信息
        let my_team_stats = futures::future::join_all(
            session.my_team.iter()
                .map(|p| self.get_player_stats(p.summoner_id))
        ).await;

        let enemy_team_stats = futures::future::join_all(
            session.their_team.iter()
                .map(|p| self.get_player_stats(p.summoner_id))
        ).await;

        // 分析阵容优势
        analysis.composition_score = analyze_team_composition(
            &session.my_team,
            &session.their_team
        );

        // 分析各路对线
        for (my_player, enemy_player) in session.my_team.iter().zip(session.their_team.iter()) {
            let advantage = analyze_lane_matchup(
                my_player.champion_id,
                enemy_player.champion_id,
                &my_team_stats,
                &enemy_team_stats
            );
            analysis.lane_advantages.insert(my_player.assigned_position.clone(), advantage);
        }

        // 分析团战能力
        analysis.teamfight_score = analyze_teamfight_potential(&session.my_team);

        // 生成战术建议
        analysis.suggested_tactics = generate_tactics(
            &analysis,
            &session.my_team,
            &session.their_team
        );

        Ok(analysis)
    }
}

// 游戏助手状态管理
#[derive(Debug)]
pub struct GameHelper {
    client: Arc<LcpClient>,
    auto_accept: Arc<Mutex<bool>>,
    auto_pick: Arc<Mutex<Option<i32>>>,
    auto_ban: Arc<Mutex<Option<i32>>>,
}

impl GameHelper {
    pub fn new(config: LcpConfig) -> Self {
        Self {
            client: Arc::new(LcpClient::new(config)),
            auto_accept: Arc::new(Mutex::new(true)),
            auto_pick: Arc::new(Mutex::new(None)),
            auto_ban: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_auto_accept(&self) {
        let client = self.client.clone();
        let auto_accept = self.auto_accept.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                
                if !*auto_accept.lock().await {
                    continue;
                }

                if let Ok(phase) = client.get_gameflow_phase().await {
                    if phase.phase == "ReadyCheck" {
                        let _ = client.accept_matchmaking().await;
                    }
                }
            }
        });
    }

    pub async fn start_auto_pick_ban(&self) {
        let client = self.client.clone();
        let auto_pick = self.auto_pick.clone();
        let auto_ban = self.auto_ban.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(500));
            loop {
                interval.tick().await;
                
                let pick_champ = *auto_pick.lock().await;
                let ban_champ = *auto_ban.lock().await;
                
                if pick_champ.is_none() && ban_champ.is_none() {
                    continue;
                }

                if let Ok(phase) = client.get_gameflow_phase().await {
                    if phase.phase != "ChampSelect" {
                        continue;
                    }

                    if let Ok(session) = client.get_champ_select_session().await {
                        for actions in session.actions {
                            for action in actions {
                                if action.completed {
                                    continue;
                                }

                                match action.action_type.as_str() {
                                    "pick" => {
                                        if let Some(champion_id) = pick_champ {
                                            let _ = client.select_champion(action.id, champion_id).await;
                                        }
                                    },
                                    "ban" => {
                                        if let Some(champion_id) = ban_champ {
                                            let _ = client.select_champion(action.id, champion_id).await;
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    // Tauri 命令接口
    #[tauri::command]
    pub async fn set_auto_accept(&self, enabled: bool) {
        *self.auto_accept.lock().await = enabled;
    }

    #[tauri::command]
    pub async fn set_auto_pick(&self, champion_id: Option<i32>) {
        *self.auto_pick.lock().await = champion_id;
    }

    #[tauri::command]
    pub async fn set_auto_ban(&self, champion_id: Option<i32>) {
        *self.auto_ban.lock().await = champion_id;
    }
}

/// 玩家统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerStats {
    /// 总场次
    pub total_games: i32,
    /// 胜场
    pub wins: i32,
    /// KDA
    pub kda: f32,
    /// 常用位置
    pub main_positions: Vec<String>,
    /// 常用英雄
    pub main_champions: Vec<i32>,
    /// 近期表现评分 (0-100)
    pub performance_score: i32,
}

/// 符文配置
#[derive(Debug, Serialize, Deserialize)]
pub struct RunePage {
    pub id: i32,
    pub name: String,
    pub primary_style_id: i32,
    pub sub_style_id: i32,
    pub selected_perk_ids: Vec<i32>,
}

/// 英雄数据
#[derive(Debug, Serialize, Deserialize)]
pub struct Champion {
    pub id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub win_rate: f32,
    pub ban_rate: f32,
    pub pick_rate: f32,
}

/// 对局分析结果
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchAnalysis {
    /// 队伍整体评分 (0-100)
    pub team_score: i32,
    /// 阵容优势评分 (-100 到 100，正数表示优势)
    pub composition_score: i32,
    /// 各路对线优势评分
    pub lane_advantages: HashMap<String, i32>,
    /// 团战能力评分 (0-100)
    pub teamfight_score: i32,
    /// 建议的战术
    pub suggested_tactics: Vec<String>,
}

// 辅助函数
fn calculate_performance_score(win_rate: f32, kda: f32, matches: &[Match]) -> i32 {
    let base_score = win_rate * 50.0 + (kda.min(5.0) / 5.0) * 30.0;
    let consistency_score = calculate_consistency(matches) * 20.0;
    (base_score + consistency_score) as i32
}

fn analyze_team_composition(my_team: &[ChampSelectPlayer], enemy_team: &[ChampSelectPlayer]) -> i32 {
    // 分析团战、分推、开团等能力
    let my_team_score = evaluate_team_strengths(my_team);
    let enemy_team_score = evaluate_team_strengths(enemy_team);
    my_team_score - enemy_team_score
}

fn analyze_lane_matchup(
    my_champion: i32,
    enemy_champion: i32,
    my_stats: &[Result<PlayerStats>],
    enemy_stats: &[Result<PlayerStats>]
) -> i32 {
    // 分析英雄克制关系和玩家熟练度
    let champion_advantage = get_champion_matchup_score(my_champion, enemy_champion);
    let player_skill_diff = compare_player_skills(my_stats, enemy_stats);
    (champion_advantage + player_skill_diff) / 2
}

fn analyze_teamfight_potential(team: &[ChampSelectPlayer]) -> i32 {
    // 分析团战能力
    let mut score = 0;
    let mut has_engage = false;
    let mut has_aoe_damage = false;
    let mut has_tank = false;

    for player in team {
        match get_champion_role(player.champion_id) {
            "Tank" => has_tank = true,
            "Engage" => has_engage = true,
            "AOE" => has_aoe_damage = true,
            _ => {}
        }
    }

    if has_engage { score += 30; }
    if has_aoe_damage { score += 40; }
    if has_tank { score += 30; }

    score
}

fn generate_tactics(
    analysis: &MatchAnalysis,
    my_team: &[ChampSelectPlayer],
    enemy_team: &[ChampSelectPlayer]
) -> Vec<String> {
    let mut tactics = Vec::new();

    // 根据分析结果生成战术建议
    if analysis.teamfight_score > 70 {
        tactics.push("建议抱团推进，寻找团战机会".to_string());
    }

    if analysis.composition_score < -30 {
        tactics.push("避免正面团战，主打分带和小规模战斗".to_string());
    }

    // 添加针对性战术建议
    for (position, advantage) in &analysis.lane_advantages {
        if *advantage > 30 {
            tactics.push(format!("{} 路优势明显，可以适当施压", position));
        } else if *advantage < -30 {
            tactics.push(format!("{} 路劣势，建议稳健发育", position));
        }
    }

    tactics
}

// 英雄角色定义
#[derive(Debug, Serialize, Deserialize)]
pub enum ChampionRole {
    Tank,
    Fighter,
    Assassin,
    Mage,
    Marksman,
    Support,
}

// 英雄数据缓存
lazy_static::lazy_static! {
    static ref CHAMPION_DATA: Mutex<HashMap<i32, Champion>> = Mutex::new(HashMap::new());
    static ref CHAMPION_MATCHUPS: Mutex<HashMap<(i32, i32), i32>> = Mutex::new(HashMap::new());
}

// Tauri 命令处理函数
#[tauri::command]
pub async fn get_champions() -> Result<Vec<Champion>, String> {
    let client = reqwest::Client::new();
    let url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champions.json";
    
    let champions = client.get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Champion>>()
        .await
        .map_err(|e| e.to_string())?;

    // 更新缓存
    let mut cache = CHAMPION_DATA.lock().await;
    for champion in &champions {
        cache.insert(champion.id, champion.clone());
    }

    Ok(champions)
}

#[tauri::command]
pub async fn get_game_info(state: tauri::State<'_, AppState>) -> Result<GameInfo, String> {
    if let Some(helper) = &state.game_helper {
        let session = helper.client.get_champ_select_session().await
            .map_err(|e| e.to_string())?;

        // 获取玩家统计信息
        let mut game_info = GameInfo {
            my_team: Vec::new(),
            their_team: Vec::new(),
        };

        // 处理我方队伍信息
        for player in session.my_team {
            let stats = helper.client.get_player_stats(player.summoner_id).await
                .ok();

            game_info.my_team.push(TeamPlayer {
                summoner: player.summoner,
                champion_id: player.champion_id,
                assigned_position: player.assigned_position,
                stats,
            });
        }

        // 处理敌方队伍信息
        for player in session.their_team {
            let stats = helper.client.get_player_stats(player.summoner_id).await
                .ok();

            game_info.their_team.push(TeamPlayer {
                summoner: player.summoner,
                champion_id: player.champion_id,
                assigned_position: player.assigned_position,
                stats,
            });
        }

        Ok(game_info)
    } else {
        Err("Game helper not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_rune_suggestion(champion_id: i32) -> Result<RunePage, String> {
    // 从第三方 API 获取符文数据
    let client = reqwest::Client::new();
    let url = format!("https://www.op.gg/_next/data/champion/{}/runes", champion_id);
    
    let rune_data = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<RunePage>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(rune_data)
}

#[tauri::command]
pub async fn analyze_match(game_info: GameInfo) -> Result<MatchAnalysis, String> {
    let champion_data = CHAMPION_DATA.lock().await;
    let matchups = CHAMPION_MATCHUPS.lock().await;

    let mut analysis = MatchAnalysis {
        team_score: 0,
        composition_score: 0,
        lane_advantages: HashMap::new(),
        teamfight_score: 0,
        suggested_tactics: Vec::new(),
    };

    // 分析阵容优势
    let my_team_roles = count_team_roles(&game_info.my_team, &champion_data);
    let enemy_team_roles = count_team_roles(&game_info.their_team, &champion_data);
    
    analysis.composition_score = evaluate_composition(
        &my_team_roles,
        &enemy_team_roles
    );

    // 分析各路对线
    for (my_player, enemy_player) in game_info.my_team.iter().zip(game_info.their_team.iter()) {
        let advantage = calculate_lane_advantage(
            my_player,
            enemy_player,
            &matchups,
            &champion_data
        );
        analysis.lane_advantages.insert(my_player.assigned_position.clone(), advantage);
    }

    // 分析团战能力
    analysis.teamfight_score = calculate_teamfight_score(
        &game_info.my_team,
        &champion_data
    );

    // 生成战术建议
    analysis.suggested_tactics = generate_tactics(
        &analysis,
        &game_info.my_team,
        &game_info.their_team,
        &champion_data
    );

    Ok(analysis)
}

#[tauri::command]
pub async fn apply_rune_page(
    state: tauri::State<'_, AppState>,
    rune_page: RunePage
) -> Result<(), String> {
    if let Some(helper) = &state.game_helper {
        helper.client.update_rune_page(rune_page.id, rune_page)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Game helper not initialized".to_string())
    }
}

// 辅助函数
fn count_team_roles(
    team: &[TeamPlayer],
    champion_data: &HashMap<i32, Champion>
) -> HashMap<ChampionRole, i32> {
    let mut role_counts = HashMap::new();
    
    for player in team {
        if let Some(champion) = champion_data.get(&player.champion_id) {
            for role in &champion.roles {
                *role_counts.entry(role.clone()).or_default() += 1;
            }
        }
    }
    
    role_counts
}

fn evaluate_composition(
    my_roles: &HashMap<ChampionRole, i32>,
    enemy_roles: &HashMap<ChampionRole, i32>
) -> i32 {
    let mut score = 0;
    
    // 评估阵容平衡性
    if my_roles.contains_key(&ChampionRole::Tank) { score += 20; }
    if my_roles.contains_key(&ChampionRole::Marksman) { score += 20; }
    if my_roles.contains_key(&ChampionRole::Support) { score += 20; }
    
    // 评估克制关系
    if my_roles.get(&ChampionRole::Assassin).unwrap_or(&0) > 0 &&
       enemy_roles.get(&ChampionRole::Marksman).unwrap_or(&0) > 0 {
        score += 15;
    }
    
    if my_roles.get(&ChampionRole::Tank).unwrap_or(&0) > 0 &&
       enemy_roles.get(&ChampionRole::Assassin).unwrap_or(&0) > 0 {
        score += 15;
    }
    
    score
}

fn calculate_lane_advantage(
    my_player: &TeamPlayer,
    enemy_player: &TeamPlayer,
    matchups: &HashMap<(i32, i32), i32>,
    champion_data: &HashMap<i32, Champion>
) -> i32 {
    let mut advantage = 0;
    
    // 英雄克制关系
    if let Some(&matchup_score) = matchups.get(&(my_player.champion_id, enemy_player.champion_id)) {
        advantage += matchup_score;
    }
    
    // 玩家熟练度
    if let (Some(my_stats), Some(enemy_stats)) = (&my_player.stats, &enemy_player.stats) {
        let my_mastery = calculate_champion_mastery(my_stats, my_player.champion_id);
        let enemy_mastery = calculate_champion_mastery(enemy_stats, enemy_player.champion_id);
        advantage += my_mastery - enemy_mastery;
    }
    
    advantage
}

fn calculate_champion_mastery(stats: &PlayerStats, champion_id: i32) -> i32 {
    if stats.main_champions.contains(&champion_id) {
        30
    } else {
        15
    }
}

fn calculate_teamfight_score(
    team: &[TeamPlayer],
    champion_data: &HashMap<i32, Champion>
) -> i32 {
    let mut score = 0;
    let mut has_engage = false;
    let mut has_aoe_damage = false;
    let mut has_tank = false;
    
    for player in team {
        if let Some(champion) = champion_data.get(&player.champion_id) {
            for role in &champion.roles {
                match role {
                    ChampionRole::Tank => has_tank = true,
                    ChampionRole::Mage => has_aoe_damage = true,
                    ChampionRole::Fighter => has_engage = true,
                    _ => {}
                }
            }
        }
    }
    
    if has_engage { score += 30; }
    if has_aoe_damage { score += 40; }
    if has_tank { score += 30; }
    
    score
}

fn generate_tactics(
    analysis: &MatchAnalysis,
    my_team: &[TeamPlayer],
    enemy_team: &[TeamPlayer],
    champion_data: &HashMap<i32, Champion>
) -> Vec<String> {
    let mut tactics = Vec::new();
    
    // 根据阵容评分生成建议
    if analysis.teamfight_score > 70 {
        tactics.push("建议抱团推进，寻找团战机会".to_string());
    }
    
    if analysis.composition_score < -30 {
        tactics.push("避免正面团战，主打分带和小规模战斗".to_string());
    }
    
    // 根据对线优势生成建议
    for (position, advantage) in &analysis.lane_advantages {
        if *advantage > 30 {
            tactics.push(format!("{} 路优势明显，可以适当施压", position));
        } else if *advantage < -30 {
            tactics.push(format!("{} 路劣势，建议稳健发育", position));
        }
    }
    
    // 分析关键威胁
    let mut threats = Vec::new();
    for player in enemy_team {
        if let Some(champion) = champion_data.get(&player.champion_id) {
            if champion.roles.contains(&ChampionRole::Assassin) {
                threats.push(format!("小心 {} 的突进切后排", champion.name));
            }
        }
    }
    
    tactics.extend(threats);
    tactics
}
