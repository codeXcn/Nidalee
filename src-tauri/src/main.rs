mod lcu;

use tauri::{
    tray::{
        TrayIconBuilder,
        MouseButtonState,
        MouseButton,
        TrayIconEvent
    },
    menu::{
        Menu,
        MenuItem,
        MenuEvent
    },
    Manager,
    Window
};
use std::sync::Arc;

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
#[tauri::command]
async fn start_matchmaking() -> Result<(), String> {
    let auth_info = get_lcu_auth_info().await?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::start_matchmaking(&client, &auth_info).await
}

#[tauri::command]
async fn stop_matchmaking() -> Result<(), String> {
    let auth_info = get_lcu_auth_info().await?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::stop_matchmaking(&client, &auth_info).await
}

#[tauri::command]
async fn accept_match() -> Result<(), String> {
    let auth_info = get_lcu_auth_info().await?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::accept_match(&client, &auth_info).await
}

#[tauri::command]
async fn decline_match() -> Result<(), String> {
    let auth_info = get_lcu_auth_info().await?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::decline_match(&client, &auth_info).await
}

#[tokio::main]
async fn main() {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            // 创建托盘菜单项
            let show_i = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let maximize_i = MenuItem::with_id(app, "maximize", "最大化", true, None::<&str>)?;
            let minimize_i = MenuItem::with_id(app, "minimize", "最小化", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

            // 创建托盘菜单
            let menu = Menu::with_items(app, &[&show_i, &maximize_i, &minimize_i, &quit_i])?;

            // 创建系统托盘
            let _tray = TrayIconBuilder::new()
                // 添加托盘图标
                .icon(app.default_window_icon().unwrap().clone())
                // 添加菜单
                .menu(&menu)
                // 添加悬浮提示
                .tooltip("Nidalee - 英雄联盟游戏助手")
                // 禁用鼠标左键点击图标显示托盘菜单
                .show_menu_on_left_click(false)
                // 监听托盘图标发出的鼠标事件
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            if let Ok(is_visible) = window.is_visible() {
                                if is_visible {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    }
                })
                // 监听菜单事件
                .on_menu_event(|app_handle, event| {
                    let window = app_handle.get_webview_window("main").unwrap();
                    match event.id.as_ref() {
                        "show" => {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                        "maximize" => {
                            let _ = window.maximize();
                            let _ = window.set_focus();
                        }
                        "minimize" => {
                            let _ = window.minimize();
                        }
                        "quit" => {
                            app_handle.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // 创建并启动轮询管理器
            let app_handle = app.handle().clone();
            tokio::spawn(async move {
                lcu::polling::start_polling(app_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_lcu_connection,
            get_lcu_auth_info,
            get_game_version,
            get_match_history,
            get_game_detail,
            start_matchmaking,
            stop_matchmaking,
            accept_match,
            decline_match,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
