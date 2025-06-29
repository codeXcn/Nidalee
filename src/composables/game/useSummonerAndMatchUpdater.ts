import { useActivityStore } from '@/stores/core/activityStore'
import { useDataStore } from '@/stores/core/dataStore'
import { invoke } from '@tauri-apps/api/core'

/**
 * 统一更新召唤师信息和战绩信息
 */
export function useSummonerAndMatchUpdater() {
  const dataStore = useDataStore()
  const activityStore = useActivityStore()

  // 单独：更新召唤师信息
  const updateSummonerInfo = async () => {
    try {
      dataStore.startLoadingSummoner()
      const summonerInfo = await invoke('get_current_summoner')
      if (summonerInfo) {
        dataStore.setSummonerInfo(summonerInfo)
        activityStore.addActivity('info', '召唤师信息已更新', 'data')
      }
    } catch (error) {
      console.error('[Updater] 获取召唤师信息失败:', error)
      dataStore.clearSummonerInfo()
    }
  }

  // 单独：更新战绩信息
  const updateMatchHistory = async () => {
    try {
      dataStore.startLoadingMatchHistory()
      const matchHistory = await invoke('get_match_history')
      if (matchHistory) {
        dataStore.setMatchStatistics(matchHistory)
        activityStore.addActivity('success', '对局历史记录已更新', 'data')
      }
    } catch (error) {
      console.error('[Updater] 获取对局历史失败:', error)
      dataStore.clearMatchHistory()
      activityStore.addActivity('error', '获取对局历史失败', 'error')
    }
  }

  // 组合：同时更新
  const updateSummonerAndMatches = async () => {
    await updateSummonerInfo()
    await updateMatchHistory()
  }

  return {
    updateSummonerAndMatches,
    updateSummonerInfo,
    updateMatchHistory
  }
}
