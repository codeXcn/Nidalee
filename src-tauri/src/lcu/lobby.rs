use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use crate::lcu::types::{LcuAuthInfo, LobbyInfo};

pub async fn get_lobby_info(client: &Client, auth_info: &LcuAuthInfo) -> Result<LobbyInfo, String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-lobby/v2/lobby",
        auth_info.app_port
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to get lobby info: {}", e))?;

    if response.status().is_success() {
        response
            .json::<LobbyInfo>()
            .await
            .map_err(|e| format!("Failed to parse lobby info: {}", e))
    } else {
        Err("Not in lobby".to_string())
    }
}
