import { ref, computed, onBeforeUnmount } from 'vue'
import { useGameStore } from '@/stores/features/gameStore'
import { useConnectionStore } from '@/stores/core/connectionStore'
import type { GamePhase, TeamData } from '@/types/match-analysis'

// 使用全局类型 MatchStatistics，添加显示名称的扩展类型
interface EnrichedMatchStatistics extends MatchStatistics {
  displayName: string
}

// === 单例状态（模块级别） ===
// 这些状态在整个应用中共享，确保所有使用 useMatchAnalysis 的地方都访问同一份数据
const myTeamData = ref<TeamData | null>(null)
const myTeamStats = ref<EnrichedMatchStatistics[]>([])
const enemyTeamData = ref<TeamData | null>(null)
const enemyTeamStats = ref<EnrichedMatchStatistics[]>([])
const enemyChampionPicks = ref<Array<{ cellId: number; championId: number | null }>>([])
const isLoading = ref(false)
const phase = ref<GamePhase>('None')
const isDestroyed = ref(false)

/**
 * 对局分析核心状态管理（单例模式）
 * 职责：管理分析相关的核心状态和数据
 * 注意：状态在模块级别定义，确保全局共享
 */
export function useMatchAnalysis() {
  const gameStore = useGameStore()
  const connectionStore = useConnectionStore()

  // === 核心状态（从 store 派生） ===
  const currentPhase = computed(() => gameStore.currentPhase as GamePhase)
  const isConnected = computed(() => connectionStore.isConnected)

  // === 计算属性 ===
  const shouldShowAnalysis = computed(() => {
    // 使用 currentPhase (gameStore 的阶段) 而不是内部 phase 状态
    // 因为 currentPhase 会自动响应游戏阶段变化
    return currentPhase.value === 'ChampSelect' || currentPhase.value === 'InProgress'
  })

  const hasMyTeamData = computed(() => {
    const result = myTeamData.value !== null && myTeamData.value.players.length > 0
    console.log('[MatchAnalysis] hasMyTeamData 计算:', {
      myTeamDataIsNull: myTeamData.value === null,
      playersLength: myTeamData.value?.players.length,
      result
    })
    return result
  })

  const hasEnemyTeamData = computed(() => {
    return enemyTeamData.value !== null && enemyTeamData.value.players.length > 0
  })

  // === 数据操作方法 ===
  const setMyTeamData = (data: TeamData | null) => {
    console.log('[MatchAnalysis] 🎯 setMyTeamData 被调用，数据:', data)
    console.log('[MatchAnalysis] 🎯 myTeamData 更新前的引用ID:', myTeamData)
    myTeamData.value = data
    console.log('[MatchAnalysis] ✅ myTeamData.value 已更新，当前值:', myTeamData.value)
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
    console.log('[MatchAnalysis] setEnemyChampionPicks 被调用，picks:', picks)
    enemyChampionPicks.value = picks
    console.log('[MatchAnalysis] enemyChampionPicks.value 已更新')
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
