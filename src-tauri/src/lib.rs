use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use tokio::time::{sleep, Duration};
use std::sync::Mutex;
use lazy_static::lazy_static;

#[cfg(target_os = "windows")]
use std::process::Command;

// 全局缓存最后成功的lockfile路径
lazy_static! {
    static ref CACHED_LOCKFILE_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
}

// 获取缓存文件路径
fn get_cache_file_path() -> Result<PathBuf, String> {
    if let Ok(appdata) = env::var("APPDATA") {
        let cache_dir = PathBuf::from(appdata).join("Nidalee");
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir).map_err(|e| format!("创建缓存目录失败: {}", e))?;
        }
        Ok(cache_dir.join("lockfile_cache.txt"))
    } else {
        Err("无法获取APPDATA目录".to_string())
    }
}

// 从文件加载缓存路径
fn load_cached_path_from_file() -> Option<PathBuf> {
    match get_cache_file_path() {
        Ok(cache_file) => {
            if cache_file.exists() {
                match fs::read_to_string(&cache_file) {
                    Ok(content) => {
                        let path = PathBuf::from(content.trim());
                        if path.exists() {
                            println!("从文件加载缓存路径: {:?}", path);
                            return Some(path);
                        } else {
                            println!("缓存路径不存在，忽略: {:?}", path);
                        }
                    }
                    Err(e) => println!("读取缓存文件失败: {}", e),
                }
            }
        }
        Err(e) => println!("获取缓存文件路径失败: {}", e),
    }
    None
}

// 保存缓存路径到文件
fn save_cached_path_to_file(path: &PathBuf) {
    match get_cache_file_path() {
        Ok(cache_file) => {
            match fs::write(&cache_file, path.display().to_string()) {
                Ok(_) => println!("缓存路径已保存到文件: {:?}", path),
                Err(e) => println!("保存缓存路径到文件失败: {}", e),
            }
        }
        Err(e) => println!("获取缓存文件路径失败: {}", e),
    }
}

// 游戏配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub process_name: String,
    pub process_id: u32,
    pub port: String,
    pub token: String,
    pub protocol: String,
}

// 获取常见lockfile搜索路径
pub fn get_lockfile_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let drives = ["C:\\", "D:\\", "E:\\", "F:\\", "G:\\"];

    // WeGame 英雄联盟路径
    for drive in &drives {
        paths.push(PathBuf::from(drive)
            .join("WeGameApps")
            .join("英雄联盟")
            .join("Riot Client Data")
            .join("User Data")
            .join("Config")
            .join("lockfile"));
        paths.push(PathBuf::from(drive)
            .join("WeGameApps")
            .join("League of Legends")
            .join("Riot Client Data")
            .join("User Data")
            .join("Config")
            .join("lockfile"));
    }

    // 标准 Riot Games 路径
    for drive in &drives {
        paths.push(PathBuf::from(drive)
            .join("Riot Games")
            .join("League of Legends")
            .join("lockfile"));
        paths.push(PathBuf::from(drive)
            .join("Program Files")
            .join("Riot Games")
            .join("League of Legends")
            .join("lockfile"));
        paths.push(PathBuf::from(drive)
            .join("Program Files (x86)")
            .join("Riot Games")
            .join("League of Legends")
            .join("lockfile"));
    }

    // 用户数据目录
    if let Ok(localappdata) = env::var("LOCALAPPDATA") {
        paths.push(PathBuf::from(localappdata)
            .join("Riot Games")
            .join("Riot Client")
            .join("Data")
            .join("lockfile"));
    }

    // 用户目录相关路径
    if let Ok(userprofile) = env::var("USERPROFILE") {
        // 用户目录下的 Riot Games 路径
        paths.push(PathBuf::from(&userprofile)
            .join("AppData")
            .join("Local")
            .join("Riot Games")
            .join("Riot Client")
            .join("Data")
            .join("lockfile"));
    }

    paths
}

// 设置缓存的lockfile路径
pub fn set_cached_lockfile_path(path: PathBuf) {
    if let Ok(mut cached_path) = CACHED_LOCKFILE_PATH.lock() {
        *cached_path = Some(path.clone());
        println!("缓存lockfile路径: {:?}", path);
        // 保存到文件实现持久化
        save_cached_path_to_file(&path);
    }
}

// 获取缓存的lockfile路径
pub fn get_cached_lockfile_path() -> Option<PathBuf> {
    if let Ok(mut cached_path) = CACHED_LOCKFILE_PATH.lock() {
        // 如果内存中没有缓存，尝试从文件加载
        if cached_path.is_none() {
            *cached_path = load_cached_path_from_file();
        }
        cached_path.clone()
    } else {
        None
    }
}

// 清除缓存的lockfile路径
pub fn clear_cached_lockfile_path() {
    if let Ok(mut cached_path) = CACHED_LOCKFILE_PATH.lock() {
        *cached_path = None;
        println!("清除lockfile路径缓存");
        // 删除缓存文件
        if let Ok(cache_file) = get_cache_file_path() {
            if cache_file.exists() {
                let _ = fs::remove_file(&cache_file);
                println!("删除缓存文件: {:?}", cache_file);
            }
        }
    }
}

// 读取文件，带重试机制处理文件占用
pub async fn read_file_with_retry(path: &Path, max_retries: u32) -> Result<String, String> {
    let mut attempts = 0;
    let mut delay = Duration::from_millis(100);

    while attempts < max_retries {
        match fs::read_to_string(path) {
            Ok(content) => return Ok(content),
            Err(e) => {
                attempts += 1;
                let error_code = e.raw_os_error().unwrap_or(0);

                // 错误代码32表示文件被占用
                if error_code == 32 && attempts < max_retries {
                    println!("文件被占用，等待{}ms后重试... (第{}/{}次)", delay.as_millis(), attempts, max_retries);
                    sleep(delay).await;
                    delay = delay.saturating_mul(2); // 指数退避
                } else {
                    return Err(format!("读取文件失败: {} (错误代码: {})", e, error_code));
                }
            }
        }
    }

    Err(format!("读取文件失败，已重试{}次", max_retries))
}

// 解析lockfile内容
pub fn parse_lockfile(content: &str) -> Result<GameConfig, String> {
    let content = content.trim();
    let parts: Vec<&str> = content.split(':').collect();

    if parts.len() != 5 {
        return Err(format!("lockfile格式无效，期望5个部分，实际{}个: {}", parts.len(), content));
    }

    let process_name = parts[0];
    let process_id_str = parts[1];
    let port = parts[2];
    let token = parts[3];
    let protocol = parts[4];

    // 解析进程ID
    let process_id = process_id_str.parse::<u32>()
        .map_err(|_| format!("无效的进程ID: {}", process_id_str))?;

    // 支持两种格式: "LeagueClient" 和 "Riot Client"
    if process_name != "LeagueClient" && process_name != "Riot Client" {
        return Err(format!("未知的进程名: {}", process_name));
    }

    Ok(GameConfig {
        process_name: process_name.to_string(),
        process_id,
        port: port.to_string(),
        token: token.to_string(),
        protocol: protocol.to_string(),
    })
}

// 检查进程是否仍在运行
#[cfg(target_os = "windows")]
pub fn is_process_running(process_id: u32) -> bool {
    let output = Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", process_id), "/FO", "CSV"])
        .output();

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            // 如果输出包含进程ID，说明进程还在运行
            stdout.contains(&process_id.to_string())
        }
        Err(_) => false,
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_process_running(process_id: u32) -> bool {
    // 非Windows系统的实现
    use std::fs;
    fs::metadata(format!("/proc/{}", process_id)).is_ok()
}

// 安全读取lockfile并验证进程
pub async fn read_and_verify_lockfile(path: &Path) -> Result<GameConfig, String> {
    // 使用重试机制安全读取
    let content = read_file_with_retry(path, 3).await?;

    // 解析lockfile
    let config = parse_lockfile(&content)?;

    // 验证进程是否还在运行
    if !is_process_running(config.process_id) {
        return Err(format!("进程 {} (PID: {}) 已经不在运行", config.process_name, config.process_id));
    }

    println!("验证成功: {} (PID: {}) 正在运行，端口: {}",
             config.process_name, config.process_id, config.port);

    Ok(config)
}
