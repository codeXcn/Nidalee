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
            commands::set_summoner_chat_profile,
            commands::get_champion_builds,
            commands::get_champion_runes,
            commands::get_all_runes,
            commands::get_lcu_rune_styles,
            commands::get_lcu_perks,
            commands::get_lcu_perk_icon,
            commands::apply_champion_build,
            commands::get_champion_build_new,
            commands::get_champions_list,
            commands::get_opgg_champion_build,
            commands::get_opgg_champion_build_raw,
            commands::get_opgg_tier_list,
            commands::get_opgg_champion_positions,
            commands::apply_opgg_runes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
