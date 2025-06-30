import { useConnectionStore } from '@/stores/core/connectionStore'
import { computed } from 'vue'

/**
 * 连接状态管理组合式函数
 * 职责：提供连接状态的响应式接口，不直接处理事件
 */
export function useConnection() {
  const connectionStore = useConnectionStore()

  // 连接状态
  const isConnected = computed(() => connectionStore.isConnected)
  const connectionState = computed(() => connectionStore.connectionState)
  const connectionError = computed(() => connectionStore.connectionError)
  const consecutiveFailures = computed(() => connectionStore.consecutiveFailures)

  // 连接状态消息
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

  // 手动检查连接
  const checkConnection = async () => {
    console.log('手动检查连接🙌')
    await connectionStore.checkConnection()
  }

  // 清除连接信息
  const clearConnection = () => {
    connectionStore.clearAuthInfo()
  }

  return {
    // 状态
    isConnected,
    connectionState,
    connectionError,
    consecutiveFailures,
    connectionMessage,

    // 方法
    checkConnection,
    clearConnection
  }
}
