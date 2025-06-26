import { getLatestVersion } from '@/lib'
import {
  useThemeStore,
  useAppSessionStore,
  useSummonerStore,
  useMatchStatisticsStore,
  useConnectionStore
} from '@/stores'
import { useGameManagement } from '@/composables'
import { listen } from '@tauri-apps/api/event'

export function useApp() {
  const themeStore = useThemeStore()
  const appSessionStore = useAppSessionStore()
  const summonerStore = useSummonerStore()
  const matchStatisticsStore = useMatchStatisticsStore()
  const connectionStore = useConnectionStore()
  const gameManagement = useGameManagement()

  const { handleChampSelectChange, handleGamePhaseChange, handleLobbyChange } = gameManagement

  const isDark = computed(() => themeStore.isDark)

  // 初始化主题
  themeStore.initTheme()

  onMounted(async () => {
    try {
      const latestVersion = await getLatestVersion()
      if (latestVersion !== appSessionStore.gameVersion) {
        appSessionStore.setGameVersion(latestVersion)
      }

      // 应用启动时主动尝试获取召唤师信息和战绩
      console.log('[App] 应用启动，尝试获取最新数据...')

      try {
        await summonerStore.fetchSummonerInfo()
        console.log('[App] 召唤师信息获取成功')
        connectionStore.setConnectionStatus(true)

        // 如果召唤师信息获取成功，则获取战绩
        try {
          await matchStatisticsStore.fetchMatchHistory()
          console.log('[App] 战绩数据获取成功')
        } catch (error) {
          console.log('[App] 战绩数据获取失败，可能客户端未连接:', error)
        }
      } catch (error) {
        console.log('[App] 召唤师信息获取失败，客户端可能未连接:', error)
        connectionStore.setConnectionStatus(false, '无法连接到客户端')
      }

      // 监听召唤师信息变化（连接状态的核心指标）
      await listen('summoner-change', async (event) => {
        console.log('[Event] summoner-change:', event.payload)

        if (event.payload) {
          // 有召唤师信息，表示连接已建立
          summonerStore.updateSummonerInfo(event.payload)
          connectionStore.setConnectionStatus(true)

          // 自动获取战绩
          try {
            await matchStatisticsStore.fetchMatchHistory()
            console.log('[Event] 战绩数据已更新')
          } catch (error) {
            console.warn('[Event] 自动获取战绩失败:', error)
          }
        } else {
          // 召唤师信息为空，表示连接已断开
          console.log('[Event] 连接断开，清理所有数据')
          connectionStore.setConnectionStatus(false, '客户端连接已断开')
          summonerStore.clearSummonerInfo()
          matchStatisticsStore.clearMatchHistory()

          // 使用gameManagement的断开连接处理逻辑
          gameManagement.handleDisconnection()
        }
      })

      // 监听游戏结束事件，刷新战绩
      await listen('game-finished', async () => {
        console.log('[Event] game-finished，刷新战绩')
        try {
          await matchStatisticsStore.fetchMatchHistory()
        } catch (error) {
          console.warn('[Event] 游戏结束后刷新战绩失败:', error)
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

      console.log('[App] 所有事件监听器已设置')
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
