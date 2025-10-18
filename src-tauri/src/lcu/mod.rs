// 保留的模块（未移动）
pub mod ddragon;

// Re-export 从新架构中重新导出的模块，保持向后兼容
pub use crate::infrastructure::game_session::*;
pub use crate::infrastructure::match_management::*;
pub use crate::infrastructure::champion_selection::*;
pub use crate::infrastructure::real_time::*;
pub use crate::infrastructure::data_services::*;

// 重新导出子模块，保持向后兼容
pub mod opgg {
    pub use crate::infrastructure::data_services::external::opgg::*;
}

pub mod ws {
    pub use crate::infrastructure::real_time::websocket::*;
}

// Re-export 共享类型和工具
pub use crate::shared::types::*;
pub use crate::shared::utils::*;

// Re-export 常用的类型和函数，便于外部使用
pub use crate::infrastructure::game_session::connection::service::ConnectionManager;
