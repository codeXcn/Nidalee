import { ref, computed, onBeforeUnmount } from 'vue'
import { useGameStore } from '@/stores/features/gameStore'
import { useConnectionStore } from '@/stores/core/connectionStore'
import type { GamePhase, TeamData } from '@/types/match-analysis'

// 使用全局类型 MatchStatistics，添加显示名称的扩展类型
interface EnrichedMatchStatistics extends MatchStatistics {
  displayName: string
}

/**
 * 对局分析核心状态管理
 * 职责：管理分析相关的核心状态和数据
 */
export function useMatchAnalysis() {
  const gameStore = useGameStore()
  const connectionStore = useConnectionStore()

  // === 核心状态 ===
  const currentPhase = computed(() => gameStore.currentPhase as GamePhase)
  const isConnected = computed(() => connectionStore.isConnected)

  // 数据状态
  const myTeamData = ref<TeamData | null>(null)
  const myTeamStats = ref<EnrichedMatchStatistics[]>([])
  const enemyTeamData = ref<TeamData | null>(null)
  const enemyTeamStats = ref<EnrichedMatchStatistics[]>([])
  const enemyChampionPicks = ref<Array<{ cellId: number; championId: number | null }>>([])

  // 控制状态
  const isLoading = ref(false)
  const phase = ref<GamePhase>('None')
  const isDestroyed = ref(false)

  // === 计算属性 ===
  const shouldShowAnalysis = computed(() => {
    return phase.value === 'ChampSelect' || phase.value === 'InProgress'
  })

  const hasMyTeamData = computed(() => {
    return myTeamData.value !== null && myTeamData.value.players.length > 0
  })

  const hasEnemyTeamData = computed(() => {
    return enemyTeamData.value !== null && enemyTeamData.value.players.length > 0
  })

  // === 数据操作方法 ===
  const setMyTeamData = (data: TeamData | null) => {
    myTeamData.value = data
  }

  const setMyTeamStats = (stats: EnrichedMatchStatistics[]) => {
    myTeamStats.value = stats
  }

  const setEnemyTeamData = (data: TeamData | null) => {
    enemyTeamData.value = data
  }

  const setEnemyTeamStats = (stats: EnrichedMatchStatistics[]) => {
    enemyTeamStats.value = stats
  }

  const setEnemyChampionPicks = (picks: Array<{ cellId: number; championId: number | null }>) => {
    enemyChampionPicks.value = picks
  }

  const setPhase = (newPhase: GamePhase) => {
    phase.value = newPhase
  }

  const setLoading = (loading: boolean) => {
    isLoading.value = loading
  }

  // === 清理方法 ===
  const clearAllData = () => {
    myTeamData.value = null
    myTeamStats.value = []
    enemyTeamData.value = null
    enemyTeamStats.value = []
    enemyChampionPicks.value = []
    phase.value = 'None'
    isLoading.value = false
  }

  // 组件销毁时清理
  onBeforeUnmount(() => {
    console.log('[MatchAnalysis] 核心状态清理')
    isDestroyed.value = true
    clearAllData()
  })

  return {
    // 状态
    currentPhase,
    isConnected,
    isLoading,
    phase,
    isDestroyed,

    // 数据
    myTeamData,
    myTeamStats,
    enemyTeamData,
    enemyTeamStats,
    enemyChampionPicks,

    // 计算属性
    shouldShowAnalysis,
    hasMyTeamData,
    hasEnemyTeamData,

    // 方法
    setMyTeamData,
    setMyTeamStats,
    setEnemyTeamData,
    setEnemyTeamStats,
    setEnemyChampionPicks,
    setPhase,
    setLoading,
    clearAllData
  }
}
