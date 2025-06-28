// 托盘管理模块 - 负责系统托盘的创建和事件处理
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager, Result as TauriResult,
};

/// 创建并配置系统托盘
pub fn setup_system_tray(app: &mut App) -> TauriResult<()> {
    // 创建托盘菜单项
    let show_i = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
    let maximize_i = MenuItem::with_id(app, "maximize", "最大化", true, None::<&str>)?;
    let minimize_i = MenuItem::with_id(app, "minimize", "最小化", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // 创建托盘菜单
    let menu = Menu::with_items(app, &[&show_i, &maximize_i, &minimize_i, &quit_i])?;

    // 创建系统托盘
    let _tray = TrayIconBuilder::new()
        // 添加托盘图标
        .icon(app.default_window_icon().unwrap().clone())
        // 添加菜单
        .menu(&menu)
        // 添加悬浮提示
        .tooltip("Nidalee - 高性能、体积小巧的智能英雄联盟游戏助手")
        // 禁用鼠标左键点击图标显示托盘菜单
        .show_menu_on_left_click(false)
        // 监听托盘图标发出的鼠标事件
        .on_tray_icon_event(handle_tray_icon_event)
        // 监听菜单事件
        .on_menu_event(handle_menu_event)
        .build(app)?;

    Ok(())
}

/// 处理托盘图标事件
fn handle_tray_icon_event(tray: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        if let Some(window) = tray.app_handle().get_webview_window("main") {
            if let Ok(is_visible) = window.is_visible() {
                if is_visible {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        }
    }
}

/// 处理托盘菜单事件
fn handle_menu_event(app_handle: &AppHandle, event: tauri::menu::MenuEvent) {
    let window = app_handle.get_webview_window("main").unwrap();
    match event.id.as_ref() {
        "show" => {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
        "maximize" => {
            let _ = window.maximize();
            let _ = window.set_focus();
        }
        "minimize" => {
            let _ = window.minimize();
        }
        "quit" => {
            app_handle.exit(0);
        }
        _ => {}
    }
}
