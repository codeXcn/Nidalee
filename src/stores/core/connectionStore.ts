import { useSummonerAndMatchUpdater } from '@/composables/game/useSummonerAndMatchUpdater'
import { useActivityStore, useDataStore, useSessionStore } from '@/stores'
import { invoke } from '@tauri-apps/api/core'

export const useConnectionStore = defineStore(
  'connection',
  () => {
    // 基础连接状态
    const isConnected = ref(false)
    const authInfo = ref<any | null>(null)
    const connectionError = ref<string | null>(null)
    const connectionState = ref<string>('Disconnected')
    const consecutiveFailures = ref(0)
    const { updateSummonerAndMatches } = useSummonerAndMatchUpdater()
    const dataStore = useDataStore()
    /**
     * 手动检查连接状态
     */
    const checkConnection = async () => {
      try {
        const info = await invoke('check_connection_state_command')
        updateConnectionInfo(info)
      } catch (error) {
        clearAuthInfo()
        console.error('[ConnectionStore] 检查连接状态失败:', error)
        connectionError.value = error instanceof Error ? error.message : '检查连接失败'
      }
    }

    /**
     * 根据 ConnectionInfo 更新认证信息
     */
    const updateConnectionInfo = (info: any) => {
      const activityStore = useActivityStore()
      const sessionStore = useSessionStore()
      console.log('[ConnectionStore] 更新连接信息:', info)
      connectionState.value = info.state
      consecutiveFailures.value = info.consecutive_failures
      connectionError.value = info.error_message
      const state = info?.state || info
      // 根据状态更新连接状态和认证信息
      switch (state) {
        case 'Connected':
          isConnected.value = true
          authInfo.value = info.auth_info
          activityStore.addActivity('success', '已连接到客户端', 'connection')
          updateSummonerAndMatches()
          sessionStore.startSession()
          break
        case 'Disconnected':
        case 'ProcessFound':
        case 'AuthExpired':
          isConnected.value = false
          authInfo.value = null
          sessionStore.stopSession()
          if (state === 'Disconnected') {
            activityStore.addActivity('error', '已断开与客户端的连接', 'connection')
            dataStore.clearAllData()
          }
          break
        case 'Unstable':
          isConnected.value = false
          sessionStore.stopSession()
          activityStore.addActivity('warning', '连接不稳定，正在重试...', 'connection')
          break
      }
    }

    /**
     * 清除认证信息
     */
    const clearAuthInfo = () => {
      authInfo.value = null
      isConnected.value = false
      connectionError.value = null
      connectionState.value = 'Disconnected'
      consecutiveFailures.value = 0
    }

    /**
     * 重置连接状态
     */
    const resetConnection = () => {
      clearAuthInfo()
    }

    // 新增：连接消息提示
    const connectionMessage = computed(() => {
      if (isConnected.value) {
        return '已连接到League客户端'
      } else if (connectionError.value) {
        return connectionError.value
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
            return '等待连接到League客户端...'
        }
      }
    })

    // 新增：监听状态
    const isListening = ref(false)

    // 新增：刷新连接（兼容 useConnection 逻辑）
    const refreshConnection = async () => {
      try {
        console.log('[ConnectionStore] 手动刷新连接状态')
        await checkConnection()
      } catch (error) {
        console.error('[ConnectionStore] 手动刷新失败:', error)
      }
    }

    return {
      // 状态
      isConnected,
      authInfo,
      connectionError,
      connectionState,
      consecutiveFailures,
      connectionMessage,
      isListening,

      // 方法
      checkConnection,
      updateConnectionInfo,
      clearAuthInfo,
      resetConnection,
      refreshConnection
    }
  },
  {
    persist: true
  }
)
