import { ref, onMounted, onUnmounted, computed } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { ChampSelectSession, ChampSelectPlayer } from '../types/champSelect'
import { useGameStore } from '@/stores/gameStore'
import { useGameAssets } from '@/hooks/useGameAssets'

export function useChampSelectSession() {
  const gameStore = useGameStore()
  const { getChampionIconUrl, getSpellIconUrl, getSummonerAvatarUrl, getRankIconUrl } = useGameAssets()

  const session = ref<ChampSelectSession | null>(null)
  const loading = ref(true)
  const error = ref<string | null>(null)
  let unlistenSession: (() => void) | null = null
  let unlistenError: (() => void) | null = null

  // 处理 session 变化
  const handleSessionChange = (event: any) => {
    try {
      console.log('收到选人 session 变化:', event.payload)
      session.value = event.payload
      loading.value = false
      error.value = null
    } catch (e) {
      error.value = '解析 session 数据失败'
      console.error('解析 session 数据失败:', e)
    }
  }

  // 处理 session 错误
  const handleSessionError = (event: any) => {
    error.value = event.payload || '获取 session 失败'
    loading.value = false
  }

  // 重试获取 session
  const retry = () => {
    loading.value = true
    error.value = null
    // 触发重新获取 session 的事件
    window.dispatchEvent(new CustomEvent('retry-champ-select-session'))
  }

  // 设置事件监听器
  const setupListeners = async () => {
    try {
      // 监听 session 变化
      unlistenSession = await listen('champ-select-session-changed', handleSessionChange)
      // 监听 session 错误
      unlistenError = await listen('champ-select-session-error', handleSessionError)
      console.log('选人 session 事件监听器已设置')
    } catch (e) {
      console.error('设置事件监听器失败:', e)
      error.value = '设置事件监听器失败'
    }
  }

  onMounted(() => {
    setupListeners()
  })

  onUnmounted(() => {
    if (unlistenSession) {
      unlistenSession()
    }
    if (unlistenError) {
      unlistenError()
    }
  })

  // 富集玩家数据
  function enrichPlayer(player: ChampSelectPlayer, localPlayerCellId: number): any {
    // 机器人
    if (player.summonerId === '0') {
      return {
        ...player,
        displayName: '机器人',
        avatar: getSummonerAvatarUrl('bot'),
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
    } else {
      // 尝试从缓存获取，或异步补全（可扩展）
      summonerInfo = gameStore.summonerCache?.[player.summonerId ?? ''] || null
    }
    return {
      ...player,
      displayName: summonerInfo?.displayName || player.displayName || '未知玩家',
      avatar: getSummonerAvatarUrl(summonerInfo?.profileIconId || player.summonerId),
      tier: summonerInfo?.tier || player.tier,
      isLocal,
      isBot: false,
      spell1Icon: player.spell1Id ? getSpellIconUrl(player.spell1Id) : undefined,
      spell2Icon: player.spell2Id ? getSpellIconUrl(player.spell2Id) : undefined,
      championIcon: player.championId ? getChampionIconUrl(player.championId) : undefined,
      rankIcon: summonerInfo?.tier || player.tier ? getRankIconUrl(summonerInfo?.tier || player.tier) : undefined
    }
  }

  // 富集 session
  const enrichedSession = computed(() => {
    if (!session.value) return null
    const localPlayerCellId = session.value.localPlayerCellId
    return {
      ...session.value,
      myTeam: session.value.myTeam.map((p) => enrichPlayer(p, localPlayerCellId)),
      theirTeam: session.value.theirTeam.map((p) => enrichPlayer(p, localPlayerCellId))
      // 其他字段原样返回
    }
  })

  return {
    session,
    enrichedSession,
    loading,
    error,
    retry
  }
}
