import { ref, computed, watch, watchEffect, onBeforeUnmount } from 'vue'
import { useGameStore } from '@/stores/features/gameStore'
import { useConnectionStore } from '@/stores/core/connectionStore'
import { useErrorHandler } from '@/composables/utils/useErrorHandler'
import { useLiveClientManager } from './useLiveClientManager'
import { useMatchHistoryFetcher } from './useMatchHistoryFetcher'
import type { GamePhase, TeamData, PlayerData } from '@/types/match-analysis'

// 使用全局类型 MatchStatistics，添加显示名称的扩展类型
interface EnrichedMatchStatistics extends MatchStatistics {
  displayName: string
}

/**
 * 对局分析管理器V2 - 修复潜在问题版本
 *
 * 主要修复：
 * 1. 并发竞态条件控制
 * 2. 内存泄漏防护
 * 3. 错误处理增强
 * 4. 重试机制
 */
export function useMatchAnalysisManagerV3() {
  const gameStore = useGameStore()
  const connectionStore = useConnectionStore()
  const errorHandler = useErrorHandler()
  const liveClientManager = useLiveClientManager()
  const matchHistoryFetcher = useMatchHistoryFetcher()

  // === 核心状态 ===
  const currentPhase = computed(() => gameStore.currentPhase)
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
  const currentOperation = ref<AbortController | null>(null)
  const retryCount = ref(0)
  const maxRetries = 3

  // === 辅助函数 ===

  const getDisplayName = (player: any): string => {
    if (player.gameName && player.tagLine) {
      return `${player.gameName}#${player.tagLine}`
    }
    return player.summonerId || '未知召唤师'
  }

  const getLocalPlayerName = (): string | undefined => {
    const session = gameStore.champSelectSession
    if (session?.localPlayerCellId !== undefined) {
      const localPlayer = session.myTeam?.find((p: any) => p.cellId === session.localPlayerCellId)
      return localPlayer ? getDisplayName(localPlayer) : undefined
    }
    return undefined
  }

  const resolvePlayerName = (displayName: string): string => {
    if (displayName.includes('#')) return displayName
    return displayName
  }

  const waitForLiveClientData = async (signal?: AbortSignal, timeout = 10000): Promise<boolean> => {
    const start = Date.now()
    const checkInterval = 500

    while (Date.now() - start < timeout) {
      if (signal?.aborted || isDestroyed.value) {
        return false
      }

      if (liveClientManager.hasData.value) {
        return true
      }

      await new Promise((resolve) => {
        const timer = setTimeout(resolve, checkInterval)
        signal?.addEventListener('abort', () => {
          clearTimeout(timer)
          resolve(undefined)
        })
      })
    }

    console.warn('[MatchAnalysis] LiveClient 数据等待超时')
    return false
  }

  // === 阶段处理函数 ===

  /**
   * ChampSelect 阶段处理（防并发版本）
   */
  const handleChampSelectPhase = async () => {
    // 防止重复执行
    if (isLoading.value && phase.value === 'ChampSelect') {
      console.log('[MatchAnalysis] ChampSelect 处理中，跳过')
      return
    }

    console.log('[MatchAnalysis] 进入 ChampSelect 阶段')

    // 取消之前的操作
    currentOperation.value?.abort()
    const controller = new AbortController()
    currentOperation.value = controller

    const session = gameStore.champSelectSession
    if (!session || !isConnected.value || isDestroyed.value) return

    try {
      isLoading.value = true
      phase.value = 'ChampSelect'
      retryCount.value = 0

      // 检查中断
      if (controller.signal.aborted) return

      // 1. 处理我方队伍数据
      myTeamData.value = {
        players: session.myTeam.map(
          (player: any): PlayerData => ({
            cellId: player.cellId,
            displayName: getDisplayName(player),
            championId: player.championId,
            championName: player.championName,
            position: player.assignedPosition,
            puuid: player.puuid,
            summonerId: player.summonerId,
            isLocal: player.cellId === session.localPlayerCellId,
            isBot: player.nameVisibilityType === 'HIDDEN' && !player.gameName,
            spells: [player.spell1Id, player.spell2Id],
            tier: player.tier
          })
        ),
        localPlayerCellId: session.localPlayerCellId
      }

      // 2. 获取我方战绩（真人玩家）
      const myRealPlayers = myTeamData.value.players
        .filter((p: PlayerData) => !p.isBot && p.displayName && p.displayName !== '未知召唤师')
        .map((p: PlayerData) => ({
          summonerName: p.displayName,
          isBot: false,
          index: myTeamData.value!.players.indexOf(p)
        }))

      if (myRealPlayers.length > 0 && !controller.signal.aborted) {
        try {
          await matchHistoryFetcher.fetchTeamMatchHistory(myRealPlayers, [], 10)
          myTeamStats.value = matchHistoryFetcher.summonerStats.value
        } catch (error) {
          console.warn('[MatchAnalysis] 我方战绩获取失败:', error)
          myTeamStats.value = []
        }
      }

      // 3. 初始化敌方英雄选择监听
      if (!controller.signal.aborted) {
        enemyChampionPicks.value = session.theirTeam.map((player: any) => ({
          cellId: player.cellId,
          championId: player.championId || null
        }))
      }

      console.log('[MatchAnalysis] ChampSelect 处理完成')
    } catch (error) {
      console.error('[MatchAnalysis] ChampSelect 处理错误:', error)
      errorHandler.handleError(error instanceof Error ? error : String(error), 'ChampSelect 阶段处理')
    } finally {
      isLoading.value = false
    }
  }

  /**
   * InProgress 阶段处理（带重试机制）
   */
  const handleInProgressPhase = async () => {
    // 防止重复执行
    if (isLoading.value && phase.value === 'InProgress') {
      console.log('[MatchAnalysis] InProgress 处理中，跳过')
      return
    }

    console.log('[MatchAnalysis] 进入 InProgress 阶段')

    // 取消之前的操作
    currentOperation.value?.abort()
    const controller = new AbortController()
    currentOperation.value = controller

    const executeWithRetry = async (): Promise<void> => {
      while (retryCount.value < maxRetries && !controller.signal.aborted && !isDestroyed.value) {
        try {
          isLoading.value = true
          phase.value = 'InProgress'

          // 1. 启动 LiveClient 监听
          const localPlayerName = getLocalPlayerName()
          await liveClientManager.resetState()

          if (controller.signal.aborted) return

          await liveClientManager.getLivePlayers(localPlayerName)
          await liveClientManager.startLiveClientAvailabilityCheck(localPlayerName)

          // 2. 等待 LiveClient 数据
          const hasData = await waitForLiveClientData(controller.signal)
          if (!hasData) {
            throw new Error('LiveClient 数据获取超时')
          }

          // 3. 处理敌方队伍数据
          if (liveClientManager.players.value.length > 0 && !controller.signal.aborted) {
            enemyTeamData.value = {
              players: liveClientManager.players.value.map(
                (player: any): PlayerData => ({
                  cellId: 0,
                  displayName: player.displayName,
                  championId: player.championId,
                  championName: player.championName,
                  position: player.assignedPosition,
                  puuid: null,
                  summonerId: player.summonerId,
                  isLocal: false,
                  isBot: player.isBot,
                  spells: [player.spell1Id, player.spell2Id],
                  tier: null
                })
              ),
              localPlayerCellId: -1
            }

            // 4. 获取敌方战绩
            const enemyRealPlayers = enemyTeamData.value.players
              .filter((p: PlayerData) => !p.isBot && p.displayName && !p.displayName.includes('电脑'))
              .map((p: PlayerData) => ({
                summonerName: resolvePlayerName(p.displayName),
                isBot: false,
                index: enemyTeamData.value!.players.indexOf(p)
              }))

            if (enemyRealPlayers.length > 0 && !controller.signal.aborted) {
              try {
                await matchHistoryFetcher.fetchTeamMatchHistory([], enemyRealPlayers, 10)
                enemyTeamStats.value = matchHistoryFetcher.theirTeamStats.value
              } catch (error) {
                console.warn('[MatchAnalysis] 敌方战绩获取失败:', error)
                enemyTeamStats.value = []
              }
            }
          }

          console.log('[MatchAnalysis] InProgress 处理完成')
          return // 成功，退出重试循环
        } catch (error) {
          retryCount.value++
          console.error(`[MatchAnalysis] InProgress 处理失败 (第${retryCount.value}次):`, error)

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
      isLoading.value = false
    }
  }

  /**
   * 清理阶段
   */
  const handleCleanupPhase = () => {
    console.log('[MatchAnalysis] 清理阶段')

    // 取消当前操作
    currentOperation.value?.abort()
    currentOperation.value = null

    // 重置状态
    phase.value = 'None'
    isLoading.value = false
    retryCount.value = 0

    // 清理数据
    myTeamData.value = null
    myTeamStats.value = []
    enemyTeamData.value = null
    enemyTeamStats.value = []
    enemyChampionPicks.value = []

    // 停止服务
    liveClientManager.resetState()
  }

  // === 监听器 ===

  // 阶段变化监听（防抖处理）
  let phaseChangeTimer: NodeJS.Timeout | null = null
  watchEffect(() => {
    const newPhase = currentPhase.value

    // 清除之前的定时器
    if (phaseChangeTimer) {
      clearTimeout(phaseChangeTimer)
    }

    // 防抖处理，避免频繁切换
    phaseChangeTimer = setTimeout(() => {
      if (isDestroyed.value) return

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
      if (phase.value === 'ChampSelect' && session?.theirTeam && !isDestroyed.value) {
        enemyChampionPicks.value = session.theirTeam.map((player: any) => ({
          cellId: player.cellId,
          championId: player.championId || null
        }))
        console.log('[MatchAnalysis] 敌方英雄选择更新:', enemyChampionPicks.value)
      }
    },
    { deep: true }
  )

  // 组件销毁时清理
  onBeforeUnmount(() => {
    console.log('[MatchAnalysis] 组件销毁，清理资源')
    isDestroyed.value = true

    if (phaseChangeTimer) {
      clearTimeout(phaseChangeTimer)
    }

    currentOperation.value?.abort()
    handleCleanupPhase()
  })

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

  return {
    // 状态
    currentPhase: phase,
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
    retry: () => {
      retryCount.value = 0
      const currentPhaseValue = currentPhase.value
      if (currentPhaseValue === 'ChampSelect') {
        handleChampSelectPhase()
      } else if (currentPhaseValue === 'InProgress') {
        handleInProgressPhase()
      }
    },

    // 调试信息
    debugInfo: computed(() => ({
      phase: phase.value,
      retryCount: retryCount.value,
      isLoading: isLoading.value,
      hasOperation: currentOperation.value !== null
    }))
  }
}
