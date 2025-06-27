pub mod auth;
pub mod gameflow;
pub mod lobby;
pub mod match_history;
pub mod summoner;
pub mod types;
pub mod ranked;
pub mod matchmaking;
pub mod champ_select;
pub mod request;
pub mod connection_manager;
pub mod unified_polling;


// Re-export 常用的类型和函数，便于外部使用
pub use connection_manager::ConnectionManager;
