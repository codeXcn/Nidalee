import { useChampSelect } from '@/composables/game/useChampSelect'
import { useActivityLogger } from '@/composables/utils/useActivityLogger'
import { useAutoFunctionStore } from '@/stores/autoFunctionStore'
import { computed, ref } from 'vue'

export const useGameStore = defineStore(
  'game',
  () => {
    // 游戏状态
    const currentPhase = ref<string>('None')
    const isInGame = ref(false)
    const isInChampSelect = ref(false)
    const isInQueue = ref(false)
    const isReadyCheck = ref(false)

    // 英雄选择状态
    const champSelectSession = ref<any>(null)
    const lobbyInfo = ref<any>(null)

    // 游戏数据
    const currentGameId = ref<string | null>(null)
    const lastGameResult = ref<'win' | 'lose' | null>(null)

    // 自动操作执行记录
    const executedActions = ref({
      banChampion: false,
      selectChampion: false
    })

    // 计算属性
    const isGameActive = computed(() => {
      return isInGame.value || isInChampSelect.value || isInQueue.value || isReadyCheck.value
    })

    const canUseAutoFunctions = computed(() => {
      return isInChampSelect.value || isReadyCheck.value
    })

    // 更新游戏阶段
    const updateGamePhase = (phase: string) => {
      console.log('[GameStore] 更新游戏阶段:', phase)
      currentPhase.value = phase

      // 更新具体状态
      isInGame.value = phase === 'InProgress'
      isInChampSelect.value = phase === 'ChampSelect'
      isInQueue.value = phase === 'Lobby'
      isReadyCheck.value = phase === 'ReadyCheck'
    }

    // 更新英雄选择会话
    const updateChampSelectSession = async (session: any) => {
      console.log('[GameStore] 更新英雄选择会话:', session)
      champSelectSession.value = session

      // 如果有选人会话，检查并执行自动操作
      if (session) {
        await checkAndExecuteAutoActions(session)
      } else {
        // 清除选人会话时，重置已执行的操作记录
        executedActions.value = {
          banChampion: false,
          selectChampion: false
        }
      }
    }

    // 检查并执行自动操作
    const checkAndExecuteAutoActions = async (session: any) => {
      try {
        const autoFunctionStore = useAutoFunctionStore()
        const { checkAndExecuteAutoActions: checkAutoActions } = useChampSelect()
        const activityLogger = useActivityLogger()

        console.log('[GameStore] 检查自动操作...')

        const hasScheduledAction = await checkAutoActions(
          session,
          autoFunctionStore.autoFunctions,
          executedActions.value
        )

        if (hasScheduledAction) {
          console.log('[GameStore] 已安排自动操作执行')
          activityLogger.log.info('已安排自动选人/禁用操作', 'auto')
        }
      } catch (error) {
        console.error('[GameStore] 检查自动操作失败:', error)
      }
    }

    // 更新大厅信息
    const updateLobbyInfo = (lobby: any) => {
      console.log('[GameStore] 更新大厅信息:', lobby)
      lobbyInfo.value = lobby
    }

    // 设置游戏ID
    const setGameId = (gameId: string) => {
      currentGameId.value = gameId
    }

    // 设置游戏结果
    const setGameResult = (result: 'win' | 'lose') => {
      lastGameResult.value = result
    }

    // 重置游戏状态
    const resetGameState = () => {
      currentPhase.value = 'None'
      isInGame.value = false
      isInChampSelect.value = false
      isInQueue.value = false
      isReadyCheck.value = false
      champSelectSession.value = null
      lobbyInfo.value = null
      currentGameId.value = null
      lastGameResult.value = null

      // 重置已执行的操作记录
      executedActions.value = {
        banChampion: false,
        selectChampion: false
      }
    }

    // 清理英雄选择状态
    const clearChampSelect = () => {
      champSelectSession.value = null
      isInChampSelect.value = false

      // 重置已执行的操作记录
      executedActions.value = {
        banChampion: false,
        selectChampion: false
      }
    }

    return {
      // 状态
      currentPhase,
      isInGame,
      isInChampSelect,
      isInQueue,
      isReadyCheck,
      champSelectSession,
      lobbyInfo,
      currentGameId,
      lastGameResult,
      executedActions,

      // 计算属性
      isGameActive,
      canUseAutoFunctions,

      // 方法
      updateGamePhase,
      updateChampSelectSession,
      updateLobbyInfo,
      setGameId,
      setGameResult,
      resetGameState,
      clearChampSelect,
      checkAndExecuteAutoActions
    }
  },
  {
    persist: true
  }
)
