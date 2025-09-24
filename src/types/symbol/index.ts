export const appContextKey = Symbol('appContext')
export type AppContext = {
  checkConnection: () => void
  fetchMatchHistory: () => void
  isConnected: Ref<boolean>
  isDark: Ref<boolean>
}
