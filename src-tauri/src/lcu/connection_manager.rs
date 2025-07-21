use crate::lcu::auth::{ensure_valid_auth_info, validate_auth_connection};
use crate::lcu::types::{ConnectionState, LcuAuthInfo};
use crate::lcu::unified_polling::UnifiedPollingManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/generated/ConnectionInfo.ts")]
pub struct ConnectionInfo {
    pub state: ConnectionState,
    pub auth_info: Option<LcuAuthInfo>,
    #[ts(skip)]
    #[serde(skip)]
    pub last_successful_connection: Option<Instant>,
    pub consecutive_failures: u32,
    pub error_message: Option<String>,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            auth_info: None,
            last_successful_connection: None,
            consecutive_failures: 0,
            error_message: None,
        }
    }
}

pub struct ConnectionManager {
    info: Arc<RwLock<ConnectionInfo>>,
    app: AppHandle,
    polling_manager: UnifiedPollingManager,
}

impl ConnectionManager {
    pub fn new(app: AppHandle) -> Self {
        let polling_manager = UnifiedPollingManager::new(app.clone());
        Self {
            info: Arc::new(RwLock::new(ConnectionInfo::default())),
            app,
            polling_manager,
        }
    }

    /// 启动连接监控
    pub async fn start_monitoring(&self) {
        let info = self.info.clone();
        let app = self.app.clone();
        let polling_manager = self.polling_manager.clone();

        tokio::spawn(async move {
            let mut manager = ConnectionManager {
                info,
                app: app.clone(),
                polling_manager: polling_manager.clone(),
            };
            manager.monitor_loop().await;
        });

        // 启动统一轮询管理器
        self.polling_manager.start().await;
    }

    async fn monitor_loop(&mut self) {
        let mut last_state = ConnectionState::Disconnected;

        loop {
            let current_state = self.check_connection_state().await;

            // 状态变化时通知前端
            if current_state != last_state {
                self.handle_state_change(&last_state, &current_state).await;
                last_state = current_state.clone();
            }

            // 根据当前状态决定下次检查间隔
            let sleep_duration = self.get_check_interval(&current_state).await;
            tokio::time::sleep(sleep_duration).await;
        }
    }

    async fn check_connection_state(&self) -> ConnectionState {
        // 1. 尝试获取认证信息
        let auth_info = ensure_valid_auth_info();

        if auth_info.is_none() {
            // 无认证信息，检查是否有进程
            if self.has_lol_process().await {
                self.update_info(|info| {
                    info.state = ConnectionState::ProcessFound;
                    info.auth_info = None;
                    info.consecutive_failures += 1;
                    info.error_message = Some("客户端进程已启动但无法获取认证信息".to_string());
                })
                .await;
                return ConnectionState::ProcessFound;
            } else {
                self.update_info(|info| {
                    info.state = ConnectionState::Disconnected;
                    info.auth_info = None;
                    info.consecutive_failures = 0;
                    info.error_message = Some("未检测到客户端进程".to_string());
                })
                .await;
                return ConnectionState::Disconnected;
            }
        }

        let auth = auth_info.unwrap();

        // 2. 验证连接是否真正可用
        let connection_valid = validate_auth_connection(&auth).await;

        if connection_valid {
            self.update_info(|info| {
                info.state = ConnectionState::Connected;
                info.auth_info = Some(auth.clone());
                info.last_successful_connection = Some(Instant::now());
                info.consecutive_failures = 0;
                info.error_message = None;
            })
            .await;

            // 连接成功时立即发送数据刷新事件
            let _ = self.app.emit("refresh-data", ());

            ConnectionState::Connected
        } else {
            // 连接失败，需要再次检查进程是否还存在
            if !self.has_lol_process().await {
                // 进程已经不存在，说明客户端已退出
                self.update_info(|info| {
                    info.state = ConnectionState::Disconnected;
                    info.auth_info = None;
                    info.consecutive_failures = 0;
                    info.error_message = Some("客户端进程已退出".to_string());
                })
                .await;
                return ConnectionState::Disconnected;
            }

            // 进程存在但连接失败，判断是网络问题还是认证过期
            let current_failures = {
                let info = self.info.read().await;
                info.consecutive_failures
            };

            if current_failures > 5 {
                self.update_info(|info| {
                    info.state = ConnectionState::AuthExpired;
                    info.consecutive_failures += 1;
                    info.error_message = Some("认证信息可能已过期，需要重新获取".to_string());
                })
                .await;
                ConnectionState::AuthExpired
            } else {
                self.update_info(|info| {
                    info.state = ConnectionState::Unstable;
                    info.consecutive_failures += 1;
                    info.error_message = Some("网络连接不稳定或API服务未就绪".to_string());
                })
                .await;
                ConnectionState::Unstable
            }
        }
    }

    async fn has_lol_process(&self) -> bool {
        use sysinfo::{ProcessRefreshKind, RefreshKind, System};

        let mut system = System::new();
        system.refresh_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );

        let possible_names = [
            "LeagueClientUx.exe",
            "LeagueClient.exe",
            "LeagueOfLegends.exe",
        ];

        for (_pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy();
            if possible_names
                .iter()
                .any(|name| process_name.eq_ignore_ascii_case(name))
            {
                return true;
            }
        }
        false
    }

    async fn handle_state_change(&self, old_state: &ConnectionState, new_state: &ConnectionState) {
        let message = match (old_state, new_state) {
            (_, ConnectionState::Disconnected) => "客户端已断开连接",
            (ConnectionState::Disconnected, ConnectionState::ProcessFound) => {
                "检测到客户端启动，正在建立连接..."
            }
            (ConnectionState::Unstable, ConnectionState::Connected) => "连接已恢复稳定",
            (_, ConnectionState::Connected) => "客户端连接成功",
            (ConnectionState::Connected, ConnectionState::Unstable) => {
                "连接变得不稳定，正在重试..."
            }
            (_, ConnectionState::AuthExpired) => "认证信息已过期，正在重新获取...",
            _ => "连接状态发生变化",
        };

        log::info!(
            "[连接管理] 状态变化: {:?} -> {:?}, {}",
            old_state,
            new_state,
            message
        );

        // 发送状态变化事件到前端
        let info = self.info.read().await;
        let _ = self.app.emit("connection-state-changed", &*info);

        // 根据状态变化触发相应的处理
        match new_state {
            ConnectionState::Connected => {
                // 连接成功时触发数据刷新
                let _ = self.app.emit("connection-established", ());
            }
            ConnectionState::Disconnected => {
                // 断开连接时清理相关数据
                let _ = self.app.emit("connection-lost", ());
            }
            ConnectionState::AuthExpired => {
                // 认证过期时清除缓存
                crate::lcu::auth::invalidate_auth_info();
            }
            _ => {}
        }
    }

    async fn get_check_interval(&self, state: &ConnectionState) -> Duration {
        let info = self.info.read().await;

        match state {
            ConnectionState::Connected => Duration::from_secs(3), // 连接正常时也保持较快检查，以便及时发现断连
            ConnectionState::Disconnected => {
                if info.consecutive_failures > 20 {
                    Duration::from_secs(10) // 长期断开时降低频率，但不要太慢
                } else {
                    Duration::from_secs(2) // 初期断开时保持快速检查
                }
            }
            ConnectionState::ProcessFound => Duration::from_secs(1), // 进程存在时快速检查
            ConnectionState::Unstable => Duration::from_secs(2),     // 不稳定时快速检查
            ConnectionState::AuthExpired => Duration::from_secs(3),  // 认证过期时快速检查
        }
    }

    async fn update_info<F>(&self, updater: F)
    where
        F: FnOnce(&mut ConnectionInfo),
    {
        let mut info = self.info.write().await;
        updater(&mut *info);
    }

    /// 获取当前连接信息
    pub async fn get_connection_info(&self) -> ConnectionInfo {
        self.info.read().await.clone()
    }

    /// 强制重新检查连接
    pub async fn force_refresh(&self) {
        log::info!("[连接管理] 收到强制刷新请求");
        // 清除认证缓存
        crate::lcu::auth::invalidate_auth_info();

        // 重置连续失败计数
        self.update_info(|info| {
            info.consecutive_failures = 0;
            info.error_message = None;
        })
        .await;
    }
}

/// Tauri 命令：获取连接状态
#[tauri::command]
pub async fn get_connection_state(
    manager: tauri::State<'_, Arc<RwLock<ConnectionManager>>>,
) -> Result<ConnectionInfo, String> {
    let manager = manager.read().await;
    Ok(manager.get_connection_info().await)
}

/// Tauri 命令：强制刷新连接
#[tauri::command]
pub async fn force_refresh_connection(
    manager: tauri::State<'_, Arc<RwLock<ConnectionManager>>>,
) -> Result<(), String> {
    let manager = manager.read().await;
    manager.force_refresh().await;
    Ok(())
}

/// Tauri 命令：手动检查连接状态
#[tauri::command]
pub async fn check_connection_state_command(
    manager: tauri::State<'_, Arc<RwLock<ConnectionManager>>>,
) -> Result<ConnectionState, String> {
    let manager = manager.read().await;
    let state = manager.check_connection_state().await;
    Ok(state)
}
