import { useChampSelectManager } from '@/composables/game/useChampSelectManager'
import { useGamePhaseManager } from '@/composables/game/useGamePhaseManager'
import { useSummonerAndMatchUpdater } from '@/composables/game/useSummonerAndMatchUpdater'
import { useActivityStore } from '@/stores'
import { useConnectionStore } from '@/stores/core/connectionStore'
import { useDataStore } from '@/stores/core/dataStore'
import { listen } from '@tauri-apps/api/event'
import { debounce } from 'radash'
/**
 * 应用事件处理组合式函数
 * 职责：监听和处理游戏相关的事件
 */
export function useAppEvents() {
  const gamePhaseManager = useGamePhaseManager()
  const champSelectManager = useChampSelectManager()
  const { updateSummonerAndMatches, updateMatchHistory, updateSummonerInfo } = useSummonerAndMatchUpdater()
  const activityStore = useActivityStore()
  const connectionStore = useConnectionStore()
  const dataStore = useDataStore()
  const isListening = ref(false)

  const { handleGamePhaseChange } = gamePhaseManager
  const { handleChampSelectChange, handleLobbyChange } = champSelectManager

  // 处理游戏阶段变化
  const handleGameflowPhaseChange = (event: any) => {
    console.log('[AppEvents] 游戏阶段变化:', event.payload)
    const phase = event.payload as string | null

    handleGamePhaseChange(phase as any)

    // 记录活动
    if (phase) {
      switch (phase) {
        case 'None':
          activityStore.addActivity('info', '返回客户端主界面', 'game')
          break
        case 'Lobby':
          activityStore.addActivity('info', '进入队列匹配中', 'game')
          break
        case 'ReadyCheck':
          activityStore.addActivity('success', '找到对局，等待接受', 'game')
          break
        case 'ChampSelect':
          activityStore.addActivity('info', '进入英雄选择阶段', 'game')
          break
        case 'InProgress':
          activityStore.addActivity('success', '游戏开始', 'game')
          break
        case 'WaitingForStats':
          activityStore.addActivity('info', '游戏结束', 'game')
          break
      }
    }
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
    const connectionInfo = event.payload
    connectionStore.updateConnectionInfo(connectionInfo)
    if (connectionInfo.state === 'Disconnected') {
      activityStore.addActivity('error', '已断开与客户端的连接', 'connection')
      dataStore.clearSummonerInfo()
      dataStore.clearMatchHistory()
    } else if (connectionInfo.state === 'Connected') {
      activityStore.addActivity('success', '已连接到客户端', 'connection')
      try {
        await updateSummonerAndMatches()
      } catch (error) {
        dataStore.clearSummonerInfo()
        dataStore.clearMatchHistory()
        activityStore.addActivity('error', '获取召唤师信息或战绩失败', 'error')
      }
    }
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
    if (isListening.value) return
    try {
      await listen('gameflow-phase-change', handleGameflowPhaseChange)
      await listen('lobby-change', handleLobbyChangeEvent)
      await listen('champ-select-session-changed', handleChampSelectSessionChanged)
      await listen('connection-state-changed', handleConnectionStateChangeDebounced)
      await listen('game-finished', handleGameFinished)
      isListening.value = true
      console.log('[AppEvents] 全局事件监听已启动')
    } catch (error) {
      console.error('[AppEvents] 启动全局事件监听失败:', error)
    }
  }

  // 停止监听
  const stopListening = () => {
    isListening.value = false
    console.log('[AppEvents] 停止全局事件监听')
  }

  // 生命周期管理
  onMounted(() => {
    startListening()
  })

  onUnmounted(() => {
    stopListening()
  })

  return {
    updateMatchHistory,
    updateSummonerInfo,
    isListening,
    startListening,
    stopListening
  }
}
