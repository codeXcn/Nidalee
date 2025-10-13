import { usePhaseHandler } from './usePhaseHandler'

/**
 * 对局分析系统初始化 - 简化版
 * 仅启动阶段处理器
 */
export function useMatchAnalysisInit() {
  console.log('[MatchAnalysisInit] 🚀 初始化对局分析系统...')

  // 启动阶段处理器
  const phaseHandler = usePhaseHandler()

  console.log('[MatchAnalysisInit] ✅ 对局分析系统已启动')

  return {
    phaseHandler
  }
}
