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

// 资源URL相关函数
export const getChampionIconUrl = (championId: number | string | null): string => {
  if (!championId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/${championId}.png`
}

export const getProfileIconUrl = (iconId: number): string => {
  if (!iconId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/${iconId}.jpg`
}

export const getItemIconUrl = (itemId: number, gameVersion: string): string => {
  if (!itemId || itemId === 0) return ''
  return `https://ddragon.leagueoflegends.com/cdn/${gameVersion}/img/item/${itemId}.png`
}

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

// 类型定义
export interface GameDetailData {
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

export interface TeamBan {
  championId: number
  pickTurn: number
}

export interface Participant {
  participantId: number
  teamId: number
  championId: number
  championName: string
  stats?: any
  rankTier?: string
  score?: number
}
export const getChampionName = (championId: number | string | null): string => {
  if (!championId) return '未选择英雄'
  const championMap: Record<number | string, string> = {
    1: '安妮',
    2: '奥拉夫',
    3: '加里奥',
    4: '崔斯特',
    5: '辛吉德',
    6: '厄加特',
    7: '乐芙兰',
    8: '弗拉基米尔',
    9: '费德提克',
    10: '凯尔',
    11: '易大师',
    12: '阿利斯塔',
    13: '瑞兹',
    14: '塞恩',
    15: '希维尔',
    16: '索拉卡',
    17: '提莫',
    18: '崔丝塔娜',
    19: '沃里克',
    20: '努努和威朗普',
    21: '厄运小姐',
    22: '艾希',
    23: '泰达米尔',
    24: '贾克斯',
    25: '莫甘娜',
    26: '基兰',
    27: '辛吉德',
    28: '伊芙琳',
    29: '图奇',
    30: '卡西奥佩娅',
    31: '科加斯',
    32: '阿木木',
    33: '拉莫斯',
    34: '阿尼维亚',
    35: '萨科',
    36: '蒙多医生',
    37: '娑娜',
    38: '卡萨丁',
    39: '伊瑞利亚',
    40: '迦娜',
    41: '普朗克',
    42: '科尔基',
    43: '卡尔玛',
    44: '塔里克',
    45: '维迦',
    48: '特朗德尔',
    50: '斯维因',
    51: '凯特琳',
    53: '布隆',
    54: '马尔扎哈',
    55: '卡特琳娜',
    56: '诺克萨斯',
    57: '茂凯',
    58: '雷克顿',
    59: '杰斯',
    60: '艾莉丝',
    61: '奥莉安娜',
    62: '维克托',
    63: '布兰德',
    64: '李青',
    67: '薇恩',
    68: '兰博',
    69: '卡西奥佩娅',
    72: '斯卡纳',
    74: '海姆丁格',
    75: '纳瑟斯',
    76: '奈德丽',
    77: '乌迪尔',
    78: '波比',
    79: '格雷福斯',
    80: '潘森',
    81: '伊泽瑞尔',
    82: '莫德凯撒',
    83: '约里克',
    84: '阿卡丽',
    85: '凯南',
    86: '盖伦',
    89: '蕾欧娜',
    90: '玛尔扎哈',
    91: '塔隆',
    92: '锐雯',
    96: '克格莫',
    98: '慎',
    99: '拉克丝',
    101: '泽拉斯',
    102: '希维尔',
    103: '阿狸',
    104: '格雷福斯',
    105: '菲兹',
    106: '沃利贝尔',
    107: '雷恩加尔',
    110: '韦鲁斯',
    111: '诺提勒斯',
    112: '维克托',
    113: '瑟庄妮',
    114: '菲奥娜',
    115: '吉格斯',
    117: '璐璐',
    119: '德莱文',
    120: '赫卡里姆',
    121: '卡兹克',
    122: '诺克萨斯',
    126: '杰斯',
    127: '丽桑卓',
    131: '戴安娜',
    133: '奎因',
    134: '辛德拉',
    136: '奥瑞利安·索尔',
    141: '凯隐',
    142: '佐伊',
    143: '婕拉',
    145: '凯莎',
    147: '瑟提',
    150: '纳尔',
    154: '扎克',
    157: '亚索',
    161: '维克兹',
    163: '塔莉垭',
    164: '卡蜜尔',
    166: '阿克尚',
    200: '贝尔维斯',
    201: '布隆',
    202: '烬',
    203: '千珏',
    221: '泽丽',
    222: '金克丝',
    223: '泰姆',
    234: '维戈',
    235: '塞娜',
    236: '卢锡安',
    238: '劫',
    240: '克烈',
    245: '艾克',
    246: '奇亚娜',
    254: '维',
    266: '亚托克斯',
    267: '娜美',
    268: '阿兹尔',
    350: '悠米',
    360: '萨米拉',
    412: '锤石',
    420: '伊莉丝',
    421: '雷克塞',
    427: '艾翁',
    429: '卡莉丝塔',
    432: '巴德',
    497: '洛',
    498: '霞',
    516: '奥恩',
    517: '赛娜',
    518: '妮蔻',
    523: '厄斐琉斯',
    526: '瑞兹',
    555: '派克',
    711: '薇古丝',
    777: '永恩',
    875: '瑟提',
    876: '莉莉娅',
    887: '格温',
    888: '蕾娜塔',
    895: '尼菈',
    897: '奎桑提',
    901: '魔腾',
    902: '米利欧',
    950: '纳亚菲利',
    955: '米利欧',
    958: '奎桑提',
    959: '纳亚菲利'
  }
  return championMap[championId] || `英雄${championId}`
}

export const getSpellIconUrl = (spellId: number | null): string => {
  if (!spellId) return ''
  return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/summoner-spells/${spellId}.png`
}

export const getSpellMeta = (spellId: number | null): { label: string, icon: string } => {
  if (!spellId) return { label: '', icon: '' }
  const spellMap: Record<number, { label: string, icon: string }> = {
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

export const getTierIconUrl = (tier: string | undefined): string => {
  if (!tier) return ''
  const tierMap: Record<string, string> = {
    IRON: new URL('@/assets/RankedIconFiles/IRON.png', import.meta.url).href,
    BRONZE: new URL('@/assets/RankedIconFiles/BRONZE.png', import.meta.url).href,
    SILVER: new URL('@/assets/RankedIconFiles/SILVER.png', import.meta.url).href,
    GOLD: new URL('@/assets/RankedIconFiles/GOLD.png', import.meta.url).href,
    PLATINUM: new URL('@/assets/RankedIconFiles/PLATINUM.png', import.meta.url).href,
    DIAMOND: new URL('@/assets/RankedIconFiles/DIAMOND.png', import.meta.url).href,
    MASTER: new URL('@/assets/RankedIconFiles/MASTER.png', import.meta.url).href,
    GRANDMASTER: new URL('@/assets/RankedIconFiles/GRANDMASTER.png', import.meta.url).href,
    CHALLENGER: new URL('@/assets/RankedIconFiles/CHALLENGER.png', import.meta.url).href
  }
  return tierMap[tier] || ''
}
