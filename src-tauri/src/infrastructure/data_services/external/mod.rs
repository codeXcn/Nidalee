//! 外部数据服务
//! 
//! 负责集成外部数据源：
//! - OP.GG 数据
//! - 其他第三方API

pub mod opgg;

// Re-export services
pub use opgg::service as opgg_service;
pub use opgg::commands as opgg_commands;
