/// 英雄数据模块
/// 提供英雄数据的加载、缓存和查询功能
pub mod service;
pub mod commands;

// 重新导出常用类型和函数
pub use service::{
    ChampionInfo,
    load_champion_data,
    get_champion_id_by_alias,
    get_champion_id_by_name,
    get_champion_info,
    get_champion_info_by_alias,
    get_champion_info_by_name,
    get_all_champions,
    is_loaded,
    get_champion_count,
};
