import { useGameStatusStore, useSummonerStore } from '@/stores'

export function useChampSelectSession() {
  const gameStatusStore = useGameStatusStore()
  const summonerStore = useSummonerStore()
  const { getChampionIconUrl, getProfileIconUrl, getRankIconUrl } = useGameAssets()

  const error = ref<string | null>(null)

  // 重试获取 session
  const retry = () => {
    error.value = null
    // 触发重新获取 session 的事件
    window.dispatchEvent(new CustomEvent('retry-champ-select-session'))
  }

  // 富集玩家数据
  function enrichPlayer(player: ChampSelectPlayer, localPlayerCellId: number): any {
    // 机器人
    if (player.summonerId === '0') {
      return {
        ...player,
        displayName: '未知召唤师',
        avatar: getProfileIconUrl(0),
        isBot: true,
        isLocal: false,
        tier: undefined
      }
    }
    // 本地玩家
    const isLocal = player.cellId === localPlayerCellId
    let summonerInfo = null
    if (isLocal) {
      summonerInfo = summonerStore.summonerInfo
    }
    return {
      ...player,
      displayName: summonerInfo?.displayName || player.displayName || '未知玩家',
      avatar: getProfileIconUrl(summonerInfo?.profileIconId || 0),
      tier: summonerInfo?.soloRankTier || player.tier,
      isLocal,
      isBot: false,
      spell1Icon: player.spell1Id ? getProfileIconUrl(player.spell1Id) : undefined,
      spell2Icon: player.spell2Id ? getProfileIconUrl(player.spell2Id) : undefined,
      championIcon: player.championId ? getChampionIconUrl(player.championId) : undefined,
      rankIcon:
        summonerInfo?.soloRankTier || player.tier
          ? getRankIconUrl(summonerInfo?.soloRankTier || player?.tier || '')
          : undefined
    }
  }

  // 富集 session
  const enrichedSession = computed(() => {
    const session = gameStatusStore.currentChampSelectSession
    if (!session) return null
    const localPlayerCellId = session.localPlayerCellId
    return {
      ...session,
      myTeam: session.myTeam.map((p: ChampSelectPlayer) => enrichPlayer(p, localPlayerCellId)),
      theirTeam: session.theirTeam.map((p: ChampSelectPlayer) => enrichPlayer(p, localPlayerCellId))
      // 其他字段原样返回
    }
  })

  const session = computed(() => gameStatusStore.currentChampSelectSession)
  const loading = computed(() => !gameStatusStore.currentChampSelectSession)

  return {
    session,
    enrichedSession,
    loading,
    error,
    retry
  }
}
