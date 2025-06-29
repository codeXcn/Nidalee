import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

// 核心stores
export * from './core'

// 功能stores
export * from './features'

// UI stores
export * from './ui'

// 自动功能store
export * from './autoFunctionStore'

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

export default pinia
