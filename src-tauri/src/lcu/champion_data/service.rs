/// 英雄数据服务层 - 核心业务逻辑
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 🔥 全局静态变量：英雄别名（英文名） -> ID 映射
static CHAMPION_ALIAS_TO_ID: OnceCell<HashMap<String, i32>> = OnceCell::new();

// 🔥 全局静态变量：英雄名称（中文名） -> ID 映射
static CHAMPION_NAME_TO_ID: OnceCell<HashMap<String, i32>> = OnceCell::new();

// 🔥 全局静态变量：英雄 ID -> 完整信息映射
static CHAMPION_DATA: OnceCell<HashMap<i32, ChampionInfo>> = OnceCell::new();

/// 英雄信息结构（根据实际 API 返回结构定义）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    pub id: i32,
    pub name: String,                 // 中文名称，如 "黑暗之女"
    pub description: String,          // 英雄称号，如 "安妮"
    pub alias: String,                // 英文别名，如 "Annie"
    pub content_id: String,           // 内容ID
    pub square_portrait_path: String, // 头像路径
    pub roles: Vec<String>,           // 英雄定位，如 ["mage", "support"]
}

/// 从 Community Dragon 获取英雄摘要数据并构建映射
pub async fn load_champion_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 检查是否已加载
    if CHAMPION_ALIAS_TO_ID.get().is_some() && CHAMPION_NAME_TO_ID.get().is_some() && CHAMPION_DATA.get().is_some() {
        log::info!("[ChampionData] ✅ 英雄数据已加载，跳过重复加载");
        return Ok(());
    }

    log::info!("[ChampionData] 🌐 正在从 Community Dragon 加载英雄摘要数据...");

    let url =
        "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/champion-summary.json";

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let response = client.get(url).send().await?.error_for_status()?;
    let champions: Vec<ChampionInfo> = response.json().await?;

    // 构建三个映射表
    let mut alias_map: HashMap<String, i32> = HashMap::new();
    let mut name_map: HashMap<String, i32> = HashMap::new();
    let mut data_map: HashMap<i32, ChampionInfo> = HashMap::new();

    for champ in champions {
        // 过滤掉 id = -1 的"无"英雄
        if champ.id < 0 {
            continue;
        }

        // 别名映射（英文名，统一转小写）
        alias_map.insert(champ.alias.to_lowercase(), champ.id);

        // 名称映射（中文名，支持多种查找方式）
        // 1. 完整名称，如 "黑暗之女"
        name_map.insert(champ.name.clone(), champ.id);
        // 2. 英雄称号，如 "安妮"（如果不为空）
        if !champ.description.is_empty() {
            name_map.insert(champ.description.clone(), champ.id);
        }

        // ID 映射
        data_map.insert(champ.id, champ);
    }

    log::info!("[ChampionData] ✅ 英雄数据加载完成，共 {} 个英雄", alias_map.len());

    // 设置全局缓存
    CHAMPION_ALIAS_TO_ID
        .set(alias_map)
        .map_err(|_| "无法设置 CHAMPION_ALIAS_TO_ID")?;
    CHAMPION_NAME_TO_ID
        .set(name_map)
        .map_err(|_| "无法设置 CHAMPION_NAME_TO_ID")?;
    CHAMPION_DATA.set(data_map).map_err(|_| "无法设置 CHAMPION_DATA")?;

    Ok(())
}

/// 根据别名获取英雄 ID（英文名，不区分大小写）
pub fn get_champion_id_by_alias(alias: &str) -> Option<i32> {
    CHAMPION_ALIAS_TO_ID.get()?.get(&alias.to_lowercase()).copied()
}

/// 根据中文名称获取英雄 ID（支持完整名称或称号）
pub fn get_champion_id_by_name(name: &str) -> Option<i32> {
    CHAMPION_NAME_TO_ID.get()?.get(name).copied()
}

/// 根据 ID 获取英雄信息
pub fn get_champion_info(id: i32) -> Option<ChampionInfo> {
    CHAMPION_DATA.get()?.get(&id).cloned()
}

/// 根据别名获取英雄信息（英文名，不区分大小写）
pub fn get_champion_info_by_alias(alias: &str) -> Option<ChampionInfo> {
    let id = get_champion_id_by_alias(alias)?;
    get_champion_info(id)
}

/// 根据中文名称获取英雄信息（支持完整名称或称号）
pub fn get_champion_info_by_name(name: &str) -> Option<ChampionInfo> {
    let id = get_champion_id_by_name(name)?;
    get_champion_info(id)
}

/// 获取所有英雄数据（按 ID 排序）
pub fn get_all_champions() -> Option<Vec<ChampionInfo>> {
    let data = CHAMPION_DATA.get()?;
    let mut champions: Vec<ChampionInfo> = data.values().cloned().collect();
    champions.sort_by_key(|c| c.id);
    Some(champions)
}

/// 检查数据是否已加载
pub fn is_loaded() -> bool {
    CHAMPION_ALIAS_TO_ID.get().is_some() && CHAMPION_NAME_TO_ID.get().is_some() && CHAMPION_DATA.get().is_some()
}

/// 获取英雄总数
pub fn get_champion_count() -> usize {
    CHAMPION_DATA.get().map(|m| m.len()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_champion_data() {
        let result = load_champion_data().await;
        assert!(result.is_ok());
        assert!(is_loaded());

        // 测试别名查询（英文名，不区分大小写）
        let yasuo_id_lower = get_champion_id_by_alias("yasuo");
        let yasuo_id_upper = get_champion_id_by_alias("Yasuo");
        assert_eq!(yasuo_id_lower, Some(157));
        assert_eq!(yasuo_id_upper, Some(157));

        // 测试中文名称查询
        let annie_id_by_name = get_champion_id_by_name("黑暗之女");
        assert_eq!(annie_id_by_name, Some(1));

        let annie_id_by_desc = get_champion_id_by_name("安妮");
        assert_eq!(annie_id_by_desc, Some(1));

        // 测试 ID 查询
        let yasuo_info = get_champion_info(157);
        assert!(yasuo_info.is_some());
        if let Some(info) = yasuo_info {
            assert_eq!(info.alias, "Yasuo");
        }

        // 测试获取所有英雄
        let all_champions = get_all_champions();
        assert!(all_champions.is_some());
        assert!(all_champions.unwrap().len() > 100); // LOL 有超过 100 个英雄
    }
}
