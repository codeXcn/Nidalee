interface TeamBan {
  championId: number
  pickTurn: number
}

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
}

interface Participant {
  participantId: number
  championId: number
  championName?: string
  teamId: number
  stats?: ParticipantStats
  rankTier?: string
}

interface GameDetailData {
  gameId: number
  gameDuration: number
  gameMode: string
  gameVersion: string
  mapId: number
  queueId: number
  teams: Team[]
  participants: Participant[]
  participantIdentities: Array<{
    participantId: number
    player: {
      summonerName: string
      profileIcon: number
      gameName?: string
      tagLine?: string
    }
  }>
  blueTeamStats?: TeamStats
  redTeamStats?: TeamStats
  bestPlayerChampionId?: number
  maxDamage?: number
  maxTankChampionId?: number
  maxTank?: number
  maxStreakChampionId?: number
  maxStreak?: number
  [key: string]: any
}

interface TeamStats {
  kills: number
  deaths: number
  assists: number
  gold_earned: number
  total_damage_dealt_to_champions: number
  vision_score: number
}
// 类型定义
interface RecentGame {
  game_id: number
  champion_name: string
  game_mode: string
  game_creation: number
  game_duration: number
  kills: number
  deaths: number
  assists: number
  win: boolean
}

interface Team {
  teamId: number
  win: string
  bans: TeamBan[]
}

interface TeamBan {
  championId: number
  pickTurn: number
}

interface Participant {
  participantId: number
  teamId: number
  championId: number
  championName?: string
  spell1Id: number
  spell2Id: number
  stats: ParticipantStats
  rankTier?: string
  [key: string]: any
}

interface ParticipantStats {
  participantId: number
  win: boolean
  kills: number
  deaths: number
  assists: number
  champLevel: number
  totalDamageDealtToChampions: number
  totalDamageTaken: number
  trueDamageDealtToChampions: number
  visionScore: number
  timeCCingOthers: number
  totalDamageShieldedOnTeammates: number
  totalHealsOnTeammates: number
  totalUnitsHealed: number
  goldEarned: number
  goldSpent: number
  turretKills: number
  totalMinionsKilled: number
  neutralMinionsKilled: number
  visionWardsBoughtInGame: number
  sightWardsPlaced: number
  wardsPlaced: number
  wardsKilled: number
  firstBloodKill: boolean
  firstBloodAssist: boolean
  firstTowerKill: boolean
  firstTowerAssist: boolean
  firstInhibitorKill: boolean
  firstInhibitorAssist: boolean
  item0: number
  item1: number
  item2: number
  item3: number
  item4: number
  item5: number
  item6: number
  [key: string]: unknown
}

interface GameDetailData {
  gameId: string
  mapId: number
  queueId: number
  gameVersion: string
  participants: Participant[]
  teams: Team[]
  gameCreation: number
  gameDuration: number
  gameMode: string
  gameType: string
  platformId: string
  [key: string]: any
}

interface ParticipantIdentity {
  participantId: number
  player: Player
}

interface Player {
  summonerName: string
  profileIcon: number
  gameName?: string
  tagLine?: string
}

interface TeamStats {
  kills: number
  gold_earned: number
  total_damage_dealt_to_champions: number
  vision_score: number
}

// 格式化挑战点数
const formatChallengePoints = (points: string): string => {
  const num = parseInt(points)
  if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}k`
  }
  return points
}

interface Column {
  key: string
  label: string
  class: string
}
interface PerkInfo {
  id: number
  iconPath: string
  name: string
}
