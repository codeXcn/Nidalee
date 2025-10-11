import { useInGameData } from '../phases/useInGameData'
import { useDataFetcher } from '../data/useDataFetcher'
import type { PlayerData, TeamData } from '@/types/match-analysis'
import type { useMatchAnalysis } from './useMatchAnalysis'

/**
 * 团队数据管理器
 * 职责：处理我方和敌方团队数据的获取、处理和更新
 */
export function useTeamDataManager(matchAnalysis: ReturnType<typeof useMatchAnalysis>) {
  const inGameData = useInGameData()
  const dataFetcher = useDataFetcher()

  // === 辅助函数 ===
  const getDisplayName = (player: any): string => {
    if (player.gameName && player.tagLine) {
      return `${player.gameName}#${player.tagLine}`
    }
    return player.summonerId || '未知召唤师'
  }

  const getLocalPlayerName = (): string | undefined => {
    // TODO: 从 gameStore 获取本地玩家信息
    return undefined
  }

  const resolvePlayerName = (displayName: string): string => {
    if (displayName.includes('#')) return displayName
    return displayName
  }

  // === 我方队伍数据处理 ===
  const processMyTeamData = async (session: any, signal?: AbortSignal) => {
    // 1. 处理我方队伍数据
    const myTeamData: TeamData = {
      players: session.myTeam.map(
        (player: any): PlayerData => ({
          cellId: player.cellId,
          displayName: getDisplayName(player),
          championId: player.championId,
          championName: player.championName,
          position: player.assignedPosition,
          puuid: player.puuid,
          summonerId: player.summonerId,
          isLocal: player.cellId === session.localPlayerCellId,
          isBot: player.nameVisibilityType === 'HIDDEN' && !player.gameName,
          spells: [player.spell1Id, player.spell2Id],
          tier: player.tier
        })
      ),
      localPlayerCellId: session.localPlayerCellId
    }

    matchAnalysis.setMyTeamData(myTeamData)

    // 2. 获取我方战绩（真人玩家）
    const myRealPlayers = myTeamData.players
      .filter((p: PlayerData) => !p.isBot && p.displayName && p.displayName !== '未知召唤师')
      .map((p: PlayerData) => ({
        summonerName: p.displayName,
        isBot: false,
        index: myTeamData.players.indexOf(p)
      }))

    if (myRealPlayers.length > 0 && !signal?.aborted) {
      try {
        await dataFetcher.fetchTeamMatchHistory(myRealPlayers, [], 10, { signal })
        matchAnalysis.setMyTeamStats(dataFetcher.summonerStats.value)
      } catch (error) {
        console.warn('[TeamDataManager] 我方战绩获取失败:', error)
        matchAnalysis.setMyTeamStats([])
      }
    }
  }

  // === 敌方队伍数据处理 ===
  const processEnemyTeamData = async (signal?: AbortSignal) => {
    // 启动 LiveClient 监听
    const localPlayerName = getLocalPlayerName()
    await inGameData.resetState()

    if (signal?.aborted) return

    await inGameData.getLivePlayers(localPlayerName)
    await inGameData.startAvailabilityCheck(localPlayerName)

    // 处理敌方队伍数据
    if (inGameData.processedPlayers.value.length > 0 && !signal?.aborted) {
      const enemyTeamData: TeamData = {
        players: inGameData.processedPlayers.value.map(
          (player: any): PlayerData => ({
            cellId: 0,
            displayName: player.displayName,
            championId: player.championId,
            championName: player.championName,
            position: player.assignedPosition,
            puuid: null,
            summonerId: player.summonerId,
            isLocal: false,
            isBot: player.isBot,
            spells: [player.spell1Id, player.spell2Id],
            tier: null
          })
        ),
        localPlayerCellId: -1
      }

      matchAnalysis.setEnemyTeamData(enemyTeamData)

      // 获取敌方战绩
      const enemyRealPlayers = enemyTeamData.players
        .filter((p: PlayerData) => !p.isBot && p.displayName && !p.displayName.includes('电脑'))
        .map((p: PlayerData) => ({
          summonerName: resolvePlayerName(p.displayName),
          isBot: false,
          index: enemyTeamData.players.indexOf(p)
        }))

      if (enemyRealPlayers.length > 0 && !signal?.aborted) {
        try {
          await dataFetcher.fetchTeamMatchHistory([], enemyRealPlayers, 10, { signal })
          matchAnalysis.setEnemyTeamStats(dataFetcher.theirTeamStats.value)
        } catch (error) {
          console.warn('[TeamDataManager] 敌方战绩获取失败:', error)
          matchAnalysis.setEnemyTeamStats([])
        }
      }
    }
  }

  // === 清理方法 ===
  const cleanup = () => {
    inGameData.resetState()
  }

  return {
    processMyTeamData,
    processEnemyTeamData,
    cleanup
  }
}
