import { ref, computed, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useErrorHandler } from './useErrorHandler'
import { useGameEvents } from './useEventBus'

/**
 * 统一轮询策略管理器
 * 职责：根据游戏阶段智能调整轮询频率，减少资源消耗
 */

export interface PollingConfig {
  // 基础轮询间隔
  baseInterval: number
  // 最大轮询间隔
  maxInterval: number
  // 连接检查间隔
  connectionCheckInterval: number
  // 数据变化时的快速轮询间隔
  fastPollInterval: number
  // 快速轮询持续时间
  fastPollDuration: number
}

export interface PollingState {
  isRunning: boolean
  currentInterval: number
  phase: string | null
  lastDataChange: number
  isFastPolling: boolean
}

export function useUnifiedPolling() {
  const errorHandler = useErrorHandler()
  const { emitGamePhaseChanged } = useGameEvents()

  // 轮询配置
  const config: PollingConfig = {
    baseInterval: 3000,      // 3秒基础间隔
    maxInterval: 30000,      // 30秒最大间隔
    connectionCheckInterval: 5000,  // 5秒连接检查
    fastPollInterval: 1000,  // 1秒快速轮询
    fastPollDuration: 10000  // 10秒快速轮询持续时间
  }

  // 状态管理
  const state = ref<PollingState>({
    isRunning: false,
    currentInterval: config.baseInterval,
    phase: null,
    lastDataChange: 0,
    isFastPolling: false
  })

  // 轮询控制
  let pollingTimer: NodeJS.Timeout | null = null
  let fastPollTimer: NodeJS.Timeout | null = null
  let connectionCheckTimer: NodeJS.Timeout | null = null

  /**
   * 开始轮询
   */
  const startPolling = async () => {
    if (state.value.isRunning) {
      console.log('[UnifiedPolling] 轮询已在运行中')
      return
    }

    state.value.isRunning = true
    console.log('[UnifiedPolling] 开始统一轮询')

    // 启动连接检查
    startConnectionCheck()

    // 启动主轮询循环
    scheduleNextPoll()
  }

  /**
   * 停止轮询
   */
  const stopPolling = () => {
    state.value.isRunning = false

    if (pollingTimer) {
      clearTimeout(pollingTimer)
      pollingTimer = null
    }

    if (fastPollTimer) {
      clearTimeout(fastPollTimer)
      fastPollTimer = null
    }

    if (connectionCheckTimer) {
      clearTimeout(connectionCheckTimer)
      connectionCheckTimer = null
    }

    console.log('[UnifiedPolling] 轮询已停止')
  }

  /**
   * 启动连接检查
   */
  const startConnectionCheck = () => {
    if (connectionCheckTimer) {
      clearTimeout(connectionCheckTimer)
    }

    connectionCheckTimer = setTimeout(async () => {
      if (!state.value.isRunning) return

      try {
        await checkConnection()
      } catch (error) {
        console.error('[UnifiedPolling] 连接检查失败:', error)
      }

      startConnectionCheck()
    }, config.connectionCheckInterval)
  }

  /**
   * 检查连接状态
   */
  const checkConnection = async () => {
    try {
      const authInfo = await invoke<any>('get_auth_info')
      const isConnected = !!authInfo

      if (isConnected && !state.value.phase) {
        console.log('[UnifiedPolling] 检测到连接建立')
        // 连接建立时立即进行一次轮询
        scheduleNextPoll()
      }
    } catch (error) {
      console.error('[UnifiedPolling] 检查连接失败:', error)
    }
  }

  /**
   * 安排下一次轮询
   */
  const scheduleNextPoll = () => {
    if (!state.value.isRunning) return

    if (pollingTimer) {
      clearTimeout(pollingTimer)
    }

    const interval = getAdaptiveInterval()
    state.value.currentInterval = interval

    pollingTimer = setTimeout(async () => {
      if (!state.value.isRunning) return

      try {
        await performPoll()
      } catch (error) {
        errorHandler.handleError(error, '轮询操作', false)
      }

      scheduleNextPoll()
    }, interval)
  }

  /**
   * 执行轮询操作
   */
  const performPoll = async () => {
    const currentPhase = await getCurrentPhase()

    // 检查阶段是否变化
    if (currentPhase !== state.value.phase) {
      state.value.phase = currentPhase
      state.value.lastDataChange = Date.now()
      emitGamePhaseChanged(currentPhase)

      console.log(`[UnifiedPolling] 游戏阶段变化: ${currentPhase}`)

      // 阶段变化时启用快速轮询
      if (!state.value.isFastPolling) {
        enableFastPolling()
      }
    }

    // 根据阶段执行相应的轮询操作
    await pollByPhase(currentPhase)
  }

  /**
   * 获取当前游戏阶段
   */
  const getCurrentPhase = async (): Promise<string | null> => {
    try {
      return await invoke<string | null>('get_gameflow_phase')
    } catch (error) {
      console.error('[UnifiedPolling] 获取游戏阶段失败:', error)
      return null
    }
  }

  /**
   * 根据阶段执行轮询
   */
  const pollByPhase = async (phase: string | null) => {
    if (!phase) return

    try {
      switch (phase) {
        case 'ChampSelect':
          await pollChampSelect()
          break
        case 'InProgress':
          await pollInProgress()
          break
        case 'Lobby':
        case 'Matchmaking':
          await pollLobbyAndMatchmaking()
          break
        default:
          console.log(`[UnifiedPolling] 未处理的阶段: ${phase}`)
      }
    } catch (error) {
      console.error(`[UnifiedPolling] 轮询阶段 ${phase} 失败:`, error)
    }
  }

  /**
   * 选人阶段轮询
   */
  const pollChampSelect = async () => {
    try {
      const session = await invoke<any>('get_champ_select_session')
      if (session) {
        // 发送选人数据到前端
        console.log('[UnifiedPolling] 获取选人数据')
      }
    } catch (error) {
      console.error('[UnifiedPolling] 获取选人数据失败:', error)
    }
  }

  /**
   * 游戏进行中轮询
   */
  const pollInProgress = async () => {
    try {
      // 检查 LiveClient 可用性
      const isAvailable = await invoke<boolean>('is_liveclient_available')
      if (isAvailable) {
        const players = await invoke<any[]>('get_live_player_list')
        if (players && players.length > 0) {
          console.log('[UnifiedPolling] 获取 LiveClient 数据')
          state.value.lastDataChange = Date.now()

          // 有数据变化时启用快速轮询
          if (!state.value.isFastPolling) {
            enableFastPolling()
          }
        }
      }
    } catch (error) {
      console.error('[UnifiedPolling] 获取游戏数据失败:', error)
    }
  }

  /**
   * 大厅和匹配轮询
   */
  const pollLobbyAndMatchmaking = async () => {
    try {
      const [lobbyInfo, matchmakingState] = await Promise.allSettled([
        invoke<any>('get_lobby_info'),
        invoke<any>('get_matchmaking_state')
      ])

      if (lobbyInfo.status === 'fulfilled' && lobbyInfo.value) {
        console.log('[UnifiedPolling] 获取大厅信息')
      }

      if (matchmakingState.status === 'fulfilled' && matchmakingState.value) {
        console.log('[UnifiedPolling] 获取匹配状态')
      }
    } catch (error) {
      console.error('[UnifiedPolling] 获取大厅/匹配数据失败:', error)
    }
  }

  /**
   * 启用快速轮询
   */
  const enableFastPolling = () => {
    if (state.value.isFastPolling) return

    state.value.isFastPolling = true
    state.value.currentInterval = config.fastPollInterval

    console.log('[UnifiedPolling] 启用快速轮询')

    // 设置快速轮询结束时间
    fastPollTimer = setTimeout(() => {
      state.value.isFastPolling = false
      state.value.currentInterval = getAdaptiveInterval()
      console.log('[UnifiedPolling] 快速轮询结束，恢复正常间隔')
    }, config.fastPollDuration)
  }

  /**
   * 获取自适应轮询间隔
   */
  const getAdaptiveInterval = (): number => {

    // 如果刚有数据变化，使用快速轮询
    if (state.value.isFastPolling) {
      return config.fastPollInterval
    }

    // 根据阶段调整间隔
    switch (state.value.phase) {
      case 'ChampSelect':
        return config.baseInterval // 选人阶段保持基础间隔
      case 'InProgress':
        return Math.min(config.baseInterval * 2, config.maxInterval) // 游戏中适当延长
      case 'Lobby':
      case 'Matchmaking':
        return Math.min(config.baseInterval * 3, config.maxInterval) // 大厅/匹配阶段更长间隔
      default:
        return config.maxInterval // 其他阶段使用最大间隔
    }
  }

  /**
   * 重置轮询状态
   */
  const resetState = () => {
    state.value = {
      isRunning: false,
      currentInterval: config.baseInterval,
      phase: null,
      lastDataChange: 0,
      isFastPolling: false
    }
  }

  // 计算属性
  const isActive = computed(() => state.value.isRunning)
  const currentInterval = computed(() => state.value.currentInterval)
  const currentPhase = computed(() => state.value.phase)

  // 组件卸载时清理
  onUnmounted(() => {
    stopPolling()
  })

  return {
    // 状态
    isActive,
    currentInterval,
    currentPhase,

    // 方法
    startPolling,
    stopPolling,
    resetState
  }
}
