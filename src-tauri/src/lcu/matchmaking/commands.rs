use crate::{http_client, lcu};

#[tauri::command]
pub async fn start_matchmaking() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::service::start_matchmaking(client).await
}

#[tauri::command]
pub async fn stop_matchmaking() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::service::stop_matchmaking(client).await
}

#[tauri::command]
pub async fn accept_match() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::service::accept_match(client).await
}

#[tauri::command]
pub async fn decline_match() -> Result<(), String> {
    let client = http_client::get_lcu_client();
    lcu::matchmaking::service::decline_match(client).await
}
