// 数据API模块
export * from './dataApi'

// 主题配置模块
export * from './theme'

// 其他辅助函数
export const getPlayerProfileIcon = (participantId: number, gameDetail: GameDetailData): number => {
  const identity = gameDetail.participantIdentities?.find((id) => id.participantId === participantId)
  return identity?.player?.profileIcon || 0
}

// 游戏相关的工具函数
export const formatGameMode = (mode: string): string => {
  const modeMap: Record<string, string> = {
    CLASSIC: '经典模式',
    ARAM: '大乱斗',
    URF: '无限火力',
    TUTORIAL: '教程',
    ONEFORALL: '克隆大作战',
    ARSR: '极地大乱斗',
    PRACTICETOOL: '训练工具',
    NEXUSBLITZ: '极地大乱斗'
  }
  return modeMap[mode] || mode
}

export const formatDuration = (seconds: number): string => {
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`
}

export const getQueueName = (queueId: number): string => {
  const queueMap: Record<number, string> = {
    400: '5v5 征召模式',
    420: '5v5 排位赛',
    430: '5v5 匹配模式',
    440: '5v5 灵活排位',
    450: '5v5 大乱斗',
    900: '4v4 无限火力',
    1020: '克隆大作战',
    1200: '极地大乱斗'
  }
  return queueMap[queueId] || `队列 ${queueId}`
}

export const getMapName = (mapId: number): string => {
  const mapNames: Record<number, string> = {
    11: '召唤师峡谷',
    12: '嚎哭深渊',
    21: '纽克萨斯对战',
    22: '训练模式'
  }
  return mapNames[mapId] || `地图${mapId}`
}

export const formatNumber = (num: number): string => {
  return num?.toLocaleString() || '0'
}

/**
 * 根据英雄ID获取英雄图标URL
 * @param championId 英雄ID，number或string
 * @returns 英雄图标URL
 */
export const getChampionIconUrl = (championId: number | string | null): string => {
  if (!championId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/${championId}.png`
}
/**
 * 根据英雄别名获取英雄图标URL
 * @param alias 英雄别名
 * @returns 英雄图标URL
 */
export const getChampionIconUrlByAlias = (alias: string): string => {
  if (!alias) return ''
  return `https://game.gtimg.cn/images/lol/act/img/champion/${alias}.png`
}
// 处理 Community Dragon 路径
export const getCommunityDragonUrl = (path: string): string => {
  if (!path) return ''
  // 移除开头的斜杠并构建完整URL
  const cleanPath = path.startsWith('/') ? path.slice(1) : path
  return `https://raw.communitydragon.org/latest/plugins/${cleanPath}`
}

/**
 * 根据玩家头像ID获取头像URL
 * @param iconId 头像ID
 * @returns 头像URL
 */
export const getProfileIconUrl = (iconId: number): string => {
  if (!iconId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/${iconId}.jpg`
}

/**
 * 根据物品ID获取物品图标URL
 * @param itemId 物品ID
 * @param gameVersion 游戏版本
 * @returns 物品图标URL
 * @example
 * import { getItemIconUrl } from '@/lib'
 * const url = getItemIconUrl(3135, '12.23.1')
 * // https://ddragon.leagueoflegends.com/cdn/12.23.1/img/item/3135.png
 */
export const getItemIconUrl = (itemId: number, gameVersion: string): string => {
  if (!itemId || itemId === 0)
    return 'data:image/svg+xml;utf8,<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg"><rect width="32" height="32" rx="6" fill="%23e5e7eb"/><text x="16" y="22" text-anchor="middle" font-size="20" fill="%239ca3af" font-family="Arial, Helvetica, sans-serif">?</text></svg>'
  return `https://ddragon.leagueoflegends.com/cdn/${gameVersion}/img/item/${itemId}.png`
}
/**
 * 根据物品ID获取物品图标URL (腾讯CDN)
 * @param itemId 物品ID
 * @param gameVersion 游戏版本
 * @returns 物品图标URL
 * @example
 * import { getItemIconByCdnUrl } from '@/lib'
 * const url = getItemIconByCdnUrl(3135, '12.23.1')
 * // https://game.gtimg.cn/images/lol/act/img/item/3135.png
 */
export const getItemIconByCdnUrl = (itemId: number): string => {
  if (!itemId || itemId === 0)
    return 'data:image/svg+xml;utf8,<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg"><rect width="32" height="32" rx="6" fill="%23e5e7eb"/><text x="16" y="22" text-anchor="middle" font-size="20" fill="%239ca3af" font-family="Arial, Helvetica, sans-serif">?</text></svg>'
  return `https://game.gtimg.cn/images/lol/act/img/item/${itemId}.png`
}

/**
 * 根据段位tier获取段位图标URL
 * @param tier 段位tier
 * @returns 段位图标URL
 * @example
 * import { getRankIconUrl } from '@/lib'
 * const url = getRankIconUrl('CHALLENGER')
 * // https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-leagues/global/default/images/challenger.png
 */
export const getRankIconUrl = (tier: string): string => {
  if (!tier) return ''
  const tierLower = tier.toLowerCase()
  return `https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-leagues/global/default/images/${tierLower}.png`
}

// 时间相关函数
export const formatRelativeTime = (timestamp: number): string => {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const hours = Math.floor(diff / (1000 * 60 * 60))

  if (hours < 1) {
    return '刚刚'
  } else if (hours < 24) {
    return `${hours}小时前`
  } else {
    const days = Math.floor(hours / 24)
    return `${days}天前`
  }
}

// 游戏数据处理函数
export const getTeamResult = (teamId: string, teams: any[]): string => {
  if (!teams) return ''
  const team = teams.find((t) => t.teamId.toString() === teamId)
  if (!team) return ''
  return team.win === 'Win' ? '胜方' : '败方'
}

export const getTeamParticipants = (teamId: string, gameDetail: any): any[] => {
  if (!gameDetail?.participants) return []
  return gameDetail.participants.filter((p: any) => p.teamId.toString() === teamId)
}

export const getTeamBans = (teamId: string, teams: any[]): any[] => {
  if (!teams) return []
  const team = teams.find((t) => t.teamId.toString() === teamId)
  return team?.bans || []
}

export const getPlayerDisplayName = (participantId: number, gameDetail: any): string => {
  const identity = gameDetail.participantIdentities?.find((id: any) => id.participantId === participantId)
  if (!identity?.player) return '未知玩家'

  const { gameName, tagLine, summonerName } = identity.player
  if (gameName && tagLine) {
    return `${gameName}#${tagLine}`
  }
  return summonerName || '未知玩家'
}

export const getChampionName = (championId: number | string | null): string => {
  if (!championId) return '未选择英雄'
  const championMap: Record<number | string, string> = {
    '1': '黑暗之女',
    '2': '狂战士',
    '3': '正义巨像',
    '4': '卡牌大师',
    '5': '德邦总管',
    '6': '无畏战车',
    '7': '诡术妖姬',
    '8': '猩红收割者',
    '9': '远古恐惧',
    '10': '正义天使',
    '11': '无极剑圣',
    '12': '牛头酋长',
    '13': '符文法师',
    '14': '亡灵战神',
    '15': '战争女神',
    '16': '众星之子',
    '17': '迅捷斥候',
    '18': '麦林炮手',
    '19': '祖安怒兽',
    '20': '雪原双子',
    '21': '赏金猎人',
    '22': '寒冰射手',
    '23': '蛮族之王',
    '24': '武器大师',
    '25': '堕落天使',
    '26': '时光守护者',
    '27': '炼金术士',
    '28': '痛苦之拥',
    '29': '瘟疫之源',
    '30': '死亡颂唱者',
    '31': '虚空恐惧',
    '32': '殇之木乃伊',
    '33': '披甲龙龟',
    '34': '冰晶凤凰',
    '35': '恶魔小丑',
    '36': '祖安狂人',
    '37': '琴瑟仙女',
    '38': '虚空行者',
    '39': '刀锋舞者',
    '40': '风暴之怒',
    '41': '海洋之灾',
    '42': '英勇投弹手',
    '43': '天启者',
    '44': '瓦洛兰之盾',
    '45': '邪恶小法师',
    '48': '巨魔之王',
    '50': '诺克萨斯统领',
    '51': '皮城女警',
    '53': '蒸汽机器人',
    '54': '熔岩巨兽',
    '55': '不祥之刃',
    '56': '永恒梦魇',
    '57': '扭曲树精',
    '58': '荒漠屠夫',
    '59': '德玛西亚皇子',
    '60': '蜘蛛女皇',
    '61': '发条魔灵',
    '62': '齐天大圣',
    '63': '复仇焰魂',
    '64': '盲僧',
    '67': '暗夜猎手',
    '68': '机械公敌',
    '69': '魔蛇之拥',
    '72': '上古领主',
    '74': '大发明家',
    '75': '沙漠死神',
    '76': '狂野女猎手',
    '77': '兽灵行者',
    '78': '圣锤之毅',
    '79': '酒桶',
    '80': '不屈之枪',
    '81': '探险家',
    '82': '铁铠冥魂',
    '83': '牧魂人',
    '84': '离群之刺',
    '85': '狂暴之心',
    '86': '德玛西亚之力',
    '89': '曙光女神',
    '90': '虚空先知',
    '91': '刀锋之影',
    '92': '放逐之刃',
    '96': '深渊巨口',
    '98': '暮光之眼',
    '99': '光辉女郎',
    '101': '远古巫灵',
    '102': '龙血武姬',
    '103': '九尾妖狐',
    '104': '法外狂徒',
    '105': '潮汐海灵',
    '106': '不灭狂雷',
    '107': '傲之追猎者',
    '110': '惩戒之箭',
    '111': '深海泰坦',
    '112': '奥术先驱',
    '113': '北地之怒',
    '114': '无双剑姬',
    '115': '爆破鬼才',
    '117': '仙灵女巫',
    '119': '荣耀行刑官',
    '120': '战争之影',
    '121': '虚空掠夺者',
    '122': '诺克萨斯之手',
    '126': '未来守护者',
    '127': '冰霜女巫',
    '131': '皎月女神',
    '133': '德玛西亚之翼',
    '134': '暗黑元首',
    '136': '铸星龙王',
    '141': '影流之镰',
    '142': '暮光星灵',
    '143': '荆棘之兴',
    '145': '虚空之女',
    '147': '星籁歌姬',
    '150': '迷失之牙',
    '154': '生化魔人',
    '157': '疾风剑豪',
    '161': '虚空之眼',
    '163': '岩雀',
    '164': '青钢影',
    '166': '影哨',
    '200': '虚空女皇',
    '201': '弗雷尔卓德之心',
    '202': '戏命师',
    '203': '永猎双子',
    '221': '祖安花火',
    '222': '暴走萝莉',
    '223': '河流之王',
    '233': '狂厄蔷薇',
    '234': '破败之王',
    '235': '涤魂圣枪',
    '236': '圣枪游侠',
    '238': '影流之主',
    '240': '暴怒骑士',
    '245': '时间刺客',
    '246': '元素女皇',
    '254': '皮城执法官',
    '266': '暗裔剑魔',
    '267': '唤潮鲛姬',
    '268': '沙漠皇帝',
    '350': '魔法猫咪',
    '360': '沙漠玫瑰',
    '412': '魂锁典狱长',
    '420': '海兽祭司',
    '421': '虚空遁地兽',
    '427': '翠神',
    '429': '复仇之矛',
    '432': '星界游神',
    '497': '幻翎',
    '498': '逆羽',
    '516': '山隐之焰',
    '517': '解脱者',
    '518': '万花通灵',
    '523': '残月之肃',
    '526': '镕铁少女',
    '555': '血港鬼影',
    '711': '愁云使者',
    '777': '封魔剑魂',
    '799': '铁血狼母',
    '800': '流光镜影',
    '875': '腕豪',
    '876': '含羞蓓蕾',
    '887': '灵罗娃娃',
    '888': '炼金男爵',
    '893': '双界灵兔',
    '895': '不羁之悦',
    '897': '纳祖芒荣耀',
    '901': '炽炎雏龙',
    '902': '明烛',
    '910': '异画师',
    '950': '百裂冥犬'
  }
  return championMap[championId] || `英雄${championId}`
}

export const getSpellIconUrl = (spellId: number | null): string => {
  if (!spellId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/summoner-spells/${spellId}.png`
}
// 召唤师技能图标
export const getSpellMeta = (spellId: number | null): { label: string; icon: string } => {
  if (!spellId) return { label: '', icon: '' }
  const spellMap: Record<number, { label: string; icon: string }> = {
    1: { label: '净化', icon: new URL('@/assets/SpellIconFiles/1.png', import.meta.url).href },
    3: { label: '虚弱', icon: new URL('@/assets/SpellIconFiles/3.png', import.meta.url).href },
    4: { label: '闪现', icon: new URL('@/assets/SpellIconFiles/4.png', import.meta.url).href },
    6: { label: '幽灵疾步', icon: new URL('@/assets/SpellIconFiles/6.png', import.meta.url).href },
    7: { label: '治疗术', icon: new URL('@/assets/SpellIconFiles/7.png', import.meta.url).href },
    11: { label: '惩戒', icon: new URL('@/assets/SpellIconFiles/11.png', import.meta.url).href },
    12: { label: '传送', icon: new URL('@/assets/SpellIconFiles/12.png', import.meta.url).href },
    13: { label: '清晰术', icon: new URL('@/assets/SpellIconFiles/13.png', import.meta.url).href },
    14: { label: '点燃', icon: new URL('@/assets/SpellIconFiles/14.png', import.meta.url).href },
    21: { label: '屏障', icon: new URL('@/assets/SpellIconFiles/21.png', import.meta.url).href },
    32: { label: '雪球', icon: new URL('@/assets/SpellIconFiles/32.png', import.meta.url).href }
  }
  return spellMap[spellId] || { label: `技能${spellId}`, icon: '' }
}
// 段位图标
export const getTierIconUrl = (tier: string | undefined): string => {
  if (!tier) return ''
  const tierMap: Record<string, string> = {
    IRON: new URL('@/assets/RankedIconFiles/IRON.png', import.meta.url).href,
    BRONZE: new URL('@/assets/RankedIconFiles/BRONZE.png', import.meta.url).href,
    SILVER: new URL('@/assets/RankedIconFiles/SILVER.png', import.meta.url).href,
    GOLD: new URL('@/assets/RankedIconFiles/GOLD.png', import.meta.url).href,
    PLATINUM: new URL('@/assets/RankedIconFiles/PLATINUM.png', import.meta.url).href,
    EMERALD: new URL('@/assets/RankedIconFiles/EMERALD.png', import.meta.url).href,
    DIAMOND: new URL('@/assets/RankedIconFiles/DIAMOND.png', import.meta.url).href,
    MASTER: new URL('@/assets/RankedIconFiles/MASTER.png', import.meta.url).href,
    GRANDMASTER: new URL('@/assets/RankedIconFiles/GRANDMASTER.png', import.meta.url).href,
    CHALLENGER: new URL('@/assets/RankedIconFiles/CHALLENGER.png', import.meta.url).href
  }
  return tierMap[tier] || ''
}
// 获取最新版本号
export const getLatestVersion = async () => {
  const response = await fetch('https://ddragon.leagueoflegends.com/api/versions.json')
  const versions = await response.json()
  const latestVersion = versions[0]
  return latestVersion || '15.12.1'
}
// 获取所有符文详细信息（含描述等）
export const getAllRunes = async (gameVersion: string, language: string = 'zh_CN') => {
  const resp = await fetch(`https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/${language}/runesReforged.json`)
  return await resp.json()
}

// 获取所有召唤师技能详细信息
export const getAllSummonerSpells = async (gameVersion: string, language: string = 'zh_CN') => {
  const resp = await fetch(`https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/${language}/summoner.json`)
  return await resp.json()
}
// 获取所有英雄基础信息
export const getAllChampions = async (gameVersion: string, language: string = 'zh_CN'): Promise<any> => {
  const resp = await fetch(`https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/${language}/champion.json`)
  return await resp.json()
}

// 获取单个英雄详细信息
export const getChampionDetail = async (
  championName: string,
  gameVersion: string,
  language: string = 'zh_CN'
): Promise<any> => {
  const resp = await fetch(
    `https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/${language}/champion/${championName}.json`
  )
  return await resp.json()
}

// 获取所有物品数据
export const getAllItems = async (gameVersion: string, language: string = 'zh_CN'): Promise<any> => {
  const resp = await fetch(`https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/${language}/item.json`)
  return await resp.json()
}
