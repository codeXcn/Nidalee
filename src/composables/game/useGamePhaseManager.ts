import { useGameStatusStore, useActivityStore } from '@/stores'
import { useAutoFunctionStore } from '@/stores/autoFunctionStore'
import { useMatchmaking } from './useMatchmaking'

// 专门处理游戏阶段变化的逻辑
export function useGamePhaseManager() {
  const gameStatusStore = useGameStatusStore()
  const activityStore = useActivityStore()
  const autoFunctionStore = useAutoFunctionStore()
  const { handleAcceptMatch } = useMatchmaking()

  // 游戏阶段变更处理
  const handleGamePhaseChange = (phase: GamePhase | null) => {
    const previousPhase = gameStatusStore.currentPhase
    console.log('[🎮 GamePhaseManager] ===== 游戏阶段变更 =====')
    console.log('[🎮 GamePhaseManager] 上一个阶段:', previousPhase)
    console.log('[🎮 GamePhaseManager] 当前阶段:', phase)
    console.log('[🎮 GamePhaseManager] 阶段变更时间:', new Date().toLocaleTimeString())
    
    gameStatusStore.updateGamePhase(phase)

    if (phase) {
      activityStore.addActivity('info', `游戏阶段变更: ${previousPhase} → ${phase}`)

      // 只处理接受对局，选人/禁用由 gameStatusStore 处理
      if (phase === 'ReadyCheck') {
        handleAutoAcceptMatch()
      }

      // 检查是否从游戏中退出
      if (previousPhase === 'InProgress' && phase !== 'InProgress') {
        console.log('[🎮 GamePhaseManager] 🏁 检测到游戏退出，清理选人和房间状态')
        gameStatusStore.updateChampSelectSession(null)
        gameStatusStore.updateLobbyInfo(null)
        activityStore.addActivity('info', '游戏已结束，已清理游戏状态')
      }
    } else {
      console.log('[🎮 GamePhaseManager] 🔄 游戏阶段重置为空')
      activityStore.addActivity('info', '游戏阶段重置')
      // 阶段为空时也清理游戏状态
      gameStatusStore.updateChampSelectSession(null)
      gameStatusStore.updateLobbyInfo(null)
    }
    console.log('[🎮 GamePhaseManager] ===== 阶段变更处理完成 =====\n')
  }

  // 处理自动接受对局
  const handleAutoAcceptMatch = async () => {
    const { autoFunctions } = autoFunctionStore
    
    if (autoFunctions.acceptMatch.enabled) {
      console.log('[🤖 GamePhaseManager] ✅ 自动接受对局已启用，延迟', autoFunctions.acceptMatch.delay, 'ms后执行')
      
      setTimeout(async () => {
        try {
          console.log('[🤖 GamePhaseManager] 🚀 开始执行自动接受对局')
          await handleAcceptMatch()
          console.log('[🤖 GamePhaseManager] ✅ 自动接受对局执行成功')
          activityStore.addActivity('success', '自动接受对局已触发')
        } catch (error) {
          console.error('[🤖 GamePhaseManager] ❌ 自动接受对局失败:', error)
          activityStore.addActivity('error', `自动接受对局失败: ${error}`)
        }
      }, autoFunctions.acceptMatch.delay)
    } else {
      console.log('[🤖 GamePhaseManager] ⚪ 自动接受对局未启用')
    }
  }

  return {
    handleGamePhaseChange
  }
}