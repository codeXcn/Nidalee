 
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

interface CachedChampSelectData {
  myTeam: any[]
  theirTeam: any[]
  session: any
}

export const useAnalysisCacheStore = defineStore(
  'analysisCache',
  () => {
    const cachedChampSelectData = ref<CachedChampSelectData | null>(null)
    const cachedAt = ref<number | null>(null)

    const MAX_AGE_MS = 15 * 60 * 1000 // 15分钟兜底，避免跨把污染

    const isExpired = computed(() => {
      if (!cachedAt.value) return true
      return Date.now() - cachedAt.value > MAX_AGE_MS
    })

    function setCachedChampSelectData(data: CachedChampSelectData) {
      cachedChampSelectData.value = data
      cachedAt.value = Date.now()
    }

    function clearCachedChampSelectData() {
      cachedChampSelectData.value = null
      cachedAt.value = null
    }

    function cleanupExpired() {
      if (isExpired.value) {
        clearCachedChampSelectData()
      }
    }

    return {
      cachedChampSelectData,
      cachedAt,
      isExpired,
      setCachedChampSelectData,
      clearCachedChampSelectData,
      cleanupExpired
    }
  },
  { persist: true }
)
