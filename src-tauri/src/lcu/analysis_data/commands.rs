use crate::lcu::types::TeamAnalysisData;

/// 🔥 获取当前缓存的分析数据（用于刷新后恢复）
///
/// 逻辑：
/// 1. 先尝试从内存缓存获取（最快）
/// 2. 如果没有缓存且在游戏中，主动构建数据（确保数据可用）
#[tauri::command]
pub async fn get_cached_analysis_data() -> Result<Option<TeamAnalysisData>, String> {
    log::info!("[AnalysisCommands] 🔍 尝试获取分析数据...");

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

    // 1. 首先尝试从内存缓存获取
    if let Some(data) = event_handler.get_cached_team_analysis_data().await {
        log::info!("[AnalysisCommands] ✅ 成功从内存缓存获取分析数据");
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

        return Ok(Some(data));
    }

    // 2. 没有缓存，检查是否在游戏中，如果是则主动构建
    log::warn!("[AnalysisCommands] ⚠️ 内存缓存中没有分析数据");

    // 检查当前游戏阶段
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    match crate::lcu::gameflow::service::get_gameflow_phase(&client).await {
        Ok(phase) if phase == "InProgress" => {
            log::info!("[AnalysisCommands] 🎮 检测到游戏进行中，主动构建分析数据...");

            // 🔥 主动调用 build_team_data_from_scratch
            match event_handler.build_and_get_team_data().await {
                Ok(data) => {
                    log::info!("[AnalysisCommands] ✅ 成功构建分析数据");
                    log::info!(
                        "[AnalysisCommands] 📊 构建数据: 我方{}人, 敌方{}人",
                        data.my_team.len(),
                        data.enemy_team.len()
                    );
                    Ok(Some(data))
                }
                Err(e) => {
                    log::error!("[AnalysisCommands] ❌ 构建分析数据失败: {}", e);
                    Err(e)
                }
            }
        }
        Ok(phase) => {
            log::info!("[AnalysisCommands] ℹ️ 当前游戏阶段: {}，无需构建数据", phase);
            Ok(None)
        }
        Err(e) => {
            log::warn!("[AnalysisCommands] ⚠️ 无法获取游戏阶段: {}", e);
            Ok(None)
        }
    }
}
