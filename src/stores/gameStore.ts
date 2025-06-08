import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { now } from '@vueuse/core'

// 定义接口类型
export interface GameSession {
  startTime: number
  duration: number
}

export interface Activity {
  id: string
  type: 'success' | 'info' | 'warning' | 'error'
  message: string
  timestamp: number
}

export interface GameStatus {
  phase: string
  queue: string | null
  isInGame: boolean
}

export interface MatchStatistics {
  total_games: number
  wins: number
  losses: number
  win_rate: number
  avg_kills: number
  avg_deaths: number
  avg_assists: number
  avg_kda: number
  favorite_champions: ChampionStats[]
  recent_performance: RecentGame[]
}

export interface ChampionStats {
  champion_name: string
  games_played: number
  wins: number
  win_rate: number
}

export interface RecentGame {
  game_id: number
  champion_name: string
  game_mode: string
  win: boolean
  kills: number
  deaths: number
  assists: number
  game_duration: number
  game_creation: number
}

export interface LcuAuthInfo {
  app_port: number
  remoting_auth_token: string
}

export interface SummonerInfo {
  // 基本信息
  displayName: string
  gameName?: string
  tagLine?: string
  summonerLevel: number
  profileIconId: number
  puuid: string
  accountId: number
  summonerId: number

  // 经验信息
  xpSinceLastLevel: number
  xpUntilNextLevel: number
  percentCompleteForNextLevel?: number

  // 游戏状态
  gameStatus?: string
  availability?: string

  // 挑战系统
  challengePoints?: string
  challengeCrystalLevel?: string

  // 排位信息 - 单人排位
  soloRankTier?: string
  soloRankDivision?: string
  soloRankWins?: number
  soloRankLosses?: number
  soloRankLP?: number

  // 排位信息 - 灵活排位
  flexRankTier?: string
  flexRankDivision?: string
  flexRankWins?: number
  flexRankLosses?: number
  flexRankLP?: number

  // 历史最高排位
  highestRankThisSeason?: string

  // 天赋信息
  currentPerkPage?: string
  primaryStyleId?: number
  subStyleId?: number
  selectedPerkIds?: number[]
}

// 游戏阶段信息
export interface GamePhase {
  phase: string
}

// 房间信息
export interface LobbyInfo {
  id: string
  partyType: string
  members: LobbyMember[]
}

export interface LobbyMember {
  summonerId: number
  displayName: string
}

export const useGameStore = defineStore(
  'game',
  () => {
    // 基础状态
    const isConnected = ref(false)
    const isConnecting = ref(false)
    const connectionError = ref<string | null>(null)
    const authInfo = ref<LcuAuthInfo | null>(null)

    // 用户信息
    const summonerInfo = ref<SummonerInfo | null>(null)

    // 游戏状态
    const gameStatus = ref<GameStatus>({
      phase: 'None',
      queue: null,
      isInGame: false
    })

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
      acceptMatch: true,
      selectChampion: false,
      runeConfig: true,
      banChampion: false
    })

    // 对局历史统计
    const matchStatistics = ref<MatchStatistics | null>(null)
    const matchHistoryLoading = ref(false)

    // 计算属性
    const connectionStatus = computed(() => {
      if (isConnecting.value) return 'connecting'
      if (isConnected.value) return 'connected'
      return 'disconnected'
    })

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

    // 检查LCU连接
    const checkConnection = async () => {
      try {
        const connected = await invoke<boolean>('check_lcu_connection')

        if (connected && !isConnected.value) {
          isConnected.value = true
          addActivity('success', 'LCU连接已建立')

          // 获取召唤师信息
          await fetchSummonerInfo()
        } else if (!connected && isConnected.value) {
          isConnected.value = false
          summonerInfo.value = null
          addActivity('warning', 'LCU连接已断开')
        }
      } catch (error) {
        console.error('检查连接失败:', error)
        if (isConnected.value) {
          isConnected.value = false
          summonerInfo.value = null
          addActivity('error', '连接检查失败')
        }
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

    // 尝试连接
    const connect = async () => {
      if (isConnecting.value) return

      isConnecting.value = true
      connectionError.value = null
      addActivity('info', '正在尝试连接LCU...')

      try {
        await invoke('connect_to_lcu')
        await checkConnection()
      } catch (error) {
        connectionError.value = error as string
        addActivity('error', `连接失败: ${error}`)
      } finally {
        isConnecting.value = false
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
    const setConnectionStatus = (status: boolean) => {
      isConnected.value = status
      if (status) {
        addActivity('success', 'LCU 连接已建立')
      } else {
        addActivity('warning', 'LCU 连接已断开')
        clearSummonerInfo()
      }
    }

    // 设置认证信息
    const setAuthInfo = (info: LcuAuthInfo) => {
      authInfo.value = info
      addActivity('success', 'LCU 认证信息已更新')
    }

    // 清除认证信息
    const clearAuthInfo = () => {
      authInfo.value = null
      addActivity('info', 'LCU 认证信息已清除')
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

    return {
      // 状态
      isConnected,
      isConnecting,
      connectionError,
      authInfo,
      summonerInfo,
      gameStatus,
      session,
      todayMatches,
      activities,
      autoFunctions,
      matchStatistics,
      matchHistoryLoading,

      // 计算属性
      connectionStatus,
      winRate,
      enabledFunctionsCount,
      sessionDuration,

      // 方法
      addActivity,
      checkConnection,
      fetchSummonerInfo,
      connect,
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
      updateMatchStatistics
    }
  },
  {
    persist: true
  }
)
