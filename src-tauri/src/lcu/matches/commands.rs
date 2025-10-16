use crate::{http_client, lcu};

#[tauri::command]
pub async fn get_match_history(
    count: Option<u32>,
    queue_id: Option<i64>,
) -> Result<lcu::types::PlayerMatchStats, String> {
    let client = http_client::get_lcu_client();
    let end_count: usize = count.unwrap_or(20) as usize;
    println!("🔢 接收到的参数: count={:?}, queue_id={:?}", count, queue_id);
    lcu::matches::service::get_match_history(client, end_count, queue_id).await
}

#[tauri::command]
pub async fn get_game_detail(game_id: u64) -> Result<lcu::types::GameDetail, String> {
    log::info!("🔍 ===== 获取游戏详细信息 =====");
    log::info!("🎮 游戏ID: {}", game_id);
    let client = http_client::get_lcu_client();
    lcu::matches::service::get_game_detail_logic(&client, game_id).await
}
