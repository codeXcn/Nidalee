use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use crate::lcu::types::{LcuAuthInfo, GameflowPhase};

pub async fn get_gameflow_phase(client: &Client, auth_info: &LcuAuthInfo) -> Result<GameflowPhase, String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-gameflow/v1/gameflow-phase",
        auth_info.app_port
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to get gameflow phase: {}", e))?;

    response
        .json::<GameflowPhase>()
        .await
        .map_err(|e| format!("Failed to parse gameflow phase: {}", e))
}
