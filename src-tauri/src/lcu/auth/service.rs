use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::{Mutex, RwLock};
use std::time::{Duration, Instant};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use crate::lcu::types::LcuAuthInfo;

pub static AUTH_INFO: Lazy<RwLock<Option<LcuAuthInfo>>> = Lazy::new(|| RwLock::new(None));
static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new()));
static AUTH_TIMESTAMP: Lazy<RwLock<Option<Instant>>> = Lazy::new(|| RwLock::new(None));
// 配置：token 最多允许缓存多久，超时自动刷新
const AUTH_REFRESH_INTERVAL: Duration = Duration::from_secs(60);

/// 获取（并自动刷新）最新有效的 LCU AuthInfo
pub fn ensure_valid_auth_info() -> Option<LcuAuthInfo> {
    // 1. 先检测缓存是否存在且未超时
    {
        let auth = AUTH_INFO.read().unwrap();
        let ts = AUTH_TIMESTAMP.read().unwrap();
        if let (Some(a), Some(t)) = (auth.as_ref(), ts.as_ref()) {
            if t.elapsed() < AUTH_REFRESH_INTERVAL {
                log::debug!(
                    "[LCU] 使用缓存的 AuthInfo，距离上次刷新: {:?}秒",
                    t.elapsed().as_secs()
                );
                return Some(a.clone());
            } else {
                log::info!("[LCU] AuthInfo 缓存已过期，准备刷新");
            }
        } else {
            log::debug!("[LCU] 当前无有效缓存，准备刷新");
        }
    }

    // 2. 带重试的自动刷新（LOL 启动初期可能需要多次尝试）
    for attempt in 1..=3 {
        match refresh_auth_info() {
            Ok(auth) => {
                log::info!("[LCU] 自动刷新 AuthInfo 成功 (尝试 {}/3)", attempt);
                return Some(auth);
            }
            Err(e) => {
                log::warn!("[LCU] 自动刷新 AuthInfo 失败 (尝试 {}/3): {}", attempt, e);
                if attempt < 3 {
                    // 短暂等待后重试
                    std::thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }

    log::error!("[LCU] 多次尝试后仍无法获取有效的 AuthInfo");
    None
}

/// 手动强制刷新 AuthInfo（一般不直接用，内部自动调用）
pub fn refresh_auth_info() -> Result<LcuAuthInfo, String> {
    log::info!("[LCU] 开始强制刷新 AuthInfo");
    let cmdline = match get_lcu_cmdline() {
        Some(cmd) => cmd,
        None => {
            log::error!("[LCU] LeagueClientUx.exe 进程未找到，无法刷新 AuthInfo");
            invalidate_auth_info();
            return Err("LeagueClientUx.exe not found".into());
        }
    };
    let riotclient_token_re = Regex::new(r"--riotclient-auth-token=([^\s]+)").unwrap();
    let riotclient_port_re = Regex::new(r"--riotclient-app-port=([^\s]+)").unwrap();
    let remoting_token_re = Regex::new(r"--remoting-auth-token=([^\s]+)").unwrap();
    let app_port_re = Regex::new(r"--app-port=([^\s]+)").unwrap();

    let riotclient_auth_token = riotclient_token_re
        .captures(&cmdline)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    let riotclient_app_port = riotclient_port_re
        .captures(&cmdline)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u16>().ok());
    let remoting_auth_token = remoting_token_re
        .captures(&cmdline)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    let app_port = app_port_re
        .captures(&cmdline)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u16>().ok());
    if let (Some(r_token), Some(r_port), Some(m_token), Some(a_port)) = (
        riotclient_auth_token,
        riotclient_app_port,
        remoting_auth_token,
        app_port,
    ) {
        let auth = LcuAuthInfo {
            riotclient_auth_token: r_token,
            riotclient_app_port: r_port,
            remoting_auth_token: m_token,
            app_port: a_port,
        };
        {
            let mut info = AUTH_INFO.write().unwrap();
            *info = Some(auth.clone());
        }
        {
            let mut ts = AUTH_TIMESTAMP.write().unwrap();
            *ts = Some(Instant::now());
        }
        log::info!(
            "[LCU] AuthInfo 刷新成功，端口: {}, token: {}... (已隐藏)",
            auth.app_port,
            &auth.remoting_auth_token[..8.min(auth.remoting_auth_token.len())]
        );
        Ok(auth)
    } else {
        log::error!("[LCU] 解析 LeagueClientUx.exe 启动参数失败，清空缓存");
        invalidate_auth_info();
        Err("Failed to parse LeagueClientUx command line".into())
    }
}

fn get_lcu_cmdline() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        get_lcu_cmdline_windows()
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        get_lcu_cmdline_unix()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        log::error!("[LCU] 当前操作系统暂不支持自动获取 LoL 参数");
        None
    }
}

pub fn invalidate_auth_info() {
    let mut info = AUTH_INFO.write().unwrap();
    *info = None;
    let mut ts = AUTH_TIMESTAMP.write().unwrap();
    *ts = None;
    log::info!("[LCU] AuthInfo 缓存已清除");
}

/// 验证 AuthInfo 是否真正可用（通过简单的 API 测试）
pub async fn validate_auth_connection(auth: &LcuAuthInfo) -> bool {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(_) => return false,
    };

    let url = format!(
        "https://127.0.0.1:{}/lol-summoner/v1/current-summoner",
        auth.app_port
    );
    let response = client
        .get(&url)
        .basic_auth("riot", Some(&auth.remoting_auth_token))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let success = resp.status().is_success();
            log::debug!("[LCU] 连接验证结果: {}, 状态码: {}", success, resp.status());
            success
        }
        Err(e) => {
            log::debug!("[LCU] 连接验证失败: {}", e);
            false
        }
    }
}
#[cfg(target_os = "windows")]
fn get_lcu_cmdline_windows() -> Option<String> {
   let mut system = SYSTEM.lock().unwrap();
    system
        .refresh_specifics(RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()));

    let mut ux_cmdline = None;
    let mut client_cmdline = None;

    // 寻找所有可能的 LoL 客户端进程
    let possible_names = [
        "LeagueClientUx.exe",
        "LeagueClient.exe",
        "LeagueOfLegends.exe",
    ];

    for (_pid, process) in system.processes() {
        let process_name_lower = process.name().to_string_lossy().to_lowercase();

        if possible_names
            .iter()
            .any(|name| process_name_lower == name.to_lowercase())
        {
            let cmdline = process
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ");

            // 优先检查 LeagueClientUx.exe
            if process_name_lower == "leagueclientux.exe" {
                if cmdline.contains("--remoting-auth-token") && cmdline.contains("--app-port") {
                    log::debug!(
                        "[LCU] 找到包含 LCU 参数的 LeagueClientUx.exe 进程, PID: {}",
                        _pid
                    );
                    return Some(cmdline); // 找到最理想的目标，直接返回
                } else {
                    ux_cmdline = Some(cmdline); // 暂存，可能没有参数
                }
            }
            // 如果没找到 Ux，再检查 LeagueClient.exe
            else if process_name_lower == "leagueclient.exe" {
                client_cmdline = Some(cmdline);
            }
        }
    }

    // 如果 Ux 进程的命令行里没有参数，尝试使用 Client 进程的命令行
    if let Some(cmd) = client_cmdline {
        if cmd.contains("--remoting-auth-token") && cmd.contains("--app-port") {
            log::warn!("[LCU] LeagueClientUx.exe 命令行中未找到认证参数，回退使用 LeagueClient.exe 的参数。");
            return Some(cmd);
        }
    }

    // 如果 Client 的参数也没有，但 Ux 进程确实存在，则返回它的（不带参数的）命令行
    if let Some(cmd) = ux_cmdline {
        log::warn!("[LCU] 无法在任何进程的命令行中找到认证参数，将使用无参数的 LeagueClientUx.exe 命令行进行后续尝试。");
        return Some(cmd);
    }

    log::debug!("[LCU] 未找到包含有效 LCU 参数的客户端进程");
    None
}
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_lcu_cmdline_unix() -> Option<String> {
    let mut system = SYSTEM.lock().unwrap();
    system.refresh_specifics(RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()));

    for (_pid, process) in system.processes() {
        let name = process.name().to_string_lossy().to_lowercase();
        if name.contains("leagueclientux") {
            let cmdline = process
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ");

            if cmdline.contains("--remoting-auth-token") && cmdline.contains("--app-port") {
                return Some(cmdline);
            }
        }
    }
    None
}

// #[cfg(target_os = "macos")]
// fn get_lcu_cmdline_macos() -> Option<String> {
//     use std::process::Command;

//     let output = Command::new("sh")
//         .arg("-c")
//         .arg("ps -ax -o command | grep 'LeagueClientUx' | grep -- '--remoting-auth-token' | grep -- '--app-port' | grep -v grep")
//         .output()
//         .ok()?;

//     if !output.status.success() {
//         return None;
//     }

//     let result = String::from_utf8_lossy(&output.stdout);
//     for line in result.lines() {
//         if line.contains("--remoting-auth-token") && line.contains("--app-port") {
//             return Some(line.to_string());
//         }
//     }

//     None
// }
// #[cfg(target_os = "linux")]
// fn get_lcu_cmdline_linux() -> Option<String> {
//     use std::process::Command;

//     let output = Command::new("sh")
//         .arg("-c")
//         .arg("ps -e -o args | grep 'LeagueClientUx' | grep -- '--remoting-auth-token' | grep -- '--app-port' | grep -v grep")
//         .output()
//         .ok()?;

//     if !output.status.success() {
//         return None;
//     }

//     let result = String::from_utf8_lossy(&output.stdout);
//     for line in result.lines() {
//         if line.contains("--remoting-auth-token") && line.contains("--app-port") {
//             return Some(line.to_string());
//         }
//     }

//     None
// }
