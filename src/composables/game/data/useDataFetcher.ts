import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useDataStore } from '@/stores/core/dataStore'
import { useActivityStore } from '@/stores'
import { useSettingsStore } from '@/stores/ui/settingsStore'
import { useErrorHandler } from '@/composables/utils/useErrorHandler'

/**
 * 统一数据获取器
 * 职责：统一管理所有数据获取操作，包括缓存、重试、错误处理
 */

// 使用全局类型，添加显示名称的扩展类型
interface EnrichedMatchStatistics extends MatchStatistics {
  displayName: string
}

interface FetchOptions {
  cache?: boolean
  retry?: number
  timeout?: number
  signal?: AbortSignal
}

interface TeamPlayer {
  summonerName: string
  isBot: boolean
  index: number
}

export function useDataFetcher() {
  const dataStore = useDataStore()
  const activityStore = useActivityStore()
  const settingsStore = useSettingsStore()
  const errorHandler = useErrorHandler()

  // === 状态管理 ===
  const loading = ref(false)
  const summonerLoading = ref(false)
  const matchHistoryLoading = ref(false)
  const teamDataLoading = ref(false)

  // 缓存
  const summonerCache = ref<Map<string, SummonerWithMatches>>(new Map())
  const teamStatsCache = ref<{
    summonerStats: EnrichedMatchStatistics[]
    theirTeamStats: EnrichedMatchStatistics[]
  }>({
    summonerStats: [],
    theirTeamStats: []
  })

  // === 辅助函数 ===

  /**
   * 计算玩家统计数据
   */
  const calculatePlayerStats = (matches: MatchStatistics): Omit<MatchStatistics, 'recentPerformance'> => {
    const recentPerformance = matches.recentPerformance || []
    const totalGames = recentPerformance.length
    const wins = recentPerformance.filter((game) => game.win).length
    const losses = totalGames - wins
    const winRate = totalGames > 0 ? (wins / totalGames) * 100 : 0

    // 计算平均KDA
    const totalKills = recentPerformance.reduce((sum, game) => sum + game.kills, 0)
    const totalDeaths = recentPerformance.reduce((sum, game) => sum + game.deaths, 0)
    const totalAssists = recentPerformance.reduce((sum, game) => sum + game.assists, 0)

    const avgKills = totalGames > 0 ? totalKills / totalGames : 0
    const avgDeaths = totalGames > 0 ? totalDeaths / totalGames : 0
    const avgAssists = totalGames > 0 ? totalAssists / totalGames : 0
    const avgKda = avgDeaths > 0 ? (avgKills + avgAssists) / avgDeaths : avgKills + avgAssists

    // 计算英雄统计
    const championStatsMap = new Map<number, { games: number; wins: number }>()
    recentPerformance.forEach((game) => {
      const existing = championStatsMap.get(game.championId) || { games: 0, wins: 0 }
      championStatsMap.set(game.championId, {
        games: existing.games + 1,
        wins: existing.wins + (game.win ? 1 : 0)
      })
    })

    const favoriteChampions = Array.from(championStatsMap.entries())
      .map(([championId, stats]) => ({
        championId,
        gamesPlayed: stats.games,
        wins: stats.wins,
        winRate: stats.games > 0 ? (stats.wins / stats.games) * 100 : 0
      }))
      .sort((a, b) => b.gamesPlayed - a.gamesPlayed)
      .slice(0, 3) // 只保留前3个最常玩的英雄

    return {
      totalGames,
      wins,
      losses,
      winRate,
      avgKills,
      avgDeaths,
      avgAssists,
      avgKda,
      favoriteChampions
    }
  }

  // === 核心数据获取方法 ===

  /**
   * 获取当前召唤师信息
   */
  const fetchCurrentSummoner = async (options: FetchOptions = {}) => {
    if (options.signal?.aborted) return null

    try {
      summonerLoading.value = true
      dataStore.startLoadingSummoner()

      const summonerInfo = await invoke<SummonerInfo>('get_current_summoner')
      if (summonerInfo) {
        dataStore.setSummonerInfo(summonerInfo)
        activityStore.addActivity('info', '召唤师信息已更新', 'data')
        return summonerInfo
      }
      return null
    } catch (error) {
      console.error('[DataFetcher] 获取召唤师信息失败:', error)
      dataStore.clearSummonerInfo()
      if (options.retry && options.retry > 0) {
        // 递归重试
        await new Promise((resolve) => setTimeout(resolve, 1000))
        return fetchCurrentSummoner({ ...options, retry: options.retry - 1 })
      }
      errorHandler.handleError(error instanceof Error ? error : String(error), '获取召唤师信息')
      return null
    } finally {
      summonerLoading.value = false
    }
  }

  /**
   * 获取战绩历史
   */
  const fetchMatchHistory = async (count?: number, options: FetchOptions = {}) => {
    if (options.signal?.aborted) return null

    try {
      matchHistoryLoading.value = true
      dataStore.startLoadingMatchHistory()

      const matchCount = count || settingsStore.defaultMatchCount
      const matchHistory = await invoke<MatchStatistics>('get_match_history', {
        count: matchCount
      })

      if (matchHistory) {
        dataStore.setMatchStatistics(matchHistory)
        activityStore.addActivity('success', '对局历史记录已更新', 'data')
        return matchHistory
      }
      return null
    } catch (error) {
      console.error('[DataFetcher] 获取对局历史失败:', error)
      dataStore.clearMatchHistory()
      activityStore.addActivity('error', '获取对局历史失败', 'error')

      if (options.retry && options.retry > 0) {
        await new Promise((resolve) => setTimeout(resolve, 1000))
        return fetchMatchHistory(count, { ...options, retry: options.retry - 1 })
      }
      errorHandler.handleError(error instanceof Error ? error : String(error), '获取对局历史')
      return null
    } finally {
      matchHistoryLoading.value = false
    }
  }

  /**
   * 根据召唤师名称查询详细信息
   */
  const fetchSummonerByNames = async (
    names: string[],
    options: FetchOptions = {}
  ): Promise<SummonerWithMatches[] | null> => {
    if (options.signal?.aborted) return null

    try {
      loading.value = true

      // 检查缓存
      if (options.cache !== false) {
        const cachedResults: SummonerWithMatches[] = []
        const uncachedNames: string[] = []

        names.forEach((name) => {
          const cached = summonerCache.value.get(name.toLowerCase())
          if (cached) {
            cachedResults.push(cached)
          } else {
            uncachedNames.push(name)
          }
        })

        if (uncachedNames.length === 0) {
          return cachedResults
        }
        names = uncachedNames
      }

      // 获取新数据
      const results = await invoke<SummonerWithMatches[]>('search_summoners_with_matches', {
        summonerNames: names
      })

      if (results && results.length > 0) {
        // 更新缓存
        results.forEach((result) => {
          if (result.summonerInfo) {
            summonerCache.value.set(result.summonerInfo.displayName.toLowerCase(), result)
          }
        })

        return results
      }
      return null
    } catch (error) {
      console.error('[DataFetcher] 查询召唤师失败:', error)

      if (options.retry && options.retry > 0) {
        await new Promise((resolve) => setTimeout(resolve, 1000))
        return fetchSummonerByNames(names, { ...options, retry: options.retry - 1 })
      }
      errorHandler.handleError(error instanceof Error ? error : String(error), '召唤师查询')
      return null
    } finally {
      loading.value = false
    }
  }

  /**
   * 获取团队战绩数据
   */
  const fetchTeamMatchHistory = async (
    myTeamPlayers: TeamPlayer[] = [],
    enemyTeamPlayers: TeamPlayer[] = [],
    count: number = 10,
    options: FetchOptions = {}
  ) => {
    if (options.signal?.aborted) return

    try {
      teamDataLoading.value = true

      const allPlayers = [...myTeamPlayers, ...enemyTeamPlayers]
      const realPlayers = allPlayers.filter((p) => !p.isBot)

      if (realPlayers.length === 0) {
        console.warn('[DataFetcher] 没有真实玩家需要获取战绩')
        return
      }

      const playerNames = realPlayers.map((p) => p.summonerName)
      const results = await fetchSummonerByNames(playerNames, options)

      if (!results) return

      // 处理我方队伍统计
      const myTeamStats: EnrichedMatchStatistics[] = []
      myTeamPlayers.forEach((player) => {
        const result = results.find(
          (r) => r.summonerInfo?.displayName.toLowerCase() === player.summonerName.toLowerCase()
        )

        if (result?.matches) {
          // 计算统计数据
          const matches = result.matches
          const stats = calculatePlayerStats(matches)
          myTeamStats[player.index] = {
            displayName: result.summonerInfo!.displayName,
            ...stats,
            recentPerformance: matches.recentPerformance || []
          }
        }
      })

      // 处理敌方队伍统计
      const enemyTeamStats: EnrichedMatchStatistics[] = []
      enemyTeamPlayers.forEach((player) => {
        const result = results.find(
          (r) => r.summonerInfo?.displayName.toLowerCase() === player.summonerName.toLowerCase()
        )

        if (result?.matches) {
          // 计算统计数据
          const matches = result.matches
          const stats = calculatePlayerStats(matches)
          enemyTeamStats[player.index] = {
            displayName: result.summonerInfo!.displayName,
            ...stats,
            recentPerformance: matches.recentPerformance || []
          }
        }
      })

      // 更新缓存
      teamStatsCache.value = {
        summonerStats: myTeamStats,
        theirTeamStats: enemyTeamStats
      }

      console.log('[DataFetcher] 团队战绩获取完成', {
        myTeam: myTeamStats.length,
        enemyTeam: enemyTeamStats.length
      })
    } catch (error) {
      console.error('[DataFetcher] 获取团队战绩失败:', error)

      if (options.retry && options.retry > 0) {
        await new Promise((resolve) => setTimeout(resolve, 1000))
        return fetchTeamMatchHistory(myTeamPlayers, enemyTeamPlayers, count, { ...options, retry: options.retry - 1 })
      }
      errorHandler.handleError(error instanceof Error ? error : String(error), '团队战绩获取')
    } finally {
      teamDataLoading.value = false
    }
  }

  /**
   * 同时更新召唤师信息和战绩
   */
  const fetchCurrentSummonerWithHistory = async (options: FetchOptions = {}) => {
    const [summonerInfo, matchHistory] = await Promise.all([
      fetchCurrentSummoner(options),
      fetchMatchHistory(undefined, options)
    ])

    return {
      summonerInfo,
      matchHistory
    }
  }

  // === 缓存管理 ===
  const clearCache = () => {
    summonerCache.value.clear()
    teamStatsCache.value = {
      summonerStats: [],
      theirTeamStats: []
    }
  }

  const clearSummonerCache = (name?: string) => {
    if (name) {
      summonerCache.value.delete(name.toLowerCase())
    } else {
      summonerCache.value.clear()
    }
  }

  // === 计算属性 ===
  const isLoading = computed(
    () => loading.value || summonerLoading.value || matchHistoryLoading.value || teamDataLoading.value
  )

  const cacheSize = computed(() => summonerCache.value.size)

  return {
    // 状态
    loading,
    summonerLoading,
    matchHistoryLoading,
    teamDataLoading,
    isLoading,

    // 数据
    summonerStats: computed(() => teamStatsCache.value.summonerStats),
    theirTeamStats: computed(() => teamStatsCache.value.theirTeamStats),

    // 核心方法
    fetchCurrentSummoner,
    fetchMatchHistory,
    fetchSummonerByNames,
    fetchTeamMatchHistory,
    fetchCurrentSummonerWithHistory,

    // 缓存管理
    clearCache,
    clearSummonerCache,
    cacheSize,

    // 调试信息
    debugInfo: computed(() => ({
      cacheSize: cacheSize.value,
      isLoading: isLoading.value,
      loadingStates: {
        summoner: summonerLoading.value,
        matchHistory: matchHistoryLoading.value,
        teamData: teamDataLoading.value
      }
    }))
  }
}
