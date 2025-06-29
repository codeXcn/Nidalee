import { getLatestVersion } from '@/lib'
import { useThemeStore, useAppSessionStore, useSummonerStore } from '@/stores'
import { useAppInitialization } from './useAppInitialization'
import { useGamePhaseManager } from '@/composables/game/useGamePhaseManager'
import { useChampSelectManager } from '@/composables/game/useChampSelectManager'
import { listen } from '@tauri-apps/api/event'
import { useConnection } from '../utils/useConnection'

export function useApp() {
  const themeStore = useThemeStore()
  const appSessionStore = useAppSessionStore()
  const appInit = useAppInitialization()
  const gamePhaseManager = useGamePhaseManager()
  const champSelectManager = useChampSelectManager()
  const summonerStore = useSummonerStore()
  const { isConnected } = useConnection()
  const { handleGamePhaseChange } = gamePhaseManager
  const { handleChampSelectChange, handleLobbyChange } = champSelectManager

  const isDark = computed(() => themeStore.isDark)
  // 初始化主题（在onMounted之前）
  themeStore.initTheme()
  watch(
    isConnected,
    (isConnected) => {
      if (isConnected) {
        console.log('[App] 连接已建立，更新主题')
        // 如果已连接，自动获取召唤师信息和对局历史
        summonerStore.fetchSummonerInfo()
        appInit.fetchMatchHistory()
      } else {
        console.log('[App] 客户端未连接')
      }
    },
    {
      immediate: true
    }
  )
  onMounted(async () => {
    try {
      // 获取游戏版本
      const latestVersion = await getLatestVersion()
      if (latestVersion !== appSessionStore.gameVersion) {
        appSessionStore.setGameVersion(latestVersion)
      }

      // 使用新的应用初始化流程
      await appInit.initializeApp()

      // 只监听游戏相关的事件，连接相关的事件由 useConnection 处理
      await listen('gameflow-phase-change', (event) => {
        console.log('[Event] gameflow:', event.payload)
        handleGamePhaseChange(event.payload as GamePhase | null)
      })

      await listen('lobby-change', (event) => {
        console.log('[Event] lobby:', event.payload)
        handleLobbyChange(event.payload as LobbyInfo | null)
      })

      await listen('champ-select-session-changed', (event) => {
        console.log('[Event] session信息变化:', event.payload)
        handleChampSelectChange(event.payload as ChampSelectSession | null)
      })

      console.log('[App] 应用初始化和事件监听完成')
    } catch (error) {
      console.error('[App] 应用初始化失败:', error)
    }
  })

  onUnmounted(() => {
    console.log('[App] 组件卸载，清理资源')
    appInit.cleanup()
  })

  return {
    isDark,
    // 暴露连接相关的状态和方法，供组件使用
    isConnected,
    connectionMessage: appInit.connectionMessage,
    refreshConnection: appInit.refreshConnection,
    fetchMatchHistory: appInit.fetchMatchHistory
  }
}
