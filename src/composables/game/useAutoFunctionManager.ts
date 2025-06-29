import { useAutoFunctionStore, useActivityStore } from '@/stores'

// 专门处理自动功能
export function useAutoFunctionManager() {
  const autoFunctionStore = useAutoFunctionStore()
  const activityStore = useActivityStore()

  // 自动功能变更处理
  const handleAutoFunctionToggle = (key: keyof ReturnType<typeof autoFunctionStore>['autoFunctions']) => {
    const result = autoFunctionStore.toggleAutoFunction(key)
    if (result.enabled) {
      activityStore.addSettingsActivity.autoFunctionEnabled(`自动${result.name}`)
    } else {
      activityStore.addSettingsActivity.autoFunctionDisabled(`自动${result.name}`)
    }
    return result
  }

  return {
    handleAutoFunctionToggle
  }
}
