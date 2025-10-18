pub mod commands;
/// 召唤师技能数据模块
/// 提供召唤师技能数据的加载、缓存和查询功能
pub mod service;

// 重新导出常用类型和函数
pub use service::{
    get_all_summoner_spells, get_spell_by_name, get_spell_count, get_spell_id_by_name, get_summoner_spell_info,
    is_loaded, load_summoner_spell_data, SummonerSpellInfo,
};
