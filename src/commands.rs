use crate::lcu::connection_manager::ConnectionManager;
use crate::lcu::types::{
    ChampSelectSession, GameDetail, GameflowPhase, LobbyInfo, MatchStatistics, SummonerInfo,
    SummonerWithMatches,
};
use tauri::AppHandle;

#[tauri::command]
pub fn is_lcu_running() -> bool {
    ConnectionManager::is_lcu_running()
}

#[tauri::command]
pub async fn get_game_detail_typed(game_id: u64) -> Result<GameDetail, String> {
    let client = crate::lcu::request::get_client();
    crate::lcu::match_history::get_game_detail(&client, game_id).await
}

#[tauri::command]
pub async fn get_match_history_typed() -> Result<MatchStatistics, String> {
    let client = crate::lcu::request::get_client();
    crate::lcu::match_history::get_match_history(&client).await
}
