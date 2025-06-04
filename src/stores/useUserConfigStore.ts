import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export interface UserConfig {
  // 应用设置
  app: {
    enableConfigCache: boolean  // 是否启用配置缓存
    enableThemeCache: boolean   // 是否启用主题缓存
    enableGameDataCache: boolean // 是否启用游戏数据缓存
  }
  
  // 通用设置
  general: {
    language: string
    autoStart: boolean
    minimizeToTray: boolean
    autoUpdate: boolean
    analytics: boolean
    hardwareAcceleration: boolean
  }
  
  // 连接设置
  connection: {
    autoReconnect: boolean
    timeout: number
    retryInterval: number
    maxRetries: number
    notifications: boolean
  }
  
  // 通知设置
  notifications: {
    desktop: boolean
    sound: boolean
    gameStart: boolean
    autoOperations: boolean
    errors: boolean
  }
  
  // 高级设置
  advanced: {
    developerMode: boolean
    verboseLogging: boolean
    logRetentionDays: number
    apiRateLimit: number
  }
  
  // 自动功能配置
  autoFunctions: {
    autoAccept: {
      enabled: boolean
      delay: number
    }
    autoPickChampion: {
      enabled: boolean
      championId: number | null
      backupChampions: Array<{ id: number; name: string }>
      autoLock: boolean
    }
    autoBanChampion: {
      enabled: boolean
      championId: number | null
    }
    autoRune: {
      enabled: boolean
      source: string
    }
    matchAnalysis: {
      enabled: boolean
    }
    opponentAnalysis: {
      enabled: boolean
    }
    tacticSuggestions: {
      enabled: boolean
    }
  }
  
  // 安全设置
  safety: {
    enableRateLimit: boolean
    hourlyLimit: number
    operationInterval: number
    randomDelay: boolean
    minDelay: number
    maxDelay: number
    humanLike: boolean
    errorSimulation: boolean
    varyResponseTime: boolean
    rankedOnly: boolean
    autoPause: boolean
    logging: boolean
  }
}

const DEFAULT_CONFIG: UserConfig = {
  app: {
    enableConfigCache: true,
    enableThemeCache: true,
    enableGameDataCache: true
  },
  general: {
    language: 'zh-CN',
    autoStart: true,
    minimizeToTray: true,
    autoUpdate: true,
    analytics: false,
    hardwareAcceleration: true
  },
  connection: {
    autoReconnect: true,
    timeout: 10,
    retryInterval: 5,
    maxRetries: 3,
    notifications: true
  },
  notifications: {
    desktop: true,
    sound: true,
    gameStart: true,
    autoOperations: false,
    errors: true
  },
  advanced: {
    developerMode: false,
    verboseLogging: false,
    logRetentionDays: 7,
    apiRateLimit: 60
  },
  autoFunctions: {
    autoAccept: {
      enabled: true,
      delay: 1
    },
    autoPickChampion: {
      enabled: false,
      championId: null,
      backupChampions: [],
      autoLock: true
    },
    autoBanChampion: {
      enabled: false,
      championId: null
    },
    autoRune: {
      enabled: true,
      source: 'op.gg'
    },
    matchAnalysis: {
      enabled: true
    },
    opponentAnalysis: {
      enabled: false
    },
    tacticSuggestions: {
      enabled: true
    }
  },
  safety: {
    enableRateLimit: true,
    hourlyLimit: 30,
    operationInterval: 5,
    randomDelay: true,
    minDelay: 1,
    maxDelay: 5,
    humanLike: true,
    errorSimulation: false,
    varyResponseTime: true,
    rankedOnly: false,
    autoPause: true,
    logging: true
  }
}

export const useUserConfigStore = defineStore('userConfig', () => {
  const config = ref<UserConfig>({ ...DEFAULT_CONFIG })
  
  // 更新配置的方法
  function updateConfig(path: string, value: any) {
    const keys = path.split('.')
    let current: any = config.value
    
    for (let i = 0; i < keys.length - 1; i++) {
      if (!(keys[i] in current)) {
        current[keys[i]] = {}
      }
      current = current[keys[i]]
    }
    
    current[keys[keys.length - 1]] = value
    
    // 如果更改了缓存设置，立即处理
    if (path.startsWith('app.enable') && path.includes('Cache')) {
      handleCacheSettingChange(path, value)
    }
  }
  
  // 获取配置值
  function getConfig(path: string): any {
    const keys = path.split('.')
    let current: any = config.value
    
    for (const key of keys) {
      if (current && typeof current === 'object' && key in current) {
        current = current[key]
      } else {
        return undefined
      }
    }
    
    return current
  }
  
  // 重置配置
  function resetConfig() {
    const resetData = { ...DEFAULT_CONFIG }
    config.value = resetData
    
    // 清除所有 localStorage 数据
    clearAllCache()
    
    return resetData
  }
  
  // 重置特定分类的配置
  function resetConfigSection(section: keyof UserConfig) {
    if (DEFAULT_CONFIG[section]) {
      config.value[section] = { ...DEFAULT_CONFIG[section] } as any
    }
  }
  
  // 导出配置
  function exportConfig(): string {
    return JSON.stringify(config.value, null, 2)
  }
  
  // 导入配置
  function importConfig(configJson: string): boolean {
    try {
      const importedConfig = JSON.parse(configJson)
      // 验证配置结构的基本检查
      if (importedConfig && typeof importedConfig === 'object') {
        config.value = { ...DEFAULT_CONFIG, ...importedConfig }
        return true
      }
      return false
    } catch (error) {
      console.error('Failed to import config:', error)
      return false
    }
  }
  
  // 处理缓存设置变化
  function handleCacheSettingChange(path: string, enabled: boolean) {
    if (!enabled) {
      // 如果禁用了缓存，清除对应的本地存储
      if (path === 'app.enableConfigCache') {
        localStorage.removeItem('userConfig')
      } else if (path === 'app.enableThemeCache') {
        localStorage.removeItem('theme')
      } else if (path === 'app.enableGameDataCache') {
        localStorage.removeItem('gameStore')
      }
    }
  }
  
  // 清除所有缓存
  function clearAllCache() {
    const storageKeys = ['userConfig', 'theme', 'gameStore']
    storageKeys.forEach(key => {
      localStorage.removeItem(key)
    })
  }
  
  // 清除特定缓存
  function clearCache(type: 'config' | 'theme' | 'gameData') {
    const keyMap = {
      config: 'userConfig',
      theme: 'theme', 
      gameData: 'gameStore'
    }
    
    localStorage.removeItem(keyMap[type])
  }
  
  // 获取缓存状态
  function getCacheStatus() {
    return {
      configCacheEnabled: config.value.app.enableConfigCache,
      themeCacheEnabled: config.value.app.enableThemeCache,
      gameDataCacheEnabled: config.value.app.enableGameDataCache,
      configCacheExists: !!localStorage.getItem('userConfig'),
      themeCacheExists: !!localStorage.getItem('theme'),
      gameDataCacheExists: !!localStorage.getItem('gameStore')
    }
  }
  
  // 监听缓存设置变化
  watch(
    () => config.value.app.enableConfigCache,
    (enabled) => {
      if (!enabled) {
        localStorage.removeItem('userConfig')
      }
    }
  )
  
  return {
    config,
    updateConfig,
    getConfig,
    resetConfig,
    resetConfigSection,
    exportConfig,
    importConfig,
    clearAllCache,
    clearCache,
    getCacheStatus,
    DEFAULT_CONFIG
  }
}, {
  persist: true
}) 