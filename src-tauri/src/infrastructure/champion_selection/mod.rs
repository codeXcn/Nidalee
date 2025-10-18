//! 英雄选择领域
//! 
//! 负责配置游戏前的所有选择：
//! - 英雄选择
//! - 召唤师技能
//! - 符文配置

pub mod champ_select;
pub mod summoner_spells;
pub mod perks;

// Re-export services
pub use champ_select::service as champ_select_service;
pub use champ_select::commands as champ_select_commands;
pub use summoner_spells::service as summoner_spells_service;
pub use summoner_spells::commands as summoner_spells_commands;
pub use perks::service as perks_service;
