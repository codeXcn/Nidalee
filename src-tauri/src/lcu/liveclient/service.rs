use serde_json::Value;
use crate::lcu::auth::service::ensure_valid_auth_info;

/// 动态检测 LiveClient 端口
/// 通常 LiveClient 使用固定端口 2999，但某些情况下可能会变化
async fn detect_liveclient_port() -> Option<u16> {
    // 首先尝试默认端口 2999
    if test_liveclient_port(2999).await {
        return Some(2999);
    }

    // 如果 2999 不可用，尝试其他可能的端口
    let possible_ports = [2998, 3000, 3001, 2997];
    for port in possible_ports {
        if test_liveclient_port(port).await {
            log::info!("[LiveClient] 检测到端口: {}", port);
            return Some(port);
        }
    }

    None
}

/// 测试指定端口是否可用
async fn test_liveclient_port(port: u16) -> bool {
    let client = match reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(2))
        .build()
    {
        Ok(client) => client,
        Err(_) => return false,
    };

    let url = format!("https://127.0.0.1:{}/liveclientdata/playerlist", port);
    match client.get(&url).send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

/// 获取游戏内玩家列表
pub async fn get_live_player_list() -> Result<Vec<crate::lcu::types::LiveClientPlayer>, String> {
    // 确保 LCU 认证信息可用（虽然 LiveClient 不需要认证，但确保游戏正在运行）
    let _auth = ensure_valid_auth_info()
        .ok_or("无法获取 LCU 认证信息，请确保游戏正在运行")?;

    // LiveClient 通常使用固定端口 2999，但我们可以尝试动态检测
    let liveclient_port = detect_liveclient_port().await.unwrap_or(2999);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("https://127.0.0.1:{}/liveclientdata/playerlist", liveclient_port);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            if e.to_string().contains("tcp connect error") {
                format!("LiveClient 服务不可用 (端口 {}), 请确保游戏已启动并进入对局", liveclient_port)
            } else {
                format!("请求失败: {}", e)
            }
        })?;

    if !resp.status().is_success() {
        return Err(format!("LiveClient API 返回错误状态: {}", resp.status()));
    }

    let text = resp.text().await.map_err(|e| e.to_string())?;

    // 先打印原始数据以便调试
    log::debug!("[LiveClient] 原始 API 响应: {}", text);

    // 尝试解析为 JSON 值来查看实际结构
    let json_value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;

    log::debug!("[LiveClient] 解析后的 JSON: {}", serde_json::to_string_pretty(&json_value).unwrap_or_default());

    // 尝试解析为我们的结构
    let players: Vec<crate::lcu::types::LiveClientPlayer> =
        serde_json::from_str(&text).map_err(|e| format!("解析为 LiveClientPlayer 失败: {}", e))?;

    Ok(players)
}

/// 获取游戏内事件数据
pub async fn get_live_events() -> Result<Vec<Value>, String> {
    let liveclient_port = detect_liveclient_port().await.unwrap_or(2999);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("https://127.0.0.1:{}/liveclientdata/eventdata", liveclient_port);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    let events: Vec<Value> = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    Ok(events)
}

/// 获取游戏状态
pub async fn get_game_stats() -> Result<Value, String> {
    let liveclient_port = detect_liveclient_port().await.unwrap_or(2999);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("https://127.0.0.1:{}/liveclientdata/gamestats", liveclient_port);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    let stats: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    Ok(stats)
}

/// 检查 LiveClient 是否可用
pub async fn is_liveclient_available() -> bool {
    detect_liveclient_port().await.is_some()
}
