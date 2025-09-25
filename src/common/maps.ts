// 地图信息接口
export interface MapInfo {
  id: number
  name: string
  description: string
  mapStringId: string
}

// 地图信息定义
export const maps: MapInfo[] = [
  {
    id: 0,
    name: '常规',
    description: '常规',
    mapStringId: ''
  },
  {
    id: 11,
    name: '召唤师峡谷',
    description:
      '正义之地最新潮最威严的战场当属召唤师峡谷。无数对立的召唤师们在这片战场上延续着战斗，这片战场也因此而得名。有三条不同的道路贯穿整个战场。与友军一起攻击敌人最脆弱的地方，摧毁他们的基地！',
    mapStringId: 'SR'
  },
  {
    id: 12,
    name: '随机地图',
    description:
      '嚎哭深渊是一个无底裂隙，位于弗雷尔卓德最为寒冷、最为残酷的部分。传说在很久以前，一场宏大的战役就发生在横跨这道天堑的一座狭窄桥梁上。没有人记得谁在此战斗，为何而战斗，但有传言说，如果你仔细聆听风声的话，仍然可以听见那些葬身于深渊之中的败亡者们在嚎哭不停。',
    mapStringId: 'HA'
  },
  {
    id: 21,
    name: '极限闪击',
    description: '极限闪击',
    mapStringId: 'NB'
  },
  {
    id: 22,
    name: '云顶之弈',
    description: '集结英雄，加入八位玩家的乱斗。想怎么打都可以，但赢家只有一个。',
    mapStringId: 'TFT'
  },
  {
    id: 30,
    name: '斗魂竞技场',
    description: '单人或与你的好友组成搭档进入这个多队互斗模式。与各队对手们在不同的竞技场中轮番展开激战。',
    mapStringId: 'TGR'
  },
  {
    id: 33,
    name: '无尽狂潮',
    description: '与一名好友组队或孤身闯入这个战场生存模式。',
    mapStringId: 'Strawberry'
  },
  {
    id: 35,
    name: '班德尔之森',
    description: '与队友同心协力，护送小兵进入敌方传送门。没有分路，没有多线，没有压力。就是一场五对五的酣战。',
    mapStringId: 'BDW'
  }
]

// 根据地图ID获取地图信息
export const getMapById = (mapId: number): MapInfo | undefined => {
  return maps.find((map) => map.id === mapId)
}

// 根据地图字符串ID获取地图信息
export const getMapByStringId = (mapStringId: string): MapInfo | undefined => {
  return maps.find((map) => map.mapStringId === mapStringId)
}

// 获取所有地图ID
export const getAllMapIds = (): number[] => {
  return maps.map((map) => map.id)
}

// 获取地图名称（兼容旧版本）
export const getMapName = (mapId: number): string => {
  const map = getMapById(mapId)
  return map?.name || `地图 ${mapId}`
}
