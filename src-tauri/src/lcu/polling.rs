use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use crate::lcu::{
    auth::{check_lcu_running, get_lcu_auth_info},
    gameflow::get_gameflow_phase,
    lobby::get_lobby_info,
    summoner::get_current_summoner,
    matchmaking::{get_matchmaking_state, get_match_info},
    champ_select::{get_champ_select_session, get_current_champion},
    types::{LcuAuthInfo, PollState, SummonerInfo, ChampSelectSession, MatchmakingState, MatchInfo},
};
use log::{info, warn};

// 防抖缓存
#[derive(Clone, Default)]
struct EmitCache {
    is_lcu_running: Option<bool>,
    auth_info: Option<LcuAuthInfo>,
    current_summoner: Option<SummonerInfo>,
    pub gameflow_phase: Option<String>,
    in_lobby: Option<bool>,
    matchmaking_state: Option<MatchmakingState>,
    match_info: Option<MatchInfo>,
    current_champ_select_session: Option<ChampSelectSession>,
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
            matchmaking_state: None,
            match_info: None,
            current_champ_select_session: None,
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
                        if s.gameflow_phase.as_ref() != Some(&phase) {
                            s.gameflow_phase = Some(phase.clone());
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
        // 6. 检查匹配状态（每1秒）
        if tick % 1 == 0 {
            let auth_info = { state.read().await.auth_info.clone() };
            if let Some(auth) = auth_info {
                println!("[Polling] Checking matchmaking state...");
                match retry(|| get_matchmaking_state(&client, &auth), 2, 200).await {
                    Ok(matchmaking_state) => {
                        let mut s = state.write().await;
                        if s.matchmaking_state.as_ref() != Some(&matchmaking_state) {
                            s.matchmaking_state = Some(matchmaking_state.clone());

                            // 输出匹配状态变化
                            if *s.matchmaking_state.as_ref().unwrap() != matchmaking_state {
                                println!("匹配状态更新: {:?}", matchmaking_state);
                            }

                            match matchmaking_state.search_state.as_str() {
                                "Found" => {
                                    // 如果找到匹配，获取匹配信息
                                    if let Ok(match_info) = get_match_info(&client, &auth).await {
                                        if s.match_info.as_ref() != Some(&match_info) {
                                            println!("找到匹配: {:?}", match_info);
                                            let match_info_clone = match_info.clone();
                                            s.match_info = Some(match_info);
                                            let _ = app.emit("match-info-changed", match_info_clone);
                                        }
                                    }
                                }
                                "Searching" => {
                                    println!("正在搜索匹配...");
                                    s.match_info = None;
                                }
                                "Invalid" => {
                                    println!("匹配状态无效，清除匹配信息");
                                    s.match_info = None;
                                }
                                _ => {
                                    println!("未知匹配状态: {}", matchmaking_state.search_state);
                                }
                            }

                            // 发送匹配状态变化事件
                            println!("匹配状态变化通知前端matchmaking-state-changed: {:?}", matchmaking_state);
                            let _ = app.emit("matchmaking-state-changed", matchmaking_state.clone());
                        }
                    }
                    Err(_) => {
                        let mut s = state.write().await;
                        if s.matchmaking_state.is_some() {
                            s.matchmaking_state = None;
                            s.match_info = None;
                        }
                    }
                }
            }
        }

        // 7. 检查当前选人 session（每1秒）
        if tick % 1 == 0 {
            println!("[Polling] 进入选人 session tick 分支");
            let auth_info = { state.read().await.auth_info.clone() };
            let gameflow_phase = { state.read().await.gameflow_phase.clone() };
            println!("[Polling] auth_info: {:?}, gameflow_phase: {:?}", auth_info.is_some(), gameflow_phase);

            // InProgress、WaitingForStats、EndOfGame 时停止轮询
            if let Some(phase) = gameflow_phase.as_deref() {
                match phase {
                    "InProgress" | "WaitingForStats" | "EndOfGame" => {
                        println!("[Polling] 当前为 {}，停止选人 session 轮询", phase);
                        return;
                    }
                    _ => {}
                }
            }

            // 只在英雄选择阶段检查
            if let Some(auth) = auth_info {
                if gameflow_phase.as_deref() == Some("ChampSelect") {
                    println!("[Polling] 进入 ChampSelect 阶段，准备获取选人 session");
                    match retry(|| get_champ_select_session(&client, &auth), 2, 200).await {
                        Ok(session) => {
                            let mut s = state.write().await;
                            if s.current_champ_select_session.as_ref() != Some(&session) {
                                println!("[Polling] 选人 Session 更新: {:?}", session);
                                s.current_champ_select_session = Some(session.clone());
                                let _ = app.emit("champ-select-session-changed", session);
                            } else {
                                println!("[Polling] 选人 Session 未变化");
                            }
                        }
                        Err(e) => {
                            if e.contains("403") {
                                println!("[Polling] 训练模式/特殊房间 403，fallback 到 current_champion");
                                match retry(|| get_current_champion(&client, &auth), 2, 200).await {
                                    Ok(champion) => {
                                        println!("[Polling] fallback 当前高亮英雄: {:?}", champion);
                                        let _ = app.emit("current-champion-fallback", champion);
                                    }
                                    Err(e2) => {
                                        println!("[Polling] fallback 拉取 current_champion 失败: {}", e2);
                                    }
                                }
                            } else {
                                println!("[Polling] 获取选人 Session 失败: {}", e);
                            }
                            let mut s = state.write().await;
                            if s.current_champ_select_session.is_some() {
                                s.current_champ_select_session = None;
                            }
                        }
                    }
                } else {
                    println!("[Polling] 当前不在 ChampSelect 阶段，清除 session 信息");
                    let mut s = state.write().await;
                    if s.current_champ_select_session.is_some() {
                        s.current_champ_select_session = None;
                    }
                }
            } else {
                println!("[Polling] 未获取到 auth_info，跳过选人 session 检查");
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
            if c.auth_info.as_ref() != s.auth_info.as_ref() {
                let _ = app.emit("auth-info-change", &s.auth_info);
                c.auth_info = s.auth_info.clone();
            }
            if c.current_summoner.as_ref() != s.current_summoner.as_ref() {
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
            if c.match_info.as_ref() != s.match_info.as_ref() {
                let _ = app.emit("match-info-change", &s.match_info);
                c.match_info = s.match_info.clone();
            }
            if c.current_champ_select_session.as_ref() != s.current_champ_select_session.as_ref() {
                let _ = app.emit("current-champ-select-session-change", &s.current_champ_select_session);
                c.current_champ_select_session = s.current_champ_select_session.clone();
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
