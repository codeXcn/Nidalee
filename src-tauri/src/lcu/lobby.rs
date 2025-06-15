use reqwest::Client;
use crate::lcu::types::LobbyInfo;
use crate::lcu::request::lcu_get;

/// 获取当前 Lobby 信息（自动带全局认证、日志、耗时）
pub async fn get_lobby_info(client: &Client) -> Result<LobbyInfo, String> {
    lcu_get(client, "/lol-lobby/v2/lobby").await
}