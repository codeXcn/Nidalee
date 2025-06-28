// LCU 数据结构定义
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

/// 兼容数字和字符串的反序列化 helper
pub fn string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> de::Visitor<'de> for StringOrNumberVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or number")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}

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

#[derive(Debug, Serialize, Deserialize, Default)]
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
#[serde(rename_all = "camelCase")]
pub struct LobbyInfo {
    pub id: String,
    pub party_type: String,
    pub members: Vec<LobbyMember>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyMember {
    #[serde(deserialize_with = "crate::lcu::types::string_or_number")]
    pub summoner_id: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    // 基本信息
    pub display_name: String,
    pub game_name: Option<String>,
    pub tag_line: Option<String>,
    pub summoner_level: i64,
    pub profile_icon_id: i64,
    pub puuid: String,
    #[serde(deserialize_with = "crate::lcu::types::string_or_number")]
    pub account_id: String,
    #[serde(deserialize_with = "crate::lcu::types::string_or_number")]
    pub summoner_id: String,

    // 经验信息
    pub xp_since_last_level: i64,
    pub xp_until_next_level: i64,
    pub percent_complete_for_next_level: Option<f64>,

    // 游戏状态
    pub game_status: Option<String>,
    pub availability: Option<String>,

    // 挑战系统
    pub challenge_points: Option<String>,
    pub challenge_crystal_level: Option<String>,

    // 排位信息 - 单人排位
    pub solo_rank_tier: Option<String>,
    pub solo_rank_division: Option<String>,
    pub solo_rank_wins: Option<i32>,
    pub solo_rank_losses: Option<i32>,
    pub solo_rank_lp: Option<i32>,

    // 排位信息 - 灵活排位
    pub flex_rank_tier: Option<String>,
    pub flex_rank_division: Option<String>,
    pub flex_rank_wins: Option<i32>,
    pub flex_rank_losses: Option<i32>,
    pub flex_rank_lp: Option<i32>,

    // 历史最高排位
    pub highest_rank_this_season: Option<String>,

    // 天赋信息
    pub current_perk_page: Option<String>,
    pub primary_style_id: Option<i32>,
    pub sub_style_id: Option<i32>,
    pub selected_perk_ids: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RankedStats {
    pub queue_map: std::collections::HashMap<String, QueueStats>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueStats {
    pub tier: String,
    pub division: String,
    pub league_points: u32,
    pub wins: u32,
    pub losses: u32,
}

// 英雄选择阶段关键信息（用于推荐）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectSession {
    pub local_player_cell_id: i32,
    pub my_team: Vec<ChampSelectPlayer>,
    pub their_team: Vec<ChampSelectPlayer>,
    pub bans: ChampSelectBans,
    pub timer: ChampSelectTimer,
    pub actions: Vec<Vec<ChampSelectAction>>, // 使用 Option<serde_json::Value> 以兼容不同类型
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectAction {
    pub actor_cell_id: Option<i32>,
    pub champion_id: Option<i32>,
    pub completed: bool,
    pub id: i32,
    pub is_ally_action: Option<bool>,
    pub is_in_progress: Option<bool>,
    pub pick_turn: Option<i32>,
    #[serde(rename = "type")]
    pub action_type: String, // "pick" 或 "ban"
    pub is_current_user: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectPlayer {
    pub cell_id: i32,
    pub puuid: Option<String>, // 使用 String 类型来兼容数字和字符串
    pub summoner_id: Option<String>,
    pub champion_id: Option<f64>,
    pub champion_pick_intent: Option<f64>,
    pub selected_skin_id: Option<f64>,
    pub spell1_id: Option<f64>,
    pub spell2_id: Option<f64>,
    pub assigned_position: Option<String>,
    pub display_name: Option<String>,
    pub tag_line: Option<String>,
    pub profile_icon_id: Option<i64>,
    pub tier: Option<String>,
    pub recent_matches: Option<Vec<SimpleMatchInfo>>, // 新增
}

#[allow(dead_code)]
impl ChampSelectPlayer {
    // 检查是否是有效的召唤师技能ID
    pub fn is_valid_spell_id(&self, spell_id: Option<f64>) -> bool {
        if let Some(id) = spell_id {
            // 检查是否是 u64::MAX 的浮点数表示
            if id == 1.8446744073709552e19 {
                return false;
            }
            // 检查是否是 0
            if id == 0.0 {
                return false;
            }
            true
        } else {
            false
        }
    }

    // 获取有效的召唤师技能ID
    pub fn get_valid_spell1_id(&self) -> Option<f64> {
        if self.is_valid_spell_id(self.spell1_id) {
            self.spell1_id
        } else {
            None
        }
    }

    pub fn get_valid_spell2_id(&self) -> Option<f64> {
        if self.is_valid_spell_id(self.spell2_id) {
            self.spell2_id
        } else {
            None
        }
    }

    // 检查是否是有效的英雄ID
    pub fn is_valid_champion_id(&self) -> bool {
        self.champion_id.map_or(false, |id| id > 0.0)
    }

    // 检查是否是有效的皮肤ID
    pub fn is_valid_skin_id(&self) -> bool {
        if let Some(skin_id) = self.selected_skin_id {
            skin_id > 0.0
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectBans {
    pub my_team_bans: Vec<Option<f64>>,
    pub their_team_bans: Vec<Option<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectTimer {
    pub phase: String,
}
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RankInfo {
    pub solo_tier: Option<String>,
    pub solo_division: Option<String>,
    pub solo_lp: Option<i32>,
    pub solo_wins: Option<i32>,
    pub solo_losses: Option<i32>,
    pub flex_tier: Option<String>,
    pub flex_division: Option<String>,
    pub flex_lp: Option<i32>,
    pub flex_wins: Option<i32>,
    pub flex_losses: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingState {
    pub errors: Vec<MatchmakingError>,
    pub low_priority_data: LowPriorityData,
    pub search_state: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingError {
    pub error_type: String,
    pub id: i32,
    pub message: String,
    pub penalized_summoner_id: i64,
    pub penalty_time_remaining: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LowPriorityData {
    pub busted_leaver_access_token: String,
    pub penalized_summoner_ids: Vec<i64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchInfo {
    pub match_id: String,
    pub players: Vec<PlayerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerInfo {
    pub summoner_name: String,
    pub champion_id: i32,
    pub team_id: i32,
}

/// 当前选择的英雄信息
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CurrentChampion {
    /// 英雄ID
    pub champion_id: i32,
    /// 选择的皮肤ID
    pub selected_skin_id: i32,
    /// 是否已锁定
    pub is_locked: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct SimpleMatchInfo {
    pub game_id: u64,
    pub champion_id: i32,
    pub win: bool,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub game_creation: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChampionStats {
    pub champion_id: i32,
    pub games_played: i32,
    pub wins: i32,
    pub win_rate: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentGame {
    pub game_id: u64,
    pub champion_id: i32,
    pub game_mode: String,
    pub win: bool,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub game_duration: i32,
    pub game_creation: i64,
    pub queue_id: i64,
    pub performance_rating: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChampSelectPlayerInfo {
    pub summoner_id: String,
    pub display_name: String,
    pub tag_line: Option<String>,
    pub profile_icon_id: i64,
    pub tier: Option<String>,
    pub puuid: String,
    pub recent_matches: Vec<SimpleMatchInfo>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChampSelectTeamInfo {
    pub my_team: Vec<ChampSelectPlayerInfo>,
    pub their_team: Vec<ChampSelectPlayerInfo>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerWithMatches {
    pub display_name: String,
    pub summoner_info: SummonerInfo,
    pub matches: MatchStatistics,
}
