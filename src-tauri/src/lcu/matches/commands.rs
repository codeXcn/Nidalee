use crate::{http_client, lcu};

#[tauri::command]
pub async fn get_match_history(count: Option<u32>) -> Result<lcu::types::MatchStatistics, String> {
    let client = http_client::get_lcu_client();
    let end_count: usize = count.unwrap_or(20) as usize;
    println!("🔢 接收到的count参数: {:?}, 转换后的end_count: {}", count, end_count);
    lcu::matches::service::get_match_history(client, end_count).await
}

#[tauri::command]
pub async fn get_game_detail(game_id: u64) -> Result<lcu::types::GameDetail, String> {
    log::info!("🔍 ===== 获取游戏详细信息 =====");
    log::info!("🎮 游戏ID: {}", game_id);
    let client = http_client::get_lcu_client();
    lcu::matches::service::get_game_detail_logic(&client, game_id).await
}
