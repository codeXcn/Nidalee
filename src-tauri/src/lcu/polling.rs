use std::sync::Arc;
use std::time::Duration;
use futures_util::FutureExt;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use crate::lcu::{
    auth::ensure_valid_auth_info,
    champ_select::get_champ_select_session,
    gameflow::get_gameflow_phase,
    lobby::get_lobby_info,
    matchmaking::{get_match_info, get_matchmaking_state},
    summoner::get_current_summoner,
    types::{ChampSelectSession, LcuAuthInfo, MatchInfo, MatchmakingState, PollState, SummonerInfo}
};
use log::info;

// 防抖缓存
#[derive(Clone, Default)]
#[allow(dead_code)]
struct EmitCache {
    // is_lcu_running: Option<bool>,
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
    info!("开始启动轮询服务...");
    let state = Arc::new(RwLock::new(PollState::default()));
    let emit_cache = Arc::new(RwLock::new(EmitCache::default()));
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client");

    info!("启动游戏流程状态轮询");
    let app3 = app.clone();
    let state3 = state.clone();
    let emit_cache3 = emit_cache.clone();
    let client3 = client.clone();
    tokio::spawn(async move {
        if let Err(e) = std::panic::AssertUnwindSafe(poll_gameflow_phase(app3, state3, emit_cache3, client3))
            .catch_unwind()
            .await
        {
            log::error!("poll_gameflow_phase 协程 panic: {:?}", e);
        }
    });

    info!("启动游戏业务轮询");
    let app4 = app.clone();
    let state4 = state.clone();
    let emit_cache4 = emit_cache.clone();
    let client4 = client.clone();
    tokio::spawn(async move {
        if let Err(e) = std::panic::AssertUnwindSafe(poll_game_business(app4, state4, emit_cache4, client4))
            .catch_unwind()
            .await
        {
            log::error!("poll_game_business 协程 panic: {:?}", e);
        }
    });

    // 监听连接状态变化，自动刷新召唤师信息
    let app5 = app.clone();
    let state5 = state.clone();
    let client5 = client.clone();
    tokio::spawn(async move {
        handle_connection_events(app5, state5, client5).await;
    });

    // 启动时尝试初始获取召唤师信息
    let app6 = app.clone();
    let state6 = state.clone();
    let client6 = client.clone();
    tokio::spawn(async move {
        if let Err(e) = std::panic::AssertUnwindSafe(fetch_summoner_info_once(app6, state6, client6))
            .catch_unwind()
            .await
        {
            log::error!("fetch_summoner_info_once 协程 panic: {:?}", e);
        }
    });

    info!("所有轮询服务已启动");
}

// 智能获取召唤师信息，带重试和延迟机制
async fn fetch_summoner_info_once(app: AppHandle, state: Arc<RwLock<PollState>>, client: reqwest::Client) {
    // 等待认证信息可用
    let mut auth_ready = false;
    for _ in 0..30 { // 最多等待60秒
        if ensure_valid_auth_info().is_some() {
            auth_ready = true;
            break;
        }
        log::debug!("[召唤师信息] 等待认证信息可用...");
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    
    if !auth_ready {
        log::warn!("[召唤师信息] 认证信息长时间不可用，跳过初始拉取");
        return;
    }
    
    // 认证可用后，额外等待一段时间确保 LCU API 服务完全启动
    log::info!("[召唤师信息] 认证信息已可用，等待 LCU 服务完全启动...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // 带重试的获取召唤师信息
    match retry(|| get_current_summoner(&client), 5, 2000).await {
        Ok(summoner) => {
            log::info!("[召唤师信息] 初始获取成功: {}", summoner.display_name);
            let mut s = state.write().await;
            s.current_summoner = Some(summoner.clone());
            let _ = app.emit("summoner-change", &Some(summoner));
        }
        Err(e) => {
            log::warn!("[召唤师信息] 初始获取失败，将在后续轮询中重试: {}", e);
        }
    }
}

// gameflow_phase 轮询，监听游戏结束后自动拉取summoner info
async fn poll_gameflow_phase(app: AppHandle, state: Arc<RwLock<PollState>>, emit_cache: Arc<RwLock<EmitCache>>, client: reqwest::Client) {
    loop {
        if ensure_valid_auth_info().is_some() {
            match retry(|| get_gameflow_phase(&client), 2, 200).await {
                Ok(phase) => {
                    let mut s = state.write().await;
                    if s.gameflow_phase.as_ref() != Some(&phase) {
                        info!("游戏流程状态变化: {:?} -> {:?}", s.gameflow_phase, phase);
                        // 如果游戏刚结束（InProgress -> 非InProgress），触发数据刷新
                        if s.gameflow_phase.as_deref() == Some("InProgress")
                            && phase != "InProgress"
                        {
                            let app_cloned = app.clone();
                            let state_cloned = state.clone();
                            let client_cloned = client.clone();
                            tokio::spawn(async move {
                                // 游戏结束后等待一段时间再刷新，让数据完全同步
                                tokio::time::sleep(Duration::from_secs(10)).await;
                                
                                // 刷新召唤师信息
                                if let Err(e) = std::panic::AssertUnwindSafe(fetch_summoner_info_once(app_cloned.clone(), state_cloned, client_cloned))
                                    .catch_unwind()
                                    .await
                                {
                                    log::error!("fetch_summoner_info_once 协程 panic: {:?}", e);
                                }
                                
                                // 发送游戏结束事件，让前端刷新战绩
                                let _ = app_cloned.emit("game-finished", ());
                                log::info!("游戏结束，已发送数据刷新信号");
                            });
                        }
                        s.gameflow_phase = Some(phase.clone());
                    }
                }
                Err(e) => {
                    let mut s = state.write().await;
                    if s.gameflow_phase.is_some() {
                        info!("游戏流程状态获取失败: {}, 清除状态", e);
                        s.gameflow_phase = None;
                    }
                }
            }
        } else {
            info!("等待获取认证信息...");
            tokio::time::sleep(Duration::from_secs(5)).await;
            continue;
        }
        emit_if_change(&app, &state, &emit_cache).await;
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

// 其它业务，只在不在游戏中时运行
async fn poll_game_business(app: AppHandle, state: Arc<RwLock<PollState>>, emit_cache: Arc<RwLock<EmitCache>>, client: reqwest::Client) {
    loop {
        let phase = { state.read().await.gameflow_phase.clone() };
        if phase.as_deref() == Some("InProgress") {
            // 游戏中，暂停业务轮询
            tokio::time::sleep(Duration::from_secs(5)).await;
            continue;
        }
        if ensure_valid_auth_info().is_some() {
            // lobby
            match retry(|| get_lobby_info(&client), 2, 200).await {
                Ok(_lobby) => {
                    let mut s = state.write().await;
                    if !s.in_lobby {
                        info!("进入游戏大厅");
                        s.in_lobby = true;
                    }
                }
                Err(_) => {
                    let mut s = state.write().await;
                    if s.in_lobby {
                        info!("离开游戏大厅");
                        s.in_lobby = false;
                    }
                }
            }
            // matchmaking
            match retry(|| get_matchmaking_state(&client), 2, 200).await {
                Ok(matchmaking_state) => {
                    let mut s = state.write().await;
                    if s.matchmaking_state.as_ref() != Some(&matchmaking_state) {
                        info!("匹配状态更新: {:?}", matchmaking_state);

                        match matchmaking_state.search_state.as_str() {
                            "Found" => {
                                // 如果找到匹配，获取匹配信息
                                if let Ok(match_info) = get_match_info(&client).await {
                                    if s.match_info.as_ref() != Some(&match_info) {
                                        info!("找到匹配: {:?}", match_info);
                                        let match_info_clone = match_info.clone();
                                        s.match_info = Some(match_info);
                                        let _ = app.emit("match-info-changed", match_info_clone);
                                    }
                                }
                            }
                            "Searching" => {
                                info!("正在搜索匹配...");
                                s.match_info = None;
                            }
                            "Invalid" => {
                                info!("匹配状态无效，清除匹配信息");
                                s.match_info = None;
                            }
                            _ => {
                                info!("未知匹配状态: {}", matchmaking_state.search_state);
                            }
                        }

                        // 发送匹配状态变化事件
                        info!("匹配状态变化通知前端matchmaking-state-changed: {:?}", matchmaking_state);
                        let _ = app.emit("matchmaking-state-changed", matchmaking_state.clone());
                    }
                }
                Err(_) => {
                    let mut s = state.write().await;
                    if s.matchmaking_state.is_some() {
                        info!("匹配状态获取失败，清除状态");
                        s.matchmaking_state = None;
                    }
                }
            }
            // champ-select
            if phase.as_deref() == Some("ChampSelect") {
              // 持续轮询获取session
              match retry(|| get_champ_select_session(&client), 2, 200).await {
                  Ok(session) => {
                      let mut s = state.write().await;
                      if s.current_champ_select_session.as_ref() != Some(&session) {
                          info!("进入英雄选择阶段");
                          s.current_champ_select_session = Some(session.clone());
                          let _ = app.emit("champ-select-session-changed", session);
                      }
                  }
                  Err(e) => {
                      let mut s = state.write().await;
                      if s.current_champ_select_session.is_some() {
                          info!("英雄选择阶段获取失败: {}, 清除状态", e);
                          s.current_champ_select_session = None;
                          let _ = app.emit("champ-select-session-changed", Option::<ChampSelectSession>::None);
                      }
                  }
              }
          }
          // else if phase.as_deref() == Some("InProgress") && last_phase.as_deref() != Some("InProgress") {
          //   match retry(|| get_champ_select_session(&client), 2, 200).await {
          //     Ok(session) => {
          //         let mut s = state.write().await;
          //         if s.current_champ_select_session.as_ref() != Some(&session) {
          //             info!("进入英雄选择阶段");
          //             s.current_champ_select_session = Some(session.clone());
          //             let _ = app.emit("champ-select-session-changed", session);
          //         }
          //     }
          //     Err(e) => {
          //         let mut s = state.write().await;
          //         if s.current_champ_select_session.is_some() {
          //             info!("英雄选择阶段获取失败: {}, 清除状态", e);
          //             s.current_champ_select_session = None;
          //             let _ = app.emit("champ-select-session-changed", Option::<ChampSelectSession>::None);
          //         }
          //     }
          // }
          // }
          else {
                let mut s = state.write().await;
                if s.current_champ_select_session.is_some() {
                    info!("离开英雄选择阶段");
                    s.current_champ_select_session = None;
                    let _ = app.emit("champ-select-session-changed", Option::<ChampSelectSession>::None);
                }
            }
        }
        emit_if_change(&app, &state, &emit_cache).await;
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

// 事件防抖，只在变化时emit
async fn emit_if_change(app: &AppHandle, state: &Arc<RwLock<PollState>>, emit_cache: &Arc<RwLock<EmitCache>>) {
    let s = state.read().await;
    let mut c = emit_cache.write().await;
    
    // 检查召唤师信息变化
    if c.current_summoner.as_ref() != s.current_summoner.as_ref() {
        let _ = app.emit("summoner-change", &s.current_summoner);
        c.current_summoner = s.current_summoner.clone();
        info!("[事件防抖] 召唤师信息发生变化，已发送事件");
    }
    
    if c.auth_info.as_ref() != s.auth_info.as_ref() {
        let _ = app.emit("auth-info-change", &s.auth_info);
        c.auth_info = s.auth_info.clone();
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
        c.current_champ_select_session = s.current_champ_select_session.clone();
    }
}

// 通用重试工具
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

// 处理连接状态变化事件
async fn handle_connection_events(app: AppHandle, state: Arc<RwLock<PollState>>, client: reqwest::Client) {
    info!("[轮询服务] 开始监听连接事件");
    
    // 监听refresh-data事件（来自connection_manager）
    let app_clone1 = app.clone();
    let state_clone1 = state.clone();
    let client_clone1 = client.clone();
    
    // 使用定期检查的方式实现事件监听的效果
    // Tauri v2的事件监听在后端比较复杂，我们用另一种方式
    
    // 定期检查召唤师信息和连接状态
    let mut last_connection_check = std::time::Instant::now();
    let mut last_connection_valid = false;
    
    loop {
        let current_auth = ensure_valid_auth_info();
        let connection_valid = current_auth.is_some();
        
        // 检测连接状态变化
        if !last_connection_valid && connection_valid {
            // 从无连接变为有连接，刷新召唤师信息
            info!("[轮询服务] 检测到连接建立，开始刷新召唤师信息");
            
            // 等待一段时间确保连接稳定
            tokio::time::sleep(Duration::from_secs(2)).await;
            
            match retry(|| get_current_summoner(&client), 5, 2000).await {
                Ok(summoner) => {
                    info!("[轮询服务] 连接建立后召唤师信息获取成功: {}", summoner.display_name);
                    let mut s = state.write().await;
                    s.current_summoner = Some(summoner.clone());
                    let _ = app.emit("summoner-change", &Some(summoner));
                }
                Err(e) => {
                    info!("[轮询服务] 连接建立后召唤师信息获取失败: {}", e);
                }
            }
        } else if last_connection_valid && !connection_valid {
            // 从有连接变为无连接，清理状态并通知前端
            info!("[轮询服务] 检测到连接丢失，清理所有状态并通知前端");
            let mut s = state.write().await;
            s.current_summoner = None;
            s.gameflow_phase = None;
            s.in_lobby = false;
            s.matchmaking_state = None;
            s.match_info = None;
            s.current_champ_select_session = None;
            
            // 重要：发送空的召唤师信息到前端，表示连接已断开
            let _ = app.emit("summoner-change", &None::<SummonerInfo>);
            // 同时发送游戏阶段重置事件
            let _ = app.emit("gameflow-phase-change", &None::<String>);
        }
        
        last_connection_valid = connection_valid;
        
        // 每隔一段时间检查是否需要刷新召唤师信息
        if last_connection_check.elapsed() > Duration::from_secs(15) {
            if connection_valid {
                // 有认证信息时尝试获取召唤师信息
                let current_summoner = { state.read().await.current_summoner.clone() };
                
                if current_summoner.is_none() {
                    match retry(|| get_current_summoner(&client), 3, 1000).await {
                        Ok(summoner) => {
                            log::info!("[轮询服务] 定期检查获取召唤师信息成功: {}", summoner.display_name);
                            let mut s = state.write().await;
                            s.current_summoner = Some(summoner.clone());
                            let _ = app.emit("summoner-change", &Some(summoner));
                        }
                        Err(e) => {
                            log::debug!("[轮询服务] 定期检查获取召唤师信息失败: {}", e);
                        }
                    }
                }
            }
            last_connection_check = std::time::Instant::now();
        }
        
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}
