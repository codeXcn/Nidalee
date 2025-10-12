// 分层模块结构
pub mod service;  // 业务逻辑层
pub mod commands; // Tauri 命令层

// 重新导出常用类型（从全局 types 模块导入）
pub use crate::lcu::types::{
    TeamAnalysisData,
    PlayerAnalysisData,
    PlayerMatchStats,
    AnalysisChampionStats,
    MatchPerformance,
};

// 重新导出服务函数
pub use service::build_team_analysis_from_session;
