import { invoke } from '@tauri-apps/api/core'

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

  // 更新游戏阶段
  const updateGamePhase = (phase: GamePhase | null) => {
    if (phase) {
      gameStatus.value.phase = phase?.phase || phase 
      gameStatus.value.isInGame = phase.phase === 'InProgress'
    } else {
      gameStatus.value.phase = 'None'
      gameStatus.value.isInGame = false
    }
  }

  // 更新选人阶段信息
  const updateChampSelectSession = async (session: ChampSelectSession | null) => {
    currentChampSelectSession.value = session

    if (session && !teamsStates.value) {
      try {
        const states = await invoke<any>('get_champselect_team_players_info')
        teamsStates.value = states
      } catch (error) {
        console.error('获取选人阶段队伍信息失败:', error)
      }
    } else if (!session) {
      teamsStates.value = null
    }
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
