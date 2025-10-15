/// 召唤师技能数据命令层 - Tauri 命令接口
use super::service::{
    SummonerSpellInfo,
    get_all_summoner_spells,
    get_summoner_spell_info,
    get_spell_by_name,
    is_loaded,
    load_summoner_spell_data,
    get_spell_count
};

/// 🌐 初始化召唤师技能数据（应用启动时调用）
#[tauri::command]
pub async fn init_summoner_spell_data() -> Result<(), String> {
    match load_summoner_spell_data().await {
        Ok(_) => {
            log::info!("[SummonerSpells] ✅ 召唤师技能数据初始化成功，共 {} 个技能", get_spell_count());
            Ok(())
        }
        Err(e) => {
            log::error!("[SummonerSpells] ❌ 召唤师技能数据初始化失败: {}", e);
            Err(format!("初始化召唤师技能数据失败: {}", e))
        }
    }
}

/// 📋 获取所有召唤师技能数据
#[tauri::command]
pub fn get_all_summoner_spell_data() -> Result<Vec<SummonerSpellInfo>, String> {
    if !is_loaded() {
        return Err("召唤师技能数据尚未加载，请先调用 init_summoner_spell_data".to_string());
    }

    get_all_summoner_spells().ok_or_else(|| "获取召唤师技能数据失败".to_string())
}

/// 🔍 根据 ID 获取召唤师技能信息
#[tauri::command]
pub fn get_summoner_spell_by_id(id: i64) -> Result<Option<SummonerSpellInfo>, String> {
    if !is_loaded() {
        return Err("召唤师技能数据尚未加载".to_string());
    }

    Ok(get_summoner_spell_info(id))
}

/// 🔍 根据名称获取召唤师技能信息
#[tauri::command]
pub fn get_summoner_spell_by_name(name: String) -> Result<Option<SummonerSpellInfo>, String> {
    if !is_loaded() {
        return Err("召唤师技能数据尚未加载".to_string());
    }

    Ok(get_spell_by_name(&name))
}

/// ✅ 检查召唤师技能数据是否已加载
#[tauri::command]
pub fn is_summoner_spell_data_loaded() -> bool {
    is_loaded()
}

/// 📊 获取召唤师技能数量
#[tauri::command]
pub fn get_summoner_spell_count() -> usize {
    get_spell_count()
}

