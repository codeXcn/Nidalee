import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

export * from './themeStore'
export * from './connectStore'
export * from './summonerStore'
export * from './gameStatusStore'
export * from './activityStore'
export * from './autoFunctionStore'
export * from './matchStatisticsStore'
export * from './appSessionStore'

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

export default pinia
