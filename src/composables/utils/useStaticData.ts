import { getLatestVersion } from '@/lib'

// 静态数据类型定义
interface RuneData {
  id: number
  key: string
  icon: string
  name: string
  slots: Array<{
    runes: Array<{
      id: number
      key: string
      icon: string
      name: string
      shortDesc: string
      longDesc: string
    }>
  }>
}

interface ItemData {
  [key: string]: {
    name: string
    description: string
    gold: {
      base: number
      total: number
      sell: number
    }
    stats: Record<string, number>
    tags: string[]
    image: {
      full: string
    }
  }
}

interface QueueData {
  id: number
  name: string
  description: string
  gameMode: string
  isRanked: boolean
  category: string
}

interface SkinData {
  id: string
  num: number
  name: string
  chromas: boolean
  file: string
}

export function useStaticData() {
  const runesData = ref<RuneData[] | null>(null)
  const itemsData = ref<ItemData | null>(null)
  const queuesData = ref<QueueData[] | null>(null)
  const skinsData = ref<SkinData[] | null>(null)
  const currentVersion = ref<string>('15.4.1') // 默认版本
  const lastUpdateTime = ref<Date | null>(null)

  const isLoaded = computed(() => runesData.value && itemsData.value && queuesData.value && skinsData.value)

  // 检查数据是否需要更新（24小时检查一次）
  const shouldUpdateData = computed(() => {
    if (!lastUpdateTime.value) return true
    const now = new Date()
    const timeDiff = now.getTime() - lastUpdateTime.value.getTime()
    const hoursDiff = timeDiff / (1000 * 3600)
    return hoursDiff >= 24 // 24小时更新一次
  })

  // 从本地存储获取缓存的数据
  const loadCachedData = () => {
    try {
      const cached = localStorage.getItem('lol-static-data')
      if (cached) {
        const data = JSON.parse(cached)
        runesData.value = data.runes
        itemsData.value = data.items
        queuesData.value = data.queues
        skinsData.value = data.skins
        currentVersion.value = data.version
        lastUpdateTime.value = new Date(data.lastUpdate)
        console.log('Loaded cached static data, version:', data.version)
        return true
      }
    } catch (error) {
      console.error('Failed to load cached data:', error)
    }
    return false
  }

  // 缓存数据到本地存储
  const cacheData = () => {
    try {
      const dataToCache = {
        runes: runesData.value,
        items: itemsData.value,
        queues: queuesData.value,
        skins: skinsData.value,
        version: currentVersion.value,
        lastUpdate: new Date().toISOString()
      }
      localStorage.setItem('lol-static-data', JSON.stringify(dataToCache))
      lastUpdateTime.value = new Date()
      console.log('Cached static data, version:', currentVersion.value)
    } catch (error) {
      console.error('Failed to cache data:', error)
    }
  }

  // 获取最新版本号
  const updateVersion = async () => {
    try {
      const version = await getLatestVersion()
      if (version && version !== currentVersion.value) {
        console.log('New version detected:', version, 'current:', currentVersion.value)
        currentVersion.value = version
        return true // 需要更新数据
      }
      return false
    } catch (error) {
      console.error('Failed to get latest version:', error)
      return false
    }
  }

  // 从 Data Dragon API 加载最新数据
  const loadFromAPI = async () => {
    try {
      console.log('=== 开始从 Data Dragon API 加载数据 ===')
      console.log('当前版本:', currentVersion.value)

      // 构建API URLs
      const runeUrl = `https://ddragon.leagueoflegends.com/cdn/${currentVersion.value}/data/zh_CN/runesReforged.json`
      const itemUrl = `https://ddragon.leagueoflegends.com/cdn/${currentVersion.value}/data/zh_CN/item.json`

      console.log('符文API:', runeUrl)
      console.log('装备API:', itemUrl)

      const [runes, items, queues, skins] = await Promise.all([
        // 从 Data Dragon API 获取符文数据
        fetch(runeUrl).then(async (res) => {
          console.log('符文API响应状态:', res.status, res.statusText)
          if (!res.ok) {
            throw new Error(`符文API失败: ${res.status} ${res.statusText}`)
          }
          const data = await res.json()
          console.log('✅ 符文数据加载成功，符文系数量:', data?.length || 0)
          return data
        }),

        // 从 Data Dragon API 获取装备数据
        fetch(itemUrl).then(async (res) => {
          console.log('装备API响应状态:', res.status, res.statusText)
          if (!res.ok) {
            throw new Error(`装备API失败: ${res.status} ${res.statusText}`)
          }
          const data = await res.json()
          const itemCount = Object.keys(data?.data || {}).length
          console.log('✅ 装备数据加载成功，装备数量:', itemCount)
          return data
        }),

        // 从本地文件获取队列数据（变化不频繁）
        fetch('/data/queue.json').then(async (res) => {
          console.log('队列数据响应状态:', res.status, res.statusText)
          if (!res.ok) {
            throw new Error(`队列数据失败: ${res.status} ${res.statusText}`)
          }
          const data = await res.json()
          console.log('✅ 队列数据加载成功，队列数量:', data?.length || 0)
          return data
        }),

        // 从本地文件获取皮肤数据
        fetch('/data/skins.json').then(async (res) => {
          console.log('皮肤数据响应状态:', res.status, res.statusText)
          if (!res.ok) {
            throw new Error(`皮肤数据失败: ${res.status} ${res.statusText}`)
          }
          const data = await res.json()
          console.log('✅ 皮肤数据加载成功，皮肤数量:', data?.length || 0)
          return data
        })
      ])

      runesData.value = runes
      itemsData.value = items.data || items
      queuesData.value = queues
      skinsData.value = skins

      // 缓存到本地存储
      cacheData()

      console.log('🎉 所有静态数据从API加载完成！')
      console.log('最终统计:')
      console.log('- 符文系统:', runesData.value?.length || 0, '个符文系')
      console.log('- 装备数据:', Object.keys(itemsData.value || {}).length, '个装备')
      console.log('- 队列模式:', queuesData.value?.length || 0, '个队列')
      console.log('- 皮肤数据:', skinsData.value?.length || 0, '个皮肤')

      return true
    } catch (error) {
      console.error('❌ 从API加载静态数据失败:', error)
      console.log('错误详情:', error.message || error)

      // 网络连接检查
      if (!navigator.onLine) {
        console.log('检测到离线状态，这可能是失败的原因')
      }

      return false
    }
  }

  // 主要的静态数据加载方法
  const loadStaticData = async (forceUpdate: boolean = false) => {
    // 首先尝试加载缓存数据
    if (!forceUpdate && loadCachedData() && !shouldUpdateData.value) {
      console.log('Using cached static data')
      return
    }

    // 检查版本更新
    const versionUpdated = await updateVersion()

    // 如果版本更新或强制更新，从API加载
    if (forceUpdate || versionUpdated || shouldUpdateData.value) {
      const success = await loadFromAPI()
      if (!success) {
        // 如果API加载失败，尝试使用本地静态文件作为备用
        console.log('API failed, trying local fallback...')
        await loadFromLocalFallback()
      }
    }
  }

  // 本地文件备用加载方法
  const loadFromLocalFallback = async () => {
    try {
      console.log('Loading static data from local files as fallback')
      const [runes, items, queues, skins] = await Promise.all([
        fetch('/data/runesReforged.json').then((res) => res.json()),
        fetch('/data/item.json').then((res) => res.json()),
        fetch('/data/queue.json').then((res) => res.json()),
        fetch('/data/skins.json').then((res) => res.json())
      ])

      runesData.value = runes
      itemsData.value = items.data || items
      queuesData.value = queues
      skinsData.value = skins

      console.log('Static data loaded successfully from local files')
    } catch (error) {
      console.error('Failed to load static data from local files:', error)
    }
  }

  // 根据ID获取符文信息
  const getRuneById = (runeId: number) => {
    if (!runesData.value) return null

    for (const tree of runesData.value) {
      for (const slot of tree.slots) {
        const rune = slot.runes.find((r) => r.id === runeId)
        if (rune) return { ...rune, tree: tree.name }
      }
    }
    return null
  }

  // 根据ID获取装备信息
  const getItemById = (itemId: string | number) => {
    if (!itemsData.value) return null
    return itemsData.value[itemId.toString()] || null
  }

  // 根据队列ID获取游戏模式信息
  const getQueueById = (queueId: number) => {
    if (!queuesData.value) return null
    return queuesData.value.find((q) => q.id === queueId) || null
  }

  // 根据英雄ID和皮肤编号获取皮肤信息
  const getSkinByChampionAndNum = (championId: number, skinNum: number) => {
    if (!skinsData.value) return null
    const skinId = `${championId}${skinNum.toString().padStart(3, '0')}`
    return skinsData.value.find((s) => s.id === skinId) || null
  }

  // 推荐符文（基于英雄和位置）
  const getRecommendedRunes = (championId: number, position: string) => {
    // 这里可以根据静态数据和英雄信息推荐符文
    // 目前返回一个示例结构
    if (!runesData.value) return null

    // 根据位置推荐主符文系
    const positionRuneMap: Record<string, string> = {
      TOP: 'Resolve',
      JUNGLE: 'Domination',
      MIDDLE: 'Sorcery',
      BOTTOM: 'Precision',
      UTILITY: 'Inspiration'
    }

    const recommendedTree = positionRuneMap[position] || 'Precision'
    return runesData.value.find((tree) => tree.key === recommendedTree)
  }

  // 推荐装备（基于英雄和游戏阶段）
  const getRecommendedItems = (_championId: number, _position: string, _gamePhase: 'early' | 'mid' | 'late') => {
    if (!itemsData.value) return []

    // 这里可以根据英雄类型和游戏阶段推荐装备
    // 示例：返回一些通用的核心装备
    const coreItems = Object.values(itemsData.value)
      .filter((item) => item.tags?.includes('Damage') || item.tags?.includes('Tank'))
      .slice(0, 6)

    return coreItems
  }

  // 测试 Data Dragon API 连接
  const testAPIConnection = async () => {
    console.log('=== 测试 Data Dragon API 连接 ===')
    try {
      const latestVersion = await getLatestVersion()
      console.log('✅ 成功获取最新版本:', latestVersion)

      // 测试符文API
      const runeUrl = `https://ddragon.leagueoflegends.com/cdn/${latestVersion}/data/zh_CN/runesReforged.json`
      const runeResponse = await fetch(runeUrl)
      console.log('符文API状态:', runeResponse.status, runeResponse.ok ? '✅' : '❌')

      // 测试装备API
      const itemUrl = `https://ddragon.leagueoflegends.com/cdn/${latestVersion}/data/zh_CN/item.json`
      const itemResponse = await fetch(itemUrl)
      console.log('装备API状态:', itemResponse.status, itemResponse.ok ? '✅' : '❌')

      return runeResponse.ok && itemResponse.ok
    } catch (error) {
      console.error('❌ API连接测试失败:', error)
      return false
    }
  }

  return {
    // 数据状态
    runesData,
    itemsData,
    queuesData,
    skinsData,
    isLoaded,
    currentVersion,
    lastUpdateTime,
    shouldUpdateData,

    // 方法
    loadStaticData,
    loadCachedData,
    updateVersion,
    loadFromAPI,
    loadFromLocalFallback,
    testAPIConnection,
    getRuneById,
    getItemById,
    getQueueById,
    getSkinByChampionAndNum,
    getRecommendedRunes,
    getRecommendedItems
  }
}
