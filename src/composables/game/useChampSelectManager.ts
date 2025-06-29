import { useGameStatusStore, useActivityStore } from '@/stores'

// 专门处理选人和房间逻辑
export function useChampSelectManager() {
  const gameStatusStore = useGameStatusStore()
  const activityStore = useActivityStore()

  // 选人阶段变更处理
  const handleChampSelectChange = async (session: ChampSelectSession | null) => {
    await gameStatusStore.updateChampSelectSession(session)
    if (session) {
      activityStore.addGameActivity.champSelect()
    } else {
      activityStore.addActivity('info', '离开英雄选择阶段', 'game')
    }
  }

  // 房间变更处理
  const handleLobbyChange = (lobby: LobbyInfo | null) => {
    gameStatusStore.updateLobbyInfo(lobby)
    if (lobby) {
      activityStore.addActivity('info', `进入房间: ${lobby.partyType} (${lobby.members.length}人)`, 'game')
    } else {
      activityStore.addActivity('info', '离开房间', 'game')
    }
  }

  return {
    handleChampSelectChange,
    handleLobbyChange
  }
}
