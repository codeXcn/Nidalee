//! 数据服务领域
//! 
//! 负责提供各种数据服务：
//! - 召唤师信息
//! - 英雄数据
//! - 外部数据源（OP.GG等）

pub mod summoner;
pub mod champion_data;
pub mod external;

// Re-export services
pub use summoner::service as summoner_service;
pub use summoner::commands as summoner_commands;
pub use champion_data::service as champion_data_service;
pub use external::opgg::service as opgg_service;
