import { getLatestVersion } from '@/lib'
import { useThemeStore, useAppSessionStore } from '@/stores'
import { useGameManagement } from '@/composables'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export function useApp() {
  const themeStore = useThemeStore()
  const appSessionStore = useAppSessionStore()
  const gameManagement = useGameManagement()

  const {
    connectionStore,
    summonerStore,
    activityStore,
    matchStatisticsStore,
    handleChampSelectChange,
    handleGamePhaseChange,
    handleLobbyChange
  } = gameManagement

  const isDark = computed(() => themeStore.isDark)

  // 获取对局历史
  const fetchMatchHistory = async () => {
    try {
      console.log('[App] 开始获取对局历史...')
      const matchHistory = await invoke<MatchStatistics>('get_match_history')
      console.log('[App] 对局历史数据:', matchHistory)
      matchStatisticsStore.updateMatchStatistics(matchHistory)
    } catch (error) {
      console.error('[App] 获取对局历史失败:', error)
      activityStore.addActivity('error', `获取对局历史失败: ${error}`)
    }
  }

  // 初始化连接和主题
  gameManagement.initializeConnection()
  themeStore.initTheme()

  onMounted(async () => {
    try {
      const latestVersion = await getLatestVersion()
      if (latestVersion !== appSessionStore.gameVersion) {
        appSessionStore.setGameVersion(latestVersion)
      }

      if (connectionStore.isConnected) {
        activityStore.addActivity('info', '从缓存恢复连接状态')

        if (summonerStore.summonerInfo) {
          activityStore.addActivity('info', `从缓存恢复召唤师信息: ${summonerStore.summonerInfo.displayName}`)
          await fetchMatchHistory()
        }

        if (connectionStore.authInfo) {
          activityStore.addActivity('info', '从缓存恢复认证信息')
        }
      }

      // 监听召唤师信息变化
      await listen('summoner-change', async (event) => {
        console.log('[Event] 召唤师信息变化:', event.payload)
        if (event.payload) {
          summonerStore.updateSummonerInfo(event.payload as SummonerInfo)
          await fetchMatchHistory()
        } else {
          summonerStore.clearSummonerInfo()
        }
      })

      // 监听认证信息变化
      await listen('auth-info-change', (event) => {
        console.log('[Event] 认证信息变化:', event.payload)
        if (event.payload) {
          connectionStore.setAuthInfo(event.payload as LcuAuthInfo)
          activityStore.addActivity('success', 'LCU 认证信息已更新')
        } else {
          connectionStore.clearAuthInfo()
          activityStore.addActivity('warning', 'LCU 认证信息已清除')
        }
      })

      // 监听游戏阶段变化
      await listen('gameflow-phase-change', (event) => {
        console.log('[Event] gameflow:', event.payload)
        handleGamePhaseChange(event.payload as GamePhase | null)
      })

      // 监听房间变化
      await listen('lobby-change', (event) => {
        console.log('[Event] lobby:', event.payload)
        handleLobbyChange(event.payload as LobbyInfo | null)
      })

      // 监听选人阶段变化
      await listen('champ-select-session-changed', (event) => {
        console.log('[Event] session信息变化:', event.payload)
        handleChampSelectChange(event.payload as ChampSelectSession | null)
      })

      console.log('[App] 事件监听器已设置')
    } catch (error) {
      console.error('[App] 设置事件监听器失败:', error)
    }
  })

  onUnmounted(() => {
    console.log('onUnmounted')
  })

  return {
    isDark
  }
}
