// 应用库 - 提供应用运行的核心功能
mod app;
mod http_client;
mod lcu;
mod tray;
mod common;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(app::setup_app)
        .invoke_handler(tauri::generate_handler![
            // 认证 / 连接
            lcu::auth::commands::get_auth_info,
            lcu::connection::commands::get_connection_state,
            lcu::connection::commands::force_refresh_connection,
            lcu::connection::commands::check_connection_state_command,

            // 游戏流程 / 英雄选择 / 匹配
            lcu::gameflow::commands::get_game_version,
            lcu::gameflow::commands::get_live_player_list,
            lcu::champ_select::commands::get_champselect_team_players_info,
            lcu::champ_select::commands::get_champ_select_session,
            lcu::champ_select::commands::get_champ_select_session_typed,
            lcu::champ_select::commands::pick_champion,
            lcu::champ_select::commands::ban_champion,
            lcu::matchmaking::commands::start_matchmaking,
            lcu::matchmaking::commands::stop_matchmaking,
            lcu::matchmaking::commands::accept_match,
            lcu::matchmaking::commands::decline_match,

            // 比赛记录
            lcu::matches::commands::get_match_history,
            lcu::matches::commands::get_game_detail,

            // 召唤师
            lcu::summoner::commands::get_current_summoner,
            lcu::summoner::commands::get_summoner_by_id,
            lcu::summoner::commands::get_recent_matches_by_puuid,
            lcu::summoner::commands::get_summoners_and_histories,
            lcu::summoner::commands::set_summoner_chat_profile,
            lcu::summoner::commands::set_summoner_background_skin,

            // 召唤师符文
            lcu::perks::commands::get_lcu_rune_styles,
            lcu::perks::commands::get_lcu_perks,
            lcu::perks::commands::get_lcu_perk_icon,

            // OPGG 相关
            lcu::opgg::commands::get_opgg_champion_build,
            lcu::opgg::commands::get_opgg_champion_build_raw,
            lcu::opgg::commands::get_opgg_tier_list,
            lcu::opgg::commands::get_opgg_champion_positions,
            lcu::opgg::commands::apply_opgg_runes,
            // LCU WS 测试命令
            lcu::ws::commands::start_lcu_ws,
            lcu::ws::commands::stop_lcu_ws,
            common::commands::machine::get_machine_hash,
            common::commands::builds::get_champions_list,
            common::commands::builds::get_champion_builds,
            common::commands::builds::get_champion_build_new,
            common::commands::builds::get_champion_runes,
            common::commands::builds::get_all_runes,
            common::commands::builds::apply_champion_build,
            common::commands::game::launch_game,
            common::commands::game::detect_game_path,
            common::commands::game::select_game_path,
            common::commands::game::save_game_path,
            common::commands::game::get_saved_game_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
