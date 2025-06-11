// src/types/champSelect.ts
export interface ChampSelectPlayer {
  cellId: number
  summonerId?: number
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

export interface ChampSelectBans {
  myTeamBans: number[]
  theirTeamBans: number[]
}

export interface ChampSelectSession {
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
