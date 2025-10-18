//! 游戏会话管理领域
//! 
//! 负责管理玩家的游戏会话生命周期：
//! - 认证授权
//! - 连接管理
//! - 游戏流程
//! - 大厅状态

pub mod auth;
pub mod connection;
pub mod gameflow;
pub mod lobby;

// Re-export services
pub use auth::service as auth_service;
pub use connection::service as connection_service;
pub use gameflow::service as gameflow_service;
pub use lobby::service as lobby_service;
