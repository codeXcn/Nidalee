import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

// 英雄信息接口
interface ChampionInfo {
  id: number
  name: string
  description: string
  alias: string
  contentId: string
  squarePortraitPath: string
  roles: string[]
}

// 自动功能配置接口
interface AutoFunctionConfig {
  enabled: boolean
  delay: number // 延迟时间(毫秒)
}

// 自动选择英雄配置
interface AutoSelectConfig extends AutoFunctionConfig {
  championId: number | null
  championInfo: ChampionInfo | null
}

// 自动禁用英雄配置
interface AutoBanConfig extends AutoFunctionConfig {
  championId: number | null
  championInfo: ChampionInfo | null
}

// 自动功能状态
interface AutoFunctions {
  acceptMatch: AutoFunctionConfig
  selectChampion: AutoSelectConfig
  runeConfig: AutoFunctionConfig
  banChampion: AutoBanConfig
}

export const useAutoFunctionStore = defineStore(
  'autoFunction',
  () => {
    // 自动功能状态
    const autoFunctions = ref<AutoFunctions>({
      acceptMatch: {
        enabled: false,
        delay: 1000 // 默认1秒延迟
      },
      selectChampion: {
        enabled: false,
        delay: 2000, // 默认2秒延迟
        championId: null,
        championInfo: null
      },
      runeConfig: {
        enabled: false,
        delay: 1500 // 默认1.5秒延迟
      },
      banChampion: {
        enabled: false,
        delay: 2000, // 默认2秒延迟
        championId: null,
        championInfo: null
      }
    })

    // 功能名称映射
    const functionNames = {
      acceptMatch: '自动接受对局',
      selectChampion: '自动选择英雄',
      runeConfig: '自动符文配置',
      banChampion: '自动禁用英雄'
    } as const

    // 切换自动功能开关
    const toggleAutoFunction = (key: keyof AutoFunctions) => {
      autoFunctions.value[key].enabled = !autoFunctions.value[key].enabled
      return {
        key,
        enabled: autoFunctions.value[key].enabled,
        name: functionNames[key]
      }
    }

    // 启用指定功能
    const enableFunction = (key: keyof AutoFunctions) => {
      console.log(`[autoFunctionStore] Enabling function:`, key)
      autoFunctions.value[key].enabled = true
      console.log(`[autoFunctionStore] Function ${key} enabled. Current state:`, autoFunctions.value[key])
      return {
        key,
        enabled: true,
        name: functionNames[key]
      }
    }

    // 禁用指定功能
    const disableFunction = (key: keyof AutoFunctions) => {
      console.log(`[autoFunctionStore] Disabling function:`, key)
      autoFunctions.value[key].enabled = false
      console.log(`[autoFunctionStore] Function ${key} disabled. Current state:`, autoFunctions.value[key])
      return {
        key,
        enabled: false,
        name: functionNames[key]
      }
    }

    // 禁用所有功能
    const disableAllFunctions = () => {
      Object.keys(autoFunctions.value).forEach((key) => {
        autoFunctions.value[key as keyof AutoFunctions].enabled = false
      })
    }

    // 设置功能延迟
    const setFunctionDelay = (key: keyof AutoFunctions, delay: number) => {
      const newDelay = Math.max(0, delay)
      console.log(`[autoFunctionStore] Setting delay for ${key}:`, {
        old: autoFunctions.value[key].delay,
        new: newDelay
      })
      autoFunctions.value[key].delay = newDelay
      console.log(`[autoFunctionStore] Delay set for ${key}. Current state:`, autoFunctions.value[key])
    }

    // 设置英雄选择
    const setChampionSelect = (championInfo: ChampionInfo) => {
      autoFunctions.value.selectChampion.championId = championInfo.id
      autoFunctions.value.selectChampion.championInfo = championInfo
    }

    // 设置禁用英雄
    const setChampionBan = (championInfo: ChampionInfo) => {
      autoFunctions.value.banChampion.championId = championInfo.id
      autoFunctions.value.banChampion.championInfo = championInfo
    }

    // 清除英雄选择
    const clearChampionSelect = () => {
      autoFunctions.value.selectChampion.championId = null
      autoFunctions.value.selectChampion.championInfo = null
    }

    // 清除禁用英雄
    const clearChampionBan = () => {
      autoFunctions.value.banChampion.championId = null
      autoFunctions.value.banChampion.championInfo = null
    }

    // 获取功能名称
    const getFunctionName = (key: keyof AutoFunctions): string => {
      return functionNames[key]
    }

    // 计算属性
    const enabledFunctionsCount = computed(() => {
      return Object.values(autoFunctions.value).filter((config) => config.enabled).length
    })

    const enabledFunctions = computed(() => {
      return Object.entries(autoFunctions.value)
        .filter(([, config]) => config.enabled)
        .map(([key]) => ({
          key: key as keyof AutoFunctions,
          name: functionNames[key as keyof AutoFunctions]
        }))
    })

    const isAnyFunctionEnabled = computed(() => enabledFunctionsCount.value > 0)

    // 获取特定功能的配置
    const getFunctionConfig = (key: keyof AutoFunctions) => {
      return autoFunctions.value[key]
    }

    return {
      // 状态
      autoFunctions,

      // 计算属性
      enabledFunctionsCount,
      enabledFunctions,
      isAnyFunctionEnabled,

      // 方法
      toggleAutoFunction,
      enableFunction,
      disableFunction,
      disableAllFunctions,
      setFunctionDelay,
      setChampionSelect,
      setChampionBan,
      clearChampionSelect,
      clearChampionBan,
      getFunctionName,
      getFunctionConfig
    }
  },
  {
    persist: true
  }
)

// 导出类型
export type { AutoFunctions, AutoFunctionConfig, AutoSelectConfig, AutoBanConfig, ChampionInfo }
