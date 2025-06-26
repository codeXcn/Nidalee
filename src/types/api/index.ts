// API 相关类型定义
export interface GameData {
  id: string
  name: string
  version: string
}

export interface SummonerInfo {
  displayName: string
  level: number
  profileIconId: number
  puuid: string
}

export interface MatchData {
  gameId: number
  gameMode: string
  gameType: string
  participants: Player[]
}

export interface Player {
  summonerName: string
  championId: number
  teamId: number
  kills: number
  deaths: number
  assists: number
}
