import { now } from '@vueuse/core'

export const useActivityStore = defineStore(
  'activity',
  () => {
    // 活动记录
    const activities = ref<Activity[]>([
      {
        id: '1',
        type: 'info',
        message: '应用启动成功',
        timestamp: now()
      }
    ])

    // 添加活动记录
    const addActivity = (type: Activity['type'], message: string) => {
      const activity: Activity = {
        id: Date.now().toString(),
        type,
        message,
        timestamp: now()
      }

      activities.value.unshift(activity)

      // 限制活动记录数量，避免内存占用过多
      if (activities.value.length > 50) {
        activities.value = activities.value.slice(0, 50)
      }
    }

    // 清除所有活动记录
    const clearActivities = () => {
      activities.value = []
      addActivity('info', '活动记录已清除')
    }

    // 清除指定类型的活动记录
    const clearActivitiesByType = (type: Activity['type']) => {
      const beforeCount = activities.value.length
      activities.value = activities.value.filter((activity) => activity.type !== type)
      const clearedCount = beforeCount - activities.value.length

      if (clearedCount > 0) {
        addActivity('info', `已清除 ${clearedCount} 条 ${type} 类型的记录`)
      }
    }

    // 根据类型获取活动记录
    const getActivitiesByType = (type: Activity['type']) => {
      return activities.value.filter((activity) => activity.type === type)
    }

    // 计算属性
    const recentActivities = computed(() => activities.value.slice(0, 10))
    const errorCount = computed(() => activities.value.filter((a) => a.type === 'error').length)
    const warningCount = computed(() => activities.value.filter((a) => a.type === 'warning').length)
    const successCount = computed(() => activities.value.filter((a) => a.type === 'success').length)

    return {
      // 状态
      activities: readonly(activities),

      // 计算属性
      recentActivities,
      errorCount,
      warningCount,
      successCount,

      // 方法
      addActivity,
      clearActivities,
      clearActivitiesByType,
      getActivitiesByType
    }
  },
  {
    persist: {
      paths: ['activities']
    }
  }
)
