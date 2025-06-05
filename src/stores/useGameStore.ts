import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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
  lockfileInfo?: {
    process_name: string
    process_id: number
    port: string
    token: string
    protocol: string
    path?: string
  }
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
  const notifications = ref<
    Array<{
      id: number
      message: string
      type: 'info' | 'success' | 'warning' | 'error'
      timestamp: number
    }>
  >([])

  // 操作日志
  const operationLogs = ref<
    Array<{
      id: number
      operation: string
      details: string
      type: 'success' | 'error' | 'warning' | 'info'
      timestamp: string
    }>
  >([])

  // 进程监控
  let processMonitorInterval: any = null
  let autoConnectInterval: any = null

  // 自动连接设置
  const autoConnectEnabled = ref(true)

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
      dailyStats.value.winRate = Math.round(
        (dailyStats.value.wins / dailyStats.value.gamesPlayed) * 100
      )
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
      addOperationLog({
        operation: '搜索 Lockfile',
        details: '正在搜索英雄联盟客户端的 lockfile...',
        type: 'info'
      })

      // 调用 Tauri 命令查找并解析 lockfile
      const lockfileData = await invoke<{
        process_name: string
        process_id: number
        port: string
        token: string
        protocol: string
      }>('find_lol_lockfile')

      // 保存 lockfile 信息
      gameStatus.value.lockfileInfo = {
        process_name: lockfileData.process_name,
        process_id: lockfileData.process_id,
        port: lockfileData.port,
        token: lockfileData.token,
        protocol: lockfileData.protocol
      }

      // 初始化游戏连接
      await invoke('init_game_connection', { config: lockfileData })

      gameStatus.value.connected = true
      gameStatus.value.connecting = false

      // 开始进程监控
      startProcessMonitoring()

      // 停止自动连接检测（手动连接成功后）
      stopAutoConnect()

      addNotification({
        message: `成功连接到英雄联盟客户端 (端口: ${lockfileData.port})`,
        type: 'success'
      })

      addOperationLog({
        operation: '连接客户端',
        details: `成功连接到英雄联盟客户端，端口: ${lockfileData.port}`,
        type: 'success'
      })

      return true
    } catch (error) {
      gameStatus.value.connecting = false

      const errorMessage = typeof error === 'string' ? error : (error as Error).message

      addNotification({
        message: `连接失败: ${errorMessage}`,
        type: 'error'
      })

      addOperationLog({
        operation: '连接客户端',
        details: `连接失败：${errorMessage}`,
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

    // 停止进程检测和自动连接
    stopProcessMonitoring()
    stopAutoConnect()

    addNotification({
      message: '已断开与游戏客户端的连接',
      type: 'info'
    })
  }

  // 获取搜索路径
  async function getSearchPaths(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_search_paths')
    } catch (error) {
      console.error('获取搜索路径失败:', error)
      return []
    }
  }

  // 测试自定义路径
  async function testLockfilePath(path: string): Promise<boolean> {
    try {
      const lockfileData = await invoke<{
        process_name: string
        process_id: number
        port: string
        token: string
        protocol: string
      }>('test_lockfile_path', { path })

      // 保存 lockfile 信息
      gameStatus.value.lockfileInfo = {
        process_name: lockfileData.process_name,
        process_id: lockfileData.process_id,
        port: lockfileData.port,
        token: lockfileData.token,
        protocol: lockfileData.protocol,
        path: path
      }

      gameStatus.value.connected = true

      addNotification({
        message: `成功从自定义路径连接: ${lockfileData.port}`,
        type: 'success'
      })

      return true
    } catch (error) {
      const errorMessage = typeof error === 'string' ? error : (error as Error).message
      addNotification({
        message: `自定义路径连接失败: ${errorMessage}`,
        type: 'error'
      })
      return false
    }
  }

  // 开始进程监控
  function startProcessMonitoring() {
    if (processMonitorInterval) {
      clearInterval(processMonitorInterval)
    }

    processMonitorInterval = setInterval(async () => {
      if (gameStatus.value.connected) {
        try {
          await invoke('verify_connection')
          // 如果验证成功，进程仍在运行，不需要做什么
        } catch (error) {
          // 进程已停止，更新状态
          console.log('检测到游戏进程已停止:', error)

          gameStatus.value.connected = false
          gameStatus.value.lockfileInfo = undefined

          addNotification({
            message: '检测到游戏客户端已关闭，等待重启后自动连接',
            type: 'warning'
          })

          addOperationLog({
            operation: '进程监控',
            details: '检测到游戏进程已停止运行，开始自动重连',
            type: 'warning'
          })

          // 停止当前监控并重新开始自动连接
          stopProcessMonitoring()
          if (autoConnectEnabled.value) {
            startAutoConnect()
          }
        }
      }
    }, 3000) // 每3秒检查一次
  }

  // 停止进程监控
  function stopProcessMonitoring() {
    if (processMonitorInterval) {
      clearInterval(processMonitorInterval)
      processMonitorInterval = null
    }
  }

  // 开始自动连接检测
  function startAutoConnect() {
    if (!autoConnectEnabled.value) return

    // 立即尝试连接一次
    if (!gameStatus.value.connected && !gameStatus.value.connecting) {
      silentConnectToGame()
    }

    // 设置定时检测
    if (autoConnectInterval) {
      clearInterval(autoConnectInterval)
    }

    autoConnectInterval = setInterval(async () => {
      if (autoConnectEnabled.value && !gameStatus.value.connected && !gameStatus.value.connecting) {
        await silentConnectToGame()
      }
    }, 5000) // 每5秒尝试连接一次
  }

  // 停止自动连接检测
  function stopAutoConnect() {
    if (autoConnectInterval) {
      clearInterval(autoConnectInterval)
      autoConnectInterval = null
    }
  }

  // 静默连接（不显示过多通知）
  async function silentConnectToGame(): Promise<boolean> {
    if (gameStatus.value.connecting || gameStatus.value.connected) {
      return true
    }

    try {
      // 调用 Tauri 命令查找并解析 lockfile（不修改connecting状态）
      const lockfileData = await invoke<{
        process_name: string
        process_id: number
        port: string
        token: string
        protocol: string
      }>('find_lol_lockfile')

      // 保存 lockfile 信息
      gameStatus.value.lockfileInfo = {
        process_name: lockfileData.process_name,
        process_id: lockfileData.process_id,
        port: lockfileData.port,
        token: lockfileData.token,
        protocol: lockfileData.protocol
      }

      // 初始化游戏连接
      await invoke('init_game_connection', { config: lockfileData })

      gameStatus.value.connected = true

      // 开始进程监控
      startProcessMonitoring()

      // 停止自动连接检测（连接成功后）
      stopAutoConnect()

      // 首次自动连接成功时显示通知
      addNotification({
        message: `自动连接成功！已连接到 ${lockfileData.process_name}`,
        type: 'success'
      })

      addOperationLog({
        operation: '自动连接',
        details: `自动连接到英雄联盟客户端成功，端口: ${lockfileData.port}`,
        type: 'success'
      })

      console.log(
        `自动连接成功: ${lockfileData.process_name} (${lockfileData.process_id}) 端口: ${lockfileData.port}`
      )

      return true
    } catch (error) {
      // 静默连接失败不显示错误通知，只记录到控制台
      console.log('自动连接尝试失败，等待下次重试...')
      return false
    }
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

  // 获取缓存状态
  async function getCacheStatus() {
    try {
      const result = await invoke('get_cache_status')
      return result
    } catch (error) {
      console.error('获取缓存状态失败:', error)
      return null
    }
  }

  // 清除路径缓存
  async function clearPathCache() {
    try {
      await invoke('clear_path_cache')
      console.log('路径缓存已清除')
      addNotification({
        message: '路径缓存已清除',
        type: 'success'
      })
      return true
    } catch (error) {
      console.error('清除路径缓存失败:', error)
      addNotification({
        message: '清除路径缓存失败',
        type: 'error'
      })
      return false
    }
  }

  return {
    // 状态
    gameStatus,
    matchData,
    dailyStats,
    notifications,
    operationLogs,
    autoConnectEnabled,

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
    getSearchPaths,
    testLockfilePath,
    startProcessMonitoring,
    stopProcessMonitoring,
    startAutoConnect,
    stopAutoConnect,
    silentConnectToGame,
    resetDailyStats,
    resetAllGameData,
    getCacheStatus,
    clearPathCache
  }
})
