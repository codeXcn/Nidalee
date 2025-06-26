import { ref, readonly } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useSummonerStore } from '@/stores/summonerStore'
import { useMatchStatisticsStore } from '@/stores/matchStatisticsStore'

// 全局连接状态
const isConnected = ref(false)
const connectionMessage = ref('等待连接到League客户端...')
const isListening = ref(false)

export function useConnection() {
  const summonerStore = useSummonerStore()
  const matchStatisticsStore = useMatchStatisticsStore()

  // 开始监听后端事件
  const startListening = async () => {
    if (isListening.value) return

    try {
      // 监听召唤师信息变化
      await listen('summoner-change', async (event) => {
        console.log('[Connection] 召唤师信息变化:', event.payload)
        
        if (event.payload) {
          // 有召唤师信息，表示已连接
          isConnected.value = true
          connectionMessage.value = '已连接到League客户端'
          
          // 更新召唤师信息到store
          summonerStore.updateSummonerInfo(event.payload)
          
          // 自动获取战绩
          try {
            await matchStatisticsStore.fetchMatchHistory()
          } catch (error) {
            console.warn('自动获取战绩失败:', error)
          }
        } else {
          // 召唤师信息为null，表示断开连接
          isConnected.value = false
          connectionMessage.value = '与League客户端断开连接'
          summonerStore.clearSummonerInfo()
        }
      })

      // 监听游戏结束事件，刷新战绩
      await listen('game-finished', async () => {
        console.log('[Connection] 游戏结束，刷新战绩')
        try {
          await matchStatisticsStore.fetchMatchHistory()
        } catch (error) {
          console.warn('游戏结束后刷新战绩失败:', error)
        }
      })

      isListening.value = true
      console.log('[Connection] 开始监听连接状态')
    } catch (error) {
      console.error('[Connection] 启动监听失败:', error)
    }
  }

  // 停止监听
  const stopListening = () => {
    isListening.value = false
  }

  // 手动刷新连接状态
  const refreshConnection = async () => {
    try {
      await summonerStore.fetchSummonerInfo()
      await matchStatisticsStore.fetchMatchHistory()
    } catch (error) {
      console.error('[Connection] 手动刷新失败:', error)
    }
  }

  return {
    // 状态
    isConnected: readonly(isConnected),
    connectionMessage: readonly(connectionMessage),
    isListening: readonly(isListening),
    
    // 方法
    startListening,
    stopListening,
    refreshConnection
  }
}
