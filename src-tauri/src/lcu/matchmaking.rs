use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use crate::lcu::types::{LcuAuthInfo, MatchmakingState, MatchInfo, PlayerInfo};

pub async fn start_matchmaking(client: &Client, auth_info: &LcuAuthInfo) -> Result<(), String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-lobby/v2/lobby/matchmaking/search",
        auth_info.app_port
    );

    client
        .post(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to start matchmaking: {}", e))?;

    Ok(())
}

pub async fn stop_matchmaking(client: &Client, auth_info: &LcuAuthInfo) -> Result<(), String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-lobby/v2/lobby/matchmaking/search",
        auth_info.app_port
    );

    client
        .delete(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to stop matchmaking: {}", e))?;

    Ok(())
}

pub async fn accept_match(client: &Client, auth_info: &LcuAuthInfo) -> Result<(), String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-matchmaking/v1/ready-check/accept",
        auth_info.app_port
    );

    client
        .post(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to accept match: {}", e))?;

    Ok(())
}

pub async fn decline_match(client: &Client, auth_info: &LcuAuthInfo) -> Result<(), String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-matchmaking/v1/ready-check/decline",
        auth_info.app_port
    );

    client
        .post(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to decline match: {}", e))?;

    Ok(())
}

pub async fn get_matchmaking_state(client: &Client, auth_info: &LcuAuthInfo) -> Result<MatchmakingState, String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-lobby/v2/lobby/matchmaking/search-state",
        auth_info.app_port
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to get matchmaking state: {}", e))?;

    let state: MatchmakingState = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse matchmaking state: {}", e))?;

    Ok(state)
}

pub async fn get_match_info(client: &Client, auth_info: &LcuAuthInfo) -> Result<MatchInfo, String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-champ-select/v1/session",
        auth_info.app_port
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await
        .map_err(|e| format!("Failed to get match info: {}", e))?;

    let session: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse match info: {}", e))?;

    // 解析对局信息
    let match_id = session["gameId"]
        .as_str()
        .ok_or_else(|| "No game ID found".to_string())?
        .to_string();

    let mut players = Vec::new();
    if let Some(team_members) = session["myTeam"].as_array() {
        for member in team_members {
            let summoner_name = member["summonerName"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();
            let champion_id = member["championId"]
                .as_i64()
                .unwrap_or(0) as i32;
            let team_id = member["team"]
                .as_i64()
                .unwrap_or(0) as i32;

            players.push(PlayerInfo {
                summoner_name,
                champion_id,
                team_id,
            });
        }
    }

    Ok(MatchInfo {
        match_id,
        players,
    })
}
