import { invoke } from '@tauri-apps/api/core'

export const useConnectionStore = defineStore(
  'connection',
  () => {
    // 基础连接状态
    const isConnected = ref(false)
    const authInfo = ref<LcuAuthInfo | null>(null)
    const connectionError = ref<string | null>(null)

    // 设置认证信息
    const setAuthInfo = (info: LcuAuthInfo | null) => {
      authInfo.value = info
      isConnected.value = !!info
      connectionError.value = null
    }

    // 清除认证信息
    const clearAuthInfo = () => {
      authInfo.value = null
      isConnected.value = false
      connectionError.value = null
    }

    // 检查 LCU 连接
    const checkConnection = async () => {
      try {
        connectionError.value = null
        const connected = await invoke<LcuAuthInfo>('get_auth_info')
        setAuthInfo(connected)
        return true
      } catch (error) {
        console.error('检查连接失败:', error)
        setAuthInfo(null)
        connectionError.value = error instanceof Error ? error.message : '连接检查失败'
        return false
      }
    }

    // 设置连接状态（用于外部事件更新）
    const setConnectionStatus = (status: boolean, error?: string) => {
      isConnected.value = status
      if (!status) {
        authInfo.value = null
        connectionError.value = error || '连接已断开'
      } else {
        connectionError.value = null
      }
    }

    return {
      // 状态
      isConnected: readonly(isConnected),
      authInfo: readonly(authInfo),
      connectionError: readonly(connectionError),

      // 方法
      setAuthInfo,
      clearAuthInfo,
      checkConnection,
      setConnectionStatus
    }
  },
  {
    persist: {
      key: 'connection-store',
      paths: ['authInfo', 'isConnected'] // 持久化连接状态和认证信息
    }
  }
)
