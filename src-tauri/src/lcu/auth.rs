use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::{RwLock, Mutex};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use crate::lcu::types::LcuAuthInfo;
use std::time::{Duration, Instant};

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new()));
static AUTH_INFO: Lazy<RwLock<Option<LcuAuthInfo>>> = Lazy::new(|| RwLock::new(None));
static AUTH_TIMESTAMP: Lazy<RwLock<Option<Instant>>> = Lazy::new(|| RwLock::new(None));
// 配置：token 最多允许缓存多久，超时自动刷新
const AUTH_REFRESH_INTERVAL: Duration = Duration::from_secs(60);
#[tauri::command]
pub fn get_auth_info() -> Option<LcuAuthInfo> {
    let auth = AUTH_INFO.read().unwrap();
    auth.as_ref().cloned()
}

/// 获取（并自动刷新）最新有效的 LCU AuthInfo
pub fn ensure_valid_auth_info() -> Option<LcuAuthInfo> {
    // 1. 先检测缓存是否存在且未超时
    {
        let auth = AUTH_INFO.read().unwrap();
        let ts = AUTH_TIMESTAMP.read().unwrap();
        if let (Some(a), Some(t)) = (auth.as_ref(), ts.as_ref()) {
            if t.elapsed() < AUTH_REFRESH_INTERVAL {
                log::debug!("[LCU] 使用缓存的 AuthInfo，距离上次刷新: {:?}秒", t.elapsed().as_secs());
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
    let mut system = SYSTEM.lock().unwrap();
    system.refresh_specifics(RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()));
    
    // 寻找所有可能的 LoL 客户端进程
    let possible_names = ["LeagueClientUx.exe", "LeagueClient.exe", "LeagueOfLegends.exe"];
    
    for (_pid, process) in system.processes() {
        let process_name = process.name();
        
        if possible_names.iter().any(|name| process_name.eq_ignore_ascii_case(name)) {
            let cmdline = process.cmd()
                .iter()
                .map(|s| s.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ");
                
            // 检查命令行是否包含 LCU 相关参数
            if cmdline.contains("--remoting-auth-token") && cmdline.contains("--app-port") {
                log::debug!("[LCU] 找到有效的 {} 进程，PID: {}", process_name.to_string_lossy(), _pid);
                log::debug!("[LCU] 启动参数: {}", cmdline);
                return Some(cmdline);
            } else {
                log::debug!("[LCU] 找到 {} 进程但不包含 LCU 参数，跳过", process_name.to_string_lossy());
            }
        }
    }
    
    log::debug!("[LCU] 未找到包含有效 LCU 参数的客户端进程");
    None
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
    
    let url = format!("https://127.0.0.1:{}/lol-summoner/v1/current-summoner", auth.app_port);
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

/// 获取（并自动刷新）最新有效的 LCU AuthInfo，带连接验证
pub async fn ensure_valid_auth_info_with_validation() -> Option<LcuAuthInfo> {
    // 1. 先尝试获取基本的认证信息
    let auth = ensure_valid_auth_info()?;
    
    // 2. 验证连接是否真正可用
    if validate_auth_connection(&auth).await {
        Some(auth)
    } else {
        log::warn!("[LCU] 认证信息存在但连接不可用，清除缓存");
        invalidate_auth_info();
        None
    }
}
