// LCU 数据结构定义
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LcuAuthInfo {
    pub app_port: u16,
    pub remoting_auth_token: String,
    pub riotclient_app_port: u16,
    pub riotclient_auth_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameDetail {
    pub game_id: u64,
    pub game_duration: i32,
    pub game_creation: i64,
    pub game_mode: String,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i32,
    pub queue_id: i32,
    pub teams: Vec<TeamInfo>,
    pub participants: Vec<ParticipantInfo>,
    pub blue_team_stats: TeamStats,
    pub red_team_stats: TeamStats,
    pub best_player_champion_id: i32,
    pub max_damage: i32,
    pub max_tank_champion_id: i32,
    pub max_tank: i32,
    pub max_streak_champion_id: i32,
    pub max_streak: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamInfo {
    pub team_id: i32,
    pub win: String,
    pub bans: Vec<BanInfo>,
    pub baron_kills: i32,
    pub dragon_kills: i32,
    pub tower_kills: i32,
    pub inhibitor_kills: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanInfo {
    pub champion_id: i32,
    pub pick_turn: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantInfo {
    pub participant_id: i32,
    pub champion_id: i32,
    pub champion_name: String,
    pub team_id: i32,
    pub rank_tier: Option<String>,
    pub stats: ParticipantStats,
    pub score: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantStats {
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub champ_level: i32,
    pub gold_earned: i32,
    pub total_damage_dealt_to_champions: i32,
    pub total_damage_taken: i32,
    pub vision_score: i32,
    pub item0: Option<i32>,
    pub item1: Option<i32>,
    pub item2: Option<i32>,
    pub item3: Option<i32>,
    pub item4: Option<i32>,
    pub item5: Option<i32>,
    pub item6: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamStats {
    pub kills: i32,
    pub gold_earned: i32,
    pub total_damage_dealt_to_champions: i32,
    pub vision_score: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameflowPhase {
  pub phase: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LobbyInfo {
  pub id: String,
  pub partyType: String,
  pub members: Vec<LobbyMember>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LobbyMember {
  pub summonerId: u64,
  pub displayName: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SummonerInfo {
    // 基本信息
    pub displayName: String,
    #[serde(default)]
    pub gameName: Option<String>,
    #[serde(default)]
    pub tagLine: Option<String>,
    pub summonerLevel: i64,
    pub profileIconId: i64,
    pub puuid: String,
    pub accountId: i64,
    pub summonerId: i64,

    // 经验信息
    pub xpSinceLastLevel: i64,
    pub xpUntilNextLevel: i64,
    #[serde(default)]
    pub percentCompleteForNextLevel: Option<f64>,

    // 游戏状态
    #[serde(default)]
    pub gameStatus: Option<String>,
    #[serde(default)]
    pub availability: Option<String>,

    // 挑战系统
    #[serde(default)]
    pub challengePoints: Option<String>,
    #[serde(default)]
    pub challengeCrystalLevel: Option<String>,

    // 排位信息 - 单人排位
    #[serde(default)]
    pub soloRankTier: Option<String>,
    #[serde(default)]
    pub soloRankDivision: Option<String>,
    #[serde(default)]
    pub soloRankWins: Option<i32>,
    #[serde(default)]
    pub soloRankLosses: Option<i32>,
    #[serde(default)]
    pub soloRankLP: Option<i32>,

    // 排位信息 - 灵活排位
    #[serde(default)]
    pub flexRankTier: Option<String>,
    #[serde(default)]
    pub flexRankDivision: Option<String>,
    #[serde(default)]
    pub flexRankWins: Option<i32>,
    #[serde(default)]
    pub flexRankLosses: Option<i32>,
    #[serde(default)]
    pub flexRankLP: Option<i32>,

    // 历史最高排位
    #[serde(default)]
    pub highestRankThisSeason: Option<String>,

    // 天赋信息
    #[serde(default)]
    pub currentPerkPage: Option<String>,
    #[serde(default)]
    pub primaryStyleId: Option<i32>,
    #[serde(default)]
    pub subStyleId: Option<i32>,
    #[serde(default)]
    pub selectedPerkIds: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankedStats {
  pub queueMap: std::collections::HashMap<String, QueueStats>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueueStats {
  pub tier: String,
  pub division: String,
  pub leaguePoints: u32,
  pub wins: u32,
  pub losses: u32,
}

// 轮询状态结构体
#[derive(Debug, Clone)]
pub struct PollState {
  pub is_lcu_running: bool,
  pub auth_info: Option<LcuAuthInfo>,
  pub current_summoner: Option<SummonerInfo>,
  pub gameflow_phase: Option<String>,
  pub in_lobby: bool,
}
