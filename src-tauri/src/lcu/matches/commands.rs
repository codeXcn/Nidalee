use crate::{http_client, lcu};

#[tauri::command]
pub async fn get_match_history() -> Result<lcu::types::MatchStatistics, String> {
    let client = http_client::get_lcu_client();
    lcu::matches::service::get_match_history(client).await
}

#[tauri::command]
pub async fn get_game_detail(game_id: u64) -> Result<lcu::types::GameDetail, String> {
    log::info!("🔍 ===== 获取游戏详细信息 =====");
    log::info!("🎮 游戏ID: {}", game_id);
    let client = http_client::get_lcu_client();
    lcu::matches::service::get_game_detail_logic(&client, game_id).await
}
