import { ref, watch } from 'vue'
import { useConnectionStore } from '@/stores/connectionStore'
import { useMatchStatisticsStore } from '@/stores/matchStatisticsStore'

// 专门处理战绩数据获取的 composable
export function useMatchDataManager() {
  const connectionStore = useConnectionStore()
  const matchStatisticsStore = useMatchStatisticsStore()
  
  const lastFetchTime = ref(0)
  const minFetchInterval = 30000 // 30秒最小间隔
  
  // 智能获取战绩（带防抖）
  const fetchMatchHistoryWithDebounce = async (force = false) => {
    const now = Date.now()
    
    // 如果不是强制刷新且距离上次获取时间太短，则跳过
    if (!force && (now - lastFetchTime.value) < minFetchInterval) {
      console.log('[MatchDataManager] 跳过频繁的战绩获取请求')
      return
    }
    
    if (!connectionStore.isConnected) {
      console.log('[MatchDataManager] 未连接，跳过战绩获取')
      return
    }
    
    try {
      console.log('[MatchDataManager] 开始获取战绩数据')
      await matchStatisticsStore.fetchMatchHistory()
      lastFetchTime.value = now
    } catch (error) {
      console.error('[MatchDataManager] 获取战绩失败:', error)
    }
  }
  
  // 监听连接状态变化，连接成功时自动获取一次
  watch(
    () => connectionStore.isConnected,
    async (connected, wasConnected) => {
      if (connected && !wasConnected) {
        // 从未连接变为连接，延迟获取战绩
        setTimeout(() => {
          fetchMatchHistoryWithDebounce(true)
        }, 2000)
      }
    }
  )
  
  // 监听游戏结束事件
  const setupGameFinishedListener = () => {
    document.addEventListener('game-finished', () => {
      console.log('[MatchDataManager] 游戏结束，刷新战绩')
      // 游戏结束后延迟获取，给后端时间更新数据
      setTimeout(() => {
        fetchMatchHistoryWithDebounce(true)
      }, 5000)
    })
  }
  
  return {
    fetchMatchHistoryWithDebounce,
    setupGameFinishedListener
  }
}
