import { ref, computed, watch, type Ref } from 'vue'

/**
 * 创建防抖的双向绑定
 * @param originalModel 原始的 model 引用
 * @param delay 防抖延迟时间（毫秒）
 * @param immediate 是否立即设置初始值
 */
export function useDebouncedModel<T>(
  originalModel: Ref<T>,
  delay: number = 300,
  immediate: boolean = true
) {
  // 内部状态，用于存储用户输入的临时值
  const internalValue = ref<T>(originalModel.value)
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  // 当原始 model 变化时，立即更新内部值（外部更新）
  watch(
    originalModel,
    (newValue) => {
      if (newValue !== internalValue.value) {
        internalValue.value = newValue
      }
    },
    { immediate }
  )

  // 创建防抖的计算属性
  const debouncedModel = computed({
    get: () => internalValue.value,
    set: (newValue: T) => {
      // 立即更新内部值（用于 UI 响应）
      internalValue.value = newValue

      // 清除之前的定时器
      if (timeoutId) {
        clearTimeout(timeoutId)
      }

      // 设置新的防抖定时器
      timeoutId = setTimeout(() => {
        // 防抖延迟后更新原始 model
        if (originalModel.value !== newValue) {
          originalModel.value = newValue
        }
        timeoutId = null
      }, delay)
    }
  })

  // 清理函数
  const cleanup = () => {
    if (timeoutId) {
      clearTimeout(timeoutId)
      timeoutId = null
    }
  }

  // 手动触发保存（跳过防抖）
  const flush = () => {
    if (timeoutId) {
      clearTimeout(timeoutId)
      timeoutId = null
    }
    if (originalModel.value !== internalValue.value) {
      originalModel.value = internalValue.value
    }
  }

  return {
    value: debouncedModel,
    internalValue: readonly(internalValue),
    cleanup,
    flush,
    isPending: computed(() => timeoutId !== null)
  }
}

/**
 * 专门用于数字类型的防抖 model，带有范围限制
 */
export function useDebouncedNumberModel(
  originalModel: Ref<number>,
  options: {
    delay?: number
    min?: number
    max?: number
    step?: number
    immediate?: boolean
  } = {}
) {
  const { delay = 300, min = 0, max = Infinity, step = 1, immediate = true } = options

  const baseDebounced = useDebouncedModel(originalModel, delay, immediate)

  // 创建带范围限制的计算属性
  const constrainedModel = computed({
    get: () => baseDebounced.value.value,
    set: (newValue: number) => {
      // 应用范围限制和步长
      let constrainedValue = Math.max(min, Math.min(max, newValue))

      // 应用步长（如果有）
      if (step > 0 && step !== 1) {
        constrainedValue = Math.round(constrainedValue / step) * step
      }

      baseDebounced.value.value = constrainedValue
    }
  })

  return {
    value: constrainedModel,
    internalValue: baseDebounced.internalValue,
    cleanup: baseDebounced.cleanup,
    flush: baseDebounced.flush,
    isPending: baseDebounced.isPending
  }
}
