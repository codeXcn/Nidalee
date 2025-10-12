import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useMatchAnalysisStore } from '@/stores/features/matchAnalysisStore'
import { usePhaseHandler } from './usePhaseHandler'

/**
 * 对局分析管理器 - Store 版本
 *
 * 主要改进：
 * 1. 使用 Pinia Store 管理状态，自动实现状态共享
 * 2. 避免了手动管理单例的复杂性
 * 3. 支持 Vue DevTools 调试
 * 4. 保持与原版本相同的 API 接口
 */
export function useMatchAnalysisManager() {
  // 使用 Store 和 Phase Handler
  const matchAnalysisStore = useMatchAnalysisStore()
  const phaseHandler = usePhaseHandler()

  // 使用 storeToRefs 将 Store 状态转为响应式 refs
  const {
    currentPhase,
    isConnected,
    isLoading,
    myTeamData,
    myTeamStats,
    enemyTeamData,
    enemyTeamStats,
    enemyChampionPicks,
    shouldShowAnalysis,
    hasMyTeamData,
    hasEnemyTeamData
  } = storeToRefs(matchAnalysisStore)

  // 保持与原版本相同的 API
  return {
    // 状态
    currentPhase,
    isConnected,
    isLoading,

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
    retry: phaseHandler.retry,

    // 调试信息
    debugInfo: computed(() => phaseHandler.debugInfo())
  }
}
