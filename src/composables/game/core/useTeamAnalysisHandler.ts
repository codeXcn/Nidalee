import { onMounted, onBeforeUnmount } from 'vue'
import { useMatchAnalysisStore } from '@/stores/features/matchAnalysisStore'
import type { EnrichedPlayerMatchStats, EnemyChampionPick } from '@/types/match-analysis'

/**
 * 专门的团队分析数据处理器 - 简化版
 * 🎉 直接使用后端 TeamAnalysisData，无需复杂适配
 */
export function useTeamAnalysisHandler() {
  const matchAnalysisStore = useMatchAnalysisStore()

  // 处理后端分析数据
  const handleBackendAnalysisData = (event: Event) => {
    const customEvent = event as CustomEvent<TeamAnalysisData | null>
    const data = customEvent.detail

    // 🔍 详细日志：打印收到的数据结构
    console.log('[TeamAnalysisHandler] === 收到后端数据 ===')
    console.log('[TeamAnalysisHandler] 数据类型:', typeof data)
    console.log('[TeamAnalysisHandler] 是否为null:', data === null)
    if (data) {
      console.log('[TeamAnalysisHandler] 数据字段:', Object.keys(data))
      console.log('[TeamAnalysisHandler] myTeam长度:', data.myTeam?.length || 0)
      console.log('[TeamAnalysisHandler] enemyTeam长度:', data.enemyTeam?.length || 0)
      console.log('[TeamAnalysisHandler] 完整数据:', JSON.stringify(data, null, 2))

      // 检查我方队伍中的段位信息
      if (data.myTeam && data.myTeam.length > 0) {
        console.log('[TeamAnalysisHandler] 我方玩家段位信息:')
        data.myTeam.forEach((player, i) => {
          console.log(`  [${i}] ${player.displayName}: tier=${player.tier}, isBot=${player.isBot}`)
        })
      }

      // 检查敌方队伍中的段位信息
      if (data.enemyTeam && data.enemyTeam.length > 0) {
        console.log('[TeamAnalysisHandler] 敌方玩家段位信息:')
        data.enemyTeam.forEach((player, i) => {
          console.log(`  [${i}] ${player.displayName}: tier=${player.tier}, isBot=${player.isBot}`)
        })
      }
    }
    console.log('[TeamAnalysisHandler] ========================')

    if (data) {
      // 将 TeamAnalysisData 转换为 Store 需要的 TeamData 格式
      const myTeamData = {
        players: data.myTeam.map((player) => ({
          cellId: player.cellId,
          displayName: player.displayName,
          summonerId: player.summonerId,
          puuid: player.puuid,
          isLocal: player.isLocal,
          isBot: player.isBot,
          championId: player.championId,
          championName: player.championName,
          championPickIntent: player.championPickIntent,
          position: player.position,
          tier: player.tier,
          profileIconId: player.profileIconId,
          tagLine: player.tagLine,
          spell1Id: player.spell1Id,
          spell2Id: player.spell2Id,
          spells: [player.spell1Id || 0, player.spell2Id || 0] as [number, number],
          matchStats: player.matchStats
        })),
        localPlayerCellId: data.localPlayerCellId
      }

      const enemyTeamData = {
        players: data.enemyTeam.map((player) => ({
          cellId: player.cellId,
          displayName: player.displayName,
          summonerId: player.summonerId,
          puuid: player.puuid,
          isLocal: player.isLocal,
          isBot: player.isBot,
          championId: player.championId,
          championName: player.championName,
          championPickIntent: player.championPickIntent,
          position: player.position,
          tier: player.tier,
          profileIconId: player.profileIconId,
          tagLine: player.tagLine,
          spell1Id: player.spell1Id,
          spell2Id: player.spell2Id,
          spells: [player.spell1Id || 0, player.spell2Id || 0] as [number, number],
          matchStats: player.matchStats
        })),
        localPlayerCellId: -1
      }

      // 生成 UI 需要的扩展数据
      const myTeamStats: EnrichedPlayerMatchStats[] = data.myTeam
        .map((player) => (player.matchStats ? { displayName: player.displayName, ...player.matchStats } : null))
        .filter(Boolean) as EnrichedPlayerMatchStats[]

      const enemyTeamStats: EnrichedPlayerMatchStats[] = data.enemyTeam
        .map((player) => (player.matchStats ? { displayName: player.displayName, ...player.matchStats } : null))
        .filter(Boolean) as EnrichedPlayerMatchStats[]

      const enemyChampionPicks: EnemyChampionPick[] = data.enemyTeam.map((player) => ({
        cellId: player.cellId,
        championId: player.championId,
        championPickIntent: player.championPickIntent
      }))

      // 使用现有的 Store 方法
      matchAnalysisStore.setMyTeamData(myTeamData)
      matchAnalysisStore.setEnemyTeamData(enemyTeamData)
      matchAnalysisStore.setMyTeamStats(myTeamStats)
      matchAnalysisStore.setEnemyTeamStats(enemyTeamStats)
      matchAnalysisStore.setEnemyChampionPicks(enemyChampionPicks)
      matchAnalysisStore.setQueueInfo(Number(data.queueId), data.isCustomGame)
    } else {
      // 清空数据
      matchAnalysisStore.clearAllData()
    }
  }

  onMounted(async () => {
    console.log('[TeamAnalysisHandler] 开始初始化，监听 backend-analysis-data 事件')
    document.addEventListener('backend-analysis-data', handleBackendAnalysisData)

    // 尝试从缓存恢复数据
    try {
      console.log('[TeamAnalysisHandler] 🔄 尝试从后端缓存恢复数据...')
      const { invoke } = await import('@tauri-apps/api/core')
      const cachedData = await invoke<TeamAnalysisData | null>('get_cached_analysis_data')

      console.log('[TeamAnalysisHandler] 缓存数据结果:', {
        hasData: !!cachedData,
        type: typeof cachedData,
        isNull: cachedData === null
      })

      if (cachedData) {
        console.log('[TeamAnalysisHandler] ✅ 找到缓存数据，开始恢复...')
        // 使用相同的处理逻辑恢复数据
        handleBackendAnalysisData(new CustomEvent('backend-analysis-data', { detail: cachedData }))
      } else {
        console.log('[TeamAnalysisHandler] ❌ 没有缓存数据')
      }
    } catch (error) {
      console.warn('[TeamAnalysisHandler] 缓存数据加载失败:', error)
    }
  })

  onBeforeUnmount(() => {
    document.removeEventListener('backend-analysis-data', handleBackendAnalysisData)
  })

  return {
    // 可以暴露一些方法供外部使用
  }
}
