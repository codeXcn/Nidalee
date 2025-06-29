export const appContextKey = Symbol('appContext')
export type AppContext = {
  refreshConnection: () => void
  fetchMatchHistory: () => void
  isConnected: Ref<boolean>
  isDark: Ref<boolean>
}
