import { useConnectionStore } from '@/stores/core/connectionStore'
import { computed } from 'vue'

/**
 * 连接状态管理组合式函数
 * 职责：提供连接状态的响应式接口，不直接处理事件
 */
export function useConnection() {
  const connectionStore = useConnectionStore()

  // 直接从 store 中获取响应式状态
  const isConnected = computed(() => connectionStore.isConnected)
  const connectionState = computed(() => connectionStore.connectionState)
  const connectionError = computed(() => connectionStore.connectionError)
  const hasAuth = computed(() => connectionStore.hasAuth)
  // 直接使用 store 中更完善的状态文本
  const connectionMessage = computed(() => connectionStore.statusText)

  // 代理 store 中的方法
  const checkConnection = () => {
    console.log('手动检查连接🙌')
    return connectionStore.checkConnection()
  }

  // 更新为调用 reset 方法
  const clearConnection = () => {
    connectionStore.reset()
  }

  return {
    // 状态
    hasAuth,
    isConnected,
    connectionState,
    connectionError,
    connectionMessage,

    // 方法
    checkConnection,
    clearConnection
  }
}
