import { useAutoFunctionStore, useActivityStore } from '@/stores'

// 专门处理自动功能
export function useAutoFunctionManager() {
  const autoFunctionStore = useAutoFunctionStore()
  const activityStore = useActivityStore()

  // 自动功能变更处理
  const handleAutoFunctionToggle = (key: keyof ReturnType<typeof autoFunctionStore>['autoFunctions']) => {
    const result = autoFunctionStore.toggleAutoFunction(key)
    const status = result.enabled ? '已启用' : '已禁用'
    activityStore.addActivity('info', `自动${result.name}${status}`)
    return result
  }

  return {
    handleAutoFunctionToggle
  }
}
