import { invoke } from '@tauri-apps/api/tauri'

export interface LCPConfig {
  token: string
  port: string
}

export interface Summoner {
  accountId: number
  displayName: string
  internalName: string
  percentCompleteForNextLevel: number
  profileIconId: number
  puuid: string
  summonerId: number
  summonerLevel: number
  xpSinceLastLevel: number
  xpUntilNextLevel: number
}

export interface GameflowPhase {
  phase: 'None' | 'Lobby' | 'Matchmaking' | 'ReadyCheck' | 'ChampSelect' | 'InProgress' | 'PreEndOfGame' | 'EndOfGame'
}

export interface ChampSelectSession {
  actions: any[]
  allowBattleBoost: boolean
  allowDuplicatePicks: boolean
  allowLockedEvents: boolean
  allowRerolling: boolean
  allowSkinSelection: boolean
  bans: {
    myTeamBans: number[]
    numBans: number
    theirTeamBans: number[]
  }
  chatDetails: {
    chatRoomName: string
    chatRoomPassword: string
  }
  counter: number
  myTeam: ChampSelectPlayer[]
  theirTeam: ChampSelectPlayer[]
  timer: {
    adjustedTimeLeftInPhase: number
    phase: string
    totalTimeInPhase: number
  }
}

export interface ChampSelectPlayer {
  assignedPosition: string
  cellId: number
  championId: number
  championPickIntent: number
  entitledFeatureType: string
  selectedSkinId: number
  spell1Id: number
  spell2Id: number
  summonerId: number
  team: number
  wardSkinId: number
}

export interface Champion {
  id: number
  name: string
  alias: string
  squarePortraitPath: string
  roles: string[]
  winRate: number
  banRate: number
  pickRate: number
}

export interface RunePage {
  id: number
  name: string
  primaryStyleId: number
  subStyleId: number
  selectedPerkIds: number[]
}

export interface PlayerStats {
  totalGames: number
  wins: number
  kda: number
  mainPositions: string[]
  mainChampions: number[]
  performanceScore: number
}

export interface MatchAnalysis {
  teamScore: number
  compositionScore: number
  laneAdvantages: Record<string, number>
  teamfightScore: number
  suggestedTactics: string[]
}

export interface GameInfo {
  myTeam: TeamPlayer[]
  theirTeam: TeamPlayer[]
}

export interface TeamPlayer {
  summoner: {
    summonerId: number
    displayName: string
  }
  championId: number
  assignedPosition: string
  stats?: PlayerStats
}

export class LCPClient {
  public readonly config: LCPConfig

  constructor(config: LCPConfig) {
    this.config = config
  }

  private getBaseUrl() {
    return `https://127.0.0.1:${this.config.port}`
  }

  private getHeaders() {
    return {
      Authorization: `Basic ${btoa(`riot:${this.config.token}`)}`,
      'Content-Type': 'application/json'
    }
  }

  async get<T = any>(path: string): Promise<T> {
    const res = await fetch(this.getBaseUrl() + path, {
      method: 'GET',
      headers: this.getHeaders()
    })
    if (!res.ok) throw new Error(await res.text())
    return res.json()
  }

  async post<T = any>(path: string, body?: any): Promise<T> {
    const res = await fetch(this.getBaseUrl() + path, {
      method: 'POST',
      headers: this.getHeaders(),
      body: body ? JSON.stringify(body) : undefined
    })
    if (!res.ok) throw new Error(await res.text())
    return res.json()
  }

  async patch<T = any>(path: string, body: any): Promise<T> {
    const res = await fetch(this.getBaseUrl() + path, {
      method: 'PATCH',
      headers: this.getHeaders(),
      body: JSON.stringify(body)
    })
    if (!res.ok) throw new Error(await res.text())
    return res.json()
  }

  async put<T = any>(path: string, body: any): Promise<T> {
    const res = await fetch(this.getBaseUrl() + path, {
      method: 'PUT',
      headers: this.getHeaders(),
      body: JSON.stringify(body)
    })
    if (!res.ok) throw new Error(await res.text())
    return res.json()
  }

  // 基础信息
  getCurrentSummoner(): Promise<Summoner> {
    return this.get('/lol-summoner/v1/current-summoner')
  }

  // 游戏流程
  getGameflowPhase(): Promise<GameflowPhase> {
    return this.get('/lol-gameflow/v1/gameflow-phase')
  }

  getGameflowSession() {
    return this.get('/lol-gameflow/v1/session')
  }

  // 自动接受对局
  acceptMatchmaking() {
    return this.post('/lol-matchmaking/v1/ready-check/accept')
  }

  // 英雄选择
  getChampSelectSession(): Promise<ChampSelectSession> {
    return this.get('/lol-champ-select/v1/session')
  }

  // 选择英雄
  selectChampion(actionId: number, championId: number) {
    return this.patch(`/lol-champ-select/v1/session/actions/${actionId}`, {
      championId,
      completed: true
    })
  }

  // 禁用英雄
  banChampion(actionId: number, championId: number) {
    return this.patch(`/lol-champ-select/v1/session/actions/${actionId}`, {
      championId,
      completed: true
    })
  }

  // 符文页
  getCurrentRunePage() {
    return this.get('/lol-perks/v1/currentpage')
  }

  updateRunePage(pageId: number, page: any) {
    return this.put(`/lol-perks/v1/pages/${pageId}`, page)
  }

  // 战绩查询
  getMatchHistory(puuid: string, begIndex = 0, endIndex = 20) {
    return this.get(`/lol-match-history/v1/products/lol/${puuid}/matches?begIndex=${begIndex}&endIndex=${endIndex}`)
  }

  // 获取召唤师信息
  getSummonerByName(name: string) {
    return this.get(`/lol-summoner/v1/summoners?name=${encodeURIComponent(name)}`)
  }

  // 获取排位信息
  getRankedStats(puuid: string) {
    return this.get(`/lol-ranked/v1/ranked-stats/${puuid}`)
  }

  // 获取所有英雄数据
  async getChampions(): Promise<Champion[]> {
    return await invoke('get_champions')
  }

  // 获取单个英雄数据
  async getChampion(id: number): Promise<Champion> {
    return this.get(`/lol-game-data/assets/v1/champions/${id}`)
  }

  async getGameInfo(): Promise<GameInfo> {
    return await invoke('get_game_info')
  }

  async getRuneSuggestion(championId: number): Promise<RunePage> {
    return await invoke('get_rune_suggestion', { championId })
  }

  async analyzeMatch(gameInfo: GameInfo): Promise<MatchAnalysis> {
    return await invoke('analyze_match', { gameInfo })
  }

  async applyRunePage(runePage: RunePage): Promise<void> {
    await invoke('apply_rune_page', { runePage })
  }
}
