// 应用库 - 提供应用运行的核心功能
mod lcu;
mod commands;
mod tray;
mod app;
mod http_client;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(app::setup_app)
        .invoke_handler(tauri::generate_handler![
            lcu::auth::get_auth_info,
            commands::get_current_summoner,
            commands::get_game_version,
            commands::get_match_history,
            commands::get_game_detail,
            commands::start_matchmaking,
            commands::stop_matchmaking,
            commands::accept_match,
            commands::decline_match,
            commands::get_summoner_by_id,
            commands::get_champselect_team_players_info,
            commands::get_summoners_and_histories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
