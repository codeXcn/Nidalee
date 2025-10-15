// 专门处理选人和房间逻辑
export function useChampSelectManager() {
  const gameStore = useGameStore()
  const activityLogger = useActivityLogger()

  // 选人阶段变更处理
  const handleChampSelectChange = async (session: ChampSelectSession | null) => {
    await gameStore.updateChampSelectSession(session)
    if (session) {
      activityLogger.logGame.champSelect()
    } else {
      activityLogger.log.info('离开英雄选择阶段', 'game')
    }
  }

  // 房间变更处理
  const handleLobbyChange = (lobby: LobbyInfo | null) => {
    gameStore.updateLobbyInfo(lobby)
    if (lobby) {
      activityLogger.log.info(`进入房间: ${lobby.partyType} (${lobby.members.length}人)`, 'game')
    } else {
      activityLogger.log.info('离开房间', 'game')
    }
  }

  return {
    handleChampSelectChange,
    handleLobbyChange
  }
}
