pub mod auth;
pub mod gameflow;
pub mod lobby;
pub mod match_history;
pub mod summoner;
pub mod types;
pub mod ranked;
pub mod polling;
pub mod matchmaking;
pub mod champ_select;
pub mod request;

// Re-export 常用的类型和函数，便于外部使用
pub use types::*;
pub use auth::get_auth_info;
pub use request::{lcu_get, lcu_post, lcu_delete};
