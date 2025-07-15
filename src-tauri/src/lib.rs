// 应用库 - 提供应用运行的核心功能
mod app;
mod commands;
mod http_client;
mod lcu;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(app::setup_app)
        .invoke_handler(tauri::generate_handler![
            lcu::auth::get_auth_info,
            lcu::connection_manager::get_connection_state,
            lcu::connection_manager::force_refresh_connection,
            lcu::connection_manager::check_connection_state_command,
            commands::get_live_player_list,
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
            commands::get_summoners_and_histories,
            commands::set_summoner_background_skin,
            commands::get_champ_select_session,
            commands::pick_champion,
            commands::ban_champion,
            commands::set_summoner_chat_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
