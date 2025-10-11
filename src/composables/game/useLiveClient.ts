import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'

export function useLiveClient() {
  const playerList = ref<any[]>([])
  const gameEvents = ref<any[]>([])
  const gameStats = ref<any>(null)
  const isAvailable = ref(false)

  // 监听 LiveClient 事件
  const startListening = async () => {
    try {
      console.log('[LiveClient] 开始启动事件监听...')

      // 监听玩家列表更新
      await listen<LiveClientPlayer[]>('liveclient-player-list', (event) => {
        console.log('[LiveClient] 收到玩家列表更新:', event.payload)
        console.log('[LiveClient] 玩家列表长度:', event.payload?.length)
        playerList.value = event.payload
      })

      // 监听游戏事件
      await listen<any[]>('liveclient-events', (event) => {
        console.log('[LiveClient] 收到游戏事件:', event.payload)
        gameEvents.value = event.payload
      })

      // 监听游戏统计
      await listen<any>('liveclient-game-stats', (event) => {
        console.log('[LiveClient] 收到游戏统计:', event.payload)
        gameStats.value = event.payload
      })

      console.log('[LiveClient] 事件监听已启动')
    } catch (error) {
      console.error('[LiveClient] 启动事件监听失败:', error)
    }
  }

  // 根据玩家列表识别队伍
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

  // 获取本地玩家信息
  const getLocalPlayer = (players: LiveClientPlayer[], localPlayerName?: string) => {
    if (!localPlayerName || !players) return null
    return players.find((p) => p.summonerName === localPlayerName) || null
  }

  return {
    playerList,
    gameEvents,
    gameStats,
    isAvailable,
    startListening,
    identifyTeams,
    getLocalPlayer
  }
}
