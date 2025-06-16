import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
export * from './gameStore'
export * from './themeStore'
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

export default pinia
