import { useMatchmaking } from '@/components/game-helper/composables/useMatchmaking'
import { invoke } from '@tauri-apps/api/core'
import { now } from '@vueuse/core'

export const useGameStore = defineStore(
  'game',
  () => {
    const gameVersion = ref('15.12.1')
    // 基础状态
    const isConnected = ref(false)
    const authInfo = ref<LcuAuthInfo | null>(null)
    // 用户信息
    const summonerInfo = ref<SummonerInfo | null>(null)
    // 游戏状态
    const gameStatus = ref<GameStatus>({
      phase: 'None',
      queue: null,
      isInGame: false
    })
    // 选人阶段信息
    const currentChampSelectSession = ref<ChampSelectSession | null>(null)

    // 会话信息
    const session = ref<GameSession>({
      startTime: Date.now(),
      duration: 0
    })

    // 今日对局统计
    const todayMatches = ref({
      total: 0,
      wins: 0,
      losses: 0
    })

    // 活动记录
    const activities = ref<Activity[]>([
      {
        id: '1',
        type: 'info',
        message: '应用启动成功',
        timestamp: now()
      }
    ])

    // 自动功能状态
    const autoFunctions = ref({
      acceptMatch: false,
      selectChampion: false,
      runeConfig: false,
      banChampion: false
    })

    // 对局历史统计
    const matchStatistics = ref<MatchStatistics | null>(null)
    const matchHistoryLoading = ref(false)
    const winRate = computed(() => {
      const total = todayMatches.value.total
      if (total === 0) return 0
      return Math.round((todayMatches.value.wins / total) * 100)
    })

    const enabledFunctionsCount = computed(() => {
      return Object.values(autoFunctions.value).filter(Boolean).length
    })

    const sessionDuration = computed(() => {
      const duration = session.value.duration || 0
      const minutes = Math.floor(duration / 60000)
      const hours = Math.floor(minutes / 60)

      if (hours > 0) {
        return `${hours}h ${minutes % 60}m`
      }
      return `${minutes}m`
    })

    // 添加活动记录
    const addActivity = (type: Activity['type'], message: string) => {
      const activity: Activity = {
        id: Date.now().toString(),
        type,
        message,
        timestamp: now()
      }

      activities.value.unshift(activity)

      // 限制活动记录数量
      if (activities.value.length > 50) {
        activities.value = activities.value.slice(0, 50)
      }
    }

    // 设置认证信息
    const setAuthInfo = async (info: LcuAuthInfo | null) => {
      if (info) {
        isConnected.value = true
        authInfo.value = info
        addActivity('success', 'LCU 认证信息已更新')
        await fetchSummonerInfo()
      } else {
        isConnected.value = false
        authInfo.value = null
        summonerInfo.value = null
        addActivity('warning', 'LCU 认证信息已清除')
      }
    }

    // 检查LCU连接
    const checkConnection = async () => {
      try {
        const connected = await invoke<LcuAuthInfo>('get_auth_info')
        setAuthInfo(connected)
        await fetchSummonerInfo()
      } catch (error) {
        console.error('检查连接失败:', error)
        setAuthInfo(null)
        addActivity('error', '连接检查失败')
      }
    }

    // 获取召唤师信息
    const fetchSummonerInfo = async () => {
      try {
        const info = await invoke<SummonerInfo>('get_current_summoner')
        summonerInfo.value = info
        addActivity('success', `欢迎回来，${info.displayName}！`)
      } catch (error) {
        console.error('获取召唤师信息失败:', error)
        addActivity('error', '获取召唤师信息失败')
      }
    }

    // 更新会话时长
    const updateSessionDuration = () => {
      const currentTime = Date.now()
      const startTime = session.value.startTime || currentTime
      session.value.duration = currentTime - startTime
    }

    // 重置会话
    const resetSession = () => {
      const currentTime = Date.now()
      session.value = {
        startTime: currentTime,
        duration: 0
      }
    }

    // 切换自动功能
    const toggleAutoFunction = (key: keyof typeof autoFunctions.value) => {
      autoFunctions.value[key] = !autoFunctions.value[key]
      console.log(autoFunctions.value)
      const status = autoFunctions.value[key] ? '已启用' : '已禁用'
      addActivity('info', `自动${getFunctionName(key)}${status}`)
    }

    // 获取功能名称
    const getFunctionName = (key: keyof typeof autoFunctions.value): string => {
      const names = {
        acceptMatch: '接受对局',
        selectChampion: '选择英雄',
        runeConfig: '符文配置',
        banChampion: '禁用英雄'
      }
      return names[key]
    }

    // 获取对局历史统计
    const fetchMatchHistory = async () => {
      if (!isConnected.value) {
        addActivity('warning', '请先连接到League客户端')
        return
      }

      matchHistoryLoading.value = true
      addActivity('info', '正在获取对局历史...')

      try {
        const statistics = await invoke<MatchStatistics>('get_match_history')
        matchStatistics.value = statistics
        addActivity('success', `成功获取 ${statistics.total_games} 场对局数据`)
      } catch (error) {
        console.error('获取对局历史失败:', error)
        addActivity('error', `获取对局历史失败: ${error}`)
      } finally {
        matchHistoryLoading.value = false
      }
    }

    // 模拟对局结果（用于测试）
    const simulateMatchResult = (won: boolean) => {
      todayMatches.value.total++
      if (won) {
        todayMatches.value.wins++
        addActivity('success', '对局胜利！')
      } else {
        todayMatches.value.losses++
        addActivity('info', '对局结束')
      }
    }

    // 更新召唤师信息
    const updateSummonerInfo = (info: SummonerInfo) => {
      summonerInfo.value = info
      addActivity('success', `召唤师信息已更新: ${info.displayName}`)
    }

    // 清除召唤师信息
    const clearSummonerInfo = () => {
      summonerInfo.value = null
      addActivity('info', '召唤师信息已清除')
    }

    // 设置连接状态
    const setConnectionStatus = async (status: boolean) => {
      isConnected.value = status
      if (status) {
        addActivity('success', 'LCU 连接已建立')
        await fetchSummonerInfo()
      } else {
        addActivity('warning', 'LCU 连接已断开')
        clearSummonerInfo()
      }
    }

    // 更新游戏阶段
    const updateGamePhase = (phase: GamePhase | null) => {
      if (phase) {
        gameStatus.value.phase = phase.phase
        gameStatus.value.isInGame = phase.phase === 'InProgress'
        addActivity('info', `游戏阶段变更: ${phase.phase}`)
      } else {
        gameStatus.value.phase = 'None'
        gameStatus.value.isInGame = false
        addActivity('info', '游戏阶段重置')
      }
    }

    // 更新房间信息
    const updateLobbyInfo = (lobby: LobbyInfo | null) => {
      if (lobby) {
        addActivity('info', `进入房间: ${lobby.partyType} (${lobby.members.length}人)`)
      } else {
        addActivity('info', '离开房间')
      }
    }

    // 更新对局历史
    const updateMatchStatistics = (stats: MatchStatistics) => {
      console.log('[Store] 更新对局历史:', stats)
      if (!stats) {
        console.warn('[Store] 对局历史数据为空')
        return
      }

      // 验证数据完整性
      if (
        typeof stats.total_games !== 'number' ||
        typeof stats.wins !== 'number' ||
        typeof stats.losses !== 'number' ||
        typeof stats.win_rate !== 'number' ||
        typeof stats.avg_kills !== 'number' ||
        typeof stats.avg_deaths !== 'number' ||
        typeof stats.avg_assists !== 'number' ||
        typeof stats.avg_kda !== 'number' ||
        !Array.isArray(stats.favorite_champions) ||
        !Array.isArray(stats.recent_performance)
      ) {
        console.error('[Store] 对局历史数据格式错误:', stats)
        return
      }

      matchStatistics.value = {
        total_games: stats.total_games,
        wins: stats.wins,
        losses: stats.losses,
        win_rate: stats.win_rate,
        avg_kills: stats.avg_kills,
        avg_deaths: stats.avg_deaths,
        avg_assists: stats.avg_assists,
        avg_kda: stats.avg_kda,
        favorite_champions: [...stats.favorite_champions],
        recent_performance: [...stats.recent_performance]
      }
      addActivity('success', `更新对局历史: ${stats.total_games} 场对局`)
    }
    const teamsStates = ref<any>(null)
    // 更新选人阶段信息
    const updateChampSelectSession = async (session: ChampSelectSession | null) => {
      currentChampSelectSession.value = session
      if (session) {
        addActivity('info', '进入英雄选择阶段')
        if (teamsStates.value) return
        const states = await invoke<any>('get_champselect_team_players_info')
        console.log(states)
        teamsStates.value = states
      } else {
        addActivity('info', '离开英雄选择阶段')
        teamsStates.value = null
      }
    }
    // 清除认证信息
    const clearAuthInfo = () => {
      setAuthInfo(null)
    }
    const { matchmakingState, matchInfo, handleMatchmaking, handleAcceptMatch, handleDeclineMatch } = useMatchmaking()

    watch(
      () => matchmakingState.value?.searchState,
      (val) => {
        console.log(val, autoFunctions.value.acceptMatch)
        val === 'Found' && autoFunctions.value.acceptMatch && handleAcceptMatch()
      }
    )
    const setGameVersion = (version: string) => {
      gameVersion.value = version
    }
    const getGameVersion = computed(() => {
      return gameVersion.value
    })
    return {
      setGameVersion,
      getGameVersion,
      // 状态
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
      // 计算属性
      winRate,
      enabledFunctionsCount,
      sessionDuration,

      // 方法
      handleMatchmaking,
      handleAcceptMatch,
      handleDeclineMatch,
      addActivity,
      checkConnection,
      fetchSummonerInfo,
      updateSessionDuration,
      resetSession,
      toggleAutoFunction,
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
  },
  {
    persist: true
  }
)
