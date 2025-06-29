import { useActivityStore } from '@/stores/core/activityStore'
import { useConnectionStore } from '@/stores/core/connectionStore'
import { useDataStore } from '@/stores/core/dataStore'
import { listen } from '@tauri-apps/api/event'
import { onMounted, onUnmounted, ref } from 'vue'

/**
 * 连接事件处理组合式函数
 * 职责：监听和处理连接相关的事件
 */
export function useConnectionEvents() {
  const connectionStore = useConnectionStore()
  const dataStore = useDataStore()
  const activityStore = useActivityStore()

  const isListening = ref(false)

  // 处理连接状态变化
  const handleConnectionStateChange = async (connectionInfo: any) => {
    console.log('[ConnectionEvents] 连接状态变化:', connectionInfo)

    // 更新连接信息
    connectionStore.updateConnectionInfo(connectionInfo)

    // 根据连接状态触发相应的操作
    if (connectionInfo.state === 'Connected' && connectionInfo.auth_info) {
      // 连接成功且有认证信息时
      try {
        // 这里需要调用相应的API获取数据
        // await dataStore.setSummonerInfo(summonerInfo)
        // await dataStore.setMatchHistory(matchHistory)
        activityStore.addActivity('success', '客户端连接成功', 'connection')
      } catch (error) {
        console.error('[ConnectionEvents] 获取召唤师信息或战绩失败:', error)
        activityStore.addActivity('error', '获取召唤师信息失败', 'error')
      }
    } else if (connectionInfo.state === 'Disconnected') {
      // 完全断开连接时
      dataStore.clearSummonerInfo()
      dataStore.clearMatchHistory()
      activityStore.addActivity('error', '客户端连接断开', 'connection')
    } else if (connectionInfo.state === 'Unstable') {
      activityStore.addActivity('warning', '客户端连接超时，正在重试...', 'connection')
    }
  }

  // 处理召唤师信息变化
  const handleSummonerChange = async (event: any) => {
    console.log('[ConnectionEvents] 召唤师信息变化:', event.payload)

    if (event.payload) {
      dataStore.setSummonerInfo(event.payload)
      activityStore.addActivity('info', '召唤师信息已更新', 'data')
    } else {
      dataStore.clearSummonerInfo()
      // 发送断开连接事件
      document.dispatchEvent(new CustomEvent('connection-lost'))
    }
  }

  // 处理游戏结束事件
  const handleGameFinished = () => {
    console.log('[ConnectionEvents] 游戏结束事件')
    // 发送自定义事件，让需要的组件自行决定是否刷新数据
    document.dispatchEvent(new CustomEvent('game-finished'))
  }

  // 开始监听事件
  const startListening = async () => {
    if (isListening.value) return

    try {
      // 监听连接状态变化
      await listen('connection-state-changed', handleConnectionStateChange)

      // 监听召唤师信息变化
      await listen('summoner-change', handleSummonerChange)

      // 监听游戏结束事件
      await listen('game-finished', handleGameFinished)

      isListening.value = true
      console.log('[ConnectionEvents] 开始监听连接事件')
    } catch (error) {
      console.error('[ConnectionEvents] 启动监听失败:', error)
    }
  }

  // 停止监听
  const stopListening = () => {
    isListening.value = false
    console.log('[ConnectionEvents] 停止监听连接事件')
  }

  // 手动刷新连接
  const refreshConnection = async () => {
    try {
      console.log('[ConnectionEvents] 手动刷新连接状态')
      await connectionStore.checkConnection()

      if (connectionStore.isConnected) {
        // 这里需要调用相应的API获取数据
        // await dataStore.setSummonerInfo(summonerInfo)
        // await dataStore.setMatchHistory(matchHistory)
        activityStore.addActivity('info', '客户端重连成功', 'connection')
      }
    } catch (error) {
      console.error('[ConnectionEvents] 手动刷新失败:', error)
      activityStore.addActivity('error', '连接失败：手动刷新失败', 'connection')
    }
  }

  // 生命周期管理
  onMounted(() => {
    startListening()
  })

  onUnmounted(() => {
    stopListening()
  })

  return {
    isListening,
    startListening,
    stopListening,
    refreshConnection
  }
}
