import { useActivityLogger } from '@/composables/utils/useActivityLogger'
import { useConnectionStore } from '@/stores/core/connectionStore'
import { useDataStore } from '@/stores/core/dataStore'
import { useGameStore } from '@/stores/features/gameStore'

// 专门处理断开连接的清理逻辑
export function useDisconnectionHandler() {
  const connectionStore = useConnectionStore()
  const dataStore = useDataStore()
  const gameStore = useGameStore()
  const activityLogger = useActivityLogger()

  // 断开连接时的清理
  const handleDisconnection = () => {
    console.log('[🔌 Disconnection Handler] 处理断开连接...')
    connectionStore.clearAuthInfo()
    dataStore.clearSummonerInfo()
    gameStore.resetGameState()
    activityLogger.logConnection.disconnected()
  }

  return {
    handleDisconnection
  }
}
