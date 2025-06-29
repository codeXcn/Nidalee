import { useGameStore } from '@/stores/features/gameStore'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export function useMatchmaking() {
  const gameStore = useGameStore()

  const matchmakingState = ref<MatchmakingState | null>(null)
  const matchInfo = ref<MatchInfo | null>(null)

  const handleMatchmaking = async () => {
    try {
      if (matchmakingState.value?.searchState === 'Searching') {
        await invoke('stop_matchmaking')
      } else {
        await invoke('start_matchmaking')
      }
    } catch (error) {
      console.error('匹配操作失败:', error)
    }
  }

  const handleAcceptMatch = async () => {
    try {
      await invoke('accept_match')
    } catch (error) {
      console.error('接受对局失败:', error)
    }
  }

  const handleDeclineMatch = async () => {
    try {
      await invoke('decline_match')
    } catch (error) {
      console.error('拒绝对局失败:', error)
    }
  }

  let unlistenMatchmakingState: any
  let unlistenMatchInfo: any

  onMounted(async () => {
    unlistenMatchmakingState = await listen('matchmaking-state-changed', (event) => {
      console.log('[Event] matchmaking-state-changed:', event.payload)
      const payload = event.payload as MatchmakingState
      matchmakingState.value = payload

      // 当匹配状态为 Invalid 且有选人阶段信息时，清除选人阶段信息
      if (payload.searchState === 'Invalid' && gameStore.champSelectSession) {
        gameStore.updateChampSelectSession(null)
        console.log('清除选人阶段信息')
      }
    })

    unlistenMatchInfo = await listen('match-info-change', (event) => {
      console.log('match-info-change', event.payload)
      matchInfo.value = event.payload as MatchInfo
    })
  })

  onUnmounted(() => {
    unlistenMatchmakingState && unlistenMatchmakingState()
    unlistenMatchInfo && unlistenMatchInfo()
  })

  return {
    matchmakingState,
    matchInfo,
    handleMatchmaking,
    handleAcceptMatch,
    handleDeclineMatch
  }
}
