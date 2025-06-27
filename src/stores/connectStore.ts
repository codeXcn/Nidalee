import { invoke } from '@tauri-apps/api/core'
export const useConnectStore = defineStore(
  'auth',
  () => {
    // 基础连接状态
    const isConnected = ref(false)
    const authInfo = ref<LcuAuthInfo | null>(null)
    const connectionError = ref<string | null>(null)
    const connectionState = ref<ConnectionState>('Disconnected')
    const consecutiveFailures = ref(0)

    /**
     * 手动检查连接状态
     * 该方法会调用Rust backend的 `check_connection_state` 命令，并将结果更新到 store 中
     * @returns {Promise<void>}
     */
    const checkConnection = async () => {
      const info = await invoke('check_connection_state_command')
      updateConnectionInfo(info)
    }
    /**
     * 根据 ConnectionInfo 更新认证信息
     * @param {ConnectionInfo} info 连接信息
     */
    const updateConnectionInfo = (info: ConnectionInfo) => {
      console.log('[AuthStore-updateConnectionInfo] 更新连接信息:', info)
      connectionState.value = info.state
      consecutiveFailures.value = info.consecutive_failures
      connectionError.value = info.error_message

      // 根据状态更新连接状态和认证信息
      switch (info.state) {
        case 'Connected':
          isConnected.value = true
          authInfo.value = info.auth_info
          break
        case 'Disconnected':
        case 'ProcessFound':
        case 'AuthExpired':
          isConnected.value = false
          authInfo.value = null
          break
        case 'Unstable':
          // 不稳定状态保持当前认证信息，但标记为未连接
          isConnected.value = false
          // 保留 authInfo 以便重连时使用
          break
      }
    }
    const clearAuthInfo = () => {
      authInfo.value = null
      isConnected.value = false
      connectionError.value = null
    }
    return {
      clearAuthInfo,
      checkConnection,
      updateConnectionInfo,
      isConnected,
      authInfo,
      connectionError,
      connectionState,
      consecutiveFailures
    }
  },
  {
    persist: true
  }
)
