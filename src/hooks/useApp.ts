import { useGameStore, useThemeStore } from '@/stores'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export function useApp() {
  const themeStore = useThemeStore()
  const gameStore = useGameStore()
  const isDark = computed(() => themeStore.isDark)

  // 获取对局历史
  const fetchMatchHistory = async () => {
    try {
      console.log('[App] 开始获取对局历史...')
      const matchHistory = await invoke<MatchStatistics>('get_match_history')
      console.log('[App] 对局历史数据:', matchHistory)
      gameStore.updateMatchStatistics(matchHistory)
    } catch (error) {
      console.error('[App] 获取对局历史失败:', error)
      gameStore.addActivity('error', `获取对局历史失败: ${error}`)
    }
  }

  gameStore.checkConnection()
  themeStore.initTheme()

  onMounted(async () => {
    try {
      if (gameStore.isConnected) {
        console.log('已连接')
        if (gameStore.isConnected) {
          gameStore.addActivity('info', '从缓存恢复连接状态')
        }
        if (gameStore.summonerInfo) {
          gameStore.addActivity('info', `从缓存恢复召唤师信息: ${gameStore.summonerInfo.displayName}`)
          await fetchMatchHistory()
        }
        if (gameStore.authInfo) {
          gameStore.addActivity('info', '从缓存恢复认证信息')
        }
      }

      await listen('summoner-change', async (event) => {
        console.log('[Event] 召唤师信息变化:', event.payload)
        if (event.payload) {
          gameStore.updateSummonerInfo(event.payload as SummonerInfo)
          await fetchMatchHistory()
        } else {
          gameStore.clearSummonerInfo()
        }
      })

      await listen('auth-info-change', (event) => {
        console.log('[Event] 认证信息变化:', event.payload)
        if (event.payload) {
          gameStore.setAuthInfo(event.payload as LcuAuthInfo)
        } else {
          gameStore.clearAuthInfo()
        }
      })

      await listen('gameflow-phase-change', (event) => {
        console.log('[Event] gameflow:', event.payload)
        gameStore.updateGamePhase(event.payload as GamePhase | null)
      })

      await listen('lobby-change', (event) => {
        console.log('[Event] lobby:', event.payload)
        gameStore.updateLobbyInfo(event.payload as LobbyInfo | null)
      })

      await listen('champ-select-session-changed', (event) => {
        console.log('[Event] session信息变化:', event.payload)
        gameStore.updateChampSelectSession(event.payload as ChampSelectSession | null)
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
