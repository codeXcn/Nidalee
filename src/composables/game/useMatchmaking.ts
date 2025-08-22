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

  return {
    matchmakingState,
    matchInfo,
    handleMatchmaking,
    handleAcceptMatch,
    handleDeclineMatch
  }
}
