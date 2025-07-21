use crate::lcu::lcu_driver::get_lcu_auth_info;
use crate::lcu::types::{ConnectionInfo, ConnectionState, LcuAuthInfo};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

#[derive(Clone)]
pub struct ConnectionManager {
    app_handle: AppHandle,
    connection_info: Arc<Mutex<ConnectionInfo>>,
}

impl ConnectionManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            connection_info: Arc::new(Mutex::new(ConnectionInfo {
                state: ConnectionState::Disconnected,
                lcu_info: None,
            })),
        }
    }

    pub fn start(&self) {
        let self_clone = self.clone();
        tokio::spawn(async move {
            self_clone.monitor_lcu_connection().await;
        });
    }

    async fn monitor_lcu_connection(&self) {
        loop {
            let is_running = Self::is_lcu_running_sync();
            let state = self.connection_info.lock().unwrap().state.clone();

            match state {
                ConnectionState::Connected => {
                    if !is_running {
                        self.update_state(ConnectionState::Disconnected, None);
                    }
                }
                ConnectionState::Disconnected => {
                    if is_running {
                        self.update_state(ConnectionState::Connecting, None);
                    }
                }
                ConnectionState::Connecting => {
                    if is_running {
                        match get_lcu_auth_info() {
                            Ok(lcu_info) => {
                                self.update_state(ConnectionState::Connected, Some(lcu_info));
                            }
                            Err(_) => {
                                // Keep trying
                            }
                        }
                    } else {
                        self.update_state(ConnectionState::Disconnected, None);
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    fn update_state(&self, new_state: ConnectionState, lcu_info: Option<LcuAuthInfo>) {
        let mut info = self.connection_info.lock().unwrap();
        info.state = new_state.clone();
        info.lcu_info = lcu_info;
        self.app_handle
            .emit_all("connection-info-update", info.clone())
            .unwrap();
    }

    pub fn is_lcu_running() -> bool {
        sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
        )
        .processes_by_name("LeagueClientUx.exe")
        .next()
        .is_some()
    }

    fn is_lcu_running_sync() -> bool {
        sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
        )
        .processes_by_name("LeagueClientUx.exe")
        .next()
        .is_some()
    }
}
