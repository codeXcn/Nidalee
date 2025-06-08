mod lcu;

use std::sync::Arc;
use tauri::{AppHandle, Manager};
use crate::lcu::polling::PollingManager;

#[tauri::command]
async fn check_lcu_connection() -> Result<bool, String> {
    Ok(lcu::auth::check_lcu_running())
}

#[tauri::command]
async fn get_lcu_auth_info() -> Result<lcu::types::LcuAuthInfo, String> {
    tokio::task::spawn_blocking(|| lcu::auth::get_lcu_auth_info())
        .await
        .map_err(|e| format!("获取认证信息失败: {}", e))?
}

#[tauri::command]
async fn get_game_version() -> Result<String, String> {
    // 尝试从公开的Riot API获取最新版本
    let client = reqwest::Client::new();

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

#[tauri::command]
async fn get_match_history() -> Result<lcu::match_history::MatchStatistics, String> {
    lcu::match_history::get_match_history().await
}


#[tauri::command]
async fn get_game_detail(game_id: u64) -> Result<serde_json::Value, String> {
    lcu::match_history::get_game_detail(game_id).await
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 创建并启动轮询管理器
            let polling_manager = Arc::new(PollingManager::new(app.handle().clone()));
            let manager_clone = polling_manager.clone();

            tokio::spawn(async move {
                manager_clone.start_polling().await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_lcu_connection,
            get_lcu_auth_info,
            get_game_version,
            get_match_history,
            get_game_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
