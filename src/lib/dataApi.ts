import { getLatestVersion } from '@/lib'
import { createFetch } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/core'

// 创建带默认配置的 fetch 实例
const useApiFetch = createFetch({
  options: {
    timeout: 10000,
    beforeFetch({ url, options }) {
      console.log(`🚀 正在请求: ${url}`)

      options.headers = {
        ...options.headers,
        'User-Agent': 'Nidalee-LoL-Assistant/1.0',
        Accept: 'application/json'
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
// 官方 Data Dragon 及 Riot 静态数据 API 调用函数补全
// =============================================================================

/**
 * 地图数据类型
 */
export interface RiotMap {
  mapId: number
  mapName: string
  notes?: string
  description?: string
}

/**
 * 游戏模式类型
 */
export interface RiotGameMode {
  gameMode: string
  description: string
}

/**
 * 游戏类型类型
 */
export interface RiotGameType {
  gametype: string
  description: string
}

/**
 * 获取地图数据
 */
export async function fetchMaps(): Promise<ApiResponse<RiotMap[]>> {
  try {
    const url = 'https://static.developer.riotgames.com/docs/lol/maps.json'
    const { data, error, statusCode } = await useApiFetch(url).json<RiotMap[]>()
    if (error.value) throw new Error(error.value)
    if (statusCode.value !== 200) throw new Error(`HTTP ${statusCode.value}`)
    if (!data.value) throw new Error('No data received')
    return { success: true, data: data.value, version: 'static' }
  } catch (error) {
    return { success: false, data: null, error: error instanceof Error ? error.message : 'Unknown error' }
  }
}

/**
 * 获取游戏模式数据
 */
export async function fetchGameModes(): Promise<ApiResponse<RiotGameMode[]>> {
  try {
    const url = 'https://static.developer.riotgames.com/docs/lol/gameModes.json'
    const { data, error, statusCode } = await useApiFetch(url).json<RiotGameMode[]>()
    if (error.value) throw new Error(error.value)
    if (statusCode.value !== 200) throw new Error(`HTTP ${statusCode.value}`)
    if (!data.value) throw new Error('No data received')
    return { success: true, data: data.value, version: 'static' }
  } catch (error) {
    return { success: false, data: null, error: error instanceof Error ? error.message : 'Unknown error' }
  }
}

/**
 * 获取游戏类型数据
 */
export async function fetchGameTypes(): Promise<ApiResponse<RiotGameType[]>> {
  try {
    const url = 'https://static.developer.riotgames.com/docs/lol/gameTypes.json'
    const { data, error, statusCode } = await useApiFetch(url).json<RiotGameType[]>()
    if (error.value) throw new Error(error.value)
    if (statusCode.value !== 200) throw new Error(`HTTP ${statusCode.value}`)
    if (!data.value) throw new Error('No data received')
    return { success: true, data: data.value, version: 'static' }
  } catch (error) {
    return { success: false, data: null, error: error instanceof Error ? error.message : 'Unknown error' }
  }
}

// ===== OP.GG API 调用函数 =====

/**
 * 获取OP.GG英雄详细原始数据
 */
export async function fetchOpggChampionBuildRaw(params: {
  region: string
  mode: string
  champion_id: number
  position?: string
  tier: string
}): Promise<ApiResponse<any>> {
  try {
    const data = await invoke<any>('get_opgg_champion_build_raw', {
      region: params.region,
      mode: params.mode,
      championId: params.champion_id,
      position: params.position,
      tier: params.tier
    })
    return { success: true, data }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取OP.GG英雄详细数据
 */
export async function fetchOpggChampionBuild(params: {
  region: string
  mode: string
  champion_id: number
  position?: string
  tier: string
}): Promise<ApiResponse<OpggChampionBuild>> {
  try {
    const data = await invoke<OpggChampionBuild>('get_opgg_champion_build', {
      region: params.region,
      mode: params.mode,
      championId: params.champion_id,
      position: params.position,
      tier: params.tier
    })
    return { success: true, data }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取OP.GG英雄层级列表
 */
export async function fetchOpggTierList(params: {
  region: string
  mode: string
  tier: string
}): Promise<ApiResponse<OpggTierList>> {
  try {
    const data = await invoke<OpggTierList>('get_opgg_tier_list', params)
    return { success: true, data }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 获取英雄可用位置列表
 */
export async function fetchOpggChampionPositions(params: {
  region: string
  champion_id: number
  tier: string
}): Promise<ApiResponse<string[]>> {
  try {
    const data = await invoke<string[]>('get_opgg_champion_positions', {
      region: params.region,
      championId: params.champion_id,
      tier: params.tier
    })
    return { success: true, data }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}

/**
 * 应用OP.GG推荐符文配置
 */
export async function applyOpggRunes(params: {
  region: string
  mode: string
  champion_id: number
  position?: string
  tier: string
  build_index?: number
}): Promise<ApiResponse<string>> {
  try {
    const data = await invoke<string>('apply_opgg_runes', {
      region: params.region,
      mode: params.mode,
      championId: params.champion_id,
      champion_name: `Champion_${params.champion_id}`,
      position: params.position,
      tier: params.tier,
      build_index: params.build_index
    })
    return { success: true, data }
  } catch (error) {
    return {
      success: false,
      data: null,
      error: error instanceof Error ? error.message : 'Unknown error'
    }
  }
}
