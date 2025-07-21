import { useChampSelectManager } from '@/composables/game/useChampSelectManager'
import { useGamePhaseManager } from '@/composables/game/useGamePhaseManager'
import { useSummonerAndMatchUpdater } from '@/composables/game/useSummonerAndMatchUpdater'
import { useConnectionStore } from '@/stores/core/connectionStore'
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

  const { handleGamePhaseChange } = gamePhaseManager
  const { handleChampSelectChange, handleLobbyChange } = champSelectManager

  // 处理游戏阶段变化
  const handleGameflowPhaseChange = (event: any) => {
    console.log('[AppEvents] 游戏阶段变化:', event.payload)
    const phase = event.payload as string | null
    handleGamePhaseChange(phase ? { phase } : null)
  }

  // 处理大厅变化
  const handleLobbyChangeEvent = (event: any) => {
    console.log('[AppEvents] 大厅变化:', event.payload)
    const lobbyInfo = event.payload as LobbyInfo | null
    handleLobbyChange(lobbyInfo)
  }

  // 处理英雄选择会话变化
  const handleChampSelectSessionChanged = (event: any) => {
    console.log('[AppEvents] 英雄选择会话变化:', event.payload)
    const session = event.payload as ChampSelectSession | null
    handleChampSelectChange(session)
  }

  // 连接状态变化处理
  const handleConnectionStateChange = async (event: any) => {
    console.log('[AppEvents-handleConnectionStateChange] 连接状态变化:', event.payload)
    const state = event.payload as ConnectedState
    await connectionStore.updateConnectionState(isObject(state) ? state.state : state)
  }

  // 防抖处理
  const handleConnectionStateChangeDebounced = debounce({ delay: 300 }, handleConnectionStateChange)

  // 新增：监听游戏结束事件
  const handleGameFinished = () => {
    console.log('[AppEvents] 游戏结束事件')
    document.dispatchEvent(new CustomEvent('game-finished'))
  }

  // 统一的事件监听启动
  const startListening = async () => {
    // 关键：检查模块级别的状态
    if (isListeningStarted) return
    isListeningStarted = true // 立即设置标记，防止并发调用

    try {
      await listen('gameflow-phase-change', handleGameflowPhaseChange)
      await listen('lobby-change', handleLobbyChangeEvent)
      await listen('champ-select-session-changed', handleChampSelectSessionChanged)
      await listen('connection-state-changed', handleConnectionStateChangeDebounced)
      await listen('game-finished', handleGameFinished)
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

  // 生命周期管理
  onMounted(() => {
    startListening()
  })

  return {
    updateMatchHistory,
    updateSummonerInfo,
    startListening,
    stopListening
  }
}
