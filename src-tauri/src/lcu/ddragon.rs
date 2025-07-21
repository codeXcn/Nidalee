use crate::lcu::request::{forin_request_json};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

/// 获取所有符文信息
pub async fn get_all_runes(client: &Client) -> Result<crate::lcu::types::AllRunesResponse, String> {
    let path = "/api/data-dragon/runes";
    forin_request_json(client, Method::GET, path, None).await
}

/// 获取推荐出装（按数据源+英雄名）
pub async fn get_builds_by_alias(
    client: &Client,
    source: &str,
    champion: &str,
) -> Result<serde_json::Value, String> {
    let path = format!("/api/source/{}/champion-alias/{}", source, champion);
    forin_request_json(client, Method::GET, &path, None).await
}

/// 获取推荐符文（按数据源+英雄名）
pub async fn get_runes_by_alias(
    client: &Client,
    source: &str,
    champion: &str,
) -> Result<Vec<crate::lcu::types::DataDragonRune>, String> {
    let path = format!("/api/source/{}/champion-alias/{}/runes", source, champion);
    forin_request_json(client, Method::GET, &path, None).await
}
