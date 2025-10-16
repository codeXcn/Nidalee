// LCU 数据结构定义
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use ts_rs::TS;

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

/// 兼容数字和字符串的 Option<String> 反序列化 helper
pub fn option_string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionStringOrNumberVisitor;

    impl<'de> de::Visitor<'de> for OptionStringOrNumberVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("null, string or number")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            string_or_number(deserializer).map(Some)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_owned()))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }
    }

    deserializer.deserialize_option(OptionStringOrNumberVisitor)
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/LiveClientPlayer.ts",
    rename_all = "camelCase"
)]
pub struct LiveClientPlayer {
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "championName")]
    pub champion_name: String,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
    #[serde(rename = "isDead")]
    pub is_dead: bool,
    #[ts(type = "any")]
    pub items: Vec<Value>,
    pub level: i32,
    pub position: String,
    #[serde(rename = "rawChampionName")]
    pub raw_champion_name: String,
    #[serde(rename = "respawnTimer")]
    pub respawn_timer: f64,
    #[ts(type = "any")]
    pub runes: Value,
    #[ts(type = "any")]
    pub scores: Value,
    #[serde(rename = "skinID")]
    pub skin_id: i32,
    #[ts(type = "any")]
    #[serde(rename = "summonerSpells")]
    pub summoner_spells: Value,
    pub team: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/LcuAuthInfo.ts",
    rename_all = "camelCase"
)]
pub struct LcuAuthInfo {
    pub app_port: u16,
    pub remoting_auth_token: String,
    pub riotclient_app_port: u16,
    pub riotclient_auth_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/GameDetail.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/TeamInfo.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct TeamInfo {
    #[serde(default)]
    pub team_id: Option<i32>,
    #[serde(default)]
    pub win: Option<String>,
    #[serde(default)]
    pub bans: Vec<BanInfo>,
    #[serde(default)]
    pub baron_kills: Option<i32>,
    #[serde(default)]
    pub dragon_kills: Option<i32>,
    #[serde(default)]
    pub tower_kills: Option<i32>,
    #[serde(default)]
    pub inhibitor_kills: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/types/generated/BanInfo.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct BanInfo {
    #[serde(default)]
    pub champion_id: Option<i32>,
    #[serde(default)]
    pub pick_turn: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ParticipantInfo.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantInfo {
    pub participant_id: i32,
    pub champion_id: i32,
    pub summoner_name: String,
    #[ts(type = "number")]
    pub profile_icon_id: i64,
    pub team_id: i32,
    pub rank_tier: Option<String>,
    pub stats: ParticipantStats,
    pub score: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ParticipantStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize, Default, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/TeamStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct TeamStats {
    pub kills: i32,
    pub gold_earned: i32,
    pub total_damage_dealt_to_champions: i32,
    pub vision_score: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/GameflowPhase.ts",
    rename_all = "camelCase"
)]
pub struct GameflowPhase {
    pub phase: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LobbyInfo {
    #[serde(default)]
    pub can_start_activity: bool,
    #[serde(default)]
    pub game_config: serde_json::Value, // 使用 Value 因为结构复杂
    #[serde(default)]
    pub invitations: Vec<serde_json::Value>,
    #[serde(default)]
    pub local_member: Option<LobbyMember>,
    #[serde(default)]
    pub members: Vec<LobbyMember>,
    #[serde(default)]
    pub muc_jwt_dto: Option<serde_json::Value>,
    #[serde(default)]
    pub multi_user_chat_id: String,
    #[serde(default)]
    pub multi_user_chat_password: String,
    #[serde(default)]
    pub party_id: String,
    #[serde(default)]
    pub party_type: String,
    #[serde(default)]
    pub restrictions: Vec<serde_json::Value>,
    #[serde(default)]
    pub warnings: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/LobbyMember.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct LobbyMember {
    // 基本信息
    #[ts(type = "string")]
    #[serde(deserialize_with = "crate::lcu::types::string_or_number")]
    pub summoner_id: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub puuid: String,
    #[serde(default)]
    pub summoner_name: String,
    #[ts(type = "number")]
    #[serde(default)]
    pub summoner_level: i64,
    #[ts(type = "number")]
    #[serde(default)]
    pub summoner_icon_id: i64,

    // 权限
    #[serde(default)]
    pub is_bot: bool,
    #[serde(default)]
    pub is_leader: bool,
    #[serde(default)]
    pub is_spectator: bool,
    #[serde(default)]
    pub ready: bool,

    // 机器人相关
    #[ts(type = "number")]
    #[serde(default)]
    pub bot_champion_id: i32,
    #[serde(default)]
    pub bot_difficulty: String,
    #[serde(default)]
    pub bot_id: String,
    #[serde(default)]
    pub bot_position: String,

    // 位置偏好
    #[serde(default)]
    pub first_position_preference: String,
    #[serde(default)]
    pub second_position_preference: String,

    // 其他字段（使用 default 避免解析失败）
    #[serde(default)]
    pub allowed_change_activity: bool,
    #[serde(default)]
    pub allowed_invite_others: bool,
    #[serde(default)]
    pub allowed_kick_others: bool,
    #[serde(default)]
    pub allowed_start_activity: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/SummonerInfo.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    // 基本信息
    pub display_name: String,
    pub game_name: Option<String>,
    pub tag_line: Option<String>,
    #[ts(type = "number")]
    pub summoner_level: i64,
    #[ts(type = "number")]
    pub profile_icon_id: i64,
    pub puuid: String,
    #[ts(type = "string")]
    #[serde(deserialize_with = "crate::lcu::types::string_or_number")]
    pub account_id: String,
    #[ts(type = "string")]
    #[serde(deserialize_with = "crate::lcu::types::string_or_number")]
    pub summoner_id: String,

    // 经验信息
    #[ts(type = "number")]
    pub xp_since_last_level: i64,
    #[ts(type = "number")]
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

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RankedStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RankedStats {
    pub queue_map: std::collections::HashMap<String, QueueStats>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/QueueStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct QueueStats {
    pub tier: String,
    pub division: String,
    pub league_points: u32,
    pub wins: u32,
    pub losses: u32,
}

// 英雄选择阶段关键信息（用于推荐）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampSelectSession.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectSession {
    pub local_player_cell_id: i32,
    pub my_team: Vec<ChampSelectPlayer>,
    pub their_team: Vec<ChampSelectPlayer>,
    pub bans: ChampSelectBans,
    pub timer: ChampSelectTimer,
    pub actions: Vec<Vec<ChampSelectAction>>, // 使用 Option<serde_json::Value> 以兼容不同类型
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampSelectAction.ts",
    rename_all = "camelCase"
)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampSelectPlayer.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectPlayer {
    pub cell_id: i32,
    pub puuid: Option<String>, // 使用 String 类型来兼容数字和字符串
    #[ts(type = "string | null")]
    #[serde(deserialize_with = "option_string_or_number")]
    pub summoner_id: Option<String>, // 支持整数和字符串两种格式
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampSelectBans.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectBans {
    pub my_team_bans: Vec<Option<f64>>,
    pub their_team_bans: Vec<Option<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampSelectTimer.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectTimer {
    pub phase: String,
}
#[derive(Debug, Default, PartialEq, Eq, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RankInfo.ts",
    rename_all = "camelCase"
)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/MatchmakingState.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingState {
    pub errors: Vec<MatchmakingError>,
    pub low_priority_data: LowPriorityData,
    pub search_state: String,
    pub estimated_queue_time: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/MatchmakingError.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingError {
    pub error_type: String,
    pub id: i32,
    pub message: String,
    pub penalized_summoner_id: i64,
    pub penalty_time_remaining: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/LowPriorityData.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct LowPriorityData {
    pub busted_leaver_access_token: String,
    pub penalized_summoner_ids: Vec<i64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/MatchInfo.ts",
    rename_all = "camelCase"
)]
pub struct MatchInfo {
    pub match_id: String,
    pub players: Vec<PlayerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/PlayerInfo.ts",
    rename_all = "camelCase"
)]
pub struct PlayerInfo {
    pub summoner_name: String,
    pub champion_id: i32,
    pub team_id: i32,
}

/// 当前选择的英雄信息
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/CurrentChampion.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct CurrentChampion {
    /// 英雄ID
    pub champion_id: i32,
    /// 英雄名称
    pub champion_name: String,
    /// 是否已选择
    pub is_picked: bool,
}

/// 符文页面数据结构
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RunePage.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RunePage {
    pub id: i64,
    pub name: String,
    pub current: bool,
    pub is_editable: bool,
    #[serde(rename = "isDeletable")]
    pub is_deletable: bool,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    pub primary_style_id: i32,
    pub sub_style_id: i32,
    pub selected_perk_ids: Vec<i32>,
}

/// 创建符文页面的请求结构
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRunePageRequest {
    pub name: String,
    pub primary_style_id: i32,
    pub sub_style_id: i32,
    pub selected_perk_ids: Vec<i32>,
}

/// 装备推荐套装数据结构
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/types/generated/ItemSet.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ItemSet {
    pub title: String,
    pub champion: String,
    pub mode: String,
    pub map: String,
    pub blocks: Vec<ItemBlock>,
}

/// 装备推荐块数据结构
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ItemBlock.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ItemBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub items: Vec<RecommendedItem>,
}

/// 推荐装备项数据结构
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RecommendedItem.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedItem {
    pub id: String,
    pub count: i32,
}

/// 简单对局信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/SimpleMatchInfo.ts",
    rename_all = "camelCase"
)]
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

/// 对局统计信息 (已废弃，请使用 PlayerMatchStats)
/// 保留此类型别名仅为向后兼容
#[deprecated(
    since = "2.0.0",
    note = "请使用 PlayerMatchStats 代替，它包含更完整的分析数据"
)]
pub type MatchStatistics = PlayerMatchStats;

/// 英雄统计信息
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampionStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub champion_id: i32,
    pub games_played: i32,
    pub wins: i32,
    pub win_rate: f32,
}

/// 分析用英雄统计数据（包含英雄名称）
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/AnalysisChampionStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisChampionStats {
    pub champion_id: i32,
    pub champion_name: String,
    pub games: u32,
    pub wins: u32,
    pub win_rate: f64,
}

/// 最近游戏信息
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RecentGame.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RecentGame {
    #[ts(type = "number")]
    pub game_id: u64,
    pub champion_id: i32,
    pub game_mode: String,
    pub win: bool,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub game_duration: i32,
    #[ts(type = "number")]
    pub game_creation: i64,
    #[ts(type = "number")]
    pub queue_id: i64,
    pub performance_rating: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampSelectPlayerInfo.ts",
    rename_all = "camelCase"
)]
pub struct ChampSelectPlayerInfo {
    pub summoner_id: String,
    pub display_name: String,
    pub tag_line: Option<String>,
    pub profile_icon_id: i64,
    pub tier: Option<String>,
    pub puuid: String,
    pub recent_matches: Vec<SimpleMatchInfo>,
}
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampSelectTeamInfo.ts",
    rename_all = "camelCase"
)]
pub struct ChampSelectTeamInfo {
    pub my_team: Vec<ChampSelectPlayerInfo>,
    pub their_team: Vec<ChampSelectPlayerInfo>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/SummonerWithMatches.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct SummonerWithMatches {
    pub display_name: String,
    pub summoner_info: SummonerInfo,
    pub matches: MatchStatistics,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[ts(export, export_to = "../../src/types/generated/ConnectionState.ts")]
pub enum ConnectionState {
    Connected,
    ProcessFound,
    Unstable,
    AuthExpired,
    Disconnected,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampionDataResponse.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampionDataResponse {
    pub id: Option<i32>,
    pub source: Option<String>,
    pub version: Option<String>,
    pub champion_alias: Option<String>,
    pub champion_id: Option<String>,
    pub content: Vec<ChampionContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampionContent.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampionContent {
    pub alias: String,
    pub id: String,
    pub index: Option<i32>,
    pub name: String,
    pub position: String,
    pub version: String,
    pub win_rate: Option<String>,
    pub pick_count: Option<i32>,
    pub timestamp: Option<i64>,
    pub official_version: Option<String>,
    pub item_builds: Option<Vec<ItemBuild>>,
    pub runes: Option<Vec<RuneSet>>,
    pub skills: Option<Vec<String>>,
    #[ts(skip)]
    pub spells: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ItemBuild.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ItemBuild {
    pub title: String,
    pub map: Option<String>,
    pub mode: Option<String>,
    pub r#type: Option<String>,
    pub sortrank: Option<f64>,
    pub started_from: Option<String>,
    pub associated_champions: Option<Vec<i32>>,
    pub associated_maps: Option<Vec<i32>>,
    #[ts(skip)]
    pub preferred_item_slots: Option<Vec<serde_json::Value>>,
    pub blocks: Option<Vec<ChampionItemBlock>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ChampionItemBlock.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ChampionItemBlock {
    #[serde(rename = "type")]
    pub block_type: Option<String>,
    pub items: Option<Vec<ItemInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/ItemInfo.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfo {
    pub id: String,
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/RuneSet.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RuneSet {
    pub alias: String,
    pub name: Option<String>,
    pub position: Option<String>,
    pub r#type: Option<String>,
    pub win_rate: Option<String>,
    pub pick_count: Option<i32>,
    pub score: Option<f64>,
    pub primary_style_id: Option<i32>,
    pub sub_style_id: Option<i32>,
    pub selected_perk_ids: Option<Vec<i32>>,
}

// 符文系统相关类型定义
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RuneSystem.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RuneSystem {
    pub icon: String,
    pub id: i32,
    pub key: String,
    pub name: String,
    pub slots: Vec<RuneSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RuneSlot.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RuneSlot {
    pub runes: Vec<RuneOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RuneOption.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RuneOption {
    pub icon: String,
    pub id: i32,
    pub key: String,
    pub long_desc: String,
    pub name: String,
    pub short_desc: String,
}

// 完整的符文数据响应
pub type AllRunesResponse = Vec<RuneSystem>;

// 保留原有的 DataDragonRune 和 RunePages 类型作为兼容
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/DataDragonRune.ts")]
pub struct DataDragonRune {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub short_desc: String,
    pub long_desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/RunePages.ts")]
pub struct RunePages {
    pub pages: Vec<DataDragonRune>,
}

// 保留原有的类型定义作为兼容
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/BuildSection.ts")]
pub struct BuildSection {
    pub title: String,
    pub item_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/Builds.ts")]
pub struct Builds {
    pub builds: Vec<BuildSection>,
}

/// LCU WebSocket 事件
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/LcuWebSocketEvent.ts",
    rename_all = "camelCase"
)]
pub struct LcuWebSocketEvent<T> {
    pub event_type: String,
    pub data: T,
}

// LCU 符文样式相关类型定义
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/RuneStyle.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct RuneStyle {
    pub allowed_sub_styles: Vec<i64>,
    pub asset_map: HashMap<String, String>,
    pub default_page_name: String,
    pub default_perks: Vec<i64>,
    pub default_sub_style: i64,
    pub icon_path: String,
    pub id: i64,
    pub id_name: String,
    pub name: String,
    pub slots: Vec<Slot>,
    pub sub_style_bonus: Vec<SubStyleBonus>,
    pub tooltip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/Slot.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub perks: Vec<i64>,
    pub slot_label: String,
    pub r#type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/SubStyleBonus.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct SubStyleBonus {
    pub perk_id: i64,
    pub style_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/Perk.ts", rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    pub icon_path: String,
    pub id: i64,
    pub long_desc: String,
    pub name: String,
    pub recommendation_descriptor: String,
    pub short_desc: String,
    pub slot_type: String,
    pub style_id: i64,
    pub style_id_name: String,
    pub tooltip: String,
}

// === 队伍分析相关类型 ===

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
    pub spell1_id: Option<i64>, // 改为 i64 以支持大数值
    pub spell2_id: Option<i64>, // 改为 i64 以支持大数值

    // 战绩数据（只有真实玩家才有）
    pub match_stats: Option<PlayerMatchStats>,
}

/// 召唤师特征标签
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "../../src/types/generated/SummonerTrait.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct SummonerTrait {
    pub name: String,
    pub description: String,
    pub score: i32,
    #[serde(rename = "type")]
    pub trait_type: String, // "good" or "bad"
}

/// 玩家战绩统计（完整版 - 包含所有分析数据）
#[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
#[ts(
    export,
    export_to = "../../src/types/generated/PlayerMatchStats.ts",
    rename_all = "camelCase"
)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchStats {
    // === 基础统计 ===
    pub total_games: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: f64,

    // === KDA 统计 ===
    pub avg_kills: f64,
    pub avg_deaths: f64,
    pub avg_assists: f64,
    pub avg_kda: f64,

    // === 今日统计 ===
    pub today_games: u32,
    pub today_wins: u32,

    // === 衍生量化指标 ===
    pub dpm: f64,   // 每分钟伤害
    pub cspm: f64,  // 每分钟补刀
    pub vspm: f64,  // 每分钟视野得分

    // === 定性特征标签 ===
    pub traits: Vec<SummonerTrait>,

    // === 常用英雄 ===
    pub favorite_champions: Vec<AnalysisChampionStats>,

    // === 最近战绩 ===
    pub recent_performance: Vec<MatchPerformance>,
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
    #[ts(type = "number")]
    pub game_creation: Option<i64>,
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
    pub queue_id: i64,        // 队列类型ID：420=单排, 440=灵活排位, 450=大乱斗等
    pub is_custom_game: bool, // 是否自定义游戏

    // 🔥 新增：选人流程相关字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Vec<ChampSelectAction>>>, // 选人/ban 动作序列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bans: Option<ChampSelectBans>, // ban 位信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer: Option<ChampSelectTimer>, // 计时器信息
}
