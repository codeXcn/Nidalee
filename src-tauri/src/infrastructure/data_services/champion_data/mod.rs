pub mod commands;
/// 英雄数据模块
/// 提供英雄数据的加载、缓存和查询功能
pub mod service;

// 重新导出常用类型和函数
pub use service::{
    get_all_champions, get_champion_count, get_champion_id_by_alias, get_champion_id_by_name, get_champion_info,
    get_champion_info_by_alias, get_champion_info_by_name, is_loaded, load_champion_data, ChampionInfo,
};
