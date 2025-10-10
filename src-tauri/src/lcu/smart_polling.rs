// 智能轮询管理器 - 统一轮询策略
use crate::lcu::{
    auth::service::ensure_valid_auth_info,
    champ_select::service::get_champ_select_session,
    gameflow::service::get_gameflow_phase,
    lobby::service::get_lobby_info,
    matchmaking::service::{get_match_info, get_matchmaking_state},
    summoner::service::get_current_summoner,
    liveclient::service::{get_live_player_list, is_liveclient_available},
    types::{ChampSelectSession, LcuAuthInfo, MatchInfo, MatchmakingState, SummonerInfo},
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct SmartPollingManager {
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
    last_phase_change: Option<Instant>,

    // 大厅和匹配状态
    in_lobby: bool,
    matchmaking_state: Option<MatchmakingState>,
    match_info: Option<MatchInfo>,

    // 选人阶段
    champ_select_session: Option<ChampSelectSession>,

    // 轮询状态
    last_data_change: Option<Instant>,
    fast_polling_until: Option<Instant>,
    consecutive_errors: u32,
}

impl SmartPollingManager {
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
        let mut is_running = self.is_running.write().await;
        if *is_running {
            log::info!("[智能轮询] 已在运行中");
            return;
        }
        *is_running = true;
        drop(is_running);

        log::info!("[智能轮询] 启动智能轮询管理器");

        // 启动主循环
        self.run_main_loop().await;
    }

    pub async fn stop(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        drop(is_running);

        log::info!("[智能轮询] 停止智能轮询管理器");
    }

    async fn run_main_loop(&self) {
        let mut connection_check_counter = 0;
        let mut last_connection_check = Instant::now();

        loop {
            // 检查是否应该停止
            if !*self.is_running.read().await {
                break;
            }

            // 连接状态检查（每5秒检查一次）
            if last_connection_check.elapsed() > Duration::from_secs(5) {
                self.check_connection_status().await;
                last_connection_check = Instant::now();
                connection_check_counter = 0;
            }

            // 只有在连接时才进行业务轮询
            let is_connected = self.state.read().await.is_connected;
            if is_connected {
                self.poll_game_data().await;
            } else {
                // 未连接时使用较长间隔
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            }

            // 计算自适应轮询间隔
            let sleep_duration = self.get_adaptive_interval().await;
            tokio::time::sleep(sleep_duration).await;
        }

        log::info!("[智能轮询] 主循环已退出");
    }

    async fn check_connection_status(&self) {
        let auth_info = ensure_valid_auth_info();
        let is_connected = auth_info.is_some();

        let mut state = self.state.write().await;

        // 连接状态变化处理
        if state.is_connected != is_connected {
            if is_connected {
                log::info!("[智能轮询] 检测到连接建立");
                state.is_connected = true;
                state.auth_info = auth_info;
                state.consecutive_errors = 0;

                // 连接建立时立即获取召唤师信息
                drop(state);
                self.fetch_summoner_info().await;
            } else {
                log::info!("[智能轮询] 检测到连接断开，清理所有状态");
                state.is_connected = false;
                state.auth_info = None;
                state.consecutive_errors = 0;

                // 清理所有游戏状态
                self.clear_all_state(&mut state).await;
            }
        }
    }

    async fn poll_game_data(&self) {
        let current_phase = self.state.read().await.gameflow_phase.clone();
        log::debug!("[智能轮询] 当前游戏阶段: {:?}", current_phase);

        // 并发获取基础数据
        let gameflow_result = self.fetch_gameflow_phase().await;
        let summoner_result = self.check_summoner_info().await;

        // 根据游戏阶段决定是否获取其他数据
        match current_phase.as_deref() {
            Some("ChampSelect") => {
                log::debug!("[智能轮询] 检测到选人阶段，开始获取选人数据");
                self.fetch_champ_select_session().await;
            }
            Some("InProgress") => {
                log::debug!("[智能轮询] 游戏进行中，获取 LiveClient 数据");
                self.fetch_liveclient_data().await;
            }
            _ => {
                // 其他状态，获取大厅和匹配数据
                log::debug!(
                    "[智能轮询] 其他状态({:?})，获取大厅和匹配数据",
                    current_phase
                );
                let lobby_result = self.fetch_lobby_info().await;
                let matchmaking_result = self.fetch_matchmaking_state().await;
            }
        }
    }

    async fn fetch_gameflow_phase(&self) -> Option<String> {
        match get_gameflow_phase(&self.client).await {
            Ok(phase) => {
                let mut state = self.state.write().await;
                let phase_changed = state.gameflow_phase.as_ref() != Some(&phase);

                if phase_changed {
                    state.gameflow_phase = Some(phase.clone());
                    state.last_phase_change = Some(Instant::now());
                    state.last_data_change = Some(Instant::now());

                    // 阶段变化时启用快速轮询
                    state.fast_polling_until = Some(Instant::now() + Duration::from_secs(10));

                    drop(state);

                    // 发送阶段变化事件
                    if let Err(e) = self.app.emit("gameflow-phase-changed", &phase) {
                        log::error!("[智能轮询] 发送阶段变化事件失败: {}", e);
                    }

                    log::info!("[智能轮询] 游戏阶段变化: {}", phase);
                }
                Some(phase)
            }
            Err(e) => {
                log::warn!("[智能轮询] 获取游戏阶段失败: {}", e);
                self.increment_error_count().await;
                None
            }
        }
    }

    async fn fetch_summoner_info(&self) -> Option<SummonerInfo> {
        match get_current_summoner(&self.client).await {
            Ok(summoner) => {
                let mut state = self.state.write().await;
                state.current_summoner = Some(summoner.clone());
                state.last_data_change = Some(Instant::now());
                drop(state);

                // 发送召唤师信息事件
                if let Err(e) = self.app.emit("summoner-info-updated", &summoner) {
                    log::error!("[智能轮询] 发送召唤师信息事件失败: {}", e);
                }

                log::debug!("[智能轮询] 获取召唤师信息成功");
                Some(summoner)
            }
            Err(e) => {
                log::warn!("[智能轮询] 获取召唤师信息失败: {}", e);
                self.increment_error_count().await;
                None
            }
        }
    }

    async fn check_summoner_info(&self) -> Option<SummonerInfo> {
        let now = Instant::now();
        let should_check = {
            let state = self.state.read().await;
            state.current_summoner.is_none()
        };

        if should_check {
            self.fetch_summoner_info().await
        } else {
            None
        }
    }

    async fn fetch_champ_select_session(&self) -> Option<ChampSelectSession> {
        match get_champ_select_session(&self.client).await {
            Ok(session) => {
                let mut state = self.state.write().await;
                let session_changed = state.champ_select_session.as_ref() != Some(&session);

                if session_changed {
                    state.champ_select_session = Some(session.clone());
                    state.last_data_change = Some(Instant::now());

                    // 数据变化时启用快速轮询
                    state.fast_polling_until = Some(Instant::now() + Duration::from_secs(5));

                    drop(state);

                    // 发送选人数据事件
                    if let Err(e) = self.app.emit("champ-select-session-updated", &session) {
                        log::error!("[智能轮询] 发送选人数据事件失败: {}", e);
                    }

                    log::debug!("[智能轮询] 选人数据更新");
                }
                Some(session)
            }
            Err(e) => {
                log::warn!("[智能轮询] 获取选人数据失败: {}", e);
                self.increment_error_count().await;
                None
            }
        }
    }

    async fn fetch_liveclient_data(&self) {
        log::debug!("[智能轮询] 开始获取 LiveClient 数据");

        // 检查 LiveClient 是否可用
        if !is_liveclient_available().await {
            log::debug!("[智能轮询] LiveClient 服务不可用");
            return;
        }

        log::debug!("[智能轮询] LiveClient 服务可用，开始获取数据");

        // 获取玩家列表
        match get_live_player_list().await {
            Ok(players) => {
                let mut state = self.state.write().await;
                state.last_data_change = Some(Instant::now());

                // 数据变化时启用快速轮询
                state.fast_polling_until = Some(Instant::now() + Duration::from_secs(5));
                drop(state);

                log::info!("[智能轮询] 获取到 {} 个玩家", players.len());
                log::debug!("[智能轮询] 玩家数据: {:?}", players);

                // 发送玩家列表事件
                if let Err(e) = self.app.emit("liveclient-player-list", &players) {
                    log::error!("[智能轮询] 发送玩家列表事件失败: {}", e);
                } else {
                    log::info!("[智能轮询] 成功发送玩家列表事件到前端");
                }
            }
            Err(e) => {
                log::warn!("[智能轮询] 获取玩家列表失败: {}", e);
                self.increment_error_count().await;
            }
        }
    }

    async fn fetch_lobby_info(&self) -> Option<bool> {
        match get_lobby_info(&self.client).await {
            Ok(lobby_info) => {
                let in_lobby = lobby_info.is_some();
                let mut state = self.state.write().await;
                let lobby_changed = state.in_lobby != in_lobby;

                if lobby_changed {
                    state.in_lobby = in_lobby;
                    state.last_data_change = Some(Instant::now());
                    drop(state);

                    // 发送大厅状态事件
                    if let Err(e) = self.app.emit("lobby-state-changed", &in_lobby) {
                        log::error!("[智能轮询] 发送大厅状态事件失败: {}", e);
                    }

                    log::debug!("[智能轮询] 大厅状态变化: {}", in_lobby);
                }
                Some(in_lobby)
            }
            Err(e) => {
                log::warn!("[智能轮询] 获取大厅信息失败: {}", e);
                self.increment_error_count().await;
                None
            }
        }
    }

    async fn fetch_matchmaking_state(&self) -> Option<MatchmakingState> {
        match get_matchmaking_state(&self.client).await {
            Ok(matchmaking_state) => {
                let mut state = self.state.write().await;
                let state_changed = state.matchmaking_state.as_ref() != Some(&matchmaking_state);

                if state_changed {
                    state.matchmaking_state = Some(matchmaking_state.clone());
                    state.last_data_change = Some(Instant::now());
                    drop(state);

                    // 发送匹配状态事件
                    if let Err(e) = self.app.emit("matchmaking-state-changed", &matchmaking_state) {
                        log::error!("[智能轮询] 发送匹配状态事件失败: {}", e);
                    }

                    log::debug!("[智能轮询] 匹配状态更新");
                }
                Some(matchmaking_state)
            }
            Err(e) => {
                log::warn!("[智能轮询] 获取匹配状态失败: {}", e);
                self.increment_error_count().await;
                None
            }
        }
    }

    async fn get_adaptive_interval(&self) -> Duration {
        let state = self.state.read().await;

        // 如果正在快速轮询
        if let Some(fast_polling_until) = state.fast_polling_until {
            if fast_polling_until > Instant::now() {
                return Duration::from_millis(1000); // 1秒快速轮询
            }
        }

        // 根据阶段和错误次数调整间隔
        let base_interval = match state.gameflow_phase.as_deref() {
            Some("ChampSelect") => 2000,      // 选人阶段：2秒
            Some("InProgress") => 3000,       // 游戏中：3秒
            Some("Lobby") | Some("Matchmaking") => 5000,  // 大厅/匹配：5秒
            _ => 10000,                       // 其他：10秒
        };

        // 根据连续错误次数增加间隔
        let error_multiplier = if state.consecutive_errors > 5 {
            3.0
        } else if state.consecutive_errors > 2 {
            2.0
        } else {
            1.0
        };

        Duration::from_millis((base_interval as f64 * error_multiplier) as u64)
    }

    async fn increment_error_count(&self) {
        let mut state = self.state.write().await;
        state.consecutive_errors += 1;
    }

    async fn clear_all_state(&self, state: &mut PollingState) {
        state.current_summoner = None;
        state.gameflow_phase = None;
        state.in_lobby = false;
        state.matchmaking_state = None;
        state.match_info = None;
        state.champ_select_session = None;
        state.last_data_change = None;
        state.fast_polling_until = None;
        state.consecutive_errors = 0;

        // 发送清理事件
        if let Err(e) = self.app.emit("game-state-cleared", ()) {
            log::error!("[智能轮询] 发送清理事件失败: {}", e);
        }

        log::info!("[智能轮询] 所有状态已清理并通知前端");
    }
}

