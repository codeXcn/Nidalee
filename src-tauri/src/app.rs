// 应用配置模块 - 负责应用的初始化和配置
use crate::{initialization, lcu, tray};
use std::sync::Arc;
use tauri::{App, Manager};
use tokio::sync::RwLock;

/// 应用启动时的设置函数
pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // 开发模式下启用日志
    #[cfg(debug_assertions)]
    {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )?;
    }

    // 设置系统托盘
    tray::setup_system_tray(app).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // 初始化连接管理器
    let connection_manager = Arc::new(RwLock::new(lcu::ConnectionManager::new(app.handle().clone())));
    app.handle().manage(connection_manager.clone());

    // 启动连接监控和轮询服务
    start_services(app, connection_manager);

    // 自动启动 LCU WebSocket
    start_websocket(app);

    // 🌐 初始化游戏数据（异步加载，不阻塞应用启动）
    initialization::start_game_data_initialization();

    Ok(())
}

/// 启动 WebSocket 连接
fn start_websocket(app: &mut App) {
    let app_handle = app.handle().clone();
    tokio::spawn(async move {
        lcu::ws::service::start_ws(app_handle).await;
    });
}

/// 启动各种后台服务
fn start_services(_app: &mut App, connection_manager: Arc<RwLock<lcu::ConnectionManager>>) {
    // 启动优化后的连接管理器（包含统一轮询）
    let connection_manager_clone = connection_manager.clone();
    tokio::spawn(async move {
        let manager = connection_manager_clone.read().await;
        manager.start_monitoring().await;
    });

    log::info!("[应用] 优化后的轮询系统已启动");
}
