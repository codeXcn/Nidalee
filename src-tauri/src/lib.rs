use serde::{Deserialize, Serialize};
use reqwest::{Client, header};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::sync::Arc;
use tokio::sync::Mutex;

// 基础配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LcpConfig {
    pub token: String,
    pub port: String,
}

// LCP客户端
#[derive(Debug, Clone)]
pub struct LcpClient {
    config: LcpConfig,
    client: Client,
}

impl LcpClient {
    pub fn new(config: LcpConfig) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    fn get_base_url(&self) -> String {
        format!("https://127.0.0.1:{}", self.config.port)
    }

    fn get_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        let auth = format!("riot:{}", self.config.token);
        let auth_header = format!("Basic {}", BASE64.encode(auth));
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_header).unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers
    }
}

// 游戏助手状态管理
#[derive(Debug)]
pub struct GameHelper {
    pub client: Arc<LcpClient>,
    auto_accept: Arc<Mutex<bool>>,
    auto_pick: Arc<Mutex<Option<i32>>>,
    auto_ban: Arc<Mutex<Option<i32>>>,
}

impl GameHelper {
    pub fn new(config: LcpConfig) -> Self {
        Self {
            client: Arc::new(LcpClient::new(config)),
            auto_accept: Arc::new(Mutex::new(true)),
            auto_pick: Arc::new(Mutex::new(None)),
            auto_ban: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_auto_accept(&self) {
        // 简化实现，后续添加具体逻辑
        println!("Auto accept started");
    }

    pub async fn start_auto_pick_ban(&self) {
        // 简化实现，后续添加具体逻辑
        println!("Auto pick/ban started");
    }

    pub async fn set_auto_accept(&self, enabled: bool) {
        *self.auto_accept.lock().await = enabled;
    }

    pub async fn set_auto_pick(&self, champion_id: Option<i32>) {
        *self.auto_pick.lock().await = champion_id;
    }

    pub async fn set_auto_ban(&self, champion_id: Option<i32>) {
        *self.auto_ban.lock().await = champion_id;
    }
} 