use crate::shared::utils::lcu_get;
use reqwest::Client;
use serde_json::Value;

/// 获取当前游戏流程阶段（如 "Lobby"、"Matchmaking"、"ChampSelect"、"InProgress"、"EndOfGame" 等）
pub async fn get_gameflow_phase(client: &Client) -> Result<String, String> {
    // 由于 LCU 该接口直接返回 JSON 字符串，所以用 lcu_get::<String>
    lcu_get(client, "/lol-gameflow/v1/gameflow-phase").await
}

/// 获取完整的 gameflow session 信息（包含游戏详细信息，如队列、地图等）
pub async fn get_gameflow_session(client: &Client) -> Result<Value, String> {
    lcu_get(client, "/lol-gameflow/v1/session").await
}
