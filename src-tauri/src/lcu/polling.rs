use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle,Emitter};
use tokio::sync::RwLock;
use crate::lcu::{
    auth::{check_lcu_running, get_lcu_auth_info},
    gameflow::get_gameflow_phase,
    lobby::get_lobby_info,
    summoner::get_current_summoner,
    types::{LcuAuthInfo, PollState, SummonerInfo, GameflowPhase, LobbyInfo},
};
use log::{info, warn};

// 防抖缓存
#[derive(Clone, Default)]
struct EmitCache {
    is_lcu_running: Option<bool>,
    auth_info: Option<LcuAuthInfo>,
    current_summoner: Option<SummonerInfo>,
    gameflow_phase: Option<String>,
    in_lobby: Option<bool>,
}

// 默认值
impl Default for PollState {
    fn default() -> Self {
        PollState {
            is_lcu_running: false,
            auth_info: None,
            current_summoner: None,
            gameflow_phase: None,
            in_lobby: false,
        }
    }
}
pub async fn start_polling(app: AppHandle) {
    let state = Arc::new(RwLock::new(PollState::default()));
    let emit_cache = Arc::new(RwLock::new(EmitCache::default()));
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    tokio::spawn(polling_task(app, state, emit_cache, client));
}

async fn polling_task(
    app: AppHandle,
    state: Arc<RwLock<PollState>>,
    emit_cache: Arc<RwLock<EmitCache>>,
    client: reqwest::Client,
) {
    let mut tick: u64 = 0;
    loop {

        // 1. 检查LCU进程（每2秒）
        if tick % 2 == 0 {
            match retry(|| async { Ok(check_lcu_running()) as Result<bool, String> }, 2, 200).await {
                Ok(is_running) => {
                    let mut s = state.write().await;
                    if s.is_lcu_running != is_running {
                        s.is_lcu_running = is_running;
                    }
                }
                Err(_) => warn!("LCU进程检测多次失败"),
            }
        }

        // 2. 检查 auth_info（每5秒）
        if tick % 5 == 0 {
            let is_running = { state.read().await.is_lcu_running };
            if is_running {
                match retry(|| async { get_lcu_auth_info() }, 2, 200).await {
                    Ok(auth) => {
                        let mut s = state.write().await;
                        if s.auth_info.as_ref() != Some(&auth) {
                            s.auth_info = Some(auth);
                        }
                    }
                    Err(_) => {
                        let mut s = state.write().await;
                        if s.auth_info.is_some() {
                            s.auth_info = None;
                        }
                    }
                }
            }
        }

        // 3. 检查召唤师信息（每10秒）
        if tick % 10 == 0 {
            let auth_info = { state.read().await.auth_info.clone() };
            if let Some(auth) = auth_info {
                match retry(|| get_current_summoner(&client, &auth), 2, 200).await {
                    Ok(summoner) => {
                        let mut s = state.write().await;
                        if s.current_summoner.as_ref() != Some(&summoner) {
                            s.current_summoner = Some(summoner);
                        }
                    }
                    Err(_) => {
                        let mut s = state.write().await;
                        if s.current_summoner.is_some() {
                            s.current_summoner = None;
                        }
                    }
                }
            }
        }

        // 4. 检查游戏阶段（每1秒）
        if tick % 1 == 0 {
            let auth_info = { state.read().await.auth_info.clone() };
            if let Some(auth) = auth_info {
                match retry(|| get_gameflow_phase(&client, &auth), 2, 200).await {
                    Ok(phase) => {
                        let mut s = state.write().await;
                        if s.gameflow_phase.as_ref() != Some(&phase.phase) {
                            s.gameflow_phase = Some(phase.phase.clone());
                        }
                    }
                    Err(_) => {
                        let mut s = state.write().await;
                        if s.gameflow_phase.is_some() {
                            s.gameflow_phase = None;
                        }
                    }
                }
            }
        }

        // 5. 检查房间信息（每3秒）
        if tick % 3 == 0 {
            let auth_info = { state.read().await.auth_info.clone() };
            if let Some(auth) = auth_info {
                match retry(|| get_lobby_info(&client, &auth), 2, 200).await {
                    Ok(_lobby) => {
                        let mut s = state.write().await;
                        if !s.in_lobby {
                            s.in_lobby = true;
                        }
                    }
                    Err(_) => {
                        let mut s = state.write().await;
                        if s.in_lobby {
                            s.in_lobby = false;
                        }
                    }
                }
            }
        }

        // === 事件防抖，只有变化才 emit ===
        {
            let s = state.read().await;
            let mut c = emit_cache.write().await;

            if c.is_lcu_running != Some(s.is_lcu_running) {
                let _ = app.emit("lcu-status-change", s.is_lcu_running);
                c.is_lcu_running = Some(s.is_lcu_running);
            }
            if c.auth_info != s.auth_info {
                let _ = app.emit("auth-info-change", &s.auth_info);
                c.auth_info = s.auth_info.clone();
            }
            if c.current_summoner != s.current_summoner {
                let _ = app.emit("summoner-change", &s.current_summoner);
                c.current_summoner = s.current_summoner.clone();
            }
           if c.gameflow_phase.as_ref() != s.gameflow_phase.as_ref() {
                let _ = app.emit("gameflow-phase-change", &s.gameflow_phase);
                c.gameflow_phase = s.gameflow_phase.clone();
            }
            if c.in_lobby != Some(s.in_lobby) {
                let _ = app.emit("lobby-change", s.in_lobby);
                c.in_lobby = Some(s.in_lobby);
            }
        }

        tick = tick.wrapping_add(1);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

// 简单重试工具
async fn retry<F, Fut, T, E>(mut f: F, retries: u32, delay_ms: u64) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut last_err = None;
    for _ in 0..retries {
        match f().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                last_err = Some(e);
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }
    Err(last_err.unwrap())
}