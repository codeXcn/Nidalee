/// 英雄数据命令层 - Tauri 命令接口
use super::service::{
    get_all_champions, get_champion_count, get_champion_id_by_name, get_champion_info, get_champion_info_by_alias,
    get_champion_info_by_name, is_loaded, load_champion_data, ChampionInfo,
};

/// 🌐 初始化英雄数据（应用启动时调用）
#[tauri::command]
pub async fn init_champion_data() -> Result<(), String> {
    match load_champion_data().await {
        Ok(_) => {
            log::info!(
                "[ChampionData] ✅ 英雄数据初始化成功，共 {} 个英雄",
                get_champion_count()
            );
            Ok(())
        }
        Err(e) => {
            log::error!("[ChampionData] ❌ 英雄数据初始化失败: {}", e);
            Err(format!("初始化英雄数据失败: {}", e))
        }
    }
}

/// 📋 获取所有英雄数据
#[tauri::command]
pub fn get_all_champion_data() -> Result<Vec<ChampionInfo>, String> {
    if !is_loaded() {
        return Err("英雄数据尚未加载，请先调用 init_champion_data".to_string());
    }

    get_all_champions().ok_or_else(|| "获取英雄数据失败".to_string())
}

/// 🔍 根据 ID 获取英雄信息
#[tauri::command]
pub fn get_champion_by_id(id: i32) -> Result<Option<ChampionInfo>, String> {
    if !is_loaded() {
        return Err("英雄数据尚未加载".to_string());
    }

    Ok(get_champion_info(id))
}

/// 🔍 根据别名获取英雄信息（英文名，不区分大小写）
#[tauri::command]
pub fn get_champion_by_alias(alias: String) -> Result<Option<ChampionInfo>, String> {
    if !is_loaded() {
        return Err("英雄数据尚未加载".to_string());
    }

    Ok(get_champion_info_by_alias(&alias))
}

/// 🔍 根据中文名称获取英雄信息（支持完整名称或称号）
#[tauri::command]
pub fn get_champion_by_name(name: String) -> Result<Option<ChampionInfo>, String> {
    if !is_loaded() {
        return Err("英雄数据尚未加载".to_string());
    }

    Ok(get_champion_info_by_name(&name))
}

pub fn get_id_by_name(name: String) -> Result<Option<i32>, String> {
    if !is_loaded() {
        return Err("英雄数据尚未加载".to_string());
    }

    Ok(get_champion_id_by_name(&name))
}

/// ✅ 检查英雄数据是否已加载
#[tauri::command]
pub fn is_champion_data_loaded() -> bool {
    is_loaded()
}

/// 📊 获取英雄数量
#[tauri::command]
pub fn get_champion_count_cmd() -> usize {
    get_champion_count()
}
