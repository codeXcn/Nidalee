/**
 * 重试机制工具函数
 * 提供指数退避、线性退避等多种重试策略
 */

export interface RetryConfig {
  maxRetries: number
  baseDelay: number
  maxDelay: number
  backoffMultiplier: number
  jitter: boolean
  retryCondition?: (error: any) => boolean
}

export interface RetryResult<T> {
  success: boolean
  data?: T
  error?: any
  attempts: number
  totalTime: number
}

/**
 * 默认重试配置
 */
export const defaultRetryConfig: RetryConfig = {
  maxRetries: 3,
  baseDelay: 1000,
  maxDelay: 10000,
  backoffMultiplier: 2,
  jitter: true
}

/**
 * 指数退避重试
 */
export async function retryWithExponentialBackoff<T>(
  fn: () => Promise<T>,
  config: Partial<RetryConfig> = {}
): Promise<RetryResult<T>> {
  const finalConfig = { ...defaultRetryConfig, ...config }
  const startTime = Date.now()
  let lastError: any

  for (let attempt = 0; attempt <= finalConfig.maxRetries; attempt++) {
    try {
      const result = await fn()
      return {
        success: true,
        data: result,
        attempts: attempt + 1,
        totalTime: Date.now() - startTime
      }
    } catch (error) {
      lastError = error

      // 检查是否应该重试
      if (finalConfig.retryCondition && !finalConfig.retryCondition(error)) {
        break
      }

      // 如果这是最后一次尝试，不再等待
      if (attempt === finalConfig.maxRetries) {
        break
      }

      // 计算延迟时间
      const delay = calculateDelay(attempt, finalConfig)

      console.log(`[Retry] 尝试 ${attempt + 1} 失败，${delay}ms 后重试:`, error)

      await new Promise(resolve => setTimeout(resolve, delay))
    }
  }

  return {
    success: false,
    error: lastError,
    attempts: finalConfig.maxRetries + 1,
    totalTime: Date.now() - startTime
  }
}

/**
 * 线性退避重试
 */
export async function retryWithLinearBackoff<T>(
  fn: () => Promise<T>,
  config: Partial<RetryConfig> = {}
): Promise<RetryResult<T>> {
  const linearConfig = { ...defaultRetryConfig, ...config, backoffMultiplier: 1 }
  return retryWithExponentialBackoff(fn, linearConfig)
}

/**
 * 固定间隔重试
 */
export async function retryWithFixedDelay<T>(
  fn: () => Promise<T>,
  config: Partial<RetryConfig> = {}
): Promise<RetryResult<T>> {
  const fixedConfig = {
    ...defaultRetryConfig,
    ...config,
    backoffMultiplier: 1,
    maxDelay: config.baseDelay || defaultRetryConfig.baseDelay
  }
  return retryWithExponentialBackoff(fn, fixedConfig)
}

/**
 * 计算延迟时间
 */
function calculateDelay(attempt: number, config: RetryConfig): number {
  // 基础延迟 * 退避倍数^尝试次数
  let delay = config.baseDelay * Math.pow(config.backoffMultiplier, attempt)

  // 限制最大延迟
  delay = Math.min(delay, config.maxDelay)

  // 添加抖动（随机性）
  if (config.jitter) {
    delay = delay * (0.5 + Math.random() * 0.5)
  }

  return Math.floor(delay)
}

/**
 * 网络请求重试装饰器
 */
export function withRetry<T extends (...args: any[]) => Promise<any>>(
  fn: T,
  config: Partial<RetryConfig> = {}
): T {
  return (async (...args: Parameters<T>) => {
    const result = await retryWithExponentialBackoff(
      () => fn(...args),
      config
    )

    if (!result.success) {
      throw result.error
    }

    return result.data
  }) as T
}

/**
 * 网络错误重试条件
 */
export const networkRetryCondition = (error: any): boolean => {
  // 网络错误、超时错误、5xx 服务器错误可以重试
  if (error.name === 'NetworkError' || error.name === 'TimeoutError') {
    return true
  }

  // HTTP 状态码
  if (error.status >= 500 && error.status < 600) {
    return true
  }

  // 连接被拒绝
  if (error.code === 'ECONNREFUSED' || error.code === 'ENOTFOUND') {
    return true
  }

  return false
}

/**
 * 认证错误重试条件
 */
export const authRetryCondition = (error: any): boolean => {
  // 401 错误可以重试（可能是 token 过期）
  if (error.status === 401) {
    return true
  }

  return networkRetryCondition(error)
}

/**
 * 游戏数据重试条件
 */
export const gameDataRetryCondition = (error: any): boolean => {
  // 游戏客户端相关的错误可以重试
  if (error.message?.includes('LiveClient') || error.message?.includes('LCU')) {
    return true
  }

  return networkRetryCondition(error)
}

/**
 * 重试状态管理
 */
export class RetryManager {
  private retryCounts = new Map<string, number>()
  private lastRetryTimes = new Map<string, number>()

  /**
   * 检查是否可以重试
   */
  canRetry(key: string, maxRetries: number = 3, cooldown: number = 5000): boolean {
    const count = this.retryCounts.get(key) || 0
    const lastTime = this.lastRetryTimes.get(key) || 0
    const now = Date.now()

    // 检查重试次数
    if (count >= maxRetries) {
      return false
    }

    // 检查冷却时间
    if (now - lastTime < cooldown) {
      return false
    }

    return true
  }

  /**
   * 记录重试
   */
  recordRetry(key: string): void {
    const count = this.retryCounts.get(key) || 0
    this.retryCounts.set(key, count + 1)
    this.lastRetryTimes.set(key, Date.now())
  }

  /**
   * 重置重试计数
   */
  resetRetry(key: string): void {
    this.retryCounts.delete(key)
    this.lastRetryTimes.delete(key)
  }

  /**
   * 获取重试次数
   */
  getRetryCount(key: string): number {
    return this.retryCounts.get(key) || 0
  }
}

// 全局重试管理器实例
export const retryManager = new RetryManager()

