export const useAutoFunctionStore = defineStore(
  'autoFunction',
  () => {
    // 自动功能状态
    const autoFunctions = ref({
      acceptMatch: false,
      selectChampion: false,
      runeConfig: false,
      banChampion: false
    })

    // 功能名称映射
    const functionNames = {
      acceptMatch: '接受对局',
      selectChampion: '选择英雄',
      runeConfig: '符文配置',
      banChampion: '禁用英雄'
    } as const

    // 切换自动功能
    const toggleAutoFunction = (key: keyof typeof autoFunctions.value) => {
      autoFunctions.value[key] = !autoFunctions.value[key]
      return {
        key,
        enabled: autoFunctions.value[key],
        name: functionNames[key]
      }
    }

    // 启用指定功能
    const enableFunction = (key: keyof typeof autoFunctions.value) => {
      autoFunctions.value[key] = true
      return {
        key,
        enabled: true,
        name: functionNames[key]
      }
    }

    // 禁用指定功能
    const disableFunction = (key: keyof typeof autoFunctions.value) => {
      autoFunctions.value[key] = false
      return {
        key,
        enabled: false,
        name: functionNames[key]
      }
    }

    // 禁用所有功能
    const disableAllFunctions = () => {
      const keys = Object.keys(autoFunctions.value) as Array<keyof typeof autoFunctions.value>
      keys.forEach((key) => {
        autoFunctions.value[key] = false
      })
    }

    // 获取功能名称
    const getFunctionName = (key: keyof typeof autoFunctions.value): string => {
      return functionNames[key]
    }

    // 计算属性
    const enabledFunctionsCount = computed(() => {
      return Object.values(autoFunctions.value).filter(Boolean).length
    })

    const enabledFunctions = computed(() => {
      return Object.entries(autoFunctions.value)
        .filter(([, enabled]) => enabled)
        .map(([key]) => ({
          key: key as keyof typeof autoFunctions.value,
          name: functionNames[key as keyof typeof autoFunctions.value]
        }))
    })

    const isAnyFunctionEnabled = computed(() => enabledFunctionsCount.value > 0)

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
      getFunctionName
    }
  },
  {
    persist: true
  }
)
