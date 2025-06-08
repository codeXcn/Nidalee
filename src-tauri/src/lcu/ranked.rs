use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use crate::lcu::types::{LcuAuthInfo, RankedStats};

pub async fn get_current_ranked_stats(client: &Client, auth_info: &LcuAuthInfo) -> Result<RankedStats, String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-ranked/v1/current-ranked-stats",
        auth_info.app_port
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to get ranked stats: {}", e))?;

    response
        .json::<RankedStats>()
        .await
        .map_err(|e| format!("Failed to parse ranked stats: {}", e))
}

pub async fn get_ranked_stats_by_id(client: &Client, auth_info: &LcuAuthInfo, summoner_id: u64) -> Result<RankedStats, String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-ranked/v1/ranked-stats/{}",
        auth_info.app_port,
        summoner_id
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to get ranked stats: {}", e))?;

    response
        .json::<RankedStats>()
        .await
        .map_err(|e| format!("Failed to parse ranked stats: {}", e))
}
