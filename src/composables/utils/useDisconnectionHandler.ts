import { useConnectStore, useSummonerStore, useGameStatusStore, useActivityStore } from '@/stores'

// 专门处理断开连接的清理逻辑
export function useDisconnectionHandler() {
  const connectionStore = useConnectStore()
  const summonerStore = useSummonerStore()
  const gameStatusStore = useGameStatusStore()
  const activityStore = useActivityStore()

  // 断开连接时的清理
  const handleDisconnection = () => {
    connectionStore.clearAuthInfo()
    summonerStore.clearSummonerInfo()
    gameStatusStore.clearGameState()
    activityStore.addActivity('warning', 'LCU 连接已断开')
  }

  return {
    handleDisconnection
  }
}
