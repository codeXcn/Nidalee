import { createPinia } from 'pinia'
import { createPersistedState } from 'pinia-plugin-persistedstate'

export const pinia = createPinia()

// 配置持久化插件
pinia.use(
  createPersistedState({
    // 全局默认配置
    storage: localStorage,
    serializer: {
      serialize: JSON.stringify,
      deserialize: JSON.parse
    }
  })
)

export * from './useGameStore'
export * from './useUserConfigStore'
export * from './useThemeStore'
