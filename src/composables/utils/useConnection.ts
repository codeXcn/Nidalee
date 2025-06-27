import { ref, readonly, computed } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useSummonerStore } from '@/stores/summonerStore'
import { useMatchStatisticsStore } from '@/stores/matchStatisticsStore'
import { useConnectStore } from '@/stores'

// 重构后的连接管理
export function useConnection() {
  const authStore = useConnectStore()

  const summonerStore = useSummonerStore()
  const matchStatisticsStore = useMatchStatisticsStore()

  const isListening = ref(false)

  // 使用 store 中的状态，不再重复定义
  const isConnected = computed(() => authStore.isConnected)
  const connectionMessage = computed(() => {
    if (authStore.isConnected) {
      return '已连接到League客户端'
    } else if (authStore.connectionError) {
      return authStore.connectionError
    } else {
      // 根据连接状态提供更具体的信息
      switch (authStore.connectionState) {
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

  // 开始监听后端事件
  const startListening = async () => {
    if (isListening.value) return

    try {
      // 监听认证信息变化（更准确的连接状态）
      // await listen('auth-info-change', async (event) => {
      //   console.log('[Connection] 认证信息变化:', event.payload)
      //   authStore.setAuthInfo(event.payload)
      // })

      // 监听召唤师信息变化（用于更新召唤师数据）
      await listen('summoner-change', async (event) => {
        console.log('[Connection] 召唤师信息变化:', event.payload)

        if (event.payload) {
          summonerStore.updateSummonerInfo(event.payload)
        } else {
          summonerStore.clearSummonerInfo()
          // 发送断开连接事件，让其他模块处理
          document.dispatchEvent(new CustomEvent('connection-lost'))
        }
      })

      // 监听游戏结束事件（独立处理）
      await listen('game-finished', async () => {
        console.log('[Connection] 游戏结束事件')
        // 发送自定义事件，让需要的组件自行决定是否刷新数据
        document.dispatchEvent(new CustomEvent('game-finished'))
      })

      // 监听连接状态变化（如果后端发送）
      await listen('connection-state-changed', async (event) => {
        console.log('[Connection] 连接状态变化:', event.payload)

        // event.payload 是完整的 ConnectionInfo 对象
        const connectionInfo = event.payload as {
          state: 'Disconnected' | 'ProcessFound' | 'Connected' | 'Unstable' | 'AuthExpired'
          auth_info: any | null
          consecutive_failures: number
          error_message: string | null
        }

        // 使用 authStore 的 updateConnectionInfo 方法处理完整的连接信息
        authStore.updateConnectionInfo(connectionInfo)

        // 根据连接状态触发相应的召唤师信息更新
        if (connectionInfo.state === 'Connected' && connectionInfo.auth_info) {
          // 连接成功且有认证信息时，尝试获取召唤师信息和战绩
          try {
            await summonerStore.fetchSummonerInfo()
            // 获取召唤师信息成功后，再获取战绩
            await matchStatisticsStore.fetchMatchHistory()
          } catch (error) {
            console.error('[Connection] 获取召唤师信息或战绩失败:', error)
          }
        } else if (connectionInfo.state === 'Disconnected') {
          // 完全断开连接时清除召唤师信息和战绩
          summonerStore.clearSummonerInfo()
          matchStatisticsStore.clearMatchHistory()
        }
      })

      isListening.value = true
      console.log('[Connection] 开始监听连接状态')
    } catch (error) {
      console.error('[Connection] 启动监听失败:', error)
    }
  }

  // 停止监听
  const stopListening = () => {
    isListening.value = false
  }

  // 手动刷新连接状态
  const refreshConnection = async () => {
    try {
      console.log('[Connection] 手动刷新连接状态')
      await authStore.checkConnection()
      if (authStore.isConnected) {
        await summonerStore.fetchSummonerInfo()
        // 获取召唤师信息成功后，再获取战绩
        await matchStatisticsStore.fetchMatchHistory()
      }
    } catch (error) {
      console.error('[Connection] 手动刷新失败:', error)
    }
  }

  return {
    // 状态（使用计算属性，数据来源于 store）
    isConnected,
    connectionMessage,
    isListening: readonly(isListening),

    // 方法
    startListening,
    stopListening,
    refreshConnection
  }
}
