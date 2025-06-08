// 轮询模块 + App 事件通知
use std::{thread, time::Duration};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::lcu::{
    auth::{check_lcu_running, get_lcu_auth_info},
    gameflow::get_gameflow_phase,
    lobby::get_lobby_info,
    summoner::get_current_summoner,
    types::PollState,
};

pub struct PollingManager {
    app: AppHandle,
    state: Arc<Mutex<PollState>>,
    client: reqwest::Client,
}

impl PollingManager {
    pub fn new(app: AppHandle) -> Self {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        let state = Arc::new(Mutex::new(PollState {
            is_lcu_running: false,
            auth_info: None,
            current_summoner: None,
            gameflow_phase: None,
            in_lobby: false,
        }));

        Self { app, state, client }
    }

    pub async fn start_polling(self: Arc<Self>) {
        let lcu_poll = self.clone();
        let auth_poll = self.clone();
        let summoner_poll = self.clone();
        let gameflow_poll = self.clone();
        let lobby_poll = self.clone();

        // LCU 进程检查 (1-2秒)
        tokio::spawn(async move {
            loop {
                let is_running = check_lcu_running();
                let mut state = lcu_poll.state.lock().await;
                if state.is_lcu_running != is_running {
                    state.is_lcu_running = is_running;
                    let _ = lcu_poll.app.emit("lcu-status-change", is_running);
                }
                drop(state);
                sleep(Duration::from_secs(2)).await;
            }
        });

        // Auth 参数检查 (3-5秒)
        tokio::spawn(async move {
            loop {
                let mut state = auth_poll.state.lock().await;
                if state.is_lcu_running {
                    match get_lcu_auth_info() {
                        Ok(new_auth) => {
                            if state.auth_info.as_ref() != Some(&new_auth) {
                                state.auth_info = Some(new_auth.clone());
                                let _ = auth_poll.app.emit("auth-info-change", &new_auth);
                            }
                        }
                        Err(_) => {
                            if state.auth_info.is_some() {
                                state.auth_info = None;
                                let _ = auth_poll.app.emit("auth-info-change", ());
                            }
                        }
                    }
                }
                drop(state);
                sleep(Duration::from_secs(5)).await;
            }
        });

        // 召唤师信息检查 (10秒)
        tokio::spawn(async move {
            loop {
                let mut state = summoner_poll.state.lock().await;
                if let Some(auth_info) = &state.auth_info {
                    match get_current_summoner(&summoner_poll.client, auth_info).await {
                        Ok(summoner) => {
                            if state.current_summoner.as_ref() != Some(&summoner) {
                                println!("[polling] summoner-change: {:?}", &summoner);
                                state.current_summoner = Some(summoner.clone());
                                let emit_result = summoner_poll.app.emit("summoner-change", &summoner);
                                println!("[polling] emit summoner-change result: {:?}", emit_result);
                            }
                        }
                        Err(_) => {
                            if state.current_summoner.is_some() {
                                println!("[polling] summoner-change: None");
                                state.current_summoner = None;
                                let emit_result = summoner_poll.app.emit("summoner-change", ());
                                println!("[polling] emit summoner-change result: {:?}", emit_result);
                            }
                        }
                    }
                }
                drop(state);
                sleep(Duration::from_secs(10)).await;
            }
        });

        // 游戏阶段检查 (1秒)
        tokio::spawn(async move {
            loop {
                let mut state = gameflow_poll.state.lock().await;
                if let Some(auth_info) = &state.auth_info {
                    match get_gameflow_phase(&gameflow_poll.client, auth_info).await {
                        Ok(phase) => {
                            if state.gameflow_phase.as_ref() != Some(&phase.phase) {
                                state.gameflow_phase = Some(phase.phase.clone());
                                let _ = gameflow_poll.app.emit("gameflow-phase-change", &phase);
                            }
                        }
                        Err(_) => {
                            if state.gameflow_phase.is_some() {
                                state.gameflow_phase = None;
                                let _ = gameflow_poll.app.emit("gameflow-phase-change", ());
                            }
                        }
                    }
                }
                drop(state);
                sleep(Duration::from_secs(1)).await;
            }
        });

        // 房间状态检查 (3秒)
        tokio::spawn(async move {
            loop {
                let mut state = lobby_poll.state.lock().await;
                if let Some(auth_info) = &state.auth_info {
                    match get_lobby_info(&lobby_poll.client, auth_info).await {
                        Ok(lobby) => {
                            if !state.in_lobby {
                                state.in_lobby = true;
                                let _ = lobby_poll.app.emit("lobby-change", &lobby);
                            }
                        }
                        Err(_) => {
                            if state.in_lobby {
                                state.in_lobby = false;
                                let _ = lobby_poll.app.emit("lobby-change", ());
                            }
                        }
                    }
                }
                drop(state);
                sleep(Duration::from_secs(3)).await;
            }
        });
    }
}
