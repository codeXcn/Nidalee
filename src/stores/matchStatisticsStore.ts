import { invoke } from '@tauri-apps/api/core'

export const useMatchStatisticsStore = defineStore(
  'matchStatistics',
  () => {
    // 对局历史统计
    const matchStatistics = ref<MatchStatistics | null>(null)
    const matchHistoryLoading = ref(false)
    const matchHistoryError = ref<string | null>(null)

    // 今日对局统计
    const todayMatches = ref({
      total: 0,
      wins: 0,
      losses: 0
    })

    // 获取对局历史统计
    const fetchMatchHistory = async () => {
      matchHistoryLoading.value = true
      matchHistoryError.value = null

      try {
        const statistics = await invoke<MatchStatistics>('get_match_history')
        updateMatchStatistics(statistics)
        return statistics
      } catch (error) {
        console.error('获取对局历史失败:', error)
        matchHistoryError.value = error instanceof Error ? error.message : '获取对局历史失败'
        throw error
      } finally {
        matchHistoryLoading.value = false
      }
    }

    // 更新对局历史
    const updateMatchStatistics = (stats: MatchStatistics) => {
      if (!stats) {
        console.warn('对局历史数据为空')
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
        console.error('对局历史数据格式错误:', stats)
        matchHistoryError.value = '对局历史数据格式错误'
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
      matchHistoryError.value = null
    }

    // 模拟对局结果（用于测试）
    const simulateMatchResult = (won: boolean) => {
      todayMatches.value.total++
      if (won) {
        todayMatches.value.wins++
      } else {
        todayMatches.value.losses++
      }
    }

    // 重置今日统计
    const resetTodayMatches = () => {
      todayMatches.value = {
        total: 0,
        wins: 0,
        losses: 0
      }
    }

    // 计算属性
    const winRate = computed(() => {
      const total = todayMatches.value.total
      if (total === 0) return 0
      return Math.round((todayMatches.value.wins / total) * 100)
    })

    const totalWinRate = computed(() => {
      return matchStatistics.value?.win_rate || 0
    })

    const averageKDA = computed(() => {
      return matchStatistics.value?.avg_kda || 0
    })

    const favoriteChampions = computed(() => {
      return matchStatistics.value?.favorite_champions || []
    })

    const recentPerformance = computed(() => {
      return matchStatistics.value?.recent_performance || []
    })

    return {
      // 状态
      matchStatistics: readonly(matchStatistics),
      matchHistoryLoading: readonly(matchHistoryLoading),
      matchHistoryError: readonly(matchHistoryError),
      todayMatches,

      // 计算属性
      winRate,
      totalWinRate,
      averageKDA,
      favoriteChampions,
      recentPerformance,

      // 方法
      fetchMatchHistory,
      updateMatchStatistics,
      simulateMatchResult,
      resetTodayMatches
    }
  },
  {
    persist: {
      paths: ['todayMatches']
    }
  }
)
