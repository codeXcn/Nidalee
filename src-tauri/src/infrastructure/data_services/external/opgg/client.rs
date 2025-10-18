use reqwest::Client;
use serde_json::Value;

/// OP.GG API 客户端
pub struct OpggClient {
    client: Client,
}

impl OpggClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    /// 获取英雄详细数据
    pub async fn get_champion_build(
        &self,
        region: &str,
        mode: &str,
        champion_id: i32,
        position: &str,
        tier: &str,
    ) -> Result<Value, String> {
        let url = if mode == "arena" {
            format!(
                "https://lol-api-champion.op.gg/api/{}/champions/{}/{}?tier={}",
                region, mode, champion_id, tier
            )
        } else {
            format!(
                "https://lol-api-champion.op.gg/api/{}/champions/{}/{}/{}?tier={}",
                region, mode, champion_id, position, tier
            )
        };

        log::info!("🌐 请求OP.GG API: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 请求失败: HTTP {}", response.status()));
        }

        let data: Value = response.json().await.map_err(|e| format!("解析 JSON 失败: {}", e))?;

        log::info!("✅ 成功获取OP.GG英雄详细数据");
        Ok(data)
    }

    /// 获取英雄层级列表
    pub async fn get_tier_list(&self, region: &str, mode: &str, tier: &str) -> Result<Value, String> {
        let url = format!(
            "https://lol-api-champion.op.gg/api/{}/champions/{}?tier={}",
            region, mode, tier
        );

        log::info!("🌐 请求OP.GG层级列表: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 请求失败: HTTP {}", response.status()));
        }

        let data: Value = response.json().await.map_err(|e| format!("解析 JSON 失败: {}", e))?;

        log::info!("✅ 成功获取OP.GG层级列表");
        Ok(data)
    }

    /// 获取英雄可用位置列表
    pub async fn get_champion_positions(
        &self,
        region: &str,
        champion_id: i32,
        tier: &str,
    ) -> Result<Vec<String>, String> {
        let url = format!(
            "https://lol-api-champion.op.gg/api/{}/champions/{}/positions?tier={}",
            region, champion_id, tier
        );

        log::info!("🌐 请求英雄位置列表: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 请求失败: HTTP {}", response.status()));
        }

        let data: Value = response.json().await.map_err(|e| format!("解析 JSON 失败: {}", e))?;

        let positions = data
            .as_array()
            .ok_or("无法解析位置数据")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        log::info!("✅ 成功获取英雄位置列表");
        Ok(positions)
    }
}
