// 统一轮询管理器 - 优化版本
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use crate::lcu::{
    auth::ensure_valid_auth_info,
    champ_select::get_champ_select_session,
    gameflow::get_gameflow_phase,
    lobby::get_lobby_info,
    matchmaking::{get_match_info, get_matchmaking_state},
    summoner::get_current_summoner,
    types::{ChampSelectSession, LcuAuthInfo, MatchInfo, MatchmakingState, SummonerInfo}
};

#[derive(Clone)]
pub struct UnifiedPollingManager {
    app: AppHandle,
    client: reqwest::Client,
    state: Arc<RwLock<PollingState>>,
    is_running: Arc<RwLock<bool>>,
}

#[derive(Default, Clone)]
struct PollingState {
    // 连接状态
    is_connected: bool,
    auth_info: Option<LcuAuthInfo>,
    
    // 游戏状态
    current_summoner: Option<SummonerInfo>,
    gameflow_phase: Option<String>,
    
    // 大厅和匹配状态
    in_lobby: bool,
    matchmaking_state: Option<MatchmakingState>,
    match_info: Option<MatchInfo>,
    
    // 选人阶段
    champ_select_session: Option<ChampSelectSession>,
}

impl UnifiedPollingManager {
    pub fn new(app: AppHandle) -> Self {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(8))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            app,
            client,
            state: Arc::new(RwLock::new(PollingState::default())),
            is_running: Arc::new(RwLock::new(false)),
        }
    }
    
    pub async fn start(&self) {
        let mut running = self.is_running.write().await;
        if *running {
            log::warn!("[统一轮询] 轮询已在运行，跳过启动");
            return;
        }
        *running = true;
        drop(running);
        
        log::info!("[统一轮询] 启动智能轮询管理器");
        
        let manager = self.clone();
        tokio::spawn(async move {
            manager.run_main_loop().await;
        });
    }
    
    pub async fn stop(&self) {
        let mut running = self.is_running.write().await;
        *running = false;
        log::info!("[统一轮询] 停止轮询管理器");
    }
    
    async fn run_main_loop(&self) {
        let mut connection_check_counter = 0;
        
        loop {
            // 检查是否应该停止
            if !*self.is_running.read().await {
                break;
            }
            
            // 每3次循环检查一次连接状态（避免过于频繁）
            connection_check_counter += 1;
            if connection_check_counter >= 3 {
                self.check_connection_status().await;
                connection_check_counter = 0;
            }
            
            // 只有在连接时才进行业务轮询
            let is_connected = self.state.read().await.is_connected;
            if is_connected {
                self.poll_game_data().await;
            } else {
                // 未连接时减少轮询频率
                log::debug!("[统一轮询] 未连接，跳过业务轮询");
                tokio::time::sleep(Duration::from_secs(8)).await;
                continue;
            }
            
            // 动态调整轮询间隔
            let sleep_duration = self.get_adaptive_interval().await;
            tokio::time::sleep(sleep_duration).await;
        }
        
        log::info!("[统一轮询] 主循环已退出");
    }
    
    async fn check_connection_status(&self) {
        let auth_info = ensure_valid_auth_info();
        let is_connected = auth_info.is_some();
        
        let mut state = self.state.write().await;
        
        // 连接状态变化处理
        if state.is_connected != is_connected {
            if is_connected {
                log::info!("[统一轮询] 检测到连接建立");
                state.is_connected = true;
                state.auth_info = auth_info;
                
                // 连接建立时立即获取召唤师信息
                drop(state);
                self.fetch_summoner_info().await;
            } else {
                log::info!("[统一轮询] 检测到连接断开，清理所有状态");
                state.is_connected = false;
                state.auth_info = None;
                
                // 清理所有游戏状态
                self.clear_all_state(&mut state).await;
            }
        }
    }
    
    async fn poll_game_data(&self) {
        // 并发获取多个数据，提高效率
        let _gameflow_result = self.fetch_gameflow_phase().await;
        let _summoner_result = self.check_summoner_info().await;
        
        // 根据游戏阶段决定是否获取其他数据
        let current_phase = self.state.read().await.gameflow_phase.clone();
        
        match current_phase.as_deref() {
            Some("ChampSelect") => {
                // 选人阶段，获取选人数据
                self.fetch_champ_select_session().await;
            }
            Some("InProgress") => {
                // 游戏中，暂停其他轮询
                log::debug!("[统一轮询] 游戏进行中，跳过大厅相关轮询");
            }
            _ => {
                // 其他状态，获取大厅和匹配数据
                let _lobby_result = self.fetch_lobby_info().await;
                let _matchmaking_result = self.fetch_matchmaking_state().await;
            }
        }
    }
    
    async fn fetch_summoner_info(&self) {
        match retry(|| get_current_summoner(&self.client), 3, 1500).await {
            Ok(summoner) => {
                let mut state = self.state.write().await;
                if state.current_summoner.as_ref() != Some(&summoner) {
                    log::info!("[统一轮询] 召唤师信息更新: {}", summoner.display_name);
                    state.current_summoner = Some(summoner.clone());
                    let _ = self.app.emit("summoner-change", &Some(summoner));
                }
            }
            Err(e) => {
                log::debug!("[统一轮询] 获取召唤师信息失败: {}", e);
            }
        }
    }
    
    async fn check_summoner_info(&self) {
        // 只有当召唤师信息为空时才重新获取
        let needs_fetch = self.state.read().await.current_summoner.is_none();
        if needs_fetch {
            self.fetch_summoner_info().await;
        }
    }
    
    async fn fetch_gameflow_phase(&self) {
        match retry(|| get_gameflow_phase(&self.client), 2, 500).await {
            Ok(phase) => {
                let mut state = self.state.write().await;
                if state.gameflow_phase.as_ref() != Some(&phase) {
                    log::info!("[统一轮询] 游戏阶段变化: {:?} -> {}", state.gameflow_phase, phase);
                    
                    // 检测游戏结束
                    let was_in_progress = state.gameflow_phase.as_deref() == Some("InProgress");
                    let now_finished = phase != "InProgress";
                    
                    state.gameflow_phase = Some(phase.clone());
                    let _ = self.app.emit("gameflow-phase-change", &Some(phase));
                    
                    // 游戏结束时的特殊处理
                    if was_in_progress && now_finished {
                        drop(state);
                        self.handle_game_finished().await;
                    }
                }
            }
            Err(_) => {
                let mut state = self.state.write().await;
                if state.gameflow_phase.is_some() {
                    log::debug!("[统一轮询] 游戏阶段获取失败，清除状态");
                    state.gameflow_phase = None;
                    let _ = self.app.emit("gameflow-phase-change", &None::<String>);
                }
            }
        }
    }
    
    async fn fetch_lobby_info(&self) {
        match retry(|| get_lobby_info(&self.client), 2, 500).await {
            Ok(_) => {
                let mut state = self.state.write().await;
                if !state.in_lobby {
                    log::debug!("[统一轮询] 进入大厅");
                    state.in_lobby = true;
                    let _ = self.app.emit("lobby-change", true);
                }
            }
            Err(_) => {
                let mut state = self.state.write().await;
                if state.in_lobby {
                    log::debug!("[统一轮询] 离开大厅");
                    state.in_lobby = false;
                    let _ = self.app.emit("lobby-change", false);
                }
            }
        }
    }
    
    async fn fetch_matchmaking_state(&self) {
        match retry(|| get_matchmaking_state(&self.client), 2, 500).await {
            Ok(matchmaking_state) => {
                let mut state = self.state.write().await;
                if state.matchmaking_state.as_ref() != Some(&matchmaking_state) {
                    log::info!("[统一轮询] 匹配状态更新: {:?}", matchmaking_state.search_state);
                    
                    // 检查是否找到匹配（在移动值之前）
                    let found_match = matchmaking_state.search_state == "Found";
                    
                    state.matchmaking_state = Some(matchmaking_state.clone());
                    let _ = self.app.emit("matchmaking-state-changed", matchmaking_state);
                    
                    // 找到匹配时获取匹配详情
                    if found_match {
                        drop(state);
                        self.fetch_match_info().await;
                    }
                }
            }
            Err(_) => {
                let mut state = self.state.write().await;
                if state.matchmaking_state.is_some() {
                    log::debug!("[统一轮询] 匹配状态获取失败，清除状态");
                    state.matchmaking_state = None;
                }
            }
        }
    }
    
    async fn fetch_match_info(&self) {
        match get_match_info(&self.client).await {
            Ok(match_info) => {
                let mut state = self.state.write().await;
                if state.match_info.as_ref() != Some(&match_info) {
                    log::info!("[统一轮询] 匹配信息更新");
                    state.match_info = Some(match_info.clone());
                    let _ = self.app.emit("match-info-changed", match_info);
                }
            }
            Err(e) => {
                log::debug!("[统一轮询] 获取匹配信息失败: {}", e);
            }
        }
    }
    
    async fn fetch_champ_select_session(&self) {
        match retry(|| get_champ_select_session(&self.client), 2, 500).await {
            Ok(session) => {
                let mut state = self.state.write().await;
                if state.champ_select_session.as_ref() != Some(&session) {
                    log::info!("[统一轮询] 选人阶段会话更新");
                    state.champ_select_session = Some(session.clone());
                    let _ = self.app.emit("champ-select-session-changed", session);
                }
            }
            Err(_) => {
                let mut state = self.state.write().await;
                if state.champ_select_session.is_some() {
                    log::debug!("[统一轮询] 选人阶段会话获取失败，清除状态");
                    state.champ_select_session = None;
                    let _ = self.app.emit("champ-select-session-changed", Option::<ChampSelectSession>::None);
                }
            }
        }
    }
    
    async fn handle_game_finished(&self) {
        log::info!("[统一轮询] 游戏结束，开始后处理");
        
        // 等待一段时间让游戏数据同步
        tokio::time::sleep(Duration::from_secs(8)).await;
        
        // 刷新召唤师信息（等级、经验可能变化）
        self.fetch_summoner_info().await;
        
        // 发送游戏结束事件让前端刷新战绩
        let _ = self.app.emit("game-finished", ());
        log::info!("[统一轮询] 游戏结束处理完成");
    }
    
    async fn clear_all_state(&self, state: &mut PollingState) {
        // 清理状态
        state.current_summoner = None;
        state.gameflow_phase = None;
        state.in_lobby = false;
        state.matchmaking_state = None;
        state.match_info = None;
        state.champ_select_session = None;
        
        // 发送清理事件
        let _ = self.app.emit("summoner-change", &None::<SummonerInfo>);
        let _ = self.app.emit("gameflow-phase-change", &None::<String>);
        let _ = self.app.emit("lobby-change", false);
        
        log::info!("[统一轮询] 所有状态已清理并通知前端");
    }
    
    async fn get_adaptive_interval(&self) -> Duration {
        let state = self.state.read().await;
        
        match state.gameflow_phase.as_deref() {
            Some("ChampSelect") => Duration::from_secs(2), // 选人阶段更频繁
            Some("InProgress") => Duration::from_secs(10), // 游戏中减少轮询
            Some("Found") => Duration::from_secs(1),       // 找到匹配时更频繁
            _ => Duration::from_secs(4),                   // 默认间隔
        }
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
                if retries > 1 {
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                }
            }
        }
    }
    Err(last_err.unwrap())
}
