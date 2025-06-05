import { LCPClient, GameflowPhase, ChampSelectSession, ChampSelectPlayer } from './lcpClient'

export interface GameConfig {
  // 自动接受对局
  autoAccept: boolean
  // 自动选择英雄
  autoPickChampion: {
    enabled: boolean
    championId: number
  }
  // 自动禁用英雄
  autoBanChampion: {
    enabled: boolean
    championId: number
  }
  // 自动符文配置
  autoRune: {
    enabled: boolean
    pageId: number
    page: any
  }
}

export class GameService {
  private client: LCPClient
  private config: GameConfig
  private gameflowInterval: NodeJS.Timeout | null = null
  private champSelectInterval: NodeJS.Timeout | null = null

  constructor(client: LCPClient, config: GameConfig) {
    this.client = client
    this.config = config
  }

  // 开始监听游戏状态
  async start() {
    this.stopAll()
    await this.watchGameflow()
    await this.watchChampSelect()
  }

  // 停止所有监听
  stopAll() {
    if (this.gameflowInterval) {
      clearInterval(this.gameflowInterval)
      this.gameflowInterval = null
    }
    if (this.champSelectInterval) {
      clearInterval(this.champSelectInterval)
      this.champSelectInterval = null
    }
  }

  // 监听游戏流程
  private async watchGameflow() {
    const checkGameflow = async () => {
      try {
        const phase = await this.client.getGameflowPhase()

        // 自动接受对局
        if (this.config.autoAccept && phase.phase === 'ReadyCheck') {
          await this.client.acceptMatchmaking()
        }
      } catch (error) {
        console.error('检查游戏流程出错:', error)
      }
    }

    // 每秒检查一次
    this.gameflowInterval = setInterval(checkGameflow, 1000)
    await checkGameflow()
  }

  // 监听英雄选择
  private async watchChampSelect() {
    let lastCounter = -1

    const checkChampSelect = async () => {
      try {
        const phase = await this.client.getGameflowPhase()
        if (phase.phase !== 'ChampSelect') return

        const session = await this.client.getChampSelectSession()

        // 避免重复处理同一个状态
        if (session.counter === lastCounter) return
        lastCounter = session.counter

        // 处理英雄选择和禁用
        await this.handleChampSelect(session)

        // 处理符文配置
        if (this.config.autoRune.enabled) {
          await this.handleRune()
        }
      } catch (error) {
        console.error('检查英雄选择出错:', error)
      }
    }

    // 每500ms检查一次
    this.champSelectInterval = setInterval(checkChampSelect, 500)
    await checkChampSelect()
  }

  // 处理英雄选择和禁用
  private async handleChampSelect(session: ChampSelectSession) {
    const currentSummoner = await this.client.getCurrentSummoner()

    for (const action of session.actions.flat()) {
      // 跳过已完成的动作
      if (action.completed) continue

      // 找到当前召唤师的动作
      const player = session.myTeam.find(p => p.summonerId === currentSummoner.summonerId)
      if (!player || action.actorCellId !== player.cellId) continue

      // 自动禁用英雄
      if (action.type === 'ban' && this.config.autoBanChampion.enabled) {
        await this.client.banChampion(action.id, this.config.autoBanChampion.championId)
      }

      // 自动选择英雄
      if (action.type === 'pick' && this.config.autoPickChampion.enabled) {
        await this.client.selectChampion(action.id, this.config.autoPickChampion.championId)
      }
    }
  }

  // 处理符文配置
  private async handleRune() {
    const { pageId, page } = this.config.autoRune
    await this.client.updateRunePage(pageId, page)
  }

  // 获取对局信息
  async getGameInfo() {
    const session = await this.client.getChampSelectSession()
    const teamInfo = await Promise.all([
      ...session.myTeam.map(this.getPlayerInfo.bind(this)),
      ...session.theirTeam.map(this.getPlayerInfo.bind(this))
    ])
    return {
      myTeam: teamInfo.slice(0, session.myTeam.length),
      theirTeam: teamInfo.slice(session.myTeam.length)
    }
  }

  // 获取玩家信息
  private async getPlayerInfo(player: ChampSelectPlayer) {
    const summoner = await this.client.getSummonerByName(player.summonerId.toString())
    const ranked = await this.client.getRankedStats(summoner.puuid)
    const history = await this.client.getMatchHistory(summoner.puuid, 0, 10)

    return {
      summoner,
      ranked,
      history,
      assignedPosition: player.assignedPosition,
      championId: player.championId
    }
  }
}
