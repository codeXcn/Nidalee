import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import { listen } from '@tauri-apps/api/event'

export function useMatchmaking() {
  const matchmakingState = ref<any>(null)
  const matchInfo = ref<MatchInfo | null>(null)
  const router = useRouter()

  // 监听标准化匹配状态事件
  onMounted(async () => {
    try {
      await listen('matchmaking-state-changed', (event) => {
        console.log('[Matchmaking] 收到匹配状态变化:', event.payload)
        matchmakingState.value = event.payload
      })
      console.log('[Matchmaking] 匹配状态监听器已注册')
    } catch (error) {
      console.error('[Matchmaking] 注册匹配状态监听器失败:', error)
    }
  })

  // 监听匹配状态变化，自动跳转到对局分析页面
  watch(
    () => matchmakingState.value?.searchState,
    (newState) => {
      if (newState === 'Searching' && router.currentRoute.value.name !== 'match-analysis') {
        console.log('[Matchmaking] 匹配状态变化为 Searching，自动跳转到对局分析页面')
        router.push({ name: 'match-analysis' })
      }
    }
  )
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
