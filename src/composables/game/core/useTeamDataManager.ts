import { useInGameData } from '../phases/useInGameData'
import { useDataFetcher } from '../data/useDataFetcher'
import { useMatchAnalysisStore } from '@/stores/features/matchAnalysisStore'
import type { TeamData, UIPlayerData } from '@/types/match-analysis'

// 类型别名，便于向后兼容
type PlayerData = UIPlayerData

/**
 * 团队数据管理器
 * 职责：处理我方和敌方团队数据的获取、处理和更新
 */
export function useTeamDataManager() {
  const matchAnalysisStore = useMatchAnalysisStore()
  const inGameData = useInGameData()
  const dataFetcher = useDataFetcher()

  // === 辅助函数 ===
  const getLocalPlayerName = (): string | undefined => {
    // TODO: 从 gameStore 获取本地玩家信息
    return undefined
  }

  const resolvePlayerName = (displayName: string): string => {
    if (displayName.includes('#')) return displayName
    return displayName
  }

  // === 注意：processMyTeamData 降级方案已移除 ===
  // 现在完全依赖后端数据处理，如果后端数据不可用则显示 Loading 状态

  // === 敌方队伍数据处理 ===
  const processEnemyTeamData = async (signal?: AbortSignal) => {
    console.log('[TeamDataManager] 🎮 InProgress 阶段：开始处理双方队伍数据...')

    // 启动 LiveClient 监听
    const localPlayerName = getLocalPlayerName()
    await inGameData.resetState()

    if (signal?.aborted) return

    await inGameData.getLivePlayers(localPlayerName)
    await inGameData.startAvailabilityCheck(localPlayerName)

    console.log('[TeamDataManager] LiveClient 数据获取完成，开始处理队伍数据')

    // 🚨 InProgress 阶段：先处理我方队伍数据
    if (inGameData.myTeamPlayers.value.length > 0 && !signal?.aborted) {
      const myTeamData: TeamData = {
        players: inGameData.myTeamPlayers.value.map(
          (player: any): PlayerData => ({
            cellId: 0,
            displayName: player.displayName,
            summonerId: player.summonerId,
            puuid: null,
            isLocal: player.displayName === localPlayerName,
            isBot: player.isBot,
            championId: player.championId,
            championName: player.championName,
            championPickIntent: null,
            position: player.assignedPosition,
            tier: null,
            profileIconId: null,
            tagLine: null,
            spell1Id: player.spell1Id,
            spell2Id: player.spell2Id,
            spells: [player.spell1Id, player.spell2Id],
            matchStats: null
          })
        ),
        localPlayerCellId: 0
      }

      console.log('[TeamDataManager] 我方队伍数据已构建 (InProgress):', {
        playerCount: myTeamData.players.length,
        players: myTeamData.players.map((p) => `${p.displayName}(${p.championName})`)
      })
      matchAnalysisStore.setMyTeamData(myTeamData)

      // 🚨 InProgress 阶段：获取我方战绩
      // 注意: index 必须对应 teamData.players 数组中的索引位置,因为 UI 使用该索引访问 teamStats
      const myRealPlayers = myTeamData.players
        .map((p: PlayerData, arrayIndex: number) => ({
          player: p,
          arrayIndex
        }))
        .filter((item) => !item.player.isBot && item.player.displayName && item.player.displayName !== '未知召唤师')
        .map((item) => ({
          summonerName: item.player.displayName,
          isBot: false,
          index: item.arrayIndex // 保持在 teamData.players 中的原始索引
        }))

      console.log('[TeamDataManager] 我方真人玩家 (InProgress):', myRealPlayers.length, myRealPlayers)

      if (myRealPlayers.length > 0 && !signal?.aborted) {
        try {
          console.log('[TeamDataManager] 开始获取我方战绩 (InProgress)...')
          await dataFetcher.fetchTeamMatchHistory(myRealPlayers, [], 10, { signal })
          matchAnalysisStore.setMyTeamStats(dataFetcher.summonerStats.value)
          console.log(
            '[TeamDataManager] ✅ 我方战绩获取成功 (InProgress), 战绩数量:',
            dataFetcher.summonerStats.value.length
          )
        } catch (error) {
          console.error('[TeamDataManager] ❌ 我方战绩获取失败 (InProgress):', error)
          matchAnalysisStore.setMyTeamStats([])
        }
      }
    }

    // 处理敌方队伍数据
    console.log('[TeamDataManager] 开始处理敌方队伍数据', {
      totalPlayers: inGameData.processedPlayers.value.length,
      myTeamPlayers: inGameData.myTeamPlayers.value.length,
      enemyTeamPlayers: inGameData.enemyTeamPlayers.value.length
    })

    if (inGameData.enemyTeamPlayers.value.length > 0 && !signal?.aborted) {
      const enemyTeamData: TeamData = {
        players: inGameData.enemyTeamPlayers.value.map(
          (player: any): PlayerData => ({
            cellId: 0,
            displayName: player.displayName,
            summonerId: player.summonerId,
            puuid: null,
            isLocal: false,
            isBot: player.isBot,
            championId: player.championId,
            championName: player.championName,
            championPickIntent: null,
            position: player.assignedPosition,
            tier: null,
            profileIconId: null,
            tagLine: null,
            spell1Id: player.spell1Id,
            spell2Id: player.spell2Id,
            spells: [player.spell1Id, player.spell2Id],
            matchStats: null
          })
        ),
        localPlayerCellId: -1
      }

      console.log('[TeamDataManager] 敌方队伍数据已构建:', {
        playerCount: enemyTeamData.players.length,
        players: enemyTeamData.players.map((p) => `${p.displayName}(${p.championName})`)
      })
      matchAnalysisStore.setEnemyTeamData(enemyTeamData)

      // 获取敌方战绩
      // 注意: index 必须对应 teamData.players 数组中的索引位置,因为 UI 使用该索引访问 teamStats
      const enemyRealPlayers = enemyTeamData.players
        .map((p: PlayerData, arrayIndex: number) => ({
          player: p,
          arrayIndex
        }))
        .filter((item) => !item.player.isBot && item.player.displayName && !item.player.displayName.includes('电脑'))
        .map((item) => ({
          summonerName: resolvePlayerName(item.player.displayName),
          isBot: false,
          index: item.arrayIndex // 保持在 teamData.players 中的原始索引
        }))

      if (enemyRealPlayers.length > 0 && !signal?.aborted) {
        try {
          await dataFetcher.fetchTeamMatchHistory([], enemyRealPlayers, 10, { signal })
          matchAnalysisStore.setEnemyTeamStats(dataFetcher.theirTeamStats.value)
        } catch (error) {
          console.warn('[TeamDataManager] 敌方战绩获取失败:', error)
          matchAnalysisStore.setEnemyTeamStats([])
        }
      }
    }
  }

  // === 清理方法 ===
  const cleanup = () => {
    inGameData.resetState()
  }

  return {
    processEnemyTeamData,
    cleanup
  }
}
