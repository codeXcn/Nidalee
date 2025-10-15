import { useMatchAnalysisStore } from '@/features/match-analysis/store'
import { listen } from '@tauri-apps/api/event'
import { debounce, isObject } from 'radash'

// 创建一个模块级别的状态，用于跟踪监听器
let unlisteners: (() => void)[] = []
let isListeningStarted = false

/**
 * 应用事件处理组合式函数
 * 职责：监听和处理游戏相关的事件
 */
export function useAppEvents() {
  const gamePhaseManager = useGamePhaseManager()
  const champSelectManager = useChampSelectManager()
  const { updateMatchHistory, updateSummonerInfo } = useSummonerAndMatchUpdater()
  const connectionStore = useConnectionStore()
  const matchmakingStore = useMatchmakingStore()
  const matchAnalysisStore = useMatchAnalysisStore()

  const { handleGamePhaseChange } = gamePhaseManager
  const { handleChampSelectChange, handleLobbyChange } = champSelectManager

  // 事件处理函数
  const handleGameFlowPhaseChange = (event: any) => {
    console.log('[AppEvents] 游戏阶段变化:', event.payload)
    const phase = event.payload as string
    handleGamePhaseChange(phase)
  }

  const handleGameflowSessionChanged = (event: any) => {
    console.log('[AppEvents] Gameflow Session 变化:', event.payload)
    // 可根据需要处理
  }

  const handleLobbyChangeEvent = (event: any) => {
    console.log('[AppEvents] 大厅变化:', event.payload)
    handleLobbyChange(event.payload as LobbyInfo | null)
  }

  const handleChampSelectSessionChanged = (event: any) => {
    console.log('[AppEvents] 英雄选择会话变化:', event.payload)
    handleChampSelectChange(event.payload as ChampSelectSession | null)
  }

  const handleMatchmakingStateChanged = (event: any) => {
    console.log('[AppEvents] 匹配状态变化:', event.payload)
    matchmakingStore.updateState(event.payload)
  }

  // 🔥 核心重构：直接将完整数据传递给 store 的原子 action
  const handleTeamAnalysisData = (event: { payload: TeamAnalysisData | null }) => {
    console.log('[AppEvents] === 收到后端团队分析数据, 调用 store action ===')
    matchAnalysisStore.setTeamAnalysisData(event.payload)
  }

  const handleConnectionStateChange = async (event: any) => {
    const state = event.payload as ConnectedState
    await connectionStore.updateConnectionState(isObject(state) ? state.state : state)
  }

  const handleConnectionStateChangeDebounced = debounce({ delay: 300 }, handleConnectionStateChange)

  const handleGameFinished = () => {
    console.log('[AppEvents] 游戏结束事件')
    matchAnalysisStore.clearAllData()
  }

  const stopListening = () => {
    if (!isListeningStarted) return
    console.log(`[AppEvents] 停止 ${unlisteners.length} 个全局事件监听器...`)
    unlisteners.forEach((unlisten) => unlisten())
    unlisteners = []
    isListeningStarted = false
    console.log('[AppEvents] 全局事件监听已停止。')
  }

  const startListening = async () => {
    if (isListeningStarted) return
    stopListening()
    isListeningStarted = true

    try {
      unlisteners = [
        await listen('gameflow-phase-change', handleGameFlowPhaseChange),
        await listen('gameflow-session-changed', handleGameflowSessionChanged),
        await listen('lobby-change', handleLobbyChangeEvent),
        await listen('champ-select-session-changed', handleChampSelectSessionChanged),
        await listen('matchmaking-state-changed', handleMatchmakingStateChanged),
        await listen('connection-state-changed', handleConnectionStateChangeDebounced),
        await listen('game-finished', handleGameFinished),
        await listen('team-analysis-data', handleTeamAnalysisData)
      ]
      console.log(`[AppEvents] ${unlisteners.length} 个全局事件监听已启动`)

      // 启动时，额外尝试从后端获取一次缓存数据
      console.log('[AppEvents] 🔄 尝试从后端缓存恢复数据...')
      const { invoke } = await import('@tauri-apps/api/core')
      const cachedData = await invoke<TeamAnalysisData | null>('get_cached_analysis_data')
      if (cachedData) {
        console.log('[AppEvents] ✅ 找到缓存数据，正在恢复...')
        handleTeamAnalysisData({ payload: cachedData })
      }
    } catch (error) {
      console.error('[AppEvents] 启动全局事件监听失败:', error)
      stopListening()
    }
  }

  return {
    updateMatchHistory,
    updateSummonerInfo,
    startListening,
    stopListening
  }
}
