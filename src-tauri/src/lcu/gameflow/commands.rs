use crate::http_client;

#[tauri::command]
pub async fn get_live_player_list() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // 👈 忽略证书错误
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get("https://127.0.0.1:2999/liveclientdata/playerlist")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

#[tauri::command]
pub async fn get_game_version() -> Result<String, String> {
    // 尝试从公开的Riot API获取最新版本
    let client = http_client::get_public_client();

    match client
        .get("https://ddragon.leagueoflegends.com/api/versions.json")
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(versions) = response.json::<Vec<String>>().await {
                if let Some(latest_version) = versions.first() {
                    return Ok(latest_version.clone());
                }
            }
        }
        Err(_) => {
            // 如果获取失败，返回一个相对较新的默认版本
            return Ok("14.23.1".to_string());
        }
    }

    // 备用默认版本
    Ok("14.23.1".to_string())
}
