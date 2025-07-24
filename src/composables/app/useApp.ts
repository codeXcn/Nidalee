import { useConnection } from '@/composables/connection/useConnection'
import { useSettingsStore } from '@/stores/ui/settingsStore'
import { computed, onMounted, onUnmounted } from 'vue'
import { useAppEvents } from '@/composables'
import { useAppInitialization } from '@/composables'

/**
 * 主应用组合式函数
 * 职责：整合各个模块，提供应用级别的状态和方法
 */
export function useApp() {
  const settingsStore = useSettingsStore()
  const appInit = useAppInitialization()
  const appEvents = useAppEvents()
  const { isConnected, connectionMessage, checkConnection, hasAuth } = useConnection()
  // 主题状态
  const isDark = computed(() => settingsStore.isDark)

  // 应用生命周期管理
  onMounted(async () => {
    try {
      // 启动事件监听
      appEvents.startListening()
      // 初始化应用
      await appInit.initializeApp()
      console.log('[App] 应用初始化和事件监听完成')
      // 检查新版本（可根据实际需求调整触发时机）
    } catch (error) {
      console.error('[App] 应用初始化失败:', error)
    }
  })

  onUnmounted(() => {
    console.log('[App] 组件卸载，清理资源')
    appInit.cleanup()
  })

  return {
    // 主题相关
    isDark,
    hasAuth,
    // 连接相关
    isConnected,
    connectionMessage,
    checkConnection,

    // 应用状态
    isInitialized: appInit.isInitialized,
    initializationError: appInit.initializationError,

    // 应用方法
    fetchMatchHistory: appEvents.updateMatchHistory,
    reinitialize: appInit.reinitialize
  }
}
