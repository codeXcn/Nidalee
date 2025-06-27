import { useGameStatusStore, useActivityStore } from '@/stores'

// 专门处理游戏阶段变化的逻辑
export function useGamePhaseManager() {
  const gameStatusStore = useGameStatusStore()
  const activityStore = useActivityStore()

  // 游戏阶段变更处理
  const handleGamePhaseChange = (phase: GamePhase | null) => {
    const previousPhase = gameStatusStore.currentPhase
    console.log('[GamePhaseManager] 处理游戏阶段变更:', phase)
    gameStatusStore.updateGamePhase(phase)

    if (phase) {
      activityStore.addActivity('info', `游戏阶段变更: ${phase}`)

      // 检查是否从游戏中退出
      if (previousPhase === 'InProgress' && phase !== 'InProgress') {
        console.log('[GamePhaseManager] 检测到游戏退出，清理选人和房间状态')
        gameStatusStore.updateChampSelectSession(null)
        gameStatusStore.updateLobbyInfo(null)
        activityStore.addActivity('info', '游戏已结束，已清理游戏状态')
      }
    } else {
      activityStore.addActivity('info', '游戏阶段重置')
      // 阶段为空时也清理游戏状态
      gameStatusStore.updateChampSelectSession(null)
      gameStatusStore.updateLobbyInfo(null)
    }
  }

  return {
    handleGamePhaseChange
  }
}
