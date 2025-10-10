/**
 * 性能监控工具
 * 用于监控和优化应用性能
 */

import { eventBus, GameEvents } from './eventBus'

interface PerformanceMetric {
  label: string
  duration: number
  timestamp: number
  metadata?: Record<string, any>
}

interface SlowOperation {
  operation: string
  duration: number
  threshold: number
  timestamp: number
  metadata?: Record<string, any>
}

class PerformanceMonitor {
  private metrics = new Map<string, number[]>()
  private slowOperations: SlowOperation[] = []
  private readonly maxSlowOperations = 100
  private readonly slowOperationThreshold = 1000 // 1秒

  /**
   * 开始性能计时
   * @param label 计时标签
   */
  startTiming(label: string): void {
    performance.mark(`${label}-start`)
  }

  /**
   * 结束性能计时并记录
   * @param label 计时标签
   * @param metadata 额外元数据
   */
  endTiming(label: string, metadata?: Record<string, any>): number {
    // 检查是否存在对应的 start mark
    const startMark = performance.getEntriesByName(`${label}-start`)[0]
    if (!startMark) {
      console.warn(`[PerformanceMonitor] 未找到开始标记: ${label}-start`)
      return 0
    }

    performance.mark(`${label}-end`)
    performance.measure(label, `${label}-start`, `${label}-end`)

    const measure = performance.getEntriesByName(label)[0]
    if (measure) {
      const duration = measure.duration

      // 记录指标
      if (!this.metrics.has(label)) {
        this.metrics.set(label, [])
      }
      this.metrics.get(label)!.push(duration)

      // 检查是否为慢操作
      if (duration > this.slowOperationThreshold) {
        const slowOp: SlowOperation = {
          operation: label,
          duration,
          threshold: this.slowOperationThreshold,
          timestamp: Date.now(),
          metadata
        }

        this.slowOperations.push(slowOp)

        // 保持数组大小
        if (this.slowOperations.length > this.maxSlowOperations) {
          this.slowOperations.shift()
        }

        // 发布慢操作事件
        eventBus.emit(GameEvents.SLOW_OPERATION, slowOp)
      }

      // 发布性能指标事件
      const metric: PerformanceMetric = {
        label,
        duration,
        timestamp: Date.now(),
        metadata
      }
      eventBus.emit(GameEvents.PERFORMANCE_METRIC, metric)

      console.log(`[Performance] ${label}: ${duration.toFixed(2)}ms`)

      // 清理性能条目
      performance.clearMarks(`${label}-start`)
      performance.clearMarks(`${label}-end`)
      performance.clearMeasures(label)

      return duration
    }

    return 0
  }

  /**
   * 获取平均执行时间
   * @param label 计时标签
   */
  getAverageTime(label: string): number {
    const times = this.metrics.get(label) || []
    if (times.length === 0) return 0
    return times.reduce((a, b) => a + b, 0) / times.length
  }

  /**
   * 获取最大执行时间
   * @param label 计时标签
   */
  getMaxTime(label: string): number {
    const times = this.metrics.get(label) || []
    return times.length > 0 ? Math.max(...times) : 0
  }

  /**
   * 获取最小执行时间
   * @param label 计时标签
   */
  getMinTime(label: string): number {
    const times = this.metrics.get(label) || []
    return times.length > 0 ? Math.min(...times) : 0
  }

  /**
   * 获取执行次数
   * @param label 计时标签
   */
  getExecutionCount(label: string): number {
    const times = this.metrics.get(label) || []
    return times.length
  }

  /**
   * 获取所有指标
   */
  getAllMetrics(): Record<string, {
    average: number
    max: number
    min: number
    count: number
  }> {
    const result: Record<string, any> = {}

    this.metrics.forEach((times, label) => {
      result[label] = {
        average: this.getAverageTime(label),
        max: this.getMaxTime(label),
        min: this.getMinTime(label),
        count: times.length
      }
    })

    return result
  }

  /**
   * 获取慢操作列表
   */
  getSlowOperations(): SlowOperation[] {
    return [...this.slowOperations]
  }

  /**
   * 获取最近的慢操作
   * @param count 数量
   */
  getRecentSlowOperations(count: number = 10): SlowOperation[] {
    return this.slowOperations.slice(-count)
  }

  /**
   * 清除所有指标
   */
  clearMetrics(): void {
    this.metrics.clear()
    this.slowOperations.length = 0
  }

  /**
   * 清除指定标签的指标
   * @param label 计时标签
   */
  clearMetric(label: string): void {
    this.metrics.delete(label)
  }

  /**
   * 生成性能报告
   */
  generateReport(): string {
    const metrics = this.getAllMetrics()
    const slowOps = this.getRecentSlowOperations(5)

    let report = '=== 性能监控报告 ===\n\n'

    // 指标统计
    report += '📊 性能指标:\n'
    Object.entries(metrics).forEach(([label, stats]) => {
      report += `  ${label}:\n`
      report += `    平均: ${stats.average.toFixed(2)}ms\n`
      report += `    最大: ${stats.max.toFixed(2)}ms\n`
      report += `    最小: ${stats.min.toFixed(2)}ms\n`
      report += `    次数: ${stats.count}\n\n`
    })

    // 慢操作
    if (slowOps.length > 0) {
      report += '🐌 最近慢操作:\n'
      slowOps.forEach(op => {
        report += `  ${op.operation}: ${op.duration.toFixed(2)}ms (阈值: ${op.threshold}ms)\n`
      })
    }

    return report
  }

  /**
   * 异步操作性能监控装饰器
   * @param label 标签
   * @param fn 异步函数
   */
  async measureAsync<T>(label: string, fn: () => Promise<T>): Promise<T> {
    this.startTiming(label)
    try {
      const result = await fn()
      this.endTiming(label)
      return result
    } catch (error) {
      this.endTiming(label, { error: error instanceof Error ? error.message : String(error) })
      throw error
    }
  }

  /**
   * 同步操作性能监控装饰器
   * @param label 标签
   * @param fn 同步函数
   */
  measureSync<T>(label: string, fn: () => T): T {
    this.startTiming(label)
    try {
      const result = fn()
      this.endTiming(label)
      return result
    } catch (error) {
      this.endTiming(label, { error: error instanceof Error ? error.message : String(error) })
      throw error
    }
  }
}

// 创建全局性能监控实例
export const performanceMonitor = new PerformanceMonitor()

// 导出类型
export type { PerformanceMetric, SlowOperation }
export default performanceMonitor
