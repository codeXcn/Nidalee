/// 通用玩家战绩分析器模块
///
/// 职责：
/// - 提供统一的战绩数据分析逻辑
/// - 可被多个模块复用（个人战绩、对局分析等）
/// - 包含量化指标计算和定性特征分析

pub mod analyzer;
pub mod traits_analyzer;

pub use analyzer::{analyze_player_stats, AnalysisContext};
pub use traits_analyzer::analyze_traits;

