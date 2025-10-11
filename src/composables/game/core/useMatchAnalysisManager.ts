import { computed } from 'vue'
import { useMatchAnalysis } from './useMatchAnalysis'
import { usePhaseHandler } from './usePhaseHandler'

/**
 * 对局分析管理器 - 重构版本
 *
 * 主要改进：
 * 1. 拆分为独立的核心模块，提高可维护性
 * 2. 单一职责原则，每个模块专注特定功能
 * 3. 更清晰的依赖关系和更好的可测试性
 * 4. 保持与原版本相同的 API 接口
 */
export function useMatchAnalysisManager() {
  // 使用拆分后的核心模块
  const matchAnalysis = useMatchAnalysis()
  const phaseHandler = usePhaseHandler()

  // 保持与原版本相同的 API
  return {
    // 状态 - 直接从 matchAnalysis 暴露
    currentPhase: matchAnalysis.currentPhase,
    isConnected: matchAnalysis.isConnected,
    isLoading: matchAnalysis.isLoading,

    // 数据 - 直接从 matchAnalysis 暴露
    myTeamData: matchAnalysis.myTeamData,
    myTeamStats: matchAnalysis.myTeamStats,
    enemyTeamData: matchAnalysis.enemyTeamData,
    enemyTeamStats: matchAnalysis.enemyTeamStats,
    enemyChampionPicks: matchAnalysis.enemyChampionPicks,

    // 计算属性 - 直接从 matchAnalysis 暴露
    shouldShowAnalysis: matchAnalysis.shouldShowAnalysis,
    hasMyTeamData: matchAnalysis.hasMyTeamData,
    hasEnemyTeamData: matchAnalysis.hasEnemyTeamData,

    // 方法 - 从 phaseHandler 暴露
    retry: phaseHandler.retry,

    // 调试信息 - 从 phaseHandler 暴露
    debugInfo: computed(() => phaseHandler.debugInfo())
  }
}
