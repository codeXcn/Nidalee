use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use crate::lcu::types::LcuAuthInfo;

pub async fn get_gameflow_phase(client: &Client, auth_info: &LcuAuthInfo) -> Result<String, String> {
    let auth_string = format!("riot:{}", auth_info.remoting_auth_token);
    let base64_auth = general_purpose::STANDARD.encode(auth_string.as_bytes());

    let url = format!(
        "https://127.0.0.1:{}/lol-gameflow/v1/gameflow-phase",
        auth_info.app_port
    );
    println!("[Gameflow] 请求 URL: {}", url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", base64_auth))
        .send()
        .await;

    match response {
        Ok(resp) => {
            println!("[Gameflow] 响应状态: {}", resp.status());
            let status = resp.status();
            let text = resp.text().await.unwrap_or_else(|_| "<无法读取响应体>".to_string());
            println!("[Gameflow] 响应内容: {}", text);
            if !status.is_success() {
                return Err(format!("Failed to get gameflow phase: status {}", status));
            }
            match serde_json::from_str::<String>(&text) {
                Ok(phase) => {
                    println!("[Gameflow] 解析成功: {:?}", phase);
                    Ok(phase)
                },
                Err(e) => {
                    println!("[Gameflow] 解析失败: {}", e);
                    Err(format!("Failed to parse gameflow phase: {}", e))
                }
            }
        }
        Err(e) => {
            println!("[Gameflow] 请求失败: {}", e);
            Err(format!("Failed to get gameflow phase: {}", e))
        }
    }
}
