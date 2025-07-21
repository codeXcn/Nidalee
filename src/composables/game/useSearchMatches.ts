import { invoke } from '@tauri-apps/api/core'

// 专门处理战绩数据获取的 composable
export function useSearchMatches() {
  const loading = ref(true)
  const error = ref('')
  const result = ref<SummonerWithMatches[] | null>(null)
  const currentRestult = ref<SummonerWithMatches | null>(null)

  const searchText = ref('')
  const cunrrentIndex = ref(-1)
  const names = ref<string[]>([])
  async function fetchSummonerInfo(names: string[]): Promise<SummonerWithMatches[] | null> {
    try {
      const matches = await invoke<SummonerWithMatches[]>('get_summoners_and_histories', { names })
      if (Array.isArray(matches) && matches.length > 0) {
        result.value = matches
        // 每次查询成功后，重置索引为0（显示第一个结果）
        cunrrentIndex.value = 0
        // 直接设置当前结果，不依赖watch
        currentRestult.value = matches[0]
        // 查询成功
        console.log('matches', matches)
        return matches
      } else {
        // 查询无结果时清空当前结果
        currentRestult.value = null
      }
      return null
    } catch (e: any) {
      error.value = e?.message || '查询失败'
      currentRestult.value = null
      return null
    } finally {
      loading.value = false
    }
  }
  const onSearch = async () => {
    loading.value = false
    error.value = ''
    result.value = null
    currentRestult.value = null
    cunrrentIndex.value = -1
    if (!searchText.value.trim()) return
    loading.value = true
    try {
      // 支持多个召唤师名，用英文逗号分割
      names.value = searchText.value
        .split(',')
        .map((n) => n.trim())
        .filter(Boolean)
      if (names.value.length === 0) return null
      await fetchSummonerInfo(names.value)
    } catch (e: any) {
      error.value = e.message || '查询失败'
    } finally {
      loading.value = false
    }
  }
  watch(cunrrentIndex, (val) => {
    if (result.value) {
      currentRestult.value = result.value[val]
    }
  })
  return {
    currentRestult,
    names,
    searchText,
    cunrrentIndex,
    onSearch,
    fetchSummonerInfo,
    loading,
    result,
    error
  }
}
