use regex::Regex;
use crate::lcu::types::LcuAuthInfo;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// 全局缓存 System 实例
static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new()));

/// 检查 LCU 是否正在运行
pub fn check_lcu_running() -> bool {
    let mut system = SYSTEM.lock().unwrap();
    system.refresh_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    for (_pid, process) in system.processes() {
        if process.name().to_string_lossy().to_lowercase().contains("leagueclientux") {
            // 只输出关键：发现进程
            println!("[LCU] 检测到 LeagueClientUx 进程正在运行");
            return true;
        }
    }

    // 关键：未发现
    println!("[LCU] 未检测到 LeagueClientUx 进程");
    false
}

/// 获取 LeagueClientUx.exe 启动参数命令行（拼接成 String）
fn get_lcu_cmdline() -> Option<String> {
    let mut system = SYSTEM.lock().unwrap();
    system.refresh_specifics(RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()));

    for (_pid, process) in system.processes() {
        if process.name().eq_ignore_ascii_case("LeagueClientUx.exe") {
            // 不再输出全部命令行
            println!("[LCU] 已获取 LeagueClientUx.exe 启动参数");
            let cmdline = process.cmd()
                .iter()
                .map(|s| s.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ");
            return Some(cmdline);
        }
    }
    println!("[LCU] 未找到 LeagueClientUx.exe 进程参数");
    None
}

/// 提取授权信息
pub fn get_lcu_auth_info() -> Result<LcuAuthInfo, String> {
    let cmdline = get_lcu_cmdline().ok_or("LeagueClientUx.exe not found")?;

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
        println!("[LCU] 授权参数解析成功");
        Ok(LcuAuthInfo {
            riotclient_auth_token: r_token,
            riotclient_app_port: r_port,
            remoting_auth_token: m_token,
            app_port: a_port,
        })
    } else {
        println!("[LCU] 授权参数解析失败");
        Err("Failed to parse LeagueClientUx command line".into())
    }
}