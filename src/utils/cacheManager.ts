/**
 * 缓存管理工具类
 * 用于统一管理应用的本地存储缓存
 */

export interface CacheInfo {
  key: string
  name: string
  size: number
  lastModified: string
  enabled: boolean
}

export class CacheManager {
  private static readonly CACHE_KEYS = {
    userConfig: 'userConfig',
    theme: 'theme',
    gameStore: 'gameStore'
  }

  /**
   * 获取所有缓存信息
   */
  static getAllCacheInfo(): CacheInfo[] {
    return Object.entries(this.CACHE_KEYS).map(([name, key]) => {
      const data = localStorage.getItem(key)
      return {
        key,
        name,
        size: data ? new Blob([data]).size : 0,
        lastModified: data ? this.getLastModified(key) : '',
        enabled: !!data
      }
    })
  }

  /**
   * 获取特定缓存的信息
   */
  static getCacheInfo(cacheType: keyof typeof CacheManager.CACHE_KEYS): CacheInfo | null {
    const key = this.CACHE_KEYS[cacheType]
    const data = localStorage.getItem(key)

    if (!data) return null

    return {
      key,
      name: cacheType,
      size: new Blob([data]).size,
      lastModified: this.getLastModified(key),
      enabled: true
    }
  }

  /**
   * 清除特定缓存
   */
  static clearCache(cacheType: keyof typeof CacheManager.CACHE_KEYS): boolean {
    try {
      const key = this.CACHE_KEYS[cacheType]
      localStorage.removeItem(key)
      return true
    } catch (error) {
      console.error(`Failed to clear cache ${cacheType}:`, error)
      return false
    }
  }

  /**
   * 清除所有缓存
   */
  static clearAllCache(): boolean {
    try {
      Object.values(this.CACHE_KEYS).forEach(key => {
        localStorage.removeItem(key)
      })
      return true
    } catch (error) {
      console.error('Failed to clear all cache:', error)
      return false
    }
  }

  /**
   * 获取缓存总大小
   */
  static getTotalCacheSize(): number {
    return Object.values(this.CACHE_KEYS).reduce((total, key) => {
      const data = localStorage.getItem(key)
      return total + (data ? new Blob([data]).size : 0)
    }, 0)
  }

  /**
   * 格式化字节大小
   */
  static formatSize(bytes: number): string {
    if (bytes === 0) return '0 B'

    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))

    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  /**
   * 检查缓存是否启用
   */
  static isCacheEnabled(cacheType: keyof typeof CacheManager.CACHE_KEYS): boolean {
    try {
      const configData = localStorage.getItem('userConfig')
      if (!configData) return true

      const config = JSON.parse(configData)
      const cacheKeyMap = {
        userConfig: 'enableConfigCache',
        theme: 'enableThemeCache',
        gameStore: 'enableGameDataCache'
      }

      return config.app?.[cacheKeyMap[cacheType]] !== false
    } catch {
      return true
    }
  }

  /**
   * 获取最后修改时间
   */
  private static getLastModified(key: string): string {
    // 由于 localStorage 没有内置的时间戳，我们使用一个约定的方式
    const timestampKey = `${key}_timestamp`
    const timestamp = localStorage.getItem(timestampKey)

    if (timestamp) {
      return new Date(parseInt(timestamp)).toLocaleString()
    }

    // 如果没有时间戳，返回当前时间
    return new Date().toLocaleString()
  }

  /**
   * 设置时间戳（在保存数据时调用）
   */
  static setTimestamp(key: string): void {
    const timestampKey = `${key}_timestamp`
    localStorage.setItem(timestampKey, Date.now().toString())
  }

  /**
   * 导出所有缓存数据
   */
  static exportAllCache(): string {
    const cacheData: Record<string, any> = {}

    Object.entries(this.CACHE_KEYS).forEach(([name, key]) => {
      const data = localStorage.getItem(key)
      if (data) {
        try {
          cacheData[name] = JSON.parse(data)
        } catch {
          cacheData[name] = data
        }
      }
    })

    return JSON.stringify(cacheData, null, 2)
  }

  /**
   * 导入缓存数据
   */
  static importCache(cacheDataJson: string): boolean {
    try {
      const cacheData = JSON.parse(cacheDataJson)

      Object.entries(this.CACHE_KEYS).forEach(([name, key]) => {
        if (cacheData[name]) {
          const dataToStore =
            typeof cacheData[name] === 'string' ? cacheData[name] : JSON.stringify(cacheData[name])
          localStorage.setItem(key, dataToStore)
          this.setTimestamp(key)
        }
      })

      return true
    } catch (error) {
      console.error('Failed to import cache:', error)
      return false
    }
  }
}
