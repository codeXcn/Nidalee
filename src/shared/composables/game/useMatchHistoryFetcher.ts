import { invoke } from '@tauri-apps/api/core'

/**
 * 战绩获取组合式函数
 * 职责：管理战绩数据的获取、过滤和缓存
 */
export function useMatchHistoryFetcher() {
  // 状态管理
  const summonerStats = ref<any[]>([])
  const theirTeamStats = ref<any[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // 缓存管理
  const matchHistoryCache = new Map<string, { data: any[]; timestamp: number }>()
  const CACHE_DURATION = 5 * 60 * 1000 // 5分钟缓存

  /**
   * 获取队伍战绩数据
   * @param myTeam 我方队伍
   * @param enemyTeam 敌方队伍
   * @param matchCount 获取战绩数量
   */
  type PlayerQueryInput = { summonerName: string; isBot: boolean; index: number }

  const fetchTeamMatchHistory = async (
    myTeam: PlayerQueryInput[],
    enemyTeam: PlayerQueryInput[],
    matchCount: number = 10
  ) => {
    isLoading.value = true
    error.value = null

    try {
      // 并行获取双方战绩
      const [myTeamResult, enemyTeamResult] = await Promise.allSettled([
        fetchMatchHistoryForTeam(myTeam, matchCount, 'myTeam'),
        fetchMatchHistoryForTeam(enemyTeam, matchCount, 'enemyTeam')
      ])

      // 处理我方战绩
      if (myTeamResult.status === 'fulfilled') {
        summonerStats.value = myTeamResult.value
      } else {
        console.error('[MatchHistory] 获取我方战绩失败:', myTeamResult.reason)
        summonerStats.value = []
      }

      // 处理敌方战绩
      if (enemyTeamResult.status === 'fulfilled') {
        theirTeamStats.value = enemyTeamResult.value
      } else {
        console.error('[MatchHistory] 获取敌方战绩失败:', enemyTeamResult.reason)
        theirTeamStats.value = []
      }
    } catch (err) {
      console.error('[MatchHistory] 获取战绩失败:', err)
      error.value = err instanceof Error ? err.message : '获取战绩失败'
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 获取单个队伍的战绩
   */
  const fetchMatchHistoryForTeam = async (
    team: PlayerQueryInput[],
    matchCount: number,
    teamType: 'myTeam' | 'enemyTeam'
  ) => {
    // 过滤出真人玩家（可查询）
    const realPlayers = team.filter((p) => !p.isBot && !!p.summonerName)

    // 预先构造与 team 同长度的结果数组，占位为 null，保证与 UI 索引对齐
    const resultByIndex: Array<any | null> = Array(team.length).fill(null)

    if (realPlayers.length === 0) {
      console.log(`[MatchHistory] ${teamType} 全部为不可查询（机器人/匿名），跳过战绩获取`)
      return resultByIndex
    }

    const playerNames = realPlayers.map((p) => p.summonerName)
    const cacheKey = `${teamType}_${playerNames.slice().sort().join('_')}_${matchCount}`

    // 检查缓存（仅当 realPlayers 完整匹配时）
    const cached = matchHistoryCache.get(cacheKey)
    if (cached && Date.now() - cached.timestamp < CACHE_DURATION) {
      console.log(`[MatchHistory] 使用缓存数据: ${teamType}`)
      const filtered = applyQueueFilter(cached.data)
      // 将缓存数据按 realPlayers 的原始 index 回填
      realPlayers.forEach((p, i) => {
        resultByIndex[p.index] = filtered?.[i] ?? null
      })
      return resultByIndex
    }

    try {
      console.log(`[MatchHistory] 请求 ${teamType} 名单:`, playerNames)
      const response = await invoke<SummonerWithMatches[]>('get_summoners_and_histories', {
        names: playerNames,
        count: matchCount
      })

      // 按 displayName 建立映射，避免顺序不一致
      const byName = new Map<string, SummonerWithMatches>()
      for (const item of response) {
        byName.set(item.displayName, item)
      }

      // 记录未命中的名字，便于排查
      const missings: string[] = []

      // 逐人填充到对应索引位
      for (const p of realPlayers) {
        const hit = byName.get(p.summonerName)
        if (hit) {
          resultByIndex[p.index] = applyQueueFilter([hit.matches])?.[0] ?? hit.matches
        } else {
          missings.push(p.summonerName)
          resultByIndex[p.index] = null
        }
      }

      // 更新缓存（仅缓存按请求顺序的纯数据数组，便于复用）
      const cachedData = realPlayers.map((p) => byName.get(p.summonerName)?.matches).filter(Boolean) as any[]
      matchHistoryCache.set(cacheKey, { data: cachedData, timestamp: Date.now() })

      console.log(
        `[MatchHistory] ${teamType} 战绩获取成功: 命中 ${realPlayers.length - missings.length}/${realPlayers.length}`
      )
      if (missings.length) console.warn(`[MatchHistory] ${teamType} 未命中:`, missings)

      return resultByIndex
    } catch (err) {
      console.error(`[MatchHistory] 获取 ${teamType} 战绩失败:`, err)
      return resultByIndex // 返回占位，避免 UI 崩溃
    }
  }

  /**
   * 应用队列类型过滤
   */
  const applyQueueFilter = (rawStats: any[]) => rawStats

  /**
   * 清除缓存
   */
  const clearCache = () => {
    matchHistoryCache.clear()
    console.log('[MatchHistory] 缓存已清除')
  }

  /**
   * 清除过期缓存
   */
  const clearExpiredCache = () => {
    const now = Date.now()
    for (const [key, value] of matchHistoryCache.entries()) {
      if (now - value.timestamp > CACHE_DURATION) {
        matchHistoryCache.delete(key)
      }
    }
  }

  // 计算属性
  const hasData = computed(() => summonerStats.value.length > 0 || theirTeamStats.value.length > 0)
  const cacheSize = computed(() => matchHistoryCache.size)

  return {
    // 状态
    summonerStats,
    theirTeamStats,
    isLoading,
    error,
    hasData,
    cacheSize,

    // 方法
    fetchTeamMatchHistory,
    clearCache,
    clearExpiredCache
  }
}
