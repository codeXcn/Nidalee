import { invoke } from '@tauri-apps/api/core'
import { useAutoFunctionStore } from './autoFunctionStore'
import { useActivityStore } from './index'
import { useChampSelect } from '@/composables/game/useChampSelect'

export const useGameStatusStore = defineStore('gameStatus', () => {
  // 游戏状态
  const gameStatus = ref<GameStatus>({
    phase: 'None',
    queue: null,
    isInGame: false
  })

  // 选人阶段信息
  const currentChampSelectSession = ref<ChampSelectSession | null>(null)
  const teamsStates = ref<any>(null)

  // 房间信息
  const lobbyInfo = ref<LobbyInfo | null>(null)

  // 跟踪已执行的自动操作
  const executedAutoActions = ref({
    banChampion: false,
    selectChampion: false,
    runeConfig: false
  })

  // 上一次的选人阶段
  const previousChampSelectPhase = ref<string | null>(null)

  // 处理自动选人/禁用操作
  const handleAutoChampSelectActions = async (session: ChampSelectSession) => {
    const autoFunctionStore = useAutoFunctionStore()
    const activityStore = useActivityStore()
    const { checkAndExecuteAutoActions } = useChampSelect()

    const currentPhase = session?.timer?.phase
    if (!currentPhase) return

    console.log('[🤖 GameStatusStore] ===== 检查自动选人操作 =====')
    console.log('[🤖 GameStatusStore] 当前选人子阶段:', currentPhase)
    console.log('[🤖 GameStatusStore] 已执行的操作:', executedAutoActions.value)

    try {
      // 使用简化逻辑检查并执行自动操作
      const hasScheduled = await checkAndExecuteAutoActions(
        session,
        autoFunctionStore.autoFunctions,
        executedAutoActions.value
      )

      if (hasScheduled) {
        console.log('[🤖 GameStatusStore] ✅ 已安排自动操作执行')
      }
    } catch (error) {
      console.error('[🤖 GameStatusStore] ❌ 自动操作检查失败:', error)
      activityStore.addActivity('error', `自动操作检查失败: ${error}`)
    }

    console.log('[🤖 GameStatusStore] ===== 自动选人操作检查完成 =====\n')
  }

  // 更新游戏阶段
  const updateGamePhase = (phase: GamePhase | null) => {
    if (phase) {
      gameStatus.value.phase = phase
      gameStatus.value.isInGame = phase === 'InProgress'
    } else {
      gameStatus.value.phase = 'None'
      gameStatus.value.isInGame = false
    }
  }

  // 更新选人阶段信息
  const updateChampSelectSession = async (session: ChampSelectSession | null) => {
    console.log('[📊 GameStatusStore] ===== 更新选人阶段信息 =====')
    console.log('[📊 GameStatusStore] 新会话数据:', session ? '有数据' : '空数据')

    // 如果是新的选人会话，重置自动操作状态
    if (session && !currentChampSelectSession.value) {
      console.log('[📊 GameStatusStore] 🔄 新的选人会话，重置自动操作状态')
      executedAutoActions.value = {
        banChampion: false,
        selectChampion: false,
        runeConfig: false
      }
      previousChampSelectPhase.value = null
    }

    currentChampSelectSession.value = session

    if (session) {
      // 处理自动选人/禁用操作
      await handleAutoChampSelectActions(session)

      // 获取队伍信息
      if (!teamsStates.value) {
        try {
          const states = await invoke<any>('get_champselect_team_players_info')
          teamsStates.value = states
          console.log('[📊 GameStatusStore] ✅ 获取队伍信息成功')
        } catch (error) {
          console.error('[📊 GameStatusStore] ❌ 获取队伍信息失败:', error)
        }
      }
    } else {
      console.log('[📊 GameStatusStore] 🧹 清空状态')
      teamsStates.value = null
      executedAutoActions.value = {
        banChampion: false,
        selectChampion: false,
        runeConfig: false
      }
      previousChampSelectPhase.value = null
    }

    console.log('[📊 GameStatusStore] ===== 选人阶段信息更新完成 =====\n')
  }

  // 更新房间信息
  const updateLobbyInfo = (lobby: LobbyInfo | null) => {
    lobbyInfo.value = lobby
  }

  // 清除所有游戏状态
  const clearGameState = () => {
    gameStatus.value = {
      phase: 'None',
      queue: null,
      isInGame: false
    }
    currentChampSelectSession.value = null
    teamsStates.value = null
    lobbyInfo.value = null
    executedAutoActions.value = {
      banChampion: false,
      selectChampion: false,
      runeConfig: false
    }
    previousChampSelectPhase.value = null
  }

  // 计算属性
  const isInChampSelect = computed(() => !!currentChampSelectSession.value)
  const isInLobby = computed(() => !!lobbyInfo.value)
  const currentPhase = computed(() => gameStatus.value.phase)
  const isInGame = computed(() => gameStatus.value.isInGame)

  return {
    // 状态
    gameStatus: readonly(gameStatus),
    currentChampSelectSession: readonly(currentChampSelectSession),
    teamsStates: readonly(teamsStates),
    lobbyInfo: readonly(lobbyInfo),
    executedAutoActions: readonly(executedAutoActions),

    // 计算属性
    isInChampSelect,
    isInLobby,
    currentPhase,
    isInGame,

    // 方法
    updateGamePhase,
    updateChampSelectSession,
    updateLobbyInfo,
    clearGameState
  }
})
