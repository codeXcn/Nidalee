export const useDataStore = defineStore(
  'data',
  () => {
    // 召唤师数据
    const summonerInfo = ref<any>(null)
    const summonerRank = ref<any>(null)
    const isSummonerLoaded = ref(false)
    const isSummonerLoading = ref(false)

    // 战绩数据
    const matchHistory = ref<any[]>([])
    const matchStatistics = ref<any>(null)
    const isMatchHistoryLoaded = ref(false)
    const isMatchHistoryLoading = ref(false)

    // 游戏数据
    const gameVersion = ref<string>('')
    const champions = ref<any[]>([])
    const items = ref<any[]>([])
    const runes = ref<any[]>([])
    const skins = ref<any[]>([])

    // 计算属性
    const summonerLevel = computed(() => {
      return summonerInfo.value?.summonerLevel || 0
    })

    const summonerName = computed(() => {
      return summonerInfo.value?.displayName || ''
    })

    const summonerIcon = computed(() => {
      return summonerInfo.value?.profileIconId || 0
    })

    const totalMatches = computed(() => {
      return matchHistory.value.length
    })

    const winRate = computed(() => {
      if (totalMatches.value === 0) return 0
      const wins = matchHistory.value.filter((match) => match.win).length
      return Math.round((wins / totalMatches.value) * 100)
    })

    // 召唤师相关方法
    const setSummonerInfo = (info: any) => {
      summonerInfo.value = info
      isSummonerLoaded.value = true
      isSummonerLoading.value = false
    }

    const setSummonerRank = (rank: any) => {
      summonerRank.value = rank
    }

    const clearSummonerInfo = () => {
      summonerInfo.value = null
      summonerRank.value = null
      isSummonerLoaded.value = false
      isSummonerLoading.value = false
    }

    const startLoadingSummoner = () => {
      isSummonerLoading.value = true
    }

    // 战绩相关方法
    const setMatchHistory = (matches: any[]) => {
      matchHistory.value = matches
      isMatchHistoryLoaded.value = true
      isMatchHistoryLoading.value = false
    }

    const addMatchToHistory = (match: any) => {
      matchHistory.value.unshift(match)
      // 限制历史记录数量
      if (matchHistory.value.length > 100) {
        matchHistory.value = matchHistory.value.slice(0, 100)
      }
    }

    const setMatchStatistics = (stats: any) => {
      matchStatistics.value = stats
      isMatchHistoryLoaded.value = true
      isMatchHistoryLoading.value = false
    }

    const clearMatchHistory = () => {
      matchHistory.value = []
      matchStatistics.value = null
      isMatchHistoryLoaded.value = false
      isMatchHistoryLoading.value = false
    }

    const startLoadingMatchHistory = () => {
      isMatchHistoryLoading.value = true
    }

    // 游戏数据相关方法
    const setGameVersion = (version: string) => {
      gameVersion.value = version
    }

    const setChampions = (champList: any[]) => {
      champions.value = champList
    }

    const setItems = (itemList: any[]) => {
      items.value = itemList
    }

    const setRunes = (runeList: any[]) => {
      runes.value = runeList
    }

    const setSkins = (skinList: any[]) => {
      skins.value = skinList
    }

    // 数据查询方法
    const getChampionById = (id: number) => {
      return champions.value.find((champ) => champ.id === id)
    }

    const getChampionByName = (name: string) => {
      return champions.value.find(
        (champ) => champ.name.toLowerCase() === name.toLowerCase() || champ.alias.toLowerCase() === name.toLowerCase()
      )
    }

    const getItemById = (id: number) => {
      return items.value.find((item) => item.id === id)
    }

    const getRuneById = (id: number) => {
      return runes.value.find((rune) => rune.id === id)
    }

    // 数据清理方法
    const clearAllData = () => {
      clearSummonerInfo()
      clearMatchHistory()
      champions.value = []
      items.value = []
      runes.value = []
      skins.value = []
    }

    // 数据状态检查
    const isDataLoaded = computed(() => {
      return isSummonerLoaded.value && isMatchHistoryLoaded.value
    })

    const isDataLoading = computed(() => {
      return isSummonerLoading.value || isMatchHistoryLoading.value
    })

    const hasGameData = computed(() => {
      return champions.value.length > 0 && items.value.length > 0 && runes.value.length > 0
    })

    return {
      // 召唤师状态
      summonerInfo,
      summonerRank,
      isSummonerLoaded,
      isSummonerLoading,

      // 战绩状态
      matchHistory,
      matchStatistics,
      isMatchHistoryLoaded,
      isMatchHistoryLoading,

      // 游戏数据状态
      gameVersion,
      champions,
      items,
      runes,
      skins,

      // 计算属性
      summonerLevel,
      summonerName,
      summonerIcon,
      totalMatches,
      winRate,
      isDataLoaded,
      isDataLoading,
      hasGameData,

      // 召唤师方法
      setSummonerInfo,
      setSummonerRank,
      clearSummonerInfo,
      startLoadingSummoner,

      // 战绩方法
      setMatchHistory,
      addMatchToHistory,
      setMatchStatistics,
      clearMatchHistory,
      startLoadingMatchHistory,

      // 游戏数据方法
      setGameVersion,
      setChampions,
      setItems,
      setRunes,
      setSkins,

      // 查询方法
      getChampionById,
      getChampionByName,
      getItemById,
      getRuneById,

      // 清理方法
      clearAllData
    }
  },
  {
    persist: {
      pick: ['summonerInfo', 'matchHistory', 'matchStatistics']
    }
  }
)
