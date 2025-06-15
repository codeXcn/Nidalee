use reqwest::Client;
use crate::lcu::request::lcu_get;

/// 获取当前游戏流程阶段（如 "Lobby"、"Matchmaking"、"ChampSelect"、"InProgress"、"EndOfGame" 等）
pub async fn get_gameflow_phase(client: &Client) -> Result<String, String> {
    // 由于 LCU 该接口直接返回 JSON 字符串，所以用 lcu_get::<String>
    lcu_get(client, "/lol-gameflow/v1/gameflow-phase").await
}