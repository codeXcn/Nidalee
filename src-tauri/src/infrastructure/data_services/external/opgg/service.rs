use super::client::OpggClient;
use super::parser::parse_champion_build;
use super::types::*;
use serde_json::Value;

pub async fn get_champion_build(
    region: &str,
    mode: &str,
    champion_id: i32,
    position: Option<String>,
    tier: &str,
) -> Result<OpggChampionBuild, String> {
    let client = OpggClient::new();
    let pos = position.clone().unwrap_or_else(|| "MID".to_string());
    let raw_data = client.get_champion_build(region, mode, champion_id, &pos, tier).await?;
    parse_champion_build(raw_data, &pos).map_err(|e| e.to_string())
}

pub async fn get_champion_build_raw(
    region: &str,
    mode: &str,
    champion_id: i32,
    position: Option<String>,
    tier: &str,
) -> Result<Value, String> {
    let client = OpggClient::new();
    let pos = position.clone().unwrap_or_else(|| "MID".to_string());
    client.get_champion_build(region, mode, champion_id, &pos, tier).await
}

pub async fn get_tier_list(region: &str, mode: &str, tier: &str) -> Result<OpggTierList, String> {
    let client = OpggClient::new();
    let raw_data = client.get_tier_list(region, mode, tier).await?;
    // 解析层级列表数据
    let meta = raw_data.get("meta").ok_or("无法获取元数据")?;
    let data = raw_data
        .get("data")
        .and_then(|v| v.as_array())
        .ok_or("无法获取层级数据")?;
    let tier_list_data: Vec<OpggTierListItem> = data
        .iter()
        .filter_map(|item| {
            let champion_id = item.get("champion_id")?.as_i64()? as i32;
            let name = item.get("name")?.as_str()?.to_string();
            let tier = item.get("tier")?.as_str()?.to_string();
            let rank = item.get("rank")?.as_i64()? as i32;
            let win_rate = item.get("win_rate")?.as_f64()? / 100.0;
            let pick_rate = item.get("pick_rate")?.as_f64()? / 100.0;
            let ban_rate = item.get("ban_rate")?.as_f64()? / 100.0;
            Some(OpggTierListItem {
                champion_id,
                name,
                tier,
                rank,
                win_rate,
                pick_rate,
                ban_rate,
            })
        })
        .collect();
    let meta_data = OpggTierListMeta {
        version: meta.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        region: region.to_string(),
        mode: mode.to_string(),
        tier: tier.to_string(),
    };
    Ok(OpggTierList {
        meta: meta_data,
        data: tier_list_data,
    })
}

pub async fn get_champion_positions(region: &str, champion_id: i32, tier: &str) -> Result<Vec<String>, String> {
    let client = OpggClient::new();
    client.get_champion_positions(region, champion_id, tier).await
}
