import {
  useConnectionStore,
  useSummonerStore,
  useGameStatusStore,
  useActivityStore,
  useAutoFunctionStore,
  useMatchStatisticsStore,
  useAppSessionStore
} from '@/stores'

export function useGameManagement() {
  // 各个 store 实例
  const connectionStore = useConnectionStore()
  const summonerStore = useSummonerStore()
  const gameStatusStore = useGameStatusStore()
  const activityStore = useActivityStore()
  const autoFunctionStore = useAutoFunctionStore()
  const matchStatisticsStore = useMatchStatisticsStore()
  const appSessionStore = useAppSessionStore()

  // 统一的连接管理
  const initializeConnection = async () => {
    try {
      const connected = await connectionStore.checkConnection()
      if (connected) {
        activityStore.addActivity('success', 'LCU 连接已建立')
        await summonerStore.fetchSummonerInfo()
        activityStore.addActivity('success', `欢迎回来，${summonerStore.displayName}！`)
      }
      return connected
    } catch (error) {
      console.error('连接初始化失败:', error)
      activityStore.addActivity('error', '连接初始化失败')
      return false
    }
  }

  // 断开连接时的清理
  const handleDisconnection = () => {
    connectionStore.clearAuthInfo()
    summonerStore.clearSummonerInfo()
    gameStatusStore.clearGameState()
    activityStore.addActivity('warning', 'LCU 连接已断开')
  }

  // 自动功能变更处理
  const handleAutoFunctionToggle = (key: keyof ReturnType<typeof autoFunctionStore>['autoFunctions']) => {
    const result = autoFunctionStore.toggleAutoFunction(key)
    const status = result.enabled ? '已启用' : '已禁用'
    activityStore.addActivity('info', `自动${result.name}${status}`)
    return result
  }

  // 游戏阶段变更处理
  const handleGamePhaseChange = (phase: GamePhase | null) => {
    gameStatusStore.updateGamePhase(phase)
    if (phase) {
      activityStore.addActivity('info', `游戏阶段变更: ${phase.phase}`)
    } else {
      activityStore.addActivity('info', '游戏阶段重置')
    }
  }

  // 选人阶段变更处理
  const handleChampSelectChange = async (session: ChampSelectSession | null) => {
    await gameStatusStore.updateChampSelectSession(session)
    if (session) {
      activityStore.addActivity('info', '进入英雄选择阶段')
    } else {
      activityStore.addActivity('info', '离开英雄选择阶段')
    }
  }

  // 房间变更处理
  const handleLobbyChange = (lobby: LobbyInfo | null) => {
    gameStatusStore.updateLobbyInfo(lobby)
    if (lobby) {
      activityStore.addActivity('info', `进入房间: ${lobby.partyType} (${lobby.members.length}人)`)
    } else {
      activityStore.addActivity('info', '离开房间')
    }
  }

  // 对局统计更新处理
  const handleMatchStatisticsUpdate = (stats: MatchStatistics) => {
    matchStatisticsStore.updateMatchStatistics(stats)
    activityStore.addActivity('success', `更新对局历史: ${stats.total_games} 场对局`)
  }

  // 获取对局历史
  const loadMatchHistory = async () => {
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

  // 聚合状态（为了兼容性）
  const aggregatedState = computed(() => ({
    // 连接状态
    isConnected: connectionStore.isConnected,
    authInfo: connectionStore.authInfo,

    // 召唤师信息
    summonerInfo: summonerStore.summonerInfo,

    // 游戏状态
    gameStatus: gameStatusStore.gameStatus,
    currentChampSelectSession: gameStatusStore.currentChampSelectSession,
    teamsStates: gameStatusStore.teamsStates,

    // 会话信息
    session: appSessionStore.session,
    gameVersion: appSessionStore.gameVersion,

    // 统计信息
    todayMatches: matchStatisticsStore.todayMatches,
    matchStatistics: matchStatisticsStore.matchStatistics,
    matchHistoryLoading: matchStatisticsStore.matchHistoryLoading,

    // 活动记录
    activities: activityStore.activities,

    // 自动功能
    autoFunctions: autoFunctionStore.autoFunctions
  }))

  // 聚合计算属性
  const aggregatedComputed = computed(() => ({
    winRate: matchStatisticsStore.winRate,
    enabledFunctionsCount: autoFunctionStore.enabledFunctionsCount,
    sessionDuration: appSessionStore.sessionDuration
  }))

  return {
    // 各个 store 实例
    connectionStore,
    summonerStore,
    gameStatusStore,
    activityStore,
    autoFunctionStore,
    matchStatisticsStore,
    appSessionStore,

    // 统一管理方法
    initializeConnection,
    handleDisconnection,
    handleAutoFunctionToggle,
    handleGamePhaseChange,
    handleChampSelectChange,
    handleLobbyChange,
    handleMatchStatisticsUpdate,
    loadMatchHistory,

    // 兼容性聚合状态
    aggregatedState,
    aggregatedComputed
  }
}
