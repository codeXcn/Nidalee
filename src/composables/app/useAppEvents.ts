import { useChampSelectManager } from '@/composables/game/useChampSelectManager'
import { useGamePhaseManager } from '@/composables/game/useGamePhaseManager'
import { useSummonerAndMatchUpdater } from '@/composables/game/useSummonerAndMatchUpdater'
import { useConnectionStore, useMatchmakingStore } from '@/stores'
import { listen } from '@tauri-apps/api/event'
import { debounce, isObject } from 'radash'

// 创建一个模块级别的状态，用于跟踪是否已启动监听
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

  const { handleGamePhaseChange } = gamePhaseManager
  const { handleChampSelectChange, handleLobbyChange } = champSelectManager

  // 事件处理函数 - 处理后端标准化事件
  const handleGameFlowPhaseChange = (event: any) => {
    console.log('[AppEvents] 游戏阶段变化:', event.payload)
    const phase = event.payload as string | null
    handleGamePhaseChange(phase ? { phase } : null)
  }

  const handleGameflowSessionChanged = (event: any) => {
    console.log('[AppEvents] Gameflow Session 变化:', event.payload)
    const session = event.payload
    // 这里可以根据需要处理完整的 gameflow session 数据
    // 目前主要用于调试和监控
    if (session) {
      console.log('[AppEvents] Session 详情:', {
        phase: session.phase,
        gameClient: session.gameClient,
        map: session.map,
        gameData: session.gameData
      })
    }
  }

  const handleLobbyChangeEvent = (event: any) => {
    console.log('[AppEvents] 大厅变化:', event.payload)
    const lobbyInfo = event.payload as LobbyInfo | null
    handleLobbyChange(lobbyInfo)
  }

  const handleChampSelectSessionChanged = (event: any) => {
    console.log('[AppEvents] 英雄选择会话变化:', event.payload)
    const session = event.payload as ChampSelectSession | null
    handleChampSelectChange(session)
  }

  const handleMatchmakingStateChanged = (event: any) => {
    console.log('[AppEvents] 匹配状态变化:', event.payload)
    // 更新 matchmakingStore
    matchmakingStore.updateState(event.payload)
  }

  // 🔥 处理后端发送的完整团队分析数据 - 保持简单的事件转发
  const handleTeamAnalysisData = (event: any) => {
    console.log('[AppEvents] === 收到后端团队分析数据 ===')
    console.log('[AppEvents] 事件类型:', typeof event)
    console.log('[AppEvents] payload类型:', typeof event.payload)
    console.log('[AppEvents] payload是否为null:', event.payload === null)

    if (event.payload) {
      console.log('[AppEvents] payload字段:', Object.keys(event.payload))
      console.log('[AppEvents] 完整payload:', JSON.stringify(event.payload, null, 2))
    }

    // 简单转发，让专门的处理器处理业务逻辑
    document.dispatchEvent(new CustomEvent('backend-analysis-data', { detail: event.payload }))
    console.log('[AppEvents] 数据已转发到 backend-analysis-data 事件')
  }

  const handleConnectionStateChange = async (event: any) => {
    console.log('[AppEvents-handleConnectionStateChange] 连接状态变化:', event.payload)
    const state = event.payload as ConnectedState
    await connectionStore.updateConnectionState(isObject(state) ? state.state : state)
  }

  const handleConnectionStateChangeDebounced = debounce({ delay: 300 }, handleConnectionStateChange)

  const handleGameFinished = () => {
    console.log('[AppEvents] 游戏结束事件')
    document.dispatchEvent(new CustomEvent('game-finished'))
  }

  const startListening = async () => {
    if (isListeningStarted) return
    isListeningStarted = true // 立即设置标记，防止并发调用

    try {
      // 监听后端标准化事件（推荐方式）
      await listen('gameflow-phase-change', handleGameFlowPhaseChange)
      await listen('gameflow-session-changed', handleGameflowSessionChanged)
      await listen('lobby-change', handleLobbyChangeEvent)
      await listen('champ-select-session-changed', handleChampSelectSessionChanged)
      await listen('matchmaking-state-changed', handleMatchmakingStateChanged)
      await listen('connection-state-changed', handleConnectionStateChangeDebounced)
      await listen('game-finished', handleGameFinished)
      // 🔥 关键：监听后端发送的完整团队分析数据
      await listen('team-analysis-data', handleTeamAnalysisData)
      console.log('[AppEvents] 全局事件监听已启动')
    } catch (error) {
      console.error('[AppEvents] 启动全局事件监听失败:', error)
      isListeningStarted = false // 如果失败，允许重试
    }
  }

  // 停止监听 (在单页应用中通常不需要，但保留以备不时之需)
  const stopListening = () => {
    // 实际的停止逻辑需要 unlisten，这里简化为只重置标记
    isListeningStarted = false
    console.log('[AppEvents] 全局事件监听已停止 (标记已重置)')
  }

  return {
    updateMatchHistory,
    updateSummonerInfo,
    startListening,
    stopListening
  }
}
