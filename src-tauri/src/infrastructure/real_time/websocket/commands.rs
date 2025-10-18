use crate::infrastructure::real_time::websocket::service;

#[tauri::command]
pub async fn start_lcu_ws(app: tauri::AppHandle) -> Result<(), String> {
    service::start_ws(app).await;
    Ok(())
}

#[tauri::command]
pub async fn stop_lcu_ws() -> Result<(), String> {
    service::stop_ws().await;
    Ok(())
}
