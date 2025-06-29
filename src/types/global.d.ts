// 全局通用类型定义，整理去重

// 选手基础信息
interface Player {
  summonerName: string
  profileIcon: number
  gameName?: string
  tagLine?: string
}

// 参赛者详细信息
interface Participant {
  participantId: number
  teamId: number
  championId: number
  championName?: string
  spell1Id?: number
  spell2Id?: number
  stats?: ParticipantStats
  rankTier?: string
  score?: number
  [key: string]: any
}

// 参赛者统计
interface ParticipantStats {
  champLevel?: number
  kills?: number
  deaths?: number
  assists?: number
  goldEarned?: number
  totalDamageDealtToChampions?: number
  item0?: number
  item1?: number
  item2?: number
  item3?: number
  item4?: number
  item5?: number
  item6?: number
  win?: boolean
  [key: string]: any
}

// 队伍信息
interface Team {
  teamId: number
  win: string
  bans?: TeamBan[]
  [key: string]: any
}

// 队伍统计
interface TeamStats {
  kills: number
  deaths?: number
  assists?: number
  gold_earned: number
  total_damage_dealt_to_champions: number
  vision_score: number
  [key: string]: any
}

// 对局详细数据
interface GameDetailData {
  gameId: number | string
  gameDuration: number
  gameMode: string
  gameVersion: string
  mapId: number
  queueId: number
  teams: Team[]
  participants: Participant[]
  participantIdentities?: Array<{
    participantId: number
    player: Player
  }>
  blueTeamStats?: TeamStats
  redTeamStats?: TeamStats
  bestPlayerChampionId?: number
  maxDamage?: number
  maxTankChampionId?: number
  maxTank?: number
  maxStreakChampionId?: number
  maxStreak?: number
  gameCreation?: number
  gameType?: string
  platformId?: string
  [key: string]: any
}

// 队伍禁用
interface TeamBan {
  championId: number
  pickTurn: number
}

// 匹配相关
interface MatchmakingError {
  errorType: string
  id: number
  message: string
  penalizedSummonerId?: number
  penaltyTimeRemaining?: number
}

interface PlayerInfo {
  summonerName: string
  championId: number
  teamId: number
}

interface MatchInfo {
  matchId: string
  players: PlayerInfo[]
}

interface LobbyInfo {
  id: string
  partyType: string
  members: Array<{ summonerId: number; displayName: string }>
}

interface ChampSelectSession {
  localPlayerCellId: number
  myTeam: ChampSelectPlayer[]
  theirTeam: ChampSelectPlayer[]
  bans: ChampSelectBans
  timer: { phase: string }
  myScore?: number
  theirScore?: number
  suggestions?: string[]
  warnings?: string[]
}

interface ChampSelectPlayer {
  cellId: number
  summonerId?: string
  championId: number
  championPickIntent?: number
  selectedSkinId?: number
  spell1Id?: number
  spell2Id?: number
  assignedPosition?: string
  displayName?: string
  tier?: string
  winRate?: number
}

interface ChampSelectBans {
  myTeamBans: number[]
  theirTeamBans: number[]
}
