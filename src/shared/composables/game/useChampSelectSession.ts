export function useChampSelectSession() {
  const gameStore = useGameStore()
  const dataStore = useDataStore()
  const { getChampionIconUrl, getProfileIconUrl, getRankIconUrl } = useGameAssets()

  const error = ref<string | null>(null)

  // 重试获取 session
  const retry = () => {
    error.value = null
    // 触发重新获取 session 的事件
    window.dispatchEvent(new CustomEvent('retry-champ-select-session'))
  }

  function enrichPlayer(player: ChampSelectPlayer, localPlayerCellId: number): EnrichedChampSelectPlayer {
    // 机器人
    if (player.summonerId === '0') {
      return {
        ...player,
        displayName: '未知召唤师',
        avatar: getProfileIconUrl(0),
        isBot: true,
        isLocal: false,
        tier: null // 保持类型一致
      }
    }
    // 本地玩家
    const isLocal = player.cellId === localPlayerCellId
    let summonerInfo = null
    if (isLocal) {
      summonerInfo = dataStore.summonerInfo
    }
    return {
      ...player,
      displayName: summonerInfo?.displayName || player.displayName || '未知玩家',
      avatar: getProfileIconUrl(summonerInfo?.profileIconId || 0),
      tier: summonerInfo?.soloRankTier || player.tier,
      isLocal,
      isBot: false,
      spell1Icon: player.spell1Id ? getChampionIconUrl(player.spell1Id) : undefined,
      spell2Icon: player.spell2Id ? getChampionIconUrl(player.spell2Id) : undefined,
      championIcon: player.championId ? getChampionIconUrl(player.championId) : undefined,
      rankIcon:
        summonerInfo?.soloRankTier || player.tier
          ? getRankIconUrl(summonerInfo?.soloRankTier || player?.tier || '')
          : undefined
    }
  }

  const enrichedSession = computed(() => {
    const session = gameStore.champSelectSession
    if (!session) return null
    const localPlayerCellId = session.localPlayerCellId
    return {
      ...session,
      myTeam: session.myTeam.map((p: ChampSelectPlayer) => enrichPlayer(p, localPlayerCellId)),
      theirTeam: session.theirTeam.map((p: ChampSelectPlayer) => enrichPlayer(p, localPlayerCellId))
      // 其他字段原样返回
    }
  })

  const session = computed(() => gameStore.champSelectSession)
  const loading = computed(() => !gameStore.champSelectSession)

  return {
    session,
    enrichedSession,
    loading,
    error,
    retry
  }
}
