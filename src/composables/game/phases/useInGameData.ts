import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getChampionIdByName } from '@/lib'

/**
 * 统一的游戏内数据管理器
 * 职责：管理 LiveClient 数据的获取、处理、事件监听和状态管理
 *
 * 合并了原本的 useLiveClient 和 useLiveClientManager 功能
 */

interface ProcessedPlayer {
  displayName: string
  championId: number
  championName: string
  assignedPosition?: string
  summonerId: string
  isBot: boolean
  spell1Id?: number
  spell2Id?: number
  team: string
  [key: string]: any
}

interface TeamData {
  myTeam: ProcessedPlayer[]
  enemyTeam: ProcessedPlayer[]
}

export function useInGameData() {
  // === 基础状态 ===
  const playerList = ref<LiveClientPlayer[]>([])
  const gameEvents = ref<any[]>([])
  const gameStats = ref<any>(null)
  const isAvailable = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const hasFetchedGameData = ref(false)

  // === 处理后的数据 ===
  const processedPlayers = ref<ProcessedPlayer[]>([])
  const myTeamPlayers = ref<ProcessedPlayer[]>([])
  const enemyTeamPlayers = ref<ProcessedPlayer[]>([])

  // 事件监听管理
  let eventListeners: (() => void)[] = []

  // === 计算属性 ===
  const hasData = computed(() => processedPlayers.value.length > 0)
  const teamData = computed(
    (): TeamData => ({
      myTeam: myTeamPlayers.value,
      enemyTeam: enemyTeamPlayers.value
    })
  )

  // === 核心功能方法 ===

  /**
   * 检查 LiveClient 可用性
   */
  const checkAvailability = async (): Promise<boolean> => {
    try {
      const available = await invoke<boolean>('is_liveclient_available')
      isAvailable.value = available

      if (available) {
        console.log('[InGameData] LiveClient 服务可用')
        return true
      } else {
        console.log('[InGameData] LiveClient 服务不可用')
        return false
      }
    } catch (err) {
      console.error('[InGameData] 检查可用性失败:', err)
      isAvailable.value = false
      return false
    }
  }

  /**
   * 获取游戏数据
   */
  const fetchGameData = async (localPlayerName?: string) => {
    isLoading.value = true
    error.value = null

    try {
      const newPlayerList = await invoke<LiveClientPlayer[]>('get_live_player_list')
      console.log('[InGameData] 获取到游戏数据:', newPlayerList)

      await processGameData(newPlayerList, localPlayerName)
      hasFetchedGameData.value = true
    } catch (err) {
      console.error('[InGameData] 获取游戏数据失败:', err)
      error.value = err instanceof Error ? err.message : '获取游戏数据失败'
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 强制拉取玩家列表（不依赖可用性预检）
   */
  const getLivePlayers = async (localPlayerName?: string) => {
    try {
      const list = await invoke<LiveClientPlayer[]>('get_live_player_list')
      if (Array.isArray(list) && list.length > 0) {
        await processGameData(list, localPlayerName)
        hasFetchedGameData.value = true
        isAvailable.value = true
        console.log('[InGameData] 快速拉取成功，已填充数据')
      } else {
        console.log('[InGameData] 快速拉取返回为空')
      }
    } catch (err) {
      console.warn('[InGameData] 快速拉取失败:', err)
    }
  }

  /**
   * 处理游戏数据
   */
  const processGameData = async (newPlayerList: LiveClientPlayer[], localPlayerName?: string) => {
    if (!Array.isArray(newPlayerList) || newPlayerList.length === 0) {
      console.log('[InGameData] 玩家列表为空')
      return
    }

    console.log('[InGameData] 处理游戏数据:', newPlayerList)

    // 更新原始数据
    playerList.value = newPlayerList

    // 处理玩家数据
    const processed: ProcessedPlayer[] = []
    for (const player of newPlayerList) {
      const processedPlayer: ProcessedPlayer = {
        displayName: player.summonerName || '未知玩家',
        championId: getChampionIdByName(player.championName) || 0,
        championName: player.championName || '',
        assignedPosition: player.position,
        summonerId: player.summonerName || '',
        isBot: player.isBot || false,
        spell1Id: player.summonerSpells?.[0]?.displayName ? parseInt(player.summonerSpells[0].displayName) : undefined,
        spell2Id: player.summonerSpells?.[1]?.displayName ? parseInt(player.summonerSpells[1].displayName) : undefined,
        team: player.team || 'UNKNOWN'
      }
      processed.push(processedPlayer)
    }

    processedPlayers.value = processed

    // 识别队伍
    const teams = identifyTeams(newPlayerList, localPlayerName)
    myTeamPlayers.value = teams.myTeam.map(mapToProcessedPlayer)
    enemyTeamPlayers.value = teams.enemyTeam.map(mapToProcessedPlayer)

    console.log('[InGameData] 数据处理完成', {
      total: processed.length,
      myTeam: myTeamPlayers.value.length,
      enemyTeam: enemyTeamPlayers.value.length
    })
  }

  /**
   * 根据玩家列表识别队伍
   */
  const identifyTeams = (players: LiveClientPlayer[], localPlayerName?: string) => {
    if (!players || players.length === 0) {
      return { myTeam: [], enemyTeam: [] }
    }

    let myTeam: LiveClientPlayer[] = []
    let enemyTeam: LiveClientPlayer[] = []

    if (localPlayerName) {
      // 根据本地玩家名称确定队伍
      const localPlayer = players.find((p) => p.summonerName === localPlayerName)
      if (localPlayer) {
        const localTeam = localPlayer.team
        myTeam = players.filter((p) => p.team === localTeam)
        enemyTeam = players.filter((p) => p.team !== localTeam)
      } else {
        // 如果找不到本地玩家，使用默认逻辑
        myTeam = players.filter((p) => p.team === 'ORDER')
        enemyTeam = players.filter((p) => p.team === 'CHAOS')
      }
    } else {
      // 默认逻辑：ORDER 是我方，CHAOS 是敌方
      myTeam = players.filter((p) => p.team === 'ORDER')
      enemyTeam = players.filter((p) => p.team === 'CHAOS')
    }

    return { myTeam, enemyTeam }
  }

  /**
   * 映射原始玩家数据到处理后的格式
   */
  const mapToProcessedPlayer = (player: LiveClientPlayer): ProcessedPlayer => ({
    displayName: player.summonerName || '未知玩家',
    championId: getChampionIdByName(player.championName) || 0,
    championName: player.championName || '',
    assignedPosition: player.position,
    summonerId: player.summonerName || '',
    isBot: player.isBot || false,
    spell1Id: player.summonerSpells?.[0]?.displayName ? parseInt(player.summonerSpells[0].displayName) : undefined,
    spell2Id: player.summonerSpells?.[1]?.displayName ? parseInt(player.summonerSpells[1].displayName) : undefined,
    team: player.team || 'UNKNOWN'
  })

  /**
   * 获取本地玩家信息
   */
  const getLocalPlayer = (localPlayerName?: string) => {
    if (!localPlayerName || !playerList.value) return null
    return playerList.value.find((p) => p.summonerName === localPlayerName) || null
  }

  // === 事件监听功能 ===

  /**
   * 启动事件监听
   */
  const startListening = async () => {
    try {
      console.log('[InGameData] 开始启动事件监听...')

      // 清理之前的监听器
      stopListening()

      // 监听玩家列表更新
      const playerListUnlisten = await listen<LiveClientPlayer[]>('liveclient-player-list', (event) => {
        console.log('[InGameData] 收到玩家列表更新:', event.payload)
        if (event.payload && event.payload.length > 0) {
          processGameData(event.payload)
        }
      })
      eventListeners.push(playerListUnlisten)

      // 监听游戏事件
      const eventsUnlisten = await listen<any[]>('liveclient-events', (event) => {
        console.log('[InGameData] 收到游戏事件:', event.payload)
        gameEvents.value = event.payload || []
      })
      eventListeners.push(eventsUnlisten)

      // 监听游戏统计
      const statsUnlisten = await listen<any>('liveclient-game-stats', (event) => {
        console.log('[InGameData] 收到游戏统计:', event.payload)
        gameStats.value = event.payload
      })
      eventListeners.push(statsUnlisten)

      console.log('[InGameData] 事件监听已启动')
    } catch (error) {
      console.error('[InGameData] 启动事件监听失败:', error)
    }
  }

  /**
   * 停止事件监听
   */
  const stopListening = () => {
    eventListeners.forEach((unlisten) => {
      try {
        unlisten()
      } catch (error) {
        console.warn('[InGameData] 停止监听器时出错:', error)
      }
    })
    eventListeners = []
    console.log('[InGameData] 已停止所有事件监听')
  }

  /**
   * 启动 LiveClient 可用性检查
   */
  const startAvailabilityCheck = async (localPlayerName?: string) => {
    const available = await checkAvailability()
    if (available) {
      await fetchGameData(localPlayerName)
      await startListening()
    }
  }

  /**
   * 重置状态
   */
  const resetState = () => {
    stopListening()
    playerList.value = []
    gameEvents.value = []
    gameStats.value = null
    processedPlayers.value = []
    myTeamPlayers.value = []
    enemyTeamPlayers.value = []
    isAvailable.value = false
    isLoading.value = false
    error.value = null
    hasFetchedGameData.value = false
    console.log('[InGameData] 状态已重置')
  }

  return {
    // 状态
    isAvailable,
    isLoading,
    error,
    hasFetchedGameData,
    hasData,

    // 原始数据
    playerList,
    gameEvents,
    gameStats,

    // 处理后的数据
    processedPlayers,
    myTeamPlayers,
    enemyTeamPlayers,
    teamData,

    // 方法
    checkAvailability,
    fetchGameData,
    getLivePlayers,
    getLocalPlayer,
    startListening,
    stopListening,
    startAvailabilityCheck,
    resetState,

    // 向后兼容的别名
    players: processedPlayers, // 兼容 useLiveClientManager
    identifyTeams, // 兼容 useLiveClient
    startLiveClientAvailabilityCheck: startAvailabilityCheck // 兼容别名
  }
}
