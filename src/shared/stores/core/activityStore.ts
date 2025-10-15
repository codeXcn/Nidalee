import type { Activity } from '@/types'
import { now } from '@vueuse/core'

export const useActivityStore = defineStore(
  'activity',
  () => {
    console.log('[🚀 ActivityStore] 初始化活动记录存储')

    // 活动记录
    const activities = ref<Activity[]>([])

    // 添加活动记录
    const addActivity = (type: Activity['type'], message: string, category: Activity['category'] = 'system') => {
      console.log(`[📝 ActivityStore] 添加活动记录: [${type}] [${category}] ${message}`)

      const activity: Activity = {
        id: Date.now().toString(),
        type,
        message,
        timestamp: now(),
        category
      }

      activities.value.unshift(activity)

      // 限制活动记录数量，避免内存占用过多
      if (activities.value.length > 100) {
        activities.value = activities.value.slice(0, 100)
        console.log('[📝 ActivityStore] 活动记录数量已限制在 100 条')
      }

      console.log(`[📝 ActivityStore] 当前活动记录总数: ${activities.value.length}`)
    }

    // 清除所有活动记录
    const clearActivities = () => {
      console.log('[🗑️ ActivityStore] 清除所有活动记录')
      const beforeCount = activities.value.length
      activities.value = []
      console.log(`[🗑️ ActivityStore] 已清除 ${beforeCount} 条活动记录`)
      addActivity('info', '活动记录已清除', 'system')
    }

    // 清除指定类型的活动记录
    const clearActivitiesByType = (type: Activity['type']) => {
      console.log(`[🗑️ ActivityStore] 清除指定类型的活动记录: ${type}`)
      const beforeCount = activities.value.length
      activities.value = activities.value.filter((activity) => activity.type !== type)
      const clearedCount = beforeCount - activities.value.length

      if (clearedCount > 0) {
        console.log(`[🗑️ ActivityStore] 已清除 ${clearedCount} 条 ${type} 类型的记录`)
        addActivity('info', `已清除 ${clearedCount} 条 ${type} 类型的记录`, 'system')
      } else {
        console.log(`[🗑️ ActivityStore] 没有找到 ${type} 类型的记录`)
      }
    }

    // 根据类型获取活动记录
    const getActivitiesByType = (type: Activity['type']) => {
      return activities.value.filter((activity) => activity.type === type)
    }

    // 根据分类获取活动记录
    const getActivitiesByCategory = (category: Activity['category']) => {
      return activities.value.filter((activity) => activity.category === category)
    }

    // 清除指定分类的活动记录
    const clearActivitiesByCategory = (category: Activity['category']) => {
      console.log(`[🗑️ ActivityStore] 清除指定分类的活动记录: ${category}`)
      const beforeCount = activities.value.length
      activities.value = activities.value.filter((activity) => activity.category !== category)
      const clearedCount = beforeCount - activities.value.length

      if (clearedCount > 0) {
        console.log(`[🗑️ ActivityStore] 已清除 ${clearedCount} 条 ${category} 分类的记录`)
        addActivity('info', `已清除 ${clearedCount} 条 ${category} 分类的记录`, 'system')
      } else {
        console.log(`[🗑️ ActivityStore] 没有找到 ${category} 分类的记录`)
      }
    }

    // 清理旧的活动记录
    const cleanupOldActivities = (daysToKeep = 7) => {
      const cutoffTime = now() - daysToKeep * 24 * 60 * 60 * 1000
      const beforeCount = activities.value.length
      activities.value = activities.value.filter((activity) => activity.timestamp > cutoffTime)
      const clearedCount = beforeCount - activities.value.length

      if (clearedCount > 0) {
        console.log(`[🗑️ ActivityStore] 清理了 ${clearedCount} 条旧活动记录`)
        addActivity('info', `清理了 ${clearedCount} 条旧活动记录`, 'system')
      }
    }

    // 获取活动统计信息
    const getActivityStats = () => {
      const stats = {
        total: activities.value.length,
        byType: {} as Record<Activity['type'], number>,
        byCategory: {} as Record<Activity['category'], number>
      }

      // 按类型统计
      activities.value.forEach((activity) => {
        stats.byType[activity.type] = (stats.byType[activity.type] || 0) + 1
        stats.byCategory[activity.category] = (stats.byCategory[activity.category] || 0) + 1
      })

      return stats
    }

    // 标记所有活动为已读
    const markAllAsRead = () => {
      activities.value.forEach((activity) => {
        activity.read = true
      })
    }

    // 标记指定活动为已读
    const markAsRead = (activityId: string) => {
      const activity = activities.value.find((a) => a.id === activityId)
      if (activity) {
        activity.read = true
      }
    }

    // 计算属性
    const recentActivities = computed(() => activities.value.slice(0, 10))
    const errorCount = computed(() => activities.value.filter((a) => a.type === 'error').length)
    const warningCount = computed(() => activities.value.filter((a) => a.type === 'warning').length)
    const successCount = computed(() => activities.value.filter((a) => a.type === 'success').length)
    const infoCount = computed(() => activities.value.filter((a) => a.type === 'info').length)

    return {
      // 状态
      activities: readonly(activities),

      // 计算属性
      recentActivities,
      errorCount,
      warningCount,
      successCount,
      infoCount,

      // 方法
      addActivity,
      clearActivities,
      clearActivitiesByType,
      clearActivitiesByCategory,
      getActivitiesByType,
      getActivitiesByCategory,
      cleanupOldActivities,
      getActivityStats,
      markAllAsRead,
      markAsRead
    }
  },
  {
    persist: true
  }
)
