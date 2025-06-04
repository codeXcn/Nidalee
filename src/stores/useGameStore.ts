import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface SummonerInfo {
  id: string
  name: string
  level: number
  profileIconId: number
  rank?: {
    tier: string
    division: string
    leaguePoints: number
  }
}

export interface GameStatus {
  connected: boolean
  connecting: boolean
  inGame: boolean
  inMatch: boolean
  gameMode: string | null
  summoner: SummonerInfo | null
}

export interface MatchData {
  teamScore: number
  enemyScore: number
  myTeam: Array<{
    summonerId: string
    summonerName: string
    championName: string
    championId: number
    position: string
    rank: string
    winRate: number
  }>
  enemyTeam: Array<{
    summonerId: string
    summonerName: string
    championName: string
    championId: number
    position: string
    rank: string
    winRate: number
  }>
  advantages: string[]
  warnings: string[]
}

export interface DailyStats {
  wins: number
  losses: number
  winRate: number
  gamesPlayed: number
}

export const useGameStore = defineStore('gameStore', () => {
  // 游戏连接状态
  const gameStatus = ref<GameStatus>({
    connected: false,
    connecting: false,
    inGame: false,
    inMatch: false,
    gameMode: null,
    summoner: null
  })
  
  // 对局数据
  const matchData = ref<MatchData | null>(null)
  
  // 每日统计
  const dailyStats = ref<DailyStats>({
    wins: 0,
    losses: 0,
    winRate: 0,
    gamesPlayed: 0
  })
  
  // 通知列表
  const notifications = ref<Array<{
    id: number
    message: string
    type: 'info' | 'success' | 'warning' | 'error'
    timestamp: number
  }>>([])
  
  // 操作日志
  const operationLogs = ref<Array<{
    id: number
    operation: string
    details: string
    type: 'success' | 'error' | 'warning' | 'info'
    timestamp: string
  }>>([])
  
  // 计算属性
  const isConnected = computed(() => gameStatus.value.connected)
  const currentSummoner = computed(() => gameStatus.value.summoner)
  const unreadNotifications = computed(() => notifications.value.length)
  
  // 方法
  function updateGameStatus(status: Partial<GameStatus>) {
    gameStatus.value = { ...gameStatus.value, ...status }
  }
  
  function setSummoner(summoner: SummonerInfo | null) {
    gameStatus.value.summoner = summoner
  }
  
  function setMatchData(data: MatchData | null) {
    matchData.value = data
  }
  
  function updateDailyStats(stats: Partial<DailyStats>) {
    dailyStats.value = { ...dailyStats.value, ...stats }
    // 重新计算胜率
    if (dailyStats.value.gamesPlayed > 0) {
      dailyStats.value.winRate = Math.round((dailyStats.value.wins / dailyStats.value.gamesPlayed) * 100)
    }
  }
  
  function addNotification(notification: {
    message: string
    type: 'info' | 'success' | 'warning' | 'error'
  }) {
    const newNotification = {
      id: Date.now(),
      ...notification,
      timestamp: Date.now()
    }
    notifications.value.unshift(newNotification)
    
    // 限制通知数量
    if (notifications.value.length > 50) {
      notifications.value = notifications.value.slice(0, 50)
    }
  }
  
  function removeNotification(id: number) {
    const index = notifications.value.findIndex(n => n.id === id)
    if (index > -1) {
      notifications.value.splice(index, 1)
    }
  }
  
  function clearNotifications() {
    notifications.value = []
  }
  
  function addOperationLog(log: {
    operation: string
    details: string
    type: 'success' | 'error' | 'warning' | 'info'
  }) {
    const newLog = {
      id: Date.now(),
      ...log,
      timestamp: new Date().toLocaleTimeString()
    }
    operationLogs.value.unshift(newLog)
    
    // 限制日志数量
    if (operationLogs.value.length > 100) {
      operationLogs.value = operationLogs.value.slice(0, 100)
    }
  }
  
  function clearOperationLogs() {
    operationLogs.value = []
  }
  
  // 连接相关方法
  async function connectToGame(): Promise<boolean> {
    gameStatus.value.connecting = true
    
    try {
      // TODO: 实现实际的游戏连接逻辑
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      gameStatus.value.connected = true
      gameStatus.value.connecting = false
      
      addNotification({
        message: '成功连接到英雄联盟客户端',
        type: 'success'
      })
      
      addOperationLog({
        operation: '连接客户端',
        details: '成功连接到英雄联盟客户端',
        type: 'success'
      })
      
      return true
    } catch (error) {
      gameStatus.value.connecting = false
      
      addNotification({
        message: '连接失败，请检查游戏客户端是否运行',
        type: 'error'
      })
      
      addOperationLog({
        operation: '连接客户端',
        details: '连接失败：' + (error as Error).message,
        type: 'error'
      })
      
      return false
    }
  }
  
  function disconnectFromGame() {
    gameStatus.value = {
      connected: false,
      connecting: false,
      inGame: false,
      inMatch: false,
      gameMode: null,
      summoner: null
    }
    
    addNotification({
      message: '已断开与游戏客户端的连接',
      type: 'info'
    })
  }
  
  // 重置每日统计（每天调用）
  function resetDailyStats() {
    dailyStats.value = {
      wins: 0,
      losses: 0,
      winRate: 0,
      gamesPlayed: 0
    }
  }
  
  // 重置所有游戏数据
  function resetAllGameData() {
    gameStatus.value = {
      connected: false,
      connecting: false,
      inGame: false,
      inMatch: false,
      gameMode: null,
      summoner: null
    }
    matchData.value = null
    dailyStats.value = {
      wins: 0,
      losses: 0,
      winRate: 0,
      gamesPlayed: 0
    }
    notifications.value = []
    operationLogs.value = []
  }
  
  return {
    // 状态
    gameStatus,
    matchData,
    dailyStats,
    notifications,
    operationLogs,
    
    // 计算属性
    isConnected,
    currentSummoner,
    unreadNotifications,
    
    // 方法
    updateGameStatus,
    setSummoner,
    setMatchData,
    updateDailyStats,
    addNotification,
    removeNotification,
    clearNotifications,
    addOperationLog,
    clearOperationLogs,
    connectToGame,
    disconnectFromGame,
    resetDailyStats,
    resetAllGameData
  }
}, {
  persist: true
}) 