/// 召唤师技能数据服务层 - 核心业务逻辑
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 🔥 全局静态变量：召唤师技能 ID -> 完整信息映射
static SUMMONER_SPELL_DATA: OnceCell<HashMap<i64, SummonerSpellInfo>> = OnceCell::new();

// 🔥 全局静态变量：召唤师技能名称 -> ID 映射
static SUMMONER_SPELL_NAME_TO_ID: OnceCell<HashMap<String, i64>> = OnceCell::new();

/// 召唤师技能信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpellInfo {
    pub id: i64, // 改为 i64 以支持大数值（API 可能返回 4294967295）
    pub name: String,
    pub description: String,
    pub summoner_level: i64, // 改为 i64
    pub cooldown: i64,       // 改为 i64
    pub game_modes: Vec<String>,
    pub icon_path: String,
}

/// 从 Community Dragon 获取召唤师技能数据并构建映射
pub async fn load_summoner_spell_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 检查是否已加载
    if SUMMONER_SPELL_DATA.get().is_some() {
        log::info!("[SummonerSpells] ✅ 召唤师技能数据已加载，跳过重复加载");
        return Ok(());
    }

    log::info!("[SummonerSpells] 🌐 正在从 Community Dragon 加载召唤师技能数据...");

    let url =
        "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/summoner-spells.json";

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let response = client.get(url).send().await?.error_for_status()?;
    let spells: Vec<SummonerSpellInfo> = response.json().await?;

    // 构建两个映射表
    let mut data_map: HashMap<i64, SummonerSpellInfo> = HashMap::new();
    let mut name_map: HashMap<String, i64> = HashMap::new();

    for spell in spells {
        // 过滤掉无效的技能（id 为 -1 或 4294967295 等无效值）
        if spell.id == -1 || spell.id == 4294967295 || spell.name.is_empty() {
            continue;
        }

        // ID 映射
        data_map.insert(spell.id, spell.clone());

        // 名称映射（用于反查 ID）
        name_map.insert(spell.name.clone(), spell.id);
    }

    log::info!(
        "[SummonerSpells] ✅ 召唤师技能数据加载完成，共 {} 个技能",
        data_map.len()
    );

    // 设置全局缓存
    SUMMONER_SPELL_DATA
        .set(data_map)
        .map_err(|_| "无法设置 SUMMONER_SPELL_DATA")?;
    SUMMONER_SPELL_NAME_TO_ID
        .set(name_map)
        .map_err(|_| "无法设置 SUMMONER_SPELL_NAME_TO_ID")?;

    Ok(())
}

/// 根据 ID 获取召唤师技能信息
pub fn get_summoner_spell_info(id: i64) -> Option<SummonerSpellInfo> {
    SUMMONER_SPELL_DATA.get()?.get(&id).cloned()
}

/// 获取所有召唤师技能数据（按 ID 排序）
pub fn get_all_summoner_spells() -> Option<Vec<SummonerSpellInfo>> {
    let data = SUMMONER_SPELL_DATA.get()?;
    let mut spells: Vec<SummonerSpellInfo> = data.values().cloned().collect();
    spells.sort_by_key(|s| s.id);
    Some(spells)
}

/// 检查数据是否已加载
pub fn is_loaded() -> bool {
    SUMMONER_SPELL_DATA.get().is_some() && SUMMONER_SPELL_NAME_TO_ID.get().is_some()
}

/// 获取召唤师技能总数
pub fn get_spell_count() -> usize {
    SUMMONER_SPELL_DATA.get().map(|m| m.len()).unwrap_or(0)
}

/// 根据名称查找召唤师技能 ID（支持中文）
pub fn get_spell_id_by_name(name: &str) -> Option<i64> {
    SUMMONER_SPELL_NAME_TO_ID.get()?.get(name).copied()
}

/// 根据名称查找召唤师技能（支持中文）
pub fn get_spell_by_name(name: &str) -> Option<SummonerSpellInfo> {
    let id = get_spell_id_by_name(name)?;
    get_summoner_spell_info(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_summoner_spell_data() {
        let result = load_summoner_spell_data().await;
        assert!(result.is_ok());
        assert!(is_loaded());

        // 测试常见技能
        let flash = get_summoner_spell_info(4);
        assert!(flash.is_some());
        assert_eq!(flash.unwrap().name, "闪现");

        let smite = get_summoner_spell_info(11);
        assert!(smite.is_some());
        assert_eq!(smite.unwrap().name, "惩戒");

        // 测试名称查询
        let ignite = get_spell_by_name("引燃");
        assert!(ignite.is_some());
        assert_eq!(ignite.unwrap().id, 14);

        // 测试获取所有技能
        let all_spells = get_all_summoner_spells();
        assert!(all_spells.is_some());
        assert!(all_spells.unwrap().len() > 10);
    }
}
