import { usePhaseHandler } from './usePhaseHandler'
import { useTeamAnalysisHandler } from './useTeamAnalysisHandler'

/**
 * 对局分析系统初始化 - 拆分职责
 * 启动阶段处理器和团队数据处理器
 */
export function useMatchAnalysisInit() {
  console.log('[MatchAnalysisInit] 🚀 初始化对局分析系统...')

  // 启动团队分析数据处理器
  const teamAnalysisHandler = useTeamAnalysisHandler()

  // 启动阶段处理器（简化版）
  const phaseHandler = usePhaseHandler()

  console.log('[MatchAnalysisInit] ✅ 对局分析系统已启动')

  return {
    teamAnalysisHandler,
    phaseHandler
  }
}
