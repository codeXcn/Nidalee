import { useActivityLogger } from '@/composables/utils/useActivityLogger'
import { useAutoFunctionStore } from '@/stores/autoFunctionStore'

// 专门处理自动功能
export function useAutoFunctionManager() {
  const autoFunctionStore = useAutoFunctionStore()
  const activityLogger = useActivityLogger()

  // 自动功能变更处理
  const handleAutoFunctionToggle = (key: 'acceptMatch' | 'selectChampion' | 'runeConfig' | 'banChampion') => {
    const result = autoFunctionStore.toggleAutoFunction(key)
    if (result.enabled) {
      activityLogger.logSettings.autoFunctionEnabled(`自动${result.name}`)
    } else {
      activityLogger.logSettings.autoFunctionDisabled(`自动${result.name}`)
    }
    return result
  }

  return {
    handleAutoFunctionToggle
  }
}
