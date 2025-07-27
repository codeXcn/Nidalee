import { invoke } from '@tauri-apps/api/core'

export function useMatchmaking() {
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

  // 只监听，不解绑
  // onMounted(async () => {
  //   await listen<MatchmakingState>('matchmaking-state-changed', (event) => {
  //     console.log('[Event] matchmaking-state-changed:', event.payload)
  //     const payload = event.payload
  //     matchmakingState.value = payload
  //     if (payload.searchState === 'Found' || payload.searchState === 'ReadyCheck') {
  //       handleAutoAcceptMatch()
  //     }
  //     // 当匹配状态为 Invalid 且有选人阶段信息时，清除选人阶段信息
  //     if (payload.searchState === 'Invalid' && gameStore.champSelectSession) {
  //       gameStore.updateChampSelectSession(null)
  //       console.log('清除选人阶段信息')
  //     }
  //   })

  //   await listen<MatchInfo>('match-info-change', (event) => {
  //     console.log('match-info-change', event.payload)
  //     matchInfo.value = event.payload
  //   })
  // })

  return {
    matchmakingState,
    matchInfo,
    handleMatchmaking,
    handleAcceptMatch,
    handleDeclineMatch
  }
}
