// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;
use lib::{GameHelper, LcpConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
struct AppState {
    game_helper: Arc<Mutex<Option<Arc<GameHelper>>>>,
}

#[tauri::command]
async fn init_game_helper(
    state: tauri::State<'_, AppState>,
    token: String,
    port: String,
) -> Result<(), String> {
    let config = LcpConfig { token, port };
    let helper = GameHelper::new(config);
    
    // 启动自动接受和自动选择/禁用功能
    helper.start_auto_accept().await;
    helper.start_auto_pick_ban().await;
    
    *state.game_helper.lock().await = Some(Arc::new(helper));
    Ok(())
}

#[tauri::command]
async fn set_auto_accept(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    if let Some(helper) = &*state.game_helper.lock().await {
        helper.set_auto_accept(enabled).await;
        Ok(())
    } else {
        Err("Game helper not initialized".to_string())
    }
}

#[tauri::command]
async fn set_auto_pick(
    state: tauri::State<'_, AppState>,
    champion_id: Option<i32>,
) -> Result<(), String> {
    if let Some(helper) = &*state.game_helper.lock().await {
        helper.set_auto_pick(champion_id).await;
        Ok(())
    } else {
        Err("Game helper not initialized".to_string())
    }
}

#[tauri::command]
async fn set_auto_ban(
    state: tauri::State<'_, AppState>,
    champion_id: Option<i32>,
) -> Result<(), String> {
    if let Some(helper) = &*state.game_helper.lock().await {
        helper.set_auto_ban(champion_id).await;
        Ok(())
    } else {
        Err("Game helper not initialized".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            init_game_helper,
            set_auto_accept,
            set_auto_pick,
            set_auto_ban,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
