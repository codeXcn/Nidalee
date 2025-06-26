/**
 * 重构后的游戏 Store（兼容层）
 * 该文件作为过渡版本，内部委托给各个专门的 store
 * 保持对外接口兼容，便于渐进式迁移
 */
import { useMatchmaking } from '@/composables'
import {
  useConnectionStore,
  useSummonerStore,
  useGameStatusStore,
  useActivityStore,
  useAutoFunctionStore,
  useMatchStatisticsStore,
  useAppSessionStore
} from '@/stores'

export const useGameStoreV2 = defineStore('gameV2', () => {
  // 获取各个专门的 store
  const connectionStore = useConnectionStore()
  const summonerStore = useSummonerStore()
  const gameStatusStore = useGameStatusStore()
  const activityStore = useActivityStore()
  const autoFunctionStore = useAutoFunctionStore()
  const matchStatisticsStore = useMatchStatisticsStore()
  const appSessionStore = useAppSessionStore()

  // === 兼容性接口：直接暴露各个 store 的状态 ===

  // 基础状态
  const isConnected = computed(() => connectionStore.isConnected)
  const authInfo = computed(() => connectionStore.authInfo)

  // 用户信息
  const summonerInfo = computed(() => summonerStore.summonerInfo)

  // 游戏状态
  const gameStatus = computed(() => gameStatusStore.gameStatus)
  const currentChampSelectSession = computed(() => gameStatusStore.currentChampSelectSession)

  // 会话信息
  const session = computed(() => appSessionStore.session)
  const gameVersion = computed(() => appSessionStore.gameVersion)

  // 今日对局统计
  const todayMatches = computed(() => matchStatisticsStore.todayMatches)

  // 活动记录
  const activities = computed(() => activityStore.activities)

  // 自动功能状态
  const autoFunctions = computed(() => autoFunctionStore.autoFunctions)

  // 对局历史统计
  const matchStatistics = computed(() => matchStatisticsStore.matchStatistics)
  const matchHistoryLoading = computed(() => matchStatisticsStore.matchHistoryLoading)

  // === 兼容性接口：计算属性 ===
  const winRate = computed(() => matchStatisticsStore.winRate)
  const enabledFunctionsCount = computed(() => autoFunctionStore.enabledFunctionsCount)
  const sessionDuration = computed(() => appSessionStore.sessionDuration)

  // === 兼容性接口：方法（委托给对应的 store） ===

  // 设置认证信息
  const setAuthInfo = async (info: LcuAuthInfo | null) => {
    connectionStore.setAuthInfo(info)
    if (info) {
      activityStore.addActivity('success', 'LCU 认证信息已更新')
      await summonerStore.fetchSummonerInfo()
    } else {
      activityStore.addActivity('warning', 'LCU 认证信息已清除')
    }
  }

  // 检查LCU连接
  const checkConnection = async () => {
    try {
      const connected = await connectionStore.checkConnection()
      if (connected) {
        await summonerStore.fetchSummonerInfo()
      }
    } catch (error) {
      console.error('检查连接失败:', error)
      activityStore.addActivity('error', '连接检查失败')
    }
  }

  // 获取召唤师信息
  const fetchSummonerInfo = async () => {
    try {
      await summonerStore.fetchSummonerInfo()
      activityStore.addActivity('success', `欢迎回来，${summonerStore.displayName}！`)
    } catch (error) {
      console.error('获取召唤师信息失败:', error)
      activityStore.addActivity('error', '获取召唤师信息失败')
    }
  }

  // 更新会话时长
  const updateSessionDuration = () => {
    appSessionStore.updateSessionDuration()
  }

  // 重置会话
  const resetSession = () => {
    appSessionStore.resetSession()
  }

  // 切换自动功能
  const toggleAutoFunction = (key: keyof ReturnType<typeof autoFunctionStore>['autoFunctions']) => {
    const result = autoFunctionStore.toggleAutoFunction(key)
    const status = result.enabled ? '已启用' : '已禁用'
    activityStore.addActivity('info', `自动${result.name}${status}`)
  }

  // 获取功能名称
  const getFunctionName = (key: keyof ReturnType<typeof autoFunctionStore>['autoFunctions']): string => {
    return autoFunctionStore.getFunctionName(key)
  }

  // 获取对局历史统计
  const fetchMatchHistory = async () => {
    if (!connectionStore.isConnected) {
      activityStore.addActivity('warning', '请先连接到League客户端')
      return
    }

    activityStore.addActivity('info', '正在获取对局历史...')
    try {
      const statistics = await matchStatisticsStore.fetchMatchHistory()
      activityStore.addActivity('success', `成功获取 ${statistics.total_games} 场对局数据`)
    } catch (error) {
      console.error('获取对局历史失败:', error)
      activityStore.addActivity('error', `获取对局历史失败: ${error}`)
    }
  }

  // 模拟对局结果（用于测试）
  const simulateMatchResult = (won: boolean) => {
    matchStatisticsStore.simulateMatchResult(won)
    if (won) {
      activityStore.addActivity('success', '对局胜利！')
    } else {
      activityStore.addActivity('info', '对局结束')
    }
  }

  // 更新召唤师信息
  const updateSummonerInfo = (info: SummonerInfo) => {
    summonerStore.updateSummonerInfo(info)
    activityStore.addActivity('success', `召唤师信息已更新: ${info.displayName}`)
  }

  // 清除召唤师信息
  const clearSummonerInfo = () => {
    summonerStore.clearSummonerInfo()
    activityStore.addActivity('info', '召唤师信息已清除')
  }

  // 设置连接状态
  const setConnectionStatus = async (status: boolean) => {
    connectionStore.setConnectionStatus(status)
    if (status) {
      activityStore.addActivity('success', 'LCU 连接已建立')
      await summonerStore.fetchSummonerInfo()
    } else {
      activityStore.addActivity('warning', 'LCU 连接已断开')
      summonerStore.clearSummonerInfo()
    }
  }

  // 更新游戏阶段
  const updateGamePhase = (phase: GamePhase | null) => {
    gameStatusStore.updateGamePhase(phase)
    if (phase) {
      activityStore.addActivity('info', `游戏阶段变更: ${phase.phase}`)
    } else {
      activityStore.addActivity('info', '游戏阶段重置')
    }
  }

  // 更新房间信息
  const updateLobbyInfo = (lobby: LobbyInfo | null) => {
    gameStatusStore.updateLobbyInfo(lobby)
    if (lobby) {
      activityStore.addActivity('info', `进入房间: ${lobby.partyType} (${lobby.members.length}人)`)
    } else {
      activityStore.addActivity('info', '离开房间')
    }
  }

  // 更新对局历史
  const updateMatchStatistics = (stats: MatchStatistics) => {
    matchStatisticsStore.updateMatchStatistics(stats)
    activityStore.addActivity('success', `更新对局历史: ${stats.total_games} 场对局`)
  }

  // 更新选人阶段信息
  const updateChampSelectSession = async (session: ChampSelectSession | null) => {
    await gameStatusStore.updateChampSelectSession(session)
    if (session) {
      activityStore.addActivity('info', '进入英雄选择阶段')
    } else {
      activityStore.addActivity('info', '离开英雄选择阶段')
    }
  }

  // 清除认证信息
  const clearAuthInfo = () => {
    connectionStore.clearAuthInfo()
  }

  // 添加活动记录
  const addActivity = (type: Activity['type'], message: string) => {
    activityStore.addActivity(type, message)
  }

  // 设置游戏版本
  const setGameVersion = (version: string) => {
    appSessionStore.setGameVersion(version)
  }

  // 获取游戏版本
  const getGameVersion = computed(() => appSessionStore.gameVersion)

  // Matchmaking 相关
  const { matchmakingState, matchInfo, handleMatchmaking, handleAcceptMatch, handleDeclineMatch } = useMatchmaking()

  // 自动接受对局监听
  watch(
    () => matchmakingState.value?.searchState,
    (val) => {
      console.log(val, autoFunctionStore.autoFunctions.acceptMatch)
      val === 'Found' && autoFunctionStore.autoFunctions.acceptMatch && handleAcceptMatch()
    }
  )

  // 兼容性：暴露 teamsStates
  const teamsStates = computed(() => gameStatusStore.teamsStates)

  return {
    // === 兼容性状态 ===
    isConnected,
    authInfo,
    summonerInfo,
    gameStatus,
    currentChampSelectSession,
    session,
    todayMatches,
    activities,
    autoFunctions,
    matchStatistics,
    matchHistoryLoading,
    matchmakingState,
    matchInfo,
    teamsStates,

    // === 兼容性计算属性 ===
    winRate,
    enabledFunctionsCount,
    sessionDuration,
    setGameVersion,
    getGameVersion,

    // === 兼容性方法 ===
    handleMatchmaking,
    handleAcceptMatch,
    handleDeclineMatch,
    addActivity,
    checkConnection,
    fetchSummonerInfo,
    updateSessionDuration,
    resetSession,
    toggleAutoFunction,
    getFunctionName,
    fetchMatchHistory,
    simulateMatchResult,
    updateSummonerInfo,
    clearSummonerInfo,
    setConnectionStatus,
    setAuthInfo,
    clearAuthInfo,
    updateGamePhase,
    updateLobbyInfo,
    updateChampSelectSession,
    updateMatchStatistics
  }
})
