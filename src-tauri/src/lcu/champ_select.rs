use crate::lcu::types::{LcuAuthInfo, CurrentChampion, ChampSelectSession};
use reqwest::Client;

/// 获取当前选人阶段的完整 session 信息
pub async fn get_champ_select_session(
    client: &Client,
    auth: &LcuAuthInfo,
) -> Result<ChampSelectSession, String> {
    let url = format!(
        "https://127.0.0.1:{}/lol-champ-select/v1/session",
        auth.app_port
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", auth.remoting_auth_token))
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "获取选人 session 失败: {}",
            response.status()
        ));
    }

    let session = response
        .json::<ChampSelectSession>()
        .await
        .map_err(|e| format!("解析 session 响应失败: {}", e))?;

    Ok(session)
}
/// 获取当前选择的英雄
pub async fn get_current_champion(client: &Client, auth: &LcuAuthInfo) -> Result<CurrentChampion, String> {
    let url = format!(
        "https://127.0.0.1:{}/lol-champ-select/v1/current-champion",
        auth.app_port
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", auth.remoting_auth_token))
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "获取当前英雄失败: {}",
            response.status()
        ));
    }

    let champion = response
        .json::<CurrentChampion>()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    Ok(champion)
}
