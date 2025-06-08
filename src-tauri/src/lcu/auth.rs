use std::process::Command;
use regex::Regex;
use crate::lcu::types::LcuAuthInfo;

pub fn get_lcu_auth_info() -> Result<LcuAuthInfo, String> {
    println!("[LCU] 开始获取 LCU 参数...");

    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-CimInstance Win32_Process -Filter \"Name = 'LeagueClientUx.exe'\" | Select-Object -ExpandProperty CommandLine"
        ])
        .output()
        .map_err(|e| format!("执行 PowerShell 命令失败: {}", e))?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("[LCU] 获取到的命令行参数: {}", output_str);

    if output_str.trim().is_empty() {
        println!("[LCU] 警告: 命令行参数为空");
        return Err("无法获取英雄联盟客户端的命令行参数。请确保：\n1. 客户端已启动\n2. 以管理员身份运行程序\n3. 没有被安全软件拦截".to_string());
    }

    // 创建所有需要的正则表达式
    let riotclient_token_re = Regex::new(r"--riotclient-auth-token=([\w-]+)").unwrap();
    let riotclient_port_re = Regex::new(r"--riotclient-app-port=(\d+)").unwrap();
    let remoting_token_re = Regex::new(r"--remoting-auth-token=([\w-]+)").unwrap();
    let app_port_re = Regex::new(r"--app-port=(\d+)").unwrap();

    // 提取所有参数
    let riotclient_auth_token = match riotclient_token_re.captures(&output_str)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string()) {
            Some(token) => {
                println!("[LCU] ✓ 已找到 riotclient-auth-token");
                token
            },
            None => {
                println!("[LCU] ✗ 未找到 riotclient-auth-token");
                return Err("未找到 RiotClient auth token".to_string());
            }
        };

    let riotclient_app_port = match riotclient_port_re.captures(&output_str)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().parse::<u16>()) {
            Some(Ok(port)) => {
                println!("[LCU] ✓ 已找到 riotclient-app-port");
                port
            },
            Some(Err(_)) => {
                println!("[LCU] ✗ riotclient-app-port 格式错误");
                return Err("RiotClient app port 格式错误".to_string());
            },
            None => {
                println!("[LCU] ✗ 未找到 riotclient-app-port");
                return Err("未找到 RiotClient app port".to_string());
            }
        };

    let remoting_auth_token = match remoting_token_re.captures(&output_str)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string()) {
            Some(token) => {
                println!("[LCU] ✓ 已找到 remoting-auth-token");
                token
            },
            None => {
                println!("[LCU] ✗ 未找到 remoting-auth-token");
                return Err("未找到 Remoting auth token".to_string());
            }
        };

    let app_port = match app_port_re.captures(&output_str)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().parse::<u16>()) {
            Some(Ok(port)) => {
                println!("[LCU] ✓ 已找到 app-port");
                port
            },
            Some(Err(_)) => {
                println!("[LCU] ✗ app-port 格式错误");
                return Err("App port 格式错误".to_string());
            },
            None => {
                println!("[LCU] ✗ 未找到 app-port");
                return Err("未找到 App port".to_string());
            }
        };

    let auth_info = LcuAuthInfo {
        app_port,
        remoting_auth_token,
        riotclient_app_port,
        riotclient_auth_token,
    };

    println!("[LCU] 成功获取所有参数: {:?}", auth_info);
    Ok(auth_info)
}

pub fn check_lcu_running() -> bool {
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-Process LeagueClientUx -ErrorAction SilentlyContinue"
        ])
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
