import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { useGameStore } from './gameStore'
import { useConnectionStore } from '../core/connectionStore'
import type { GamePhase, TeamData, EnrichedPlayerMatchStats } from '@/types/match-analysis'

// 类型别名，保持向后兼容
type EnrichedMatchStatistics = EnrichedPlayerMatchStats

/**
 * 对局分析 Store
 * 职责：管理对局分析相关的核心状态和数据
 */
export const useMatchAnalysisStore = defineStore('matchAnalysis', () => {
  const gameStore = useGameStore()
  const connectionStore = useConnectionStore()

  // === 核心状态（从其他 store 派生） ===
  const currentPhase = computed(() => gameStore.currentPhase as GamePhase)
  const isConnected = computed(() => connectionStore.isConnected)

  // === 数据状态 ===
  const myTeamData = ref<TeamData | null>(null)
  const myTeamStats = ref<(EnrichedMatchStatistics | null)[]>([])
  const enemyTeamData = ref<TeamData | null>(null)
  const enemyTeamStats = ref<(EnrichedMatchStatistics | null)[]>([])
  const enemyChampionPicks = ref<
    Array<{ cellId: number; championId: number | null; championPickIntent?: number | null }>
  >([])

  // === 对局信息 ===
  const queueId = ref<number>(0) // 队列类型ID：420=单排, 440=灵活排位, 450=大乱斗等
  const isCustomGame = ref<boolean>(false) // 是否自定义游戏

  // === 控制状态 ===
  const isLoading = ref(false)
  const isEnemyTeamLoading = ref(false) // 专门用于敌方队伍的 Loading 状态
  const phase = ref<GamePhase>('None')
  const isDestroyed = ref(false)

  // === 计算属性（Getters） ===
  const shouldShowAnalysis = computed(() => {
    // 使用 currentPhase (gameStore 的阶段) 而不是内部 phase 状态
    return currentPhase.value === 'ChampSelect' || currentPhase.value === 'InProgress'
  })

  const hasMyTeamData = computed(() => {
    const result = myTeamData.value !== null && myTeamData.value.players.length > 0
    console.log('[MatchAnalysisStore] hasMyTeamData 计算:', {
      myTeamDataIsNull: myTeamData.value === null,
      playersLength: myTeamData.value?.players.length,
      result
    })
    return result
  })

  const hasEnemyTeamData = computed(() => {
    return enemyTeamData.value !== null && enemyTeamData.value.players.length > 0
  })

  // 对局类型标签和图标
  const queueTypeLabel = computed(() => {
    if (isCustomGame.value) return '自定义游戏'
    switch (queueId.value) {
      case 420:
        return '单/双排位'
      case 440:
        return '灵活排位'
      case 450:
        return '极地大乱斗'
      case 400:
        return '匹配模式'
      case 430:
        return '匹配模式'
      case 900:
        return '无限火力'
      case 1020:
        return '云顶之弈'
      default:
        return queueId.value > 0 ? `队列 ${queueId.value}` : '未知模式'
    }
  })

  const queueTypeIcon = computed(() => {
    if (isCustomGame.value) return '🎮'
    switch (queueId.value) {
      case 420:
      case 440:
        return '🏆'
      case 450:
        return '❄️'
      case 400:
      case 430:
        return '🎲'
      case 900:
        return '🔥'
      case 1020:
        return '♟️'
      default:
        return '🎮'
    }
  })

  const isRankedGame = computed(() => {
    return queueId.value === 420 || queueId.value === 440
  })

  // === Actions（数据操作方法） ===
  const setMyTeamData = (data: TeamData | null) => {
    console.log('[MatchAnalysisStore] 🎯 setMyTeamData 被调用', {
      isNull: data === null,
      playersCount: data?.players.length,
      players: data?.players
    })
    myTeamData.value = data
    console.log('[MatchAnalysisStore] ✅ myTeamData 已更新，当前值:', {
      isNull: myTeamData.value === null,
      playersCount: myTeamData.value?.players.length
    })
  }

  const setMyTeamStats = (stats: (EnrichedMatchStatistics | null)[]) => {
    console.log('[matchAnalysisStore] 🎯 setMyTeamStats 被调用:', {
      statsLength: stats.length,
      filledCount: stats.filter((s) => s !== null).length,
      stats: stats,
      sample: stats.find((s) => s !== null) // 打印第一个非 null 数据作为示例
    })
    myTeamStats.value = stats
  }

  const setEnemyTeamData = (data: TeamData | null) => {
    enemyTeamData.value = data
  }

  const setEnemyTeamStats = (stats: (EnrichedMatchStatistics | null)[]) => {
    enemyTeamStats.value = stats
  }

  const setEnemyChampionPicks = (
    picks: Array<{ cellId: number; championId: number | null; championPickIntent?: number | null }>
  ) => {
    console.log('[MatchAnalysisStore] setEnemyChampionPicks 被调用，picks:', picks)
    enemyChampionPicks.value = picks
    console.log('[MatchAnalysisStore] enemyChampionPicks 已更新')
  }

  const setPhase = (newPhase: GamePhase) => {
    phase.value = newPhase
  }

  const setLoading = (loading: boolean) => {
    isLoading.value = loading
  }

  const setEnemyTeamLoading = (loading: boolean) => {
    isEnemyTeamLoading.value = loading
  }

  const setDestroyed = (destroyed: boolean) => {
    isDestroyed.value = destroyed
  }

  const setQueueInfo = (queue: number, custom: boolean) => {
    console.log('[MatchAnalysisStore] 🎮 设置队列信息:', { queueId: queue, isCustomGame: custom })
    queueId.value = queue
    isCustomGame.value = custom
  }

  // === 清理方法 ===
  const clearAllData = () => {
    console.log('[MatchAnalysisStore] 清理所有数据')
    myTeamData.value = null
    myTeamStats.value = []
    enemyTeamData.value = null
    enemyTeamStats.value = []
    enemyChampionPicks.value = []
    queueId.value = 0
    isCustomGame.value = false
    phase.value = 'None'
    isLoading.value = false
    isEnemyTeamLoading.value = false
  }

  const $reset = () => {
    console.log('[MatchAnalysisStore] 重置 Store')
    clearAllData()
    isDestroyed.value = false
  }

  return {
    // 状态
    currentPhase,
    isConnected,
    isLoading,
    isEnemyTeamLoading,
    phase,
    isDestroyed,

    // 数据
    myTeamData,
    myTeamStats,
    enemyTeamData,
    enemyTeamStats,
    enemyChampionPicks,

    // 对局信息
    queueId,
    isCustomGame,

    // 计算属性
    shouldShowAnalysis,
    hasMyTeamData,
    hasEnemyTeamData,
    queueTypeLabel,
    queueTypeIcon,
    isRankedGame,

    // Actions
    setMyTeamData,
    setMyTeamStats,
    setEnemyTeamData,
    setEnemyTeamStats,
    setEnemyChampionPicks,
    setQueueInfo,
    setPhase,
    setLoading,
    setEnemyTeamLoading,
    setDestroyed,
    clearAllData,
    $reset
  }
})
