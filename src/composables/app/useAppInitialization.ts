import { useConnection } from '../utils/useConnectionRefactored'
import { useMatchDataManager } from '../utils/useMatchDataManager'
import { useDisconnectionHandler } from '../utils/useDisconnectionHandler'

// 应用级别的初始化管理
export function useAppInitialization() {
  const connection = useConnection()
  const matchDataManager = useMatchDataManager()
  const disconnectionHandler = useDisconnectionHandler()
  
  // 初始化应用
  const initializeApp = async () => {
    console.log('[AppInit] 开始初始化应用')
    
    try {
      // 1. 启动连接监听
      await connection.startListening()
      
      // 2. 设置战绩数据管理
      matchDataManager.setupGameFinishedListener()
      
      // 3. 设置断开连接监听
      document.addEventListener('connection-lost', () => {
        console.log('[AppInit] 处理连接丢失事件')
        disconnectionHandler.handleDisconnection()
      })
      
      // 4. 尝试初始连接检查
      await connection.refreshConnection()
      
      console.log('[AppInit] 应用初始化完成')
    } catch (error) {
      console.error('[AppInit] 应用初始化失败:', error)
    }
  }
  
  // 清理资源
  const cleanup = () => {
    connection.stopListening()
  }
  
  return {
    initializeApp,
    cleanup,
    
    // 暴露子模块的状态和方法
    isConnected: connection.isConnected,
    connectionMessage: connection.connectionMessage,
    refreshConnection: connection.refreshConnection,
    fetchMatchHistory: matchDataManager.fetchMatchHistoryWithDebounce
  }
}
