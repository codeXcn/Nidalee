import { ref, computed } from 'vue'
import { useGameStore } from '@/stores/gameStore'
import { useGameAssets } from '@/hooks/useGameAssets'
import type { ChampSelectPlayer } from 'types/global'

export function useChampSelectSession() {
  const gameStore = useGameStore()
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
        displayName: '机器人',
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
      summonerInfo = gameStore.summonerInfo
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
          ? getRankIconUrl(summonerInfo?.soloRankTier || player.tier)
          : undefined
    }
  }

  // 富集 session
  const enrichedSession = computed(() => {
    const session = gameStore.currentChampSelectSession
    if (!session) return null
    const localPlayerCellId = session.localPlayerCellId
    return {
      ...session,
      myTeam: session.myTeam.map((p: ChampSelectPlayer) => enrichPlayer(p, localPlayerCellId)),
      theirTeam: session.theirTeam.map((p: ChampSelectPlayer) => enrichPlayer(p, localPlayerCellId))
      // 其他字段原样返回
    }
  })
  const session = computed(() => gameStore.currentChampSelectSession)
  const loading = computed(() => !gameStore.currentChampSelectSession)
  return {
    session,
    enrichedSession,
    loading,
    error,
    retry
  }
}
