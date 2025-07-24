// Tauri 命令模块 - 集中管理所有的 Tauri 命令
use crate::{http_client, lcu};
use reqwest::Client;
use serde_json;
use serde_json::Value;
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

// 删除 start_matchmaking、stop_matchmaking、accept_match、decline_match 的 tauri 命令实现

// ===== 英雄出装和符文相关命令 =====

#[tauri::command]
pub async fn get_champion_builds(
    source: String,
    champion_alias: String,
) -> Result<serde_json::Value, String> {
    let client = http_client::get_lcu_client();
    lcu::ddragon::get_builds_by_alias(&client, &source, &champion_alias).await
}

#[tauri::command]
pub async fn get_champion_runes(
    source: String,
    champion_alias: String,
) -> Result<Vec<lcu::types::DataDragonRune>, String> {
    let client = http_client::get_lcu_client();
    lcu::ddragon::get_runes_by_alias(&client, &source, &champion_alias).await
}

#[tauri::command]
pub async fn get_all_runes() -> Result<lcu::types::AllRunesResponse, String> {
    let client = http_client::get_lcu_client();
    lcu::ddragon::get_all_runes(&client).await
}

/// 应用英雄详细（符文配置）
#[tauri::command]
pub async fn apply_champion_build(
    champion_alias: String,
    build_index: usize,
) -> Result<String, String> {
    log::info!(
        "🚀 开始应用英雄详细: {} (详细索引: {})",
        champion_alias,
        build_index
    );

    // 获取LCU连接
    let client = http_client::get_lcu_client();

    // 获取英雄详细数据
    let build_data = match get_champion_builds("op.gg".to_string(), champion_alias.clone()).await {
        Ok(data) => data,
        Err(e) => {
            log::error!("❌ 获取英雄详细数据失败: {}", e);
            return Err(format!("获取英雄详细数据失败: {}", e));
        }
    };

    // 解析详细数据
    let content = build_data
        .get("content")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.get(0))
        .ok_or("无法获取详细数据")?;

    let runes_array = content
        .get("runes")
        .and_then(|r| r.as_array())
        .ok_or("无法获取符文配置数组")?;

    // 检查详细索引是否有效
    if build_index >= runes_array.len() {
        let msg = format!(
            "详细索引 {} 超出范围，总共有 {} 个详细",
            build_index,
            runes_array.len()
        );
        log::error!("❌ {}", msg);
        return Err(msg);
    }

    let rune_build = &runes_array[build_index];
    log::info!("📋 使用符文详细索引: {}", build_index);

    // 提取符文配置信息
    let primary_style_id = rune_build
        .get("primaryStyleId")
        .and_then(|v| v.as_i64())
        .ok_or("无法获取主系符文ID")? as i32;

    let sub_style_id = rune_build
        .get("subStyleId")
        .and_then(|v| v.as_i64())
        .ok_or("无法获取副系符文ID")? as i32;

    let selected_perk_ids: Vec<i32> = rune_build
        .get("selectedPerkIds")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_i64().map(|i| i as i32))
                .collect()
        })
        .unwrap_or_default();

    log::info!(
        "🔧 符文配置 - 主系: {}, 副系: {}, 符文数量: {}",
        primary_style_id,
        sub_style_id,
        selected_perk_ids.len()
    );

    // 应用符文配置
    match lcu::perks::service::apply_rune_build(
        client,
        &champion_alias,
        primary_style_id,
        sub_style_id,
        selected_perk_ids,
    )
    .await
    {
        Ok(message) => {
            log::info!("✅ 符文应用成功: {}", message);
            Ok(format!("符文配置应用成功！{}", message))
        }
        Err(e) => {
            log::error!("❌ 符文应用失败: {}", e);
            Err(format!("符文配置应用失败: {}", e))
        }
    }
}

/// 获取英雄详细数据 - 使用新的 OP.GG API
#[tauri::command]
pub async fn get_champion_build_new(
    region: String,
    mode: String,
    champion_id: i32,
    position: Option<String>,
    tier: String,
) -> Result<Value, String> {
    log::info!(
        "🚀 获取英雄详细数据: 区域={}, 模式={}, 英雄ID={}, 位置={:?}, 段位={}",
        region,
        mode,
        champion_id,
        position,
        tier
    );

    let client = Client::new();

    // 详细 API URL
    let url = if mode == "arena" {
        format!(
            "https://lol-api-champion.op.gg/api/{}/champions/{}/{}?tier={}",
            region, mode, champion_id, tier
        )
    } else {
        let pos = position.unwrap_or("MID".to_string());
        format!(
            "https://lol-api-champion.op.gg/api/{}/champions/{}/{}/{}?tier={}",
            region, mode, champion_id, pos, tier
        )
    };

    log::info!("🌐 请求URL: {}", url);

    // 发送请求
    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API 请求失败: HTTP {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;

    log::info!("✅ 成功获取英雄详细数据");
    Ok(data)
}

/// 获取所有英雄列表
#[tauri::command]
pub async fn get_champions_list(
    region: String,
    mode: String,
    tier: String,
) -> Result<Value, String> {
    log::info!(
        "🚀 获取英雄列表: 区域={}, 模式={}, 段位={}",
        region,
        mode,
        tier
    );

    let client = Client::new();

    let url = format!(
        "https://lol-api-champion.op.gg/api/{}/champions/{}?tier={}",
        region, mode, tier
    );

    log::info!("🌐 请求URL: {}", url);

    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API 请求失败: HTTP {}", response.status()));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;

    log::info!("✅ 成功获取英雄列表数据");
    Ok(data)
}

// 内部辅助函数，用于调用其他命令
async fn invoke_command_internal(
    command: &str,
    params: Vec<(String, String)>,
) -> Result<serde_json::Value, String> {
    match command {
        "get_champion_builds" => {
            let source = params
                .iter()
                .find(|(k, _)| k == "source")
                .map(|(_, v)| v.clone())
                .unwrap_or_default();
            let champion_alias = params
                .iter()
                .find(|(k, _)| k == "championAlias")
                .map(|(_, v)| v.clone())
                .unwrap_or_default();
            get_champion_builds(source, champion_alias).await
        }
        _ => Err("未知命令".to_string()),
    }
}
#[tauri::command]
pub async fn get_machine_hash() -> Result<String, String> {
    use std::process::Command;
    use sha2::{Sha256, Digest};

    // 获取主板序列号
    #[cfg(target_os = "windows")]
    fn get_board_sn() -> Option<String> {
        let output = Command::new("wmic")
            .args(&["baseboard", "get", "serialnumber"])
            .output()
            .ok()?;
        String::from_utf8(output.stdout).ok()?.lines().nth(1).map(|s| s.trim().to_string())
    }
    #[cfg(target_os = "linux")]
    fn get_board_sn() -> Option<String> {
        let output = Command::new("cat")
            .arg("/sys/class/dmi/id/board_serial")
            .output()
            .ok()?;
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    #[cfg(target_os = "macos")]
    fn get_board_sn() -> Option<String> {
        let output = Command::new("ioreg")
            .args(&["-l"])
            .output()
            .ok()?;
        let out = String::from_utf8_lossy(&output.stdout);
        out.lines()
            .find(|line| line.contains("IOPlatformSerialNumber"))
            .and_then(|line| line.split('=').nth(1))
            .map(|s| s.replace("\"", "").trim().to_string())
    }

    // 获取MAC地址
    fn get_mac() -> Option<String> {
        use mac_address::get_mac_address;
        match get_mac_address() {
            Ok(Some(ma)) => Some(ma.to_string()),
            _ => None,
        }
    }

    let board_sn = get_board_sn().unwrap_or_default();
    let mac = get_mac().unwrap_or_default();

    log::info!("主板序列号: {}", board_sn);
    log::info!("MAC地址: {}", mac);

    if board_sn.is_empty() && mac.is_empty() {
        return Err("无法获取主板序列号和MAC地址".to_string());
    }

    let raw = format!("{}-{}", board_sn, mac);

    // SHA256哈希
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    Ok(hash_hex)
}

