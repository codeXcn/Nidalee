// 应用配置模块 - 负责应用的初始化和配置
use tauri::{App, Manager};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{tray, lcu};

/// 应用启动时的设置函数
pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 设置系统托盘
    tray::setup_system_tray(app).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // 初始化连接管理器
    let connection_manager = Arc::new(RwLock::new(
        lcu::ConnectionManager::new(app.handle().clone())
    ));
    app.handle().manage(connection_manager.clone());

    // 启动连接监控和轮询服务
    start_services(app, connection_manager);

    Ok(())
}

/// 启动各种后台服务
fn start_services(app: &mut App, connection_manager: Arc<RwLock<lcu::ConnectionManager>>) {
    let app_handle = app.handle().clone();
    
    // 启动连接管理器
    tokio::spawn(async move {
        let manager = connection_manager.read().await;
        manager.start_monitoring().await;
    });
    
    // 启动传统轮询服务（保持兼容性）
    let app_handle_2 = app.handle().clone();
    tokio::spawn(async move {
        lcu::polling::start_polling(app_handle_2).await;
    });
}
