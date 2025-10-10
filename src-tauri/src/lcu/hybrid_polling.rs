// 混合轮询管理器 - WebSocket + 轮询兜底策略
use crate::lcu::{
    auth::service::ensure_valid_auth_info,
    summoner::service::get_current_summoner,
    liveclient::service::{get_live_player_list, is_liveclient_available},
    types::{LcuAuthInfo, SummonerInfo},
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct HybridPollingManager {
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

    // 轮询状态
    last_summoner_check: Option<Instant>,
    last_liveclient_check: Option<Instant>,
    consecutive_errors: u32,

    // WebSocket 状态
    websocket_healthy: bool,
    last_websocket_heartbeat: Option<Instant>,

    // LiveClient 数据状态
    has_liveclient_data: bool,
}

impl HybridPollingManager {
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
            log::info!("[混合轮询] 已在运行中");
            return;
        }
        *is_running = true;
        drop(is_running);

        log::info!("[混合轮询] 启动混合轮询管理器");

        // 启动主循环
        self.run_main_loop().await;
    }

    pub async fn stop(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        drop(is_running);

        log::info!("[混合轮询] 停止混合轮询管理器");
    }

    async fn run_main_loop(&self) {
        let mut last_connection_check = Instant::now();

        loop {
            // 检查是否应该停止
            if !*self.is_running.read().await {
                break;
            }

            // 连接状态检查（每10秒检查一次）
            if last_connection_check.elapsed() > Duration::from_secs(10) {
                self.check_connection_status().await;
                last_connection_check = Instant::now();
            }

            // 只有在连接时才进行业务轮询
            let is_connected = self.state.read().await.is_connected;
            if is_connected {
                self.poll_essential_data().await;
            } else {
                // 未连接时使用较长间隔
                tokio::time::sleep(Duration::from_secs(15)).await;
                continue;
            }

            // 计算自适应轮询间隔
            let sleep_duration = self.get_adaptive_interval().await;
            tokio::time::sleep(sleep_duration).await;
        }

        log::info!("[混合轮询] 主循环已退出");
    }

    async fn check_connection_status(&self) {
        let auth_info = ensure_valid_auth_info();
        let is_connected = auth_info.is_some();

        let mut state = self.state.write().await;

        // 连接状态变化处理
        if state.is_connected != is_connected {
            if is_connected {
                log::info!("[混合轮询] 检测到连接建立");
                state.is_connected = true;
                state.auth_info = auth_info;
                state.consecutive_errors = 0;

                // 连接建立时立即获取召唤师信息
                drop(state);
                self.fetch_summoner_info().await;
            } else {
                log::info!("[混合轮询] 检测到连接断开，清理所有状态");
                state.is_connected = false;
                state.auth_info = None;
                state.consecutive_errors = 0;

                // 清理所有游戏状态
                self.clear_all_state(&mut state).await;
            }
        }
    }

    async fn poll_essential_data(&self) {
        let state = self.state.read().await;
        let current_phase = state.gameflow_phase.clone();
        drop(state);

        // 根据游戏阶段决定轮询内容
        match current_phase.as_deref() {
            Some("InProgress") => {
                // 游戏中：只轮询 LiveClient 数据（WebSocket 不支持）
                self.poll_liveclient_data().await;
            }
            _ => {
                // 其他阶段：只轮询召唤师信息（定期检查）
                self.poll_summoner_info().await;
            }
        }
    }

    async fn poll_summoner_info(&self) {
        let now = Instant::now();
        let should_check = {
            let state = self.state.read().await;
            state.current_summoner.is_none()
                || state.last_summoner_check.map_or(true, |last| now.duration_since(last) > Duration::from_secs(60))
        };

        if should_check {
            self.fetch_summoner_info().await;
        }
    }

    async fn fetch_summoner_info(&self) -> Option<SummonerInfo> {
        match get_current_summoner(&self.client).await {
            Ok(summoner) => {
                let mut state = self.state.write().await;
                state.current_summoner = Some(summoner.clone());
                state.last_summoner_check = Some(Instant::now());
                state.consecutive_errors = 0;
                drop(state);

                // 发送召唤师信息事件
                if let Err(e) = self.app.emit("summoner-info-updated", &summoner) {
                    log::error!("[混合轮询] 发送召唤师信息事件失败: {}", e);
                }

                log::debug!("[混合轮询] 获取召唤师信息成功");
                Some(summoner)
            }
            Err(e) => {
                log::warn!("[混合轮询] 获取召唤师信息失败: {}", e);
                self.increment_error_count().await;
                None
            }
        }
    }

    async fn poll_liveclient_data(&self) {
        let now = Instant::now();
        let should_check = {
            let state = self.state.read().await;
            // 如果已经获取到数据，停止轮询
            if state.has_liveclient_data {
                log::debug!("[混合轮询] LiveClient 数据已获取，停止轮询");
                return;
            }

            state.last_liveclient_check.map_or(true, |last| now.duration_since(last) > Duration::from_secs(5))
        };

        if should_check {
            self.fetch_liveclient_data().await;
        }
    }

    async fn fetch_liveclient_data(&self) {
        log::debug!("[混合轮询] 开始获取 LiveClient 数据");

        // 检查 LiveClient 是否可用
        if !is_liveclient_available().await {
            log::debug!("[混合轮询] LiveClient 服务不可用");
            return;
        }

        // 获取玩家列表
        match get_live_player_list().await {
            Ok(players) => {
                let mut state = self.state.write().await;
                state.last_liveclient_check = Some(Instant::now());
                state.consecutive_errors = 0;
                state.has_liveclient_data = true; // 标记数据已获取
                drop(state);

                log::info!("[混合轮询] 获取到 {} 个玩家，停止后续轮询", players.len());

                // 发送玩家列表事件
                if let Err(e) = self.app.emit("liveclient-player-list", &players) {
                    log::error!("[混合轮询] 发送玩家列表事件失败: {}", e);
                } else {
                    log::debug!("[混合轮询] 成功发送玩家列表事件到前端");
                }
            }
            Err(e) => {
                log::warn!("[混合轮询] 获取玩家列表失败: {}", e);
                self.increment_error_count().await;
            }
        }
    }

    async fn get_adaptive_interval(&self) -> Duration {
        let state = self.state.read().await;

        // 根据阶段和 WebSocket 状态调整间隔
        let base_interval = match state.gameflow_phase.as_deref() {
            Some("InProgress") => {
                // 游戏中：LiveClient 数据需要较频繁检查
                if state.websocket_healthy {
                    10000  // WebSocket 健康时：10秒
                } else {
                    5000   // WebSocket 不健康时：5秒
                }
            }
            _ => {
                // 其他阶段：主要依赖 WebSocket，轮询间隔较长
                if state.websocket_healthy {
                    60000  // WebSocket 健康时：60秒
                } else {
                    30000  // WebSocket 不健康时：30秒
                }
            }
        };

        // 根据连续错误次数增加间隔
        let error_multiplier = if state.consecutive_errors > 3 {
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
        state.last_summoner_check = None;
        state.last_liveclient_check = None;
        state.consecutive_errors = 0;

        // 发送清理事件
        if let Err(e) = self.app.emit("game-state-cleared", ()) {
            log::error!("[混合轮询] 发送清理事件失败: {}", e);
        }

        log::info!("[混合轮询] 所有状态已清理并通知前端");
    }

    // WebSocket 状态更新方法
    pub async fn update_websocket_health(&self, healthy: bool) {
        let mut state = self.state.write().await;
        state.websocket_healthy = healthy;
        state.last_websocket_heartbeat = Some(Instant::now());
        log::debug!("[混合轮询] WebSocket 健康状态更新: {}", healthy);
    }

    pub async fn update_gameflow_phase(&self, phase: String) {
        let mut state = self.state.write().await;
        state.gameflow_phase = Some(phase);

        // 当游戏阶段变化时，重置 LiveClient 数据状态
        if phase != "InProgress" {
            state.has_liveclient_data = false;
            log::debug!("[混合轮询] 游戏阶段变化，重置 LiveClient 数据状态");
        }

        log::debug!("[混合轮询] 游戏阶段更新: {}", phase);
    }
}
