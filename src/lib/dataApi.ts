import { getLatestVersion } from '@/lib'
import { createFetch } from '@vueuse/core'

// 创建带默认配置的 fetch 实例
const useApiFetch = createFetch({
  options: {
    timeout: 10000,
    beforeFetch({ url, options }) {
      console.log(`🚀 正在请求: ${url}`)

      options.headers = {
        ...options.headers,
        'User-Agent': 'Nidalee-LoL-Assistant/1.0',
        'Accept': 'application/json'
      }

      return { options }
    },
    afterFetch({ data, response }) {
      console.log(`✅ 请求成功: ${response.url} (${response.status})`)
      return { data }
    },
    onFetchError({ error, response }) {
      console.error(`❌ 请求失败: ${response?.url} (${response?.status})`, error)
      return { error }
    }
  },
  fetchOptions: {
    mode: 'cors'
  }
})

// 导出自定义fetch实例供其他函数使用
export { useApiFetch }

// API响应类型定义
export interface ApiResponse<T> {
  success: boolean
  data: T | null
  error?: string
  version?: string
}

// 官方Data Dragon API类型定义
export interface DDragonVersions extends Array<string> {}

export interface DDragonRune {
  id: number
  key: string
  icon: string
  name: string
  shortDesc: string
  longDesc: string
}

export interface DDragonRuneTree {
  id: number
  key: string
  icon: string
  name: string
  slots: Array<{
    runes: DDragonRune[]
  }>
}

export interface DDragonItem {
  name: string
  description: string
  colloq: string
  plaintext: string
  into?: string[]
  from?: string[]
  image: {
    full: string
    sprite: string
    group: string
    x: number
    y: number
    w: number
    h: number
  }
  gold: {
    base: number
    purchasable: boolean
    total: number
    sell: number
  }
  tags: string[]
  maps: Record<string, boolean>
  stats: Record<string, number>
}

export interface DDragonItemsResponse {
  type: string
  format: string
  version: string
  data: Record<string, DDragonItem>
}

export interface DDragonChampion {
  version: string
  id: string
  key: string
  name: string
  title: string
  blurb: string
  info: {
    attack: number
    defense: number
    magic: number
    difficulty: number
  }
  image: {
    full: string
    sprite: string
    group: string
    x: number
    y: number
    w: number
    h: number
  }
  tags: string[]
  partype: string
  stats: Record<string, number>
}

export interface DDragonChampionsResponse {
  type: string
  format: string
  version: string
  data: Record<string, DDragonChampion>
}

// Community Dragon API类型定义
export interface CommunityDragonChampionSummary {
  id: number
  name: string
  description: string
  alias: string
  contentId: string
  squarePortraitPath: string
  roles: string[]
}

export interface CommunityDragonSkin {
  id: number
  isBase: boolean
  name: string
  splashPath: string
  uncenteredSplashPath: string
  tilePath: string
  loadScreenPath: string
  skinType: string
  rarity: string
  isLegacy: boolean
  chromas: Array<{
    id: number
    name: string
    chromaPath: string
    colors: string[]
  }>
  questSkinInfo?: any
  description?: string
  regionRarityId?: number
  rarityGemPath?: string
}

export interface CommunityDragonChampion {
  id: number
  name: string
  alias: string
  squarePortraitPath: string
  roles: string[]
  skins: CommunityDragonSkin[]
  passive: {
    name: string
    abilityIconPath: string
    description: string
  }
  spells: Array<{
    spellKey: string
    name: string
    abilityIconPath: string
    description: string
    dynamicDescription: string
    range: number[]
    cooldown: number[]
    cost: number[]
    costType: string
    maxLevel: number
  }>
}

export interface QueueInfo {
  queueId: number
  map: string
  description: string
  notes?: string
}

// =============================================================================
// 官方 Data Dragon API 调用函数
// =============================================================================

/**
 * 获取版本列表
 */
export async function fetchVersions(): Promise<ApiResponse<DDragonVersions>> {
  try {
    const { data, error, statusCode } = await useApiFetch(
      'https://ddragon.leagueoflegends.com/api/versions.json'
    ).json<DDragonVersions>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: data.value[0] // 最新版本
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取符文数据
 */
export async function fetchRunes(version?: string): Promise<ApiResponse<DDragonRuneTree[]>> {
  try {
    const gameVersion = version || (await getLatestVersion())
    const url = `https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/zh_CN/runesReforged.json`

    const { data, error, statusCode } = await useApiFetch(url).json<DDragonRuneTree[]>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: gameVersion
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取装备数据
 */
export async function fetchItems(version?: string): Promise<ApiResponse<DDragonItemsResponse>> {
  try {
    const gameVersion = version || (await getLatestVersion())
    const url = `https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/zh_CN/item.json`

    const { data, error, statusCode } = await useApiFetch(url).json<DDragonItemsResponse>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: gameVersion
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取英雄列表数据
 */
export async function fetchChampions(version?: string): Promise<ApiResponse<DDragonChampionsResponse>> {
  try {
    const gameVersion = version || (await getLatestVersion())
    const url = `https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/zh_CN/champion.json`

    const { data, error, statusCode } = await useApiFetch(url).json<DDragonChampionsResponse>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: gameVersion
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取召唤师技能数据
 */
export async function fetchSummonerSpells(version?: string): Promise<ApiResponse<any>> {
  try {
    const gameVersion = version || (await getLatestVersion())
    const url = `https://ddragon.leagueoflegends.com/cdn/${gameVersion}/data/zh_CN/summoner.json`

    const { data, error, statusCode } = await useApiFetch(url).json<any>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: gameVersion
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

// =============================================================================
// Community Dragon API 调用函数
// =============================================================================

/**
 * 获取皮肤数据
 */
export async function fetchSkins(): Promise<ApiResponse<CommunityDragonSkin[]>> {
  try {
    const url = 'https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/skins.json'

    const { data, error, statusCode } = await useApiFetch(url).json<CommunityDragonSkin[]>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: 'latest'
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取队列信息数据
 */
export async function fetchQueues(): Promise<ApiResponse<QueueInfo[]>> {
  try {
    // 优先使用官方的队列数据
    const officialUrl = 'https://static.developer.riotgames.com/docs/lol/queues.json'

    const { data, error, statusCode } = await useApiFetch(officialUrl).json<QueueInfo[]>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: 'static'
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取指定英雄的详细信息
 */
export async function fetchChampionDetails(championId: number): Promise<ApiResponse<CommunityDragonChampion>> {
  try {
    const url = `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/champions/${championId}.json`

    const { data, error, statusCode } = await useApiFetch(url).json<CommunityDragonChampion>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: 'latest'
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取英雄摘要数据（包含头像路径）
 */
export async function fetchChampionSummary(): Promise<ApiResponse<CommunityDragonChampionSummary[]>> {
  try {
    const url =
      'https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/zh_cn/v1/champion-summary.json'

    const { data, error, statusCode } = await useApiFetch(url).json<CommunityDragonChampionSummary[]>()

    if (error.value) {
      throw new Error(error.value)
    }

    if (statusCode.value !== 200) {
      throw new Error(`HTTP ${statusCode.value}`)
    }

    if (!data.value) {
      throw new Error('No data received')
    }

    return {
      success: true,
      data: data.value,
      version: 'latest'
    }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

// =============================================================================
// 综合调用函数
// =============================================================================

/**
 * 批量获取所有基础数据
 */
export async function fetchAllBasicData(version?: string): Promise<{
  versions: ApiResponse<DDragonVersions>
  runes: ApiResponse<DDragonRuneTree[]>
  items: ApiResponse<DDragonItemsResponse>
  champions: ApiResponse<DDragonChampionsResponse>
  skins: ApiResponse<CommunityDragonSkin[]>
  queues: ApiResponse<QueueInfo[]>
  championSummary: ApiResponse<CommunityDragonChampionSummary[]>
}> {
  console.log('🚀 开始批量获取所有基础数据...')

  const results = await Promise.allSettled([
    fetchVersions(),
    fetchRunes(version),
    fetchItems(version),
    fetchChampions(version),
    fetchSkins(),
    fetchQueues(),
    fetchChampionSummary()
  ])

  const [versions, runes, items, champions, skins, queues, championSummary] = results.map((result) =>
    result.status === 'fulfilled'
      ? result.value
      : {
          success: false,
          data: null,
          error: result.status === 'rejected' ? result.reason : 'Unknown error'
        }
  )

  console.log('📊 数据获取结果统计:')
  console.log('- 版本信息:', versions.success ? '✅' : '❌')
  console.log('- 符文数据:', runes.success ? '✅' : '❌')
  console.log('- 装备数据:', items.success ? '✅' : '❌')
  console.log('- 英雄数据:', champions.success ? '✅' : '❌')
  console.log('- 皮肤数据:', skins.success ? '✅' : '❌')
  console.log('- 队列数据:', queues.success ? '✅' : '❌')
  console.log('- 英雄摘要:', championSummary.success ? '✅' : '❌')

  return {
    versions,
    runes,
    items,
    champions,
    skins,
    queues,
    championSummary
  }
}

/**
 * 测试所有API连接
 */
export async function testAllApiConnections(): Promise<{
  dataDragon: boolean
  communityDragon: boolean
  riotDeveloper: boolean
  details: Record<string, boolean>
}> {
  console.log('🔍 测试所有API连接...')

  const tests = await Promise.allSettled([
    fetchVersions(),
    fetchRunes(),
    fetchItems(),
    fetchChampions(),
    fetchSkins(),
    fetchQueues(),
    fetchChampionDetails(1) // 测试英雄详情API
  ])

  const [versions, runes, items, champions, skins, queues, championDetails] = tests.map(
    (result) => result.status === 'fulfilled' && result.value.success
  )

  const details = {
    versions,
    runes,
    items,
    champions,
    skins,
    queues,
    championDetails
  }

  const dataDragon = versions && runes && items && champions
  const communityDragon = skins && championDetails
  const riotDeveloper = queues

  console.log('🌐 API连接测试结果:')
  console.log('- Data Dragon:', dataDragon ? '✅' : '❌')
  console.log('- Community Dragon:', communityDragon ? '✅' : '❌')
  console.log('- Riot Developer:', riotDeveloper ? '✅' : '❌')

  return {
    dataDragon,
    communityDragon,
    riotDeveloper,
    details
  }
}

/**
 * 综合测试所有API（避免重复请求）
 */
export async function testAllApiConnectionsOptimized(version?: string): Promise<{
  dataDragon: boolean
  communityDragon: boolean
  riotDeveloper: boolean
  details: Record<string, boolean>
  data: {
    versions: ApiResponse<DDragonVersions>
    runes: ApiResponse<DDragonRuneTree[]>
    items: ApiResponse<DDragonItemsResponse>
    champions: ApiResponse<DDragonChampionsResponse>
    skins: ApiResponse<CommunityDragonSkin[]>
    queues: ApiResponse<QueueInfo[]>
    championSummary: ApiResponse<CommunityDragonChampionSummary[]>
    championDetails?: ApiResponse<CommunityDragonChampion>
  }
}> {
  console.log('🔍 优化版API连接测试（避免重复请求）...')

  // 1. 批量获取基础数据
  const batchData = await fetchAllBasicData(version)

  // 2. 只在英雄数据成功时测试英雄详情
  let championDetails: ApiResponse<CommunityDragonChampion> | undefined
  if (batchData.champions.success) {
    championDetails = await fetchChampionDetails(1) // 测试安妮
  }

  // 3. 汇总结果
  const details = {
    versions: batchData.versions.success,
    runes: batchData.runes.success,
    items: batchData.items.success,
    champions: batchData.champions.success,
    skins: batchData.skins.success,
    queues: batchData.queues.success,
    championSummary: batchData.championSummary.success,
    championDetails: championDetails?.success ?? false
  }

  const dataDragon = details.versions && details.runes && details.items && details.champions
  const communityDragon = details.skins && details.championSummary && details.championDetails
  const riotDeveloper = details.queues

  console.log('🌐 优化版API连接测试结果:')
  console.log('- Data Dragon:', dataDragon ? '✅' : '❌')
  console.log('- Community Dragon:', communityDragon ? '✅' : '❌')
  console.log('- Riot Developer:', riotDeveloper ? '✅' : '❌')

  return {
    dataDragon,
    communityDragon,
    riotDeveloper,
    details,
    data: {
      ...batchData,
      championDetails
    }
  }
}
