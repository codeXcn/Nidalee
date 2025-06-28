pub mod auth;
pub mod champ_select;
pub mod connection_manager;
pub mod gameflow;
pub mod lobby;
pub mod match_history;
pub mod matchmaking;
pub mod ranked;
pub mod request;
pub mod summoner;
pub mod types;
pub mod unified_polling;

// Re-export 常用的类型和函数，便于外部使用
pub use connection_manager::ConnectionManager;
