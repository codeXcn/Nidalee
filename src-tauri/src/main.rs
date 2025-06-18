#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod lcu;
use std::collections::HashMap;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use crate::lcu::summoner::fill_summoner_extra_info;

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
async fn get_match_history() -> Result<lcu::types::MatchStatistics, String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");
    lcu::match_history::get_match_history(&client).await
}
#[tauri::command]
async fn get_game_detail(game_id: u64) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");
    lcu::match_history::get_game_detail(&client, game_id).await
}
#[tauri::command]
async fn start_matchmaking() -> Result<(), String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::start_matchmaking(&client).await
}

#[tauri::command]
async fn stop_matchmaking() -> Result<(), String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::stop_matchmaking(&client).await
}

#[tauri::command]
async fn accept_match() -> Result<(), String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::accept_match(&client).await
}

#[tauri::command]
async fn decline_match() -> Result<(), String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::matchmaking::decline_match(&client).await
}
#[tauri::command]
async fn get_current_summoner() -> Result<(lcu::types::SummonerInfo), String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    lcu::summoner::get_current_summoner(&client).await
}
#[tauri::command]
async fn get_summoner_by_id(id: u64) -> Result<Option<lcu::types::SummonerInfo>, String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    match crate::lcu::summoner::get_summoner_by_id(&client, id).await {
        Ok(info) => Ok(Some(info)),
        Err(e) => {
            if e.contains("404") {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}
#[tauri::command]
async fn get_champselect_team_players_info(
) -> Result<HashMap<String, lcu::types::MatchStatistics>, String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    lcu::champ_select::get_champselect_team_players_info(&client).await
}

#[tauri::command]
async fn get_summoners_and_histories(
    names: Vec<String>,
) -> Result<Vec<lcu::types::SummonerWithMatches>, String> {
    println!("[get_summoners_and_histories] 查询召唤师列表: {:?}", names);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    // 1. 批量查召唤师信息
    let mut summoners = lcu::summoner::get_summoners_by_names(&client, names)
        .await
        .map_err(|e| format!("批量获取召唤师信息失败: {}", e))?;
    // 2. 合并所有召唤师的信息
    let mut result = Vec::new();
    for summoner in &mut summoners {
        let puuid = summoner.puuid.clone();
        println!("[get_summoners_and_histories] summoner puuid: {}", puuid);
        if !puuid.is_empty() {
            fill_summoner_extra_info(&client, summoner).await;
            match lcu::match_history::get_recent_matches_by_summoner_id(&client, &puuid, 10).await {
                Ok(history) => {
                    println!("[get_summoners_and_histories] 获取 {} 战绩成功", puuid);
                    result.push(lcu::types::SummonerWithMatches {
                        display_name: summoner.display_name.clone(),
                        summoner_info: summoner.clone(),
                        matches: history,
                    });
                }
                Err(e) => eprintln!(
                    "[get_summoners_and_histories] 获取召唤师 {} 战绩失败: {}",
                    puuid, e
                ),
            }
        } else {
            println!("[get_summoners_and_histories] summoner puuid 为空，跳过");
        }
    }
    Ok(result)
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
                    } = event
                    {
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
            lcu::auth::get_auth_info,
            get_current_summoner,
            get_game_version,
            get_match_history,
            get_game_detail,
            start_matchmaking,
            stop_matchmaking,
            accept_match,
            decline_match,
            get_summoner_by_id,
            get_champselect_team_players_info,
            get_summoners_and_histories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
