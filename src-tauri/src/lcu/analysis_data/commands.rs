use crate::lcu::types::TeamAnalysisData;

/// 🔥 获取当前缓存的分析数据（用于刷新后恢复）
///
/// 直接返回缓存的 TeamAnalysisData，无需重新构建
#[tauri::command]
pub async fn get_cached_analysis_data() -> Result<Option<TeamAnalysisData>, String> {
    log::info!("[AnalysisCommands] 🔍 尝试获取缓存的分析数据...");

    // 从全局获取事件处理器
    let event_handler = match crate::lcu::ws::service::get_event_handler() {
        Some(handler) => {
            log::info!("[AnalysisCommands] ✅ WebSocket 事件处理器已获取");
            handler
        }
        None => {
            log::warn!("[AnalysisCommands] ❌ WebSocket 事件处理器未初始化");
            return Ok(None);
        }
    };

    // 直接获取缓存的分析数据
    match event_handler.get_cached_team_analysis_data().await {
        Some(data) => {
            log::info!("[AnalysisCommands] ✅ 成功从缓存获取分析数据");
            log::info!(
                "[AnalysisCommands] 📊 数据摘要: 我方{}人, 敌方{}人",
                data.my_team.len(),
                data.enemy_team.len()
            );

            // 🔥 详细打印数据内容用于调试
            if log::log_enabled!(log::Level::Debug) {
                log::debug!("[AnalysisCommands] 📋 完整数据:");
                if let Ok(json_str) = serde_json::to_string_pretty(&data) {
                    log::debug!("{}", json_str);
                }
            }

            Ok(Some(data))
        }
        None => {
            log::warn!("[AnalysisCommands] ⚠️ 没有缓存的分析数据");

            // 🔥 额外调试：检查缓存状态
            let (has_session, has_analysis, stats_count) = event_handler.get_cache_debug_info().await;
            log::info!("[AnalysisCommands] 🔍 缓存状态调试:");
            log::info!("  - champ_select_session: {}", has_session);
            log::info!("  - team_analysis_data: {}", has_analysis);
            log::info!("  - match_stats_cache 大小: {}", stats_count);
            Ok(None)
        }
    }
}
