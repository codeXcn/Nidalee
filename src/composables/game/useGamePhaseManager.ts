import { useActivityLogger } from '@/composables/utils/useActivityLogger'
import { useGameStore } from '@/stores/features/gameStore'
import { useMatchmaking } from './useMatchmaking'
import { useSummonerAndMatchUpdater } from './useSummonerAndMatchUpdater'
import { useAutoFunctionStore } from '@/stores'
import { useRouter } from 'vue-router'

// 专门处理游戏阶段变化的逻辑
export function useGamePhaseManager() {
  const gameStore = useGameStore()
  const activityLogger = useActivityLogger()
  const autoFunctionStore = useAutoFunctionStore()
  const { handleAcceptMatch } = useMatchmaking()
  const { updateSummonerAndMatches } = useSummonerAndMatchUpdater()
  const router = useRouter()

  // 游戏阶段变更处理
  const handleGamePhaseChange = (phaseObj: GameflowPhase | null) => {
    const phase = phaseObj?.phase
    const previousPhase = gameStore.currentPhase
    // 过滤掉重复的阶段变更事件
    if (previousPhase === phase) {
      return
    }
    console.log('[🎮 GamePhaseManager] ===== 游戏阶段变更 =====')
    console.log('[🎮 GamePhaseManager] 上一个阶段:', previousPhase)
    console.log('[🎮 GamePhaseManager] 当前阶段:', phase)
    console.log('[🎮 GamePhaseManager] 阶段变更时间:', new Date().toLocaleTimeString())

    gameStore.updateGamePhase(phase || 'None')

    if (phase) {
      // 只处理接受对局，选人/禁用由 gameStore 处理
      if (phase === 'ReadyCheck') {
        handleAutoAcceptMatch()
      }
      if (phase && previousPhase !== phase) {
        // 只记录具体阶段活动
        switch (phase) {
          case 'None':
            activityLogger.log.info('返回客户端主界面', 'game')
            break
          case 'Lobby':
            activityLogger.log.info('进入房间', 'game')
            gameStore.clearChampSelect()
            break
          case 'Matchmaking':
            activityLogger.log.info('进入队列匹配中', 'game')
            gameStore.clearChampSelect()
            // 自动跳转到对局分析页面
            if (router.currentRoute.value.name !== 'match-analysis') {
              console.log('[🎮 GamePhaseManager] 开始匹配，自动跳转到对局分析页面')
              router.push({ name: 'match-analysis' })
            }
            break
          case 'ReadyCheck':
            activityLogger.log.success('找到对局，等待接受', 'game')
            gameStore.clearChampSelect()
            break
          case 'ChampSelect':
            activityLogger.log.info('进入英雄选择阶段', 'game')
            break
          case 'InProgress':
            activityLogger.log.success('游戏开始', 'game')
            break
          case 'WaitingForStats':
            activityLogger.log.info('游戏结束', 'game')
            break
        }
      }
      // 检查是否从游戏中退出
      if (previousPhase === 'InProgress' && phase !== 'InProgress') {
        console.log('[🎮 GamePhaseManager] 🏁 检测到游戏退出，清理选人和房间状态')
        gameStore.clearChampSelect()
        gameStore.updateLobbyInfo(null)
        activityLogger.log.info('游戏已结束，已清理游戏状态', 'game')
        updateSummonerAndMatches()
      }
    } else {
      console.log('[🎮 GamePhaseManager] 🔄 游戏阶段重置为空')
      // 阶段为空时也清理游戏状态
      gameStore.clearChampSelect()
      gameStore.updateLobbyInfo(null)
    }
    console.log('[🎮 GamePhaseManager] ===== 阶段变更处理完成 =====\n')
  }

  const handleAutoAcceptMatch = async () => {
    const { autoFunctions } = autoFunctionStore

    if (autoFunctions.acceptMatch.enabled) {
      console.log('[🤖 GamePhaseManager] ✅ 自动接受对局已启用，延迟', autoFunctions.acceptMatch.delay, 'ms后执行')

      setTimeout(async () => {
        try {
          console.log('[🤖 GamePhaseManager] 🚀 开始执行自动接受对局')
          await handleAcceptMatch()
          console.log('[🤖 GamePhaseManager] ✅ 自动接受对局执行成功')
          activityLogger.logAutoFunction.acceptMatch.success()
        } catch (error) {
          console.error('[🤖 GamePhaseManager] ❌ 自动接受对局失败:', error)
          activityLogger.logAutoFunction.acceptMatch.failed(String(error))
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
