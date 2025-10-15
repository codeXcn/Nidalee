/**
 * 性能监控组合式函数
 * 提供 Vue 组件中使用性能监控的便捷方法
 */

import { onUnmounted, ref } from 'vue'
import { performanceMonitor, type PerformanceMetric, type SlowOperation } from '@/lib/performanceMonitor'
import { useGameEvents } from './useEventBus'

export function usePerformanceMonitor() {
  const { onPerformanceMetric, onSlowOperation } = useGameEvents()

  // 响应式性能指标
  const metrics = ref<Record<string, any>>({})
  const slowOperations = ref<SlowOperation[]>([])
  const isMonitoring = ref(false)

  /**
   * 开始性能监控
   */
  const startMonitoring = () => {
    if (isMonitoring.value) return

    isMonitoring.value = true

    // 监听性能指标事件
    onPerformanceMetric((metric: PerformanceMetric) => {
      const label = metric.label
      if (!metrics.value[label]) {
        metrics.value[label] = {
          average: 0,
          max: 0,
          min: 0,
          count: 0,
          recent: []
        }
      }

      const stats = metrics.value[label]
      stats.recent.push(metric.duration)

      // 保持最近10次记录
      if (stats.recent.length > 10) {
        stats.recent.shift()
      }

      // 更新统计
      stats.average = performanceMonitor.getAverageTime(label)
      stats.max = performanceMonitor.getMaxTime(label)
      stats.min = performanceMonitor.getMinTime(label)
      stats.count = performanceMonitor.getExecutionCount(label)
    })

    // 监听慢操作事件
    onSlowOperation((operation: SlowOperation) => {
      slowOperations.value.unshift(operation)

      // 保持最近20个慢操作
      if (slowOperations.value.length > 20) {
        slowOperations.value.pop()
      }
    })

    console.log('[PerformanceMonitor] 性能监控已启动')
  }

  /**
   * 停止性能监控
   */
  const stopMonitoring = () => {
    isMonitoring.value = false
    console.log('[PerformanceMonitor] 性能监控已停止')
  }

  /**
   * 开始计时
   * @param label 标签
   */
  const startTiming = (label: string) => {
    performanceMonitor.startTiming(label)
  }

  /**
   * 结束计时
   * @param label 标签
   * @param metadata 元数据
   */
  const endTiming = (label: string, metadata?: Record<string, any>) => {
    return performanceMonitor.endTiming(label, metadata)
  }

  /**
   * 测量异步操作
   * @param label 标签
   * @param fn 异步函数
   */
  const measureAsync = async <T>(label: string, fn: () => Promise<T>): Promise<T> => {
    return performanceMonitor.measureAsync(label, fn)
  }

  /**
   * 测量同步操作
   * @param label 标签
   * @param fn 同步函数
   */
  const measureSync = <T>(label: string, fn: () => T): T => {
    return performanceMonitor.measureSync(label, fn)
  }

  /**
   * 获取性能报告
   */
  const getReport = () => {
    return performanceMonitor.generateReport()
  }

  /**
   * 清除所有指标
   */
  const clearMetrics = () => {
    performanceMonitor.clearMetrics()
    metrics.value = {}
    slowOperations.value = []
  }

  /**
   * 获取指定标签的统计信息
   * @param label 标签
   */
  const getMetricStats = (label: string) => {
    return {
      average: performanceMonitor.getAverageTime(label),
      max: performanceMonitor.getMaxTime(label),
      min: performanceMonitor.getMinTime(label),
      count: performanceMonitor.getExecutionCount(label)
    }
  }

  // 组件卸载时停止监控
  onUnmounted(() => {
    stopMonitoring()
  })

  return {
    // 状态
    metrics: readonly(metrics),
    slowOperations: readonly(slowOperations),
    isMonitoring: readonly(isMonitoring),

    // 方法
    startMonitoring,
    stopMonitoring,
    startTiming,
    endTiming,
    measureAsync,
    measureSync,
    getReport,
    clearMetrics,
    getMetricStats
  }
}

/**
 * 自动性能监控装饰器
 * 自动监控组件方法的性能
 */
export function useAutoPerformanceMonitor() {
  const { startTiming, endTiming } = usePerformanceMonitor()

  /**
   * 创建性能监控方法
   * @param methodName 方法名
   * @param fn 原始方法
   */
  const createMonitoredMethod = <T extends (...args: any[]) => any>(methodName: string, fn: T): T => {
    return ((...args: any[]) => {
      startTiming(methodName)
      try {
        const result = fn(...args)

        // 如果是 Promise，等待完成
        if (result instanceof Promise) {
          return result.finally(() => {
            endTiming(methodName)
          })
        } else {
          endTiming(methodName)
          return result
        }
      } catch (error) {
        endTiming(methodName, { error: error instanceof Error ? error.message : String(error) })
        throw error
      }
    }) as T
  }

  return {
    createMonitoredMethod
  }
}
