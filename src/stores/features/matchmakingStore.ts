import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

/**
 * 匹配状态 Store
 * 职责：管理匹配相关的状态数据
 */
export const useMatchmakingStore = defineStore('matchmaking', () => {
  // 状态
  const state = ref<any>(null)

  // 计算属性
  const searchState = computed(() => state.value?.searchState || null)
  const isSearching = computed(() => searchState.value === 'Searching')
  const estimatedQueueTime = computed(() => state.value?.estimatedQueueTime || 0)
  const timeInQueue = computed(() => state.value?.timeInQueue || 0)

  // 操作方法
  function updateState(newState: any) {
    console.log('[MatchmakingStore] 更新匹配状态:', newState)
    state.value = newState
  }

  function clearState() {
    console.log('[MatchmakingStore] 清除匹配状态')
    state.value = null
  }

  return {
    // 状态
    state,

    // 计算属性
    searchState,
    isSearching,
    estimatedQueueTime,
    timeInQueue,

    // 方法
    updateState,
    clearState
  }
})
