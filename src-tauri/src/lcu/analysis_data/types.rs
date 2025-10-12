use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 玩家完整分析数据（包含战绩）
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/PlayerAnalysisData.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAnalysisData {
    // 基础信息
    pub cell_id: i32,
    pub display_name: String,
    pub summoner_id: Option<String>,
    pub puuid: Option<String>,
    pub is_local: bool,
    pub is_bot: bool,

    // 英雄信息
    pub champion_id: Option<i32>,
    pub champion_name: Option<String>,
    pub champion_pick_intent: Option<i32>,
    pub position: Option<String>,

    // 召唤师信息
    pub tier: Option<String>,
    pub profile_icon_id: Option<i32>,
    pub tag_line: Option<String>,
    pub spell1_id: Option<i32>,
    pub spell2_id: Option<i32>,

    // 战绩数据（只有真实玩家才有）
    pub match_stats: Option<PlayerMatchStats>,
}

/// 玩家战绩统计
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/PlayerMatchStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchStats {
    pub total_games: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: f64,

    // KDA统计
    pub avg_kills: f64,
    pub avg_deaths: f64,
    pub avg_assists: f64,
    pub avg_kda: f64,

    // 常用英雄
    pub favorite_champions: Vec<ChampionStats>,

    // 最近战绩
    pub recent_performance: Vec<MatchPerformance>,
}

/// 英雄统计数据
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/AnalysisChampionStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub champion_id: i32,
    pub champion_name: String,
    pub games: u32,
    pub wins: u32,
    pub win_rate: f64,
}

/// 单场比赛表现
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/MatchPerformance.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct MatchPerformance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<u64>,
    pub win: bool,
    pub champion_id: i32,
    pub champion_name: String,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub kda: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
}

/// 队伍分析数据
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/TeamAnalysisData.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct TeamAnalysisData {
    pub my_team: Vec<PlayerAnalysisData>,
    pub enemy_team: Vec<PlayerAnalysisData>,
    pub local_player_cell_id: i32,
    pub game_phase: String,
    pub queue_id: i64, // 队列类型ID：420=单排, 440=灵活排位, 450=大乱斗等
    pub is_custom_game: bool, // 是否自定义游戏

    // 🔥 新增：选人流程相关字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Vec<crate::lcu::types::ChampSelectAction>>>, // 选人/ban 动作序列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bans: Option<crate::lcu::types::ChampSelectBans>, // ban 位信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer: Option<crate::lcu::types::ChampSelectTimer>, // 计时器信息
}
