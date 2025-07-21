//! LCU 符文相关 API
use crate::lcu::request::{lcu_get, lcu_request_raw};
use crate::lcu::types::{RuneStyle, Perk};
use reqwest::Client;

/// 获取所有符文样式
/// 对应 LCU API: /lol-perks/v1/styles
pub async fn list_all_styles(client: &Client) -> Result<Vec<RuneStyle>, String> {
    let path = "/lol-perks/v1/styles";
    lcu_get(client, path).await
}

/// 获取所有符文详细信息
/// 对应 LCU API: /lol-perks/v1/perks
pub async fn list_all_perks(client: &Client) -> Result<Vec<Perk>, String> {
    let path = "/lol-perks/v1/perks";
    lcu_get(client, path).await
}

/// 获取符文图标资源
/// 对应 LCU API: GET /lol-game-data/assets/v1/perk-images/...
pub async fn get_perk_icon(client: &Client, icon_path: &str) -> Result<Vec<u8>, String> {
    // 确保路径以 / 开头
    let path = if icon_path.starts_with('/') {
        icon_path.to_string()
    } else {
        format!("/{}", icon_path)
    };

    let response = lcu_request_raw(client, reqwest::Method::GET, &path, None).await?;

    if !response.status().is_success() {
        return Err(format!("获取图标失败，状态码: {}", response.status()));
    }

    let bytes = response.bytes().await.map_err(|e| format!("读取图片数据失败: {}", e))?;
    Ok(bytes.to_vec())
}
