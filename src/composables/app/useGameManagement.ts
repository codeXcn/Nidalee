import {
  useConnectionStore,
  useSummonerStore,
  useGameStatusStore,
  useActivityStore,
  useAutoFunctionStore,
  useMatchStatisticsStore,
  useAppSessionStore
} from '@/stores'
import { useGamePhaseManager } from '@/composables/game/useGamePhaseManager'
import { useChampSelectManager } from '@/composables/game/useChampSelectManager'
import { useAutoFunctionManager } from '@/composables/game/useAutoFunctionManager'
import { useDisconnectionHandler } from '@/composables/utils/useDisconnectionHandler'

/**
 * @deprecated 请直接使用对应的专门管理器：
 * - useGamePhaseManager 处理游戏阶段
 * - useChampSelectManager 处理选人和房间
 * - useAutoFunctionManager 处理自动功能
 * - useDisconnectionHandler 处理断开连接
 * 或者直接使用 store
 */
export function useGameManagement() {
  // 为了兼容性，暂时保留但委托给新的管理器
  const gamePhaseManager = useGamePhaseManager()
  const champSelectManager = useChampSelectManager()
  const autoFunctionManager = useAutoFunctionManager()
  const disconnectionHandler = useDisconnectionHandler()

  // 各个 store 实例（为了兼容性）
  const connectionStore = useConnectionStore()
  const summonerStore = useSummonerStore()
  const gameStatusStore = useGameStatusStore()
  const activityStore = useActivityStore()
  const autoFunctionStore = useAutoFunctionStore()
  const matchStatisticsStore = useMatchStatisticsStore()
  const appSessionStore = useAppSessionStore()

  return {
    // 各个 store 实例
    connectionStore,
    summonerStore,
    gameStatusStore,
    activityStore,
    autoFunctionStore,
    matchStatisticsStore,
    appSessionStore,

    // 委托给新管理器的方法
    handleGamePhaseChange: gamePhaseManager.handleGamePhaseChange,
    handleChampSelectChange: champSelectManager.handleChampSelectChange,
    handleLobbyChange: champSelectManager.handleLobbyChange,
    handleAutoFunctionToggle: autoFunctionManager.handleAutoFunctionToggle,
    handleDisconnection: disconnectionHandler.handleDisconnection,

    // 对局历史相关（保留为兼容性）
    loadMatchHistory: async () => {
      if (!connectionStore.isConnected) {
        activityStore.addActivity('warning', '请先连接到League客户端')
        return false
      }

      activityStore.addActivity('info', '正在获取对局历史...')
      try {
        const statistics = await matchStatisticsStore.fetchMatchHistory()
        activityStore.addActivity('success', `成功获取 ${statistics.total_games} 场对局数据`)
        return true
      } catch (error) {
        activityStore.addActivity('error', `获取对局历史失败: ${error}`)
        return false
      }
    }
  }
}
