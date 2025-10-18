//! 共享层
//! 
//! 包含跨领域共享的代码：
//! - 类型定义
//! - 工具函数
//! - 错误处理

pub mod types;
pub mod utils;
pub mod errors;

// 包含工具模块
pub mod request;
pub mod optimized_polling;

// Re-export commonly used items
pub use types::*;
