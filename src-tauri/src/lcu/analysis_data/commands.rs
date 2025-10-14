use super::service;
use crate::http_client;
use crate::lcu::types::TeamAnalysisData;

/// 测试新的队伍分析数据生成功能
///
/// 这是一个测试命令，用于验证分析数据生成流程
#[tauri::command]
pub async fn test_team_analysis_data() -> Result<TeamAnalysisData, String> {
    let client = http_client::get_lcu_client();

    // 创建一个模拟的 ChampSelect session 用于测试
    let mock_session = serde_json::json!({
        "localPlayerCellId": 0,
        "myTeam": [
            {
                "cellId": 0,
                "summonerId": "123456",
                "championId": 157, // Yasuo
                "championName": "Yasuo",
                "displayName": "",
                "gameName": "TestPlayer",
                "tagLine": "NA1",
                "spell1Id": 4,
                "spell2Id": 14,
                "assignedPosition": "MIDDLE",
                "profileIconId": 1,
                "puuid": "test-puuid-1",
                "nameVisibilityType": "VISIBLE"
            }
        ],
        "theirTeam": [
            {
                "cellId": 5,
                "summonerId": "789012",
                "championId": 238, // Zed
                "championName": "Zed",
                "displayName": "",
                "gameName": "EnemyPlayer",
                "tagLine": "NA1",
                "spell1Id": 4,
                "spell2Id": 14,
                "assignedPosition": "MIDDLE",
                "profileIconId": 2,
                "puuid": "test-puuid-2",
                "nameVisibilityType": "VISIBLE"
            }
        ]
    });

    // 调用服务层（测试命令使用空缓存）
    let mut temp_cache = std::collections::HashMap::new();
    match service::build_team_analysis_from_session(&mock_session, client, &mut temp_cache).await {
        Ok(analysis_data) => {
            log::info!("[AnalysisCommands] ✅ 成功生成分析数据");
            Ok(analysis_data)
        }
        Err(e) => {
            log::error!("[AnalysisCommands] ❌ 生成分析数据失败: {}", e);
            Err(format!("生成分析数据失败: {}", e))
        }
    }
}

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
