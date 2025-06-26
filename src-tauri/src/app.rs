// 应用配置模块 - 负责应用的初始化和配置
use tauri::App;
use crate::{tray, lcu};

/// 应用启动时的设置函数
pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 设置系统托盘
    tray::setup_system_tray(app).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // 启动LCU轮询管理器
    start_lcu_polling(app);

    Ok(())
}

/// 启动LCU轮询管理器
fn start_lcu_polling(app: &mut App) {
    let app_handle = app.handle().clone();
    tokio::spawn(async move {
        lcu::polling::start_polling(app_handle).await;
    });
}
