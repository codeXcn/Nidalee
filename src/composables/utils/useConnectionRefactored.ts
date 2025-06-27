import { ref, readonly, computed } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useConnectionStore } from '@/stores/connectionStore'
import { useSummonerStore } from '@/stores/summonerStore'

// 重构后的连接管理
export function useConnection() {
  const connectionStore = useConnectionStore()
  const summonerStore = useSummonerStore()
  
  const isListening = ref(false)
  
  // 使用 store 中的状态，不再重复定义
  const isConnected = computed(() => connectionStore.isConnected)
  const connectionMessage = computed(() => {
    if (connectionStore.isConnected) {
      return '已连接到League客户端'
    } else if (connectionStore.connectionError) {
      return connectionStore.connectionError
    } else {
      return '等待连接到League客户端...'
    }
  })

  // 开始监听后端事件
  const startListening = async () => {
    if (isListening.value) return

    try {
      // 监听认证信息变化（更准确的连接状态）
      await listen('auth-info-change', async (event) => {
        console.log('[Connection] 认证信息变化:', event.payload)
        connectionStore.setAuthInfo(event.payload)
      })

      // 监听召唤师信息变化（用于更新召唤师数据）
      await listen('summoner-change', async (event) => {
        console.log('[Connection] 召唤师信息变化:', event.payload)
        
        if (event.payload) {
          summonerStore.updateSummonerInfo(event.payload)
        } else {
          summonerStore.clearSummonerInfo()
          // 发送断开连接事件，让其他模块处理
          document.dispatchEvent(new CustomEvent('connection-lost'))
        }
      })

      // 监听游戏结束事件（独立处理）
      await listen('game-finished', async () => {
        console.log('[Connection] 游戏结束事件')
        // 发送自定义事件，让需要的组件自行决定是否刷新数据
        document.dispatchEvent(new CustomEvent('game-finished'))
      })

      // 监听连接状态变化（如果后端发送）
      await listen('connection-state-changed', async (event) => {
        console.log('[Connection] 连接状态变化:', event.payload)
        connectionStore.setAuthInfo(event.payload)
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
      await connectionStore.checkConnection()
      if (connectionStore.isConnected) {
        await summonerStore.fetchSummonerInfo()
      }
    } catch (error) {
      console.error('[Connection] 手动刷新失败:', error)
    }
  }

  return {
    // 状态（使用计算属性，数据来源于 store）
    isConnected,
    connectionMessage,
    isListening: readonly(isListening),
    
    // 方法
    startListening,
    stopListening,
    refreshConnection
  }
}
