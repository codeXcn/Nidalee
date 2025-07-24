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
            lcu::auth::commands::get_auth_info,
            lcu::connection::commands::get_connection_state,
            lcu::connection::commands::force_refresh_connection,
            lcu::connection::commands::check_connection_state_command,
            commands::get_live_player_list,
            lcu::summoner::commands::get_current_summoner,
            lcu::gameflow::commands::get_game_version,
            lcu::matches::commands::get_match_history,
            lcu::matches::commands::get_game_detail,
            lcu::matchmaking::commands::start_matchmaking,
            lcu::matchmaking::commands::stop_matchmaking,
            lcu::matchmaking::commands::accept_match,
            lcu::matchmaking::commands::decline_match,
            lcu::summoner::commands::get_summoner_by_id,
            lcu::champ_select::commands::get_champselect_team_players_info,
            lcu::champ_select::commands::get_champ_select_session,
            lcu::champ_select::commands::get_champ_select_session_typed,
            lcu::champ_select::commands::pick_champion,
            lcu::champ_select::commands::ban_champion,
            lcu::summoner::commands::set_summoner_chat_profile,
            commands::get_champion_builds,
            commands::get_champion_runes,
            commands::get_all_runes,
            lcu::perks::commands::get_lcu_rune_styles,
            lcu::perks::commands::get_lcu_perks,
            lcu::perks::commands::get_lcu_perk_icon,
            commands::apply_champion_build,
            commands::get_champion_build_new,
            commands::get_champions_list,
            lcu::opgg::commands::get_opgg_champion_build,
            lcu::opgg::commands::get_opgg_champion_build_raw,
            lcu::opgg::commands::get_opgg_tier_list,
            lcu::opgg::commands::get_opgg_champion_positions,
            lcu::opgg::commands::apply_opgg_runes,
            commands::get_machine_hash,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
