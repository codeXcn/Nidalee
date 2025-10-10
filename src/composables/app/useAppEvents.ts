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
  // 事件源去重：优先使用后端标准化事件，短时间内忽略从 lcu-ws 转发的同类事件
  // 避免重复更新或状态抖动
  const RECENT_NORMALIZED_GRACE_MS = 600
  const recentNormalizedAt: Record<'phase' | 'lobby' | 'champ', number> = {
    phase: 0,
    lobby: 0,
    champ: 0
  }
  const isWithinGrace = (key: 'phase' | 'lobby' | 'champ') =>
    Date.now() - recentNormalizedAt[key] < RECENT_NORMALIZED_GRACE_MS
  const markNormalized = (key: 'phase' | 'lobby' | 'champ') => {
    recentNormalizedAt[key] = Date.now()
  }

  // 标准化事件处理：记录为 normalized
  const handleGameFlowPhaseChange = (event: any) => {
    console.log('[AppEvents] 游戏阶段变化:', event.payload)
    const phase = event.payload as string | null
    handleGamePhaseChange(phase ? { phase } : null)
    markNormalized('phase')
  }

  const handleLobbyChangeEvent = (event: any) => {
    console.log('[AppEvents] 大厅变化:', event.payload)
    const lobbyInfo = event.payload as LobbyInfo | null
    handleLobbyChange(lobbyInfo)
    markNormalized('lobby')
  }

  const handleChampSelectSessionChanged = (event: any) => {
    console.log('[AppEvents] 英雄选择会话变化:', event.payload)
    const session = event.payload as ChampSelectSession | null
    handleChampSelectChange(session)
    markNormalized('champ')
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
      await listen('gameflow-phase-change', handleGameFlowPhaseChange)
      await listen('lobby-change', handleLobbyChangeEvent)
      await listen('champ-select-session-changed', handleChampSelectSessionChanged)
      await listen('connection-state-changed', handleConnectionStateChangeDebounced)
      await listen('game-finished', handleGameFinished)
      // 新增：直接消费 lcu-ws，避免完全依赖后端转发事件
      await listen<string>('lcu-ws', (e) => {
        try {
          const data = JSON.parse(e.payload)
          if (!Array.isArray(data) || data.length < 3) return
          const [msgType, evtName, payload] = data
          if (msgType !== 8 || evtName !== 'OnJsonApiEvent' || !payload) return
          const uri = payload.uri as string
          const eventType = payload.eventType as string
          const body = payload.data

          switch (uri) {
            case '/lol-gameflow/v1/gameflow-phase': {
              // 直接驱动阶段变化
              // 若刚刚收到过同类的标准化事件，则跳过 lcu-ws 派生事件，避免重复
              if (!isWithinGrace('phase')) {
                handleGameFlowPhaseChange({ payload: body })
              }
              break
            }
            case '/lol-gameflow/v1/session': {
              // 兼容：从 session.phase 推阶段
              if (body?.phase && !isWithinGrace('phase')) {
                handleGameFlowPhaseChange({ payload: body.phase })
              }
              break
            }
            case '/lol-champ-select/v1/session': {
              // 直接驱动选人会话
              if (!isWithinGrace('champ')) {
                handleChampSelectSessionChanged({ payload: eventType === 'Delete' ? null : body })
              }
              break
            }
            case '/lol-lobby/v2/lobby': {
              // 直接驱动大厅变化（存在/不存在即可）
              if (!isWithinGrace('lobby')) {
                handleLobbyChangeEvent({ payload: eventType === 'Delete' ? null : body })
              }
              break
            }
            default:
              break
          }
        } catch {
          // 静默解析错误
        }
      })
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
