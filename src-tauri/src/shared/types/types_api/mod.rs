use serde::Deserialize;

// Using #[serde(default)] for many fields to be robust against missing data in the API response.

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchDto {
    pub game_id: u64,
    #[serde(default)]
    pub queue_id: i64,
    #[serde(default)]
    pub game_mode: String,
    #[serde(default)]
    pub game_duration: i64,
    #[serde(default)]
    pub game_creation: i64,
    #[serde(default)]
    pub participant_identities: Vec<ParticipantIdentityDto>,
    #[serde(default)]
    pub participants: Vec<ParticipantDto>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentityDto {
    pub participant_id: i64,
    pub player: PlayerDto,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDto {
    pub puuid: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDto {
    pub participant_id: i64,
    pub team_id: i64,
    pub champion_id: i32,
    pub timeline: ParticipantTimelineDto,
    pub stats: ParticipantStatsDto,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantStatsDto {
    #[serde(default)]
    pub win: bool,
    #[serde(default)]
    pub kills: i64,
    #[serde(default)]
    pub deaths: i64,
    #[serde(default)]
    pub assists: i64,
    #[serde(default)]
    pub total_damage_dealt_to_champions: i64,
    #[serde(default)]
    pub vision_score: i64,
    #[serde(default)]
    pub total_minions_killed: i64,
    #[serde(default)]
    pub neutral_minions_killed: i64,
    #[serde(default)]
    pub total_damage_taken: i64,
    #[serde(default)]
    pub damage_dealt_to_objectives: i64,
    #[serde(default)]
    pub damage_dealt_to_turrets: i64,
    #[serde(default)]
    pub wards_killed: i64,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTimelineDto {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub lane: String,
}
