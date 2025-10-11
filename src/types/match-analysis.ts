// 对局分析模块专用类型定义

export type GamePhase = 'None' | 'Lobby' | 'Matchmaking' | 'ChampSelect' | 'InProgress' | 'EndOfGame'

export interface PlayerData {
  cellId: number
  displayName: string
  championId: number | null
  championName: string | null
  position: string | null
  puuid: string | null
  summonerId: string | null
  isLocal: boolean
  isBot: boolean
  spells: [number | null, number | null]
  tier: string | null
}

export interface TeamData {
  players: PlayerData[]
  localPlayerCellId: number
}

// PlayerAnalysisData 已移除，使用全局类型 MatchStatistics 替代
// EnrichedPlayerAnalysisData 重命名为 EnrichedMatchStatistics 保持一致性
export interface EnrichedMatchStatistics extends MatchStatistics {
  displayName: string
}

export interface ChampionStats {
  championId: number
  gamesPlayed: number
  wins: number
  winRate: number
}

export interface RecentGame {
  gameId: number
  championId: number
  queueId: number
  gameMode: string
  win: boolean
  kills: number
  deaths: number
  assists: number
  gameDuration: number
  gameCreation: number
  performanceRating: string
}

export interface EnemyChampionPick {
  cellId: number
  championId: number | null
}

export interface MatchAnalysisState {
  phase: GamePhase
  isConnected: boolean
  isLoading: boolean
  myTeam: TeamData | null
  myTeamStats: EnrichedMatchStatistics[]
  enemyTeam: TeamData | null
  enemyTeamStats: EnrichedMatchStatistics[]
  enemyChampionPicks: EnemyChampionPick[]
}
