import { useSummonerAndMatchUpdater } from '@/composables/game/useSummonerAndMatchUpdater'
import { useActivityStore, useDataStore, useSessionStore, useGameStore } from '@/stores'
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { shallowRef, computed } from 'vue'

export const useConnectionStore = defineStore('connection', () => {
  // 核心状态：使用 shallowRef 进行优化，因为我们只处理原始类型
  const connectionState = shallowRef<ConnectionState>('Disconnected')
  const connectionError = shallowRef<string | null>(null)

  // 计算属性
  const isConnected = computed(() => connectionState.value === 'Connected')
  const isConnecting = computed(
    () =>
      connectionState.value === ('Connecting' as ConnectionState) ||
      connectionState.value === ('ProcessFound' as ConnectionState)
  )
  const isDisconnected = computed(() => connectionState.value === 'Disconnected')
  const { updateSummonerAndMatches } = useSummonerAndMatchUpdater()
  const dataStore = useDataStore()
  const activityStore = useActivityStore()
  const sessionStore = useSessionStore()
  const gameStore = useGameStore()

  async function checkConnection() {
    try {
      const state = await invoke<ConnectionState>('check_connection_state_command')
      await updateConnectionState(state)
      console.log('[ConnectionStore] Initial connection check:', state)
    } catch (error) {
      console.error('[ConnectionStore] Failed to check initial connection:', error)
      await updateConnectionState('Disconnected', 'Failed to communicate with backend')
    }
  }

  async function updateConnectionState(state: ConnectionState, errorMsg: string | null = null) {
    console.log(`[ConnectionStore] Updating connection state: ${state}`, errorMsg || '')
    connectionState.value = state
    connectionError.value = errorMsg

    switch (state) {
      case 'Connected':
        activityStore.addActivity('success', '已连接到客户端', 'connection')
        // 连接成功，后端已自动处理认证，前端直接开始业务逻辑
        updateSummonerAndMatches()
        sessionStore.startSession()
        break
      case 'Disconnected':
        sessionStore.stopSession()
        activityStore.addActivity('error', '已断开与客户端的连接', 'connection')
        // 直接在这里执行完整的清理逻辑
        dataStore.clearAllData()
        gameStore.resetGameState()
        break
      case 'ProcessFound':
      case 'AuthExpired':
      case 'Unstable':
        sessionStore.stopSession()
        if (state === 'Unstable') {
          activityStore.addActivity('warning', '连接不稳定，正在重试...', 'connection')
        }
        break
    }
  }

  function reset() {
    connectionState.value = 'Disconnected'
    connectionError.value = null
  }

  const statusText = computed(() => {
    if (connectionError.value) {
      return `连接出错: ${connectionError.value}`
    } else {
      switch (connectionState.value) {
        case 'Disconnected':
          return '等待连接到League客户端...'
        case 'ProcessFound':
          return '检测到客户端进程，正在建立连接...'
        case 'Unstable':
          return '连接不稳定，正在重试...'
        case 'AuthExpired':
          return '认证信息已过期，正在重新获取...'
        default:
          return isConnected.value ? '已连接' : '未知状态'
      }
    }
  })

  return {
    connectionState,
    connectionError,
    isConnected,
    isConnecting,
    isDisconnected,
    statusText,
    checkConnection,
    updateConnectionState,
    reset
  }
})
