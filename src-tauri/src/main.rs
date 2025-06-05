 // Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;

use lib::GameConfig;
use std::sync::Arc;
use tokio::sync::Mutex;

// 应用状态
#[derive(Default)]
struct AppState {
    game_config: Arc<Mutex<Option<GameConfig>>>,
    auto_accept_enabled: Arc<Mutex<bool>>,
}

// Tauri命令：查找并解析lockfile
#[tauri::command]
async fn find_lol_lockfile() -> Result<GameConfig, String> {
    println!("开始搜索lockfile...");

        // 首先检查缓存的路径
    if let Some(cached_dir) = lib::get_cached_lockfile_path() {
        println!("检查缓存目录: {}", cached_dir.display());

        let cached_lockfile = cached_dir.join("lockfile");
        if cached_lockfile.exists() {
            println!("缓存路径中找到lockfile: {}", cached_lockfile.display());

            // 找到lockfile就直接使用，不管验证是否成功
            match lib::read_and_verify_lockfile(&cached_lockfile).await {
                Ok(config) => {
                    println!("缓存路径验证成功: 进程 {} (PID: {}) 端口: {}",
                             config.process_name, config.process_id, config.port);
                    return Ok(config);
                }
                Err(e) => {
                    println!("缓存路径验证失败: {}, 但已找到lockfile，直接返回错误", e);
                    // 有lockfile但验证失败，直接返回错误，不进行完整搜索
                    return Err(e);
                }
            }
        } else {
            println!("缓存路径中暂时没有lockfile，进行完整搜索");
            // 缓存路径中没有lockfile，才进行完整搜索
        }
    }

    // 缓存路径中没有lockfile，进行完整搜索
    println!("开始完整搜索...");
    let paths = lib::get_lockfile_paths();
    println!("共{}个搜索路径", paths.len());

    for (index, path) in paths.iter().enumerate() {
        println!("检查路径 {}/{}: {}", index + 1, paths.len(), path.display());

        if path.exists() {
            println!("找到lockfile: {}", path.display());

            // 找到lockfile就缓存路径（去掉lockfile文件名，只保留目录）
            if let Some(parent_dir) = path.parent() {
                lib::set_cached_lockfile_path(parent_dir.to_path_buf());
            }

            match lib::read_and_verify_lockfile(path).await {
                Ok(config) => {
                    println!("lockfile验证成功: 进程 {} (PID: {}) 端口: {}",
                             config.process_name, config.process_id, config.port);
                    return Ok(config);
                }
                Err(e) => {
                    println!("读取或验证失败: {}", e);

                    // 如果错误信息包含"已经不在运行"，说明找到了lockfile但进程已停止
                    // 这种情况下不应该继续搜索其他路径，直接返回错误（但路径已经缓存了）
                    if e.contains("已经不在运行") || e.contains("不在运行") {
                        return Err(format!("找到lockfile但进程已停止: {}", e));
                    }

                    // 其他错误（如文件读取失败等）继续搜索
                    continue;
                }
            }
        }
    }

    Err("未找到任何lockfile".to_string())
}

// Tauri命令：测试自定义路径
#[tauri::command]
async fn test_lockfile_path(path: String) -> Result<GameConfig, String> {
    let lockfile_path = std::path::PathBuf::from(path);

    if !lockfile_path.exists() {
        return Err("指定路径的lockfile不存在".to_string());
    }

    lib::read_and_verify_lockfile(&lockfile_path).await
}

// Tauri命令：获取所有搜索路径
#[tauri::command]
fn get_search_paths() -> Vec<String> {
    lib::get_lockfile_paths()
        .iter()
        .map(|p| p.display().to_string())
        .collect()
}

// 获取缓存状态信息
#[tauri::command]
fn get_cache_status() -> Result<serde_json::Value, String> {
    let cached_path = lib::get_cached_lockfile_path();

    Ok(serde_json::json!({
        "hasCachedPath": cached_path.is_some(),
        "cachedPath": cached_path.as_ref().map(|p| p.display().to_string()),
        "isValid": if let Some(path) = &cached_path {
            path.join("lockfile").exists()
        } else {
            false
        }
    }))
}

// 清除路径缓存
#[tauri::command]
fn clear_path_cache() -> Result<(), String> {
    lib::clear_cached_lockfile_path();
    Ok(())
}

// 初始化游戏连接
#[tauri::command]
async fn init_game_connection(
    state: tauri::State<'_, AppState>,
    config: GameConfig,
) -> Result<(), String> {
    *state.game_config.lock().await = Some(config);
    println!("游戏连接已初始化");
    Ok(())
}

// 设置自动接受
#[tauri::command]
async fn set_auto_accept(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    *state.auto_accept_enabled.lock().await = enabled;
    println!("自动接受设置为: {}", enabled);
    Ok(())
}

// 获取当前游戏连接状态
#[tauri::command]
async fn get_connection_status(
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let config = state.game_config.lock().await;
    Ok(config.is_some())
}

// 验证当前连接是否仍然有效
#[tauri::command]
async fn verify_connection(
    state: tauri::State<'_, AppState>,
) -> Result<GameConfig, String> {
    let config_guard = state.game_config.lock().await;

    match config_guard.as_ref() {
        Some(config) => {
            // 检查进程是否还在运行
            if lib::is_process_running(config.process_id) {
                Ok(config.clone())
            } else {
                Err(format!("进程 {} (PID: {}) 已经停止运行", config.process_name, config.process_id))
            }
        }
        None => Err("没有活跃的游戏连接".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            find_lol_lockfile,
            test_lockfile_path,
            get_search_paths,
            get_cache_status,
            clear_path_cache,
            init_game_connection,
            set_auto_accept,
            get_connection_status,
            verify_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
