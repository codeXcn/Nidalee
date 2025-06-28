import { useConnection } from '@/composables/utils/useConnection'
import { invoke } from '@tauri-apps/api/core'
import { useConnectStore } from './connectStore'

export const useSummonerStore = defineStore('summoner', () => {
  // 召唤师信息
  const summonerInfo = ref<SummonerInfo | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // 获取召唤师信息
  const fetchSummonerInfo = async () => {
    loading.value = true
    error.value = null

    try {
      const info = await invoke<SummonerInfo>('get_current_summoner')
      summonerInfo.value = info
      return info
    } catch (err) {
      console.error('获取召唤师信息失败:', err)
      if (err === '认证信息不存在，请检查LCU进程或重试') {
        useConnectStore().checkConnection()
      }
      error.value = err instanceof Error ? err.message : '获取召唤师信息失败'
      summonerInfo.value = null
      throw err
    } finally {
      loading.value = false
    }
  }

  // 更新召唤师信息
  const updateSummonerInfo = (info: SummonerInfo) => {
    summonerInfo.value = info
    error.value = null
  }

  // 清除召唤师信息
  const clearSummonerInfo = () => {
    summonerInfo.value = null
    error.value = null
  }

  // 计算属性
  const displayName = computed(() => summonerInfo.value?.displayName || '')
  const summonerLevel = computed(() => summonerInfo.value?.summonerLevel || 0)
  const profileIconId = computed(() => summonerInfo.value?.profileIconId || 0)

  return {
    // 状态
    summonerInfo: readonly(summonerInfo),
    loading: readonly(loading),
    error: readonly(error),

    // 计算属性
    displayName,
    summonerLevel,
    profileIconId,

    // 方法
    fetchSummonerInfo,
    updateSummonerInfo,
    clearSummonerInfo
  }
})
