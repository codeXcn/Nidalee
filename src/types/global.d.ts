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
// 定义接口类型
interface GameSession {
  startTime: number
  duration: number
}

interface Activity {
  id: string
  type: 'success' | 'info' | 'warning' | 'error'
  message: string
  timestamp: number
  category: 'connection' | 'game' | 'auto' | 'data' | 'settings' | 'error' | 'system'
}

interface GameStatus {
  phase: string
  queue: string | null
  isInGame: boolean
}

interface MatchStatistics {
  total_games: number
  wins: number
  losses: number
  win_rate: number
  avg_kills: number
  avg_deaths: number
  avg_assists: number
  avg_kda: number
  favorite_champions: ChampionStats[]
  recent_performance: RecentGame[]
}

interface ChampionStats {
  champion_name: string
  games_played: number
  wins: number
  win_rate: number
}

interface RecentGame {
  game_id: number
  champion_name: string
  game_mode: string
  win: boolean
  kills: number
  deaths: number
  assists: number
  game_duration: number
  game_creation: number
}

interface LcuAuthInfo {
  app_port: number
  remoting_auth_token: string
}

interface SummonerInfo {
  // 基本信息
  displayName: string
  gameName?: string
  tagLine?: string
  summonerLevel: number
  profileIconId: number
  puuid: string
  accountId: number
  summonerId: number

  // 经验信息
  xpSinceLastLevel: number
  xpUntilNextLevel: number
  percentCompleteForNextLevel?: number

  // 游戏状态
  gameStatus?: string
  availability?: string

  // 挑战系统
  challengePoints?: string
  challengeCrystalLevel?: string

  // 排位信息 - 单人排位
  soloRankTier?: string
  soloRankDivision?: string
  soloRankWins?: number
  soloRankLosses?: number
  soloRankLP?: number

  // 排位信息 - 灵活排位
  flexRankTier?: string
  flexRankDivision?: string
  flexRankWins?: number
  flexRankLosses?: number
  flexRankLP?: number

  // 历史最高排位
  highestRankThisSeason?: string

  // 天赋信息
  currentPerkPage?: string
  primaryStyleId?: number
  subStyleId?: number
  selectedPerkIds?: number[]
}

// 游戏阶段信息
interface GamePhase {
  phase: string
}

// 房间信息
interface LobbyInfo {
  id: string
  partyType: string
  members: LobbyMember[]
}

interface LobbyMember {
  summonerId: number
  displayName: string
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
  // 可扩展：段位、胜率等
  displayName?: string
  tier?: string
  winRate?: number
}

interface ChampSelectBans {
  myTeamBans: number[]
  theirTeamBans: number[]
}

interface ChampSelectSession {
  localPlayerCellId: number
  myTeam: ChampSelectPlayer[]
  theirTeam: ChampSelectPlayer[]
  bans: ChampSelectBans
  timer: { phase: string }
  // 可扩展：双方分数、建议等
  myScore?: number
  theirScore?: number
  suggestions?: string[]
  warnings?: string[]
}
interface MatchmakingError {
  errorType: string
  id: number
  message: string
  penalizedSummonerId: number
  penaltyTimeRemaining: number
}

interface LowPriorityData {
  bustedLeaverAccessToken: string
  penalizedSummonerIds: number[]
  penaltyTime: number
  penaltyTimeRemaining: number
  reason: string
}

interface MatchmakingState {
  errors: MatchmakingError[]
  lowPriorityData: LowPriorityData
  searchState: string
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
interface GameDetailData {
  gameId: number
  gameDuration: number
  gameMode: string
  gameVersion: string
  mapId: number
  queueId: number
  teams: any[]
  participants: any[]
  participantIdentities: any[]
  blueTeamStats: any
  redTeamStats: any
  bestPlayerChampionId?: number
  maxDamage?: number
  maxTankChampionId?: number
  maxTank?: number
  maxStreakChampionId?: number
  maxStreak?: number
}

interface TeamBan {
  championId: number
  pickTurn: number
}

interface Participant {
  participantId: number
  teamId: number
  championId: number
  championName: string
  stats?: any
  rankTier?: string
  score?: number
}

interface MatchmakingError {
  errorType: string
  id: number
  message: string
  penalizedSummonerId: number
  penaltyTimeRemaining: number
}

interface LowPriorityData {
  bustedLeaverAccessToken: string
  penalizedSummonerIds: number[]
  penaltyTime: number
  penaltyTimeRemaining: number
  reason: string
}

interface MatchmakingState {
  errors: MatchmakingError[]
  lowPriorityData: LowPriorityData
  searchState: string
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
