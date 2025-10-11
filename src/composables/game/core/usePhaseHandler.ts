import { ref, watch, watchEffect, onBeforeUnmount } from 'vue'
import { useGameStore } from '@/stores/features/gameStore'
import { useErrorHandler } from '@/composables/utils/useErrorHandler'
import { useMatchAnalysis } from './useMatchAnalysis'
import { useTeamDataManager } from './useTeamDataManager'

/**
 * 游戏阶段处理器
 * 职责：处理不同游戏阶段的切换逻辑和自动化操作
 */
export function usePhaseHandler() {
  const gameStore = useGameStore()
  const errorHandler = useErrorHandler()
  const matchAnalysis = useMatchAnalysis()
  const teamDataManager = useTeamDataManager(matchAnalysis)

  // 控制状态
  const currentOperation = ref<AbortController | null>(null)
  const retryCount = ref(0)
  const maxRetries = 3

  // === 辅助函数 ===
  const waitForLiveClientData = async (signal?: AbortSignal, timeout = 10000): Promise<boolean> => {
    const start = Date.now()
    const checkInterval = 500

    while (Date.now() - start < timeout) {
      if (signal?.aborted || matchAnalysis.isDestroyed.value) {
        return false
      }

      // 这里需要检查 LiveClient 数据，暂时返回 true
      // TODO: 后续集成 LiveClient 管理器
      await new Promise((resolve) => {
        const timer = setTimeout(resolve, checkInterval)
        signal?.addEventListener('abort', () => {
          clearTimeout(timer)
          resolve(undefined)
        })
      })

      return true // 临时返回，实际需要检查数据
    }

    console.warn('[PhaseHandler] LiveClient 数据等待超时')
    return false
  }

  // === 阶段处理函数 ===

  /**
   * ChampSelect 阶段处理
   */
  const handleChampSelectPhase = async () => {
    // 防止重复执行
    if (matchAnalysis.isLoading.value && matchAnalysis.phase.value === 'ChampSelect') {
      console.log('[PhaseHandler] ChampSelect 处理中，跳过')
      return
    }

    console.log('[PhaseHandler] 进入 ChampSelect 阶段')

    // 取消之前的操作
    currentOperation.value?.abort()
    const controller = new AbortController()
    currentOperation.value = controller

    const session = gameStore.champSelectSession
    if (!session || !matchAnalysis.isConnected.value || matchAnalysis.isDestroyed.value) return

    try {
      matchAnalysis.setLoading(true)
      matchAnalysis.setPhase('ChampSelect')
      retryCount.value = 0

      // 检查中断
      if (controller.signal.aborted) return

      // 处理我方队伍数据
      await teamDataManager.processMyTeamData(session, controller.signal)

      // 初始化敌方英雄选择监听
      if (!controller.signal.aborted) {
        const enemyPicks = session.theirTeam.map((player: any) => ({
          cellId: player.cellId,
          championId: player.championId || null
        }))
        matchAnalysis.setEnemyChampionPicks(enemyPicks)
      }

      console.log('[PhaseHandler] ChampSelect 处理完成')
    } catch (error) {
      console.error('[PhaseHandler] ChampSelect 处理错误:', error)
      errorHandler.handleError(error instanceof Error ? error : String(error), 'ChampSelect 阶段处理')
    } finally {
      matchAnalysis.setLoading(false)
    }
  }

  /**
   * InProgress 阶段处理（带重试机制）
   */
  const handleInProgressPhase = async () => {
    // 防止重复执行
    if (matchAnalysis.isLoading.value && matchAnalysis.phase.value === 'InProgress') {
      console.log('[PhaseHandler] InProgress 处理中，跳过')
      return
    }

    console.log('[PhaseHandler] 进入 InProgress 阶段')

    // 取消之前的操作
    currentOperation.value?.abort()
    const controller = new AbortController()
    currentOperation.value = controller

    const executeWithRetry = async (): Promise<void> => {
      while (retryCount.value < maxRetries && !controller.signal.aborted && !matchAnalysis.isDestroyed.value) {
        try {
          matchAnalysis.setLoading(true)
          matchAnalysis.setPhase('InProgress')

          // 等待 LiveClient 数据
          const hasData = await waitForLiveClientData(controller.signal)
          if (!hasData) {
            throw new Error('LiveClient 数据获取超时')
          }

          // 处理敌方队伍数据
          await teamDataManager.processEnemyTeamData(controller.signal)

          console.log('[PhaseHandler] InProgress 处理完成')
          return // 成功，退出重试循环
        } catch (error) {
          retryCount.value++
          console.error(`[PhaseHandler] InProgress 处理失败 (第${retryCount.value}次):`, error)

          if (retryCount.value >= maxRetries) {
            errorHandler.handleError(error instanceof Error ? error : new Error(String(error)), 'InProgress 阶段处理')
            return
          }

          // 等待后重试
          await new Promise((resolve) => {
            const timer = setTimeout(resolve, Math.pow(2, retryCount.value) * 1000) // 指数退避
            controller.signal.addEventListener('abort', () => {
              clearTimeout(timer)
              resolve(undefined)
            })
          })
        }
      }
    }

    try {
      await executeWithRetry()
    } finally {
      matchAnalysis.setLoading(false)
    }
  }

  /**
   * 清理阶段
   */
  const handleCleanupPhase = () => {
    console.log('[PhaseHandler] 清理阶段')

    // 取消当前操作
    currentOperation.value?.abort()
    currentOperation.value = null

    // 重置状态
    retryCount.value = 0

    // 清理数据
    matchAnalysis.clearAllData()
  }

  // === 监听器 ===

  // 阶段变化监听（防抖处理）
  let phaseChangeTimer: NodeJS.Timeout | null = null
  watchEffect(() => {
    const newPhase = matchAnalysis.currentPhase.value

    // 清除之前的定时器
    if (phaseChangeTimer) {
      clearTimeout(phaseChangeTimer)
    }

    // 防抖处理，避免频繁切换
    phaseChangeTimer = setTimeout(() => {
      if (matchAnalysis.isDestroyed.value) return

      switch (newPhase) {
        case 'ChampSelect':
          handleChampSelectPhase()
          break
        case 'InProgress':
          handleInProgressPhase()
          break
        case 'None':
        case 'Lobby':
        case 'EndOfGame':
          handleCleanupPhase()
          break
      }
    }, 100) // 100ms 防抖
  })

  // 选人会话变化监听
  watch(
    () => gameStore.champSelectSession,
    (session) => {
      if (matchAnalysis.phase.value === 'ChampSelect' && session?.theirTeam && !matchAnalysis.isDestroyed.value) {
        const enemyPicks = session.theirTeam.map((player: any) => ({
          cellId: player.cellId,
          championId: player.championId || null
        }))
        matchAnalysis.setEnemyChampionPicks(enemyPicks)
        console.log('[PhaseHandler] 敌方英雄选择更新:', enemyPicks)
      }
    },
    { deep: true }
  )

  // 组件销毁时清理
  onBeforeUnmount(() => {
    console.log('[PhaseHandler] 组件销毁，清理资源')

    if (phaseChangeTimer) {
      clearTimeout(phaseChangeTimer)
    }

    currentOperation.value?.abort()
    handleCleanupPhase()
  })

  return {
    // 方法
    retry: () => {
      retryCount.value = 0
      const currentPhaseValue = matchAnalysis.currentPhase.value
      if (currentPhaseValue === 'ChampSelect') {
        handleChampSelectPhase()
      } else if (currentPhaseValue === 'InProgress') {
        handleInProgressPhase()
      }
    },

    // 调试信息
    debugInfo: () => ({
      phase: matchAnalysis.phase.value,
      retryCount: retryCount.value,
      isLoading: matchAnalysis.isLoading.value,
      hasOperation: currentOperation.value !== null
    })
  }
}
