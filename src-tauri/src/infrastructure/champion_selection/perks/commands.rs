use crate::{http_client, lcu};

#[tauri::command]
pub async fn get_lcu_rune_styles() -> Result<Vec<lcu::types::RuneStyle>, String> {
    let client = http_client::get_lcu_client();
    lcu::perks::service::list_all_styles(&client).await
}

#[tauri::command]
pub async fn get_lcu_perks() -> Result<Vec<lcu::types::Perk>, String> {
    let client = http_client::get_lcu_client();
    lcu::perks::service::list_all_perks(&client).await
}

#[tauri::command]
pub async fn get_lcu_perk_icon(icon_path: String) -> Result<Vec<u8>, String> {
    let client = http_client::get_lcu_client();
    lcu::perks::service::get_perk_icon(&client, &icon_path).await
}
