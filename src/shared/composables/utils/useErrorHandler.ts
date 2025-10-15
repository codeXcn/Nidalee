import { toast } from 'vue-sonner'

/**
 * 统一错误处理组合式函数
 * 职责：提供统一的错误处理、重试机制和用户友好的错误提示
 */

export interface ErrorInfo {
  message: string
  code?: string
  timestamp: number
  context?: string
  retryable?: boolean
}

export interface RetryConfig {
  maxRetries: number
  baseDelay: number
  maxDelay: number
  backoffMultiplier: number
}

export function useErrorHandler() {
  const errors = ref<ErrorInfo[]>([])
  const isRetrying = ref(false)

  // 默认重试配置
  const defaultRetryConfig: RetryConfig = {
    maxRetries: 3,
    baseDelay: 1000,
    maxDelay: 10000,
    backoffMultiplier: 2
  }

  /**
   * 记录错误
   */
  const recordError = (error: Error | string, context?: string, retryable: boolean = false) => {
    const errorInfo: ErrorInfo = {
      message: error instanceof Error ? error.message : error,
      timestamp: Date.now(),
      context,
      retryable
    }

    errors.value.push(errorInfo)

    // 限制错误记录数量
    if (errors.value.length > 50) {
      errors.value = errors.value.slice(-50)
    }

    console.error(`[ErrorHandler] ${context ? `[${context}] ` : ''}${errorInfo.message}`)
  }

  /**
   * 显示用户友好的错误提示
   */
  const showErrorToast = (error: Error | string, context?: string) => {
    const message = error instanceof Error ? error.message : error
    const title = context ? `${context} 失败` : '操作失败'

    toast.error(title, {
      description: message,
      duration: 5000
    })
  }

  /**
   * 处理错误并显示提示
   */
  const handleError = (error: Error | string, context?: string, showToast: boolean = true) => {
    recordError(error, context)

    if (showToast) {
      showErrorToast(error, context)
    }
  }

  /**
   * 带重试的异步操作
   */
  const withRetry = async <T>(
    operation: () => Promise<T>,
    config: Partial<RetryConfig> = {},
    context?: string
  ): Promise<T> => {
    const retryConfig = { ...defaultRetryConfig, ...config }
    let lastError: Error | null = null

    for (let attempt = 0; attempt <= retryConfig.maxRetries; attempt++) {
      try {
        if (attempt > 0) {
          isRetrying.value = true
          const delay = Math.min(
            retryConfig.baseDelay * Math.pow(retryConfig.backoffMultiplier, attempt - 1),
            retryConfig.maxDelay
          )

          console.log(`[ErrorHandler] 重试 ${attempt}/${retryConfig.maxRetries}，延迟 ${delay}ms`)
          await new Promise((resolve) => setTimeout(resolve, delay))
        }

        const result = await operation()

        if (attempt > 0) {
          console.log(`[ErrorHandler] 重试成功 (${attempt}/${retryConfig.maxRetries})`)
          toast.success('操作成功', {
            description: '重试后操作完成'
          })
        }

        isRetrying.value = false
        return result
      } catch (error) {
        lastError = error instanceof Error ? error : new Error(String(error))

        if (attempt === retryConfig.maxRetries) {
          // 最后一次重试失败
          const errorMessage = `${context || '操作'} 失败，已重试 ${retryConfig.maxRetries} 次`
          handleError(errorMessage, context)
          isRetrying.value = false
          throw lastError
        } else {
          // 记录重试错误
          console.warn(`[ErrorHandler] 尝试 ${attempt + 1} 失败:`, lastError.message)
        }
      }
    }

    isRetrying.value = false
    throw lastError || new Error('未知错误')
  }

  /**
   * 清除错误记录
   */
  const clearErrors = () => {
    errors.value = []
  }

  /**
   * 清除特定上下文的错误
   */
  const clearErrorsByContext = (context: string) => {
    errors.value = errors.value.filter((error) => error.context !== context)
  }

  /**
   * 获取最近错误
   */
  const getRecentErrors = (limit: number = 5) => {
    return errors.value.slice(-limit)
  }

  /**
   * 检查是否有特定上下文的错误
   */
  const hasErrorInContext = (context: string) => {
    return errors.value.some((error) => error.context === context)
  }

  // 计算属性
  const hasErrors = computed(() => errors.value.length > 0)
  const errorCount = computed(() => errors.value.length)
  const recentErrorCount = computed(() => {
    const fiveMinutesAgo = Date.now() - 5 * 60 * 1000
    return errors.value.filter((error) => error.timestamp > fiveMinutesAgo).length
  })

  return {
    // 状态
    errors: readonly(errors),
    isRetrying: readonly(isRetrying),
    hasErrors,
    errorCount,
    recentErrorCount,

    // 方法
    recordError,
    showErrorToast,
    handleError,
    withRetry,
    clearErrors,
    clearErrorsByContext,
    getRecentErrors,
    hasErrorInContext
  }
}
