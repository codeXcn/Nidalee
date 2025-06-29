import { now } from '@vueuse/core'
import type { Activity } from '@/types'
import { ActivityCategoryLabels } from '@/types'

export const useActivityStore = defineStore(
  'activity',
  () => {
    console.log('[🚀 ActivityStore] 初始化活动记录存储')

    // 活动记录
    const activities = ref<Activity[]>([
      // {
      //   id: '1',
      //   type: 'info',
      //   message: '应用启动成功',
      //   timestamp: now(),
      //   category: 'system'
      // }
    ])

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

    // === 连接状态相关 ===
    const addConnectionActivity = {
      connected: () => {
        console.log('[🌐 ActivityStore] 记录连接成功事件')
        addActivity('success', '客户端连接成功', 'connection')
      },
      disconnected: () => {
        console.log('[🌐 ActivityStore] 记录连接断开事件')
        addActivity('error', '客户端连接断开', 'connection')
      },
      reconnected: () => {
        console.log('[🌐 ActivityStore] 记录重连成功事件')
        addActivity('info', '客户端重连成功', 'connection')
      },
      timeout: () => {
        console.log('[🌐 ActivityStore] 记录连接超时事件')
        addActivity('warning', '客户端连接超时，正在重试...', 'connection')
      },
      failed: (reason?: string) => {
        console.log('[🌐 ActivityStore] 记录连接失败事件:', reason)
        addActivity('error', `连接失败${reason ? `: ${reason}` : ''}`, 'connection')
      }
    }

    // === 游戏状态相关 ===
    const addGameActivity = {
      queueing: () => {
        console.log('[🎮 ActivityStore] 记录队列匹配事件')
        addActivity('info', '进入队列匹配中', 'game')
      },
      readyCheck: () => {
        console.log('[🎮 ActivityStore] 记录找到对局事件')
        addActivity('success', '找到对局，等待接受', 'game')
      },
      champSelect: () => {
        console.log('[🎮 ActivityStore] 记录英雄选择事件')
        addActivity('info', '进入英雄选择阶段', 'game')
      },
      gameStart: () => {
        console.log('[🎮 ActivityStore] 记录游戏开始事件')
        addActivity('success', '游戏开始', 'game')
      },
      gameEnd: (result?: 'win' | 'lose') => {
        console.log('[🎮 ActivityStore] 记录游戏结束事件:', result)
        const message = result ? `游戏结束 - ${result === 'win' ? '胜利' : '失败'}` : '游戏结束'
        addActivity('info', message, 'game')
      },
      backToLobby: () => {
        console.log('[🎮 ActivityStore] 记录返回大厅事件')
        addActivity('info', '返回客户端主界面', 'game')
      }
    }

    // === 自动功能执行 ===
    const addAutoFunctionActivity = {
      acceptMatch: {
        success: () => {
          console.log('[🤖 ActivityStore] 记录自动接受对局成功')
          addActivity('success', '自动接受对局成功', 'auto')
        },
        failed: (reason = '超时') => {
          console.log('[🤖 ActivityStore] 记录自动接受对局失败:', reason)
          addActivity('error', `自动接受对局失败：${reason}`, 'auto')
        }
      },
      selectChampion: {
        success: (championName: string) => {
          console.log('[🤖 ActivityStore] 记录自动选择英雄成功:', championName)
          addActivity('success', `自动选择英雄：${championName}`, 'auto')
        },
        failed: (championName: string, reason = '不可用') => {
          console.log('[🤖 ActivityStore] 记录自动选择英雄失败:', championName, reason)
          addActivity('error', `自动选择英雄失败：${championName} ${reason}`, 'auto')
        }
      },
      banChampion: {
        success: (championName: string) => {
          console.log('[🤖 ActivityStore] 记录自动禁用英雄成功:', championName)
          addActivity('success', `自动禁用英雄：${championName}`, 'auto')
        },
        noConfig: () => {
          console.log('[🤖 ActivityStore] 记录自动禁用英雄未配置')
          addActivity('warning', '自动禁用英雄：未设置禁用英雄', 'auto')
        }
      },
      configRunes: {
        success: (runeName: string) => {
          console.log('[🤖 ActivityStore] 记录自动配置符文成功:', runeName)
          addActivity('success', `自动配置符文：${runeName}`, 'auto')
        },
        failed: (reason = '符文页面已满') => {
          console.log('[🤖 ActivityStore] 记录自动配置符文失败:', reason)
          addActivity('error', `自动符文配置失败：${reason}`, 'auto')
        }
      }
    }

    // === 数据更新相关 ===
    const addDataActivity = {
      summonerUpdated: () => addActivity('info', '召唤师信息已更新', 'data'),
      rankUpdated: () => addActivity('info', '排位信息已刷新', 'data'),
      matchHistoryUpdated: (count?: number) => {
        const message = count ? `对局历史记录已更新，新增 ${count} 场对局` : '对局历史记录已更新'
        addActivity('success', message, 'data')
      },
      championDataUpdated: () => addActivity('info', '英雄数据已同步', 'data'),
      skinDataUpdated: () => addActivity('info', '皮肤数据已更新', 'data')
    }

    // === 用户设置操作 ===
    const addSettingsActivity = {
      setCareerBackground: (backgroundName: string) =>
        addActivity('success', `设置主页背景：${backgroundName}`, 'settings'),
      autoFunctionEnabled: (functionName: string) => addActivity('success', `已开启${functionName}`, 'settings'),
      autoFunctionDisabled: (functionName: string) => addActivity('info', `已关闭${functionName}`, 'settings'),
      championSet: (championName: string) => addActivity('success', `设置自动选择英雄：${championName}`, 'settings'),
      themeChanged: (themeName: string) => addActivity('info', `切换至${themeName}主题`, 'settings'),
      colorChanged: (colorName: string) => addActivity('info', `主题颜色已更改为：${colorName}`, 'settings'),
      settingsSaved: () => addActivity('info', '应用设置已保存', 'settings'),
      settingsReset: () => addActivity('info', '配置已重置为默认值', 'settings')
    }

    // === 错误和异常 ===
    const addErrorActivity = {
      apiError: (error: string) => addActivity('error', `API 请求失败：${error}`, 'error'),
      networkTimeout: () => addActivity('warning', '数据加载超时，请检查网络连接', 'error'),
      permissionDenied: () => addActivity('error', '自动功能执行异常：权限不足', 'error'),
      versionMismatch: () => addActivity('warning', '检测到游戏版本更新，部分功能可能受影响', 'error'),
      rateLimited: () => addActivity('warning', '操作过于频繁，请稍后再试', 'error'),
      diskSpaceError: () => addActivity('error', '保存设置失败：磁盘空间不足', 'error')
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
        console.log(`[🗑️ ActivityStore] 已清除 ${clearedCount} 条 ${ActivityCategoryLabels[category]} 记录`)
        addActivity('info', `已清除 ${clearedCount} 条 ${ActivityCategoryLabels[category]} 记录`, 'system')
      } else {
        console.log(`[🗑️ ActivityStore] 没有找到 ${category} 分类的记录`)
      }
    }

    // 清理过期记录（超过指定天数的记录）
    const cleanupOldActivities = (daysToKeep = 7) => {
      console.log(`[🧹 ActivityStore] 清理超过 ${daysToKeep} 天的过期记录`)
      const cutoffTime = Date.now() - daysToKeep * 24 * 60 * 60 * 1000
      const beforeCount = activities.value.length
      activities.value = activities.value.filter((activity) => activity.timestamp > cutoffTime)
      const clearedCount = beforeCount - activities.value.length

      if (clearedCount > 0) {
        console.log(`[🧹 ActivityStore] 已清理 ${clearedCount} 条过期记录`)
        addActivity('info', `清理了 ${clearedCount} 条超过 ${daysToKeep} 天的记录`, 'system')
      } else {
        console.log('[🧹 ActivityStore] 没有找到过期记录')
      }
    }

    // 获取活动统计信息
    const getActivityStats = () => {
      const stats = {
        total: activities.value.length,
        byType: {
          success: successCount.value,
          info: activities.value.filter((a) => a.type === 'info').length,
          warning: warningCount.value,
          error: errorCount.value
        },
        byCategory: {} as Record<string, number>
      }

      // 按分类统计
      activities.value.forEach((activity) => {
        stats.byCategory[activity.category] = (stats.byCategory[activity.category] || 0) + 1
      })

      return stats
    }

    // 标记所有活动为已读
    const markAllAsRead = () => {
      console.log('[👁️ ActivityStore] 标记所有活动为已读')
      const unreadCount = activities.value.filter((a) => !a.read).length
      activities.value.forEach((activity) => {
        ;(activity as any).read = true
      })
      console.log(`[👁️ ActivityStore] 已标记 ${unreadCount} 条活动为已读`)
    }

    // 标记指定活动为已读
    const markAsRead = (activityId: string) => {
      console.log('[👁️ ActivityStore] 标记指定活动为已读:', activityId)
      const activity = activities.value.find((a) => a.id === activityId)
      if (activity) {
        ;(activity as any).read = true
        console.log('[👁️ ActivityStore] 活动已标记为已读')
      } else {
        console.log('[👁️ ActivityStore] 未找到指定活动')
      }
    }

    // 计算属性
    const recentActivities = computed(() => activities.value.slice(0, 10))
    const errorCount = computed(() => activities.value.filter((a) => a.type === 'error').length)
    const warningCount = computed(() => activities.value.filter((a) => a.type === 'warning').length)
    const successCount = computed(() => activities.value.filter((a) => a.type === 'success').length)

    // 按分类统计
    const connectionActivities = computed(() => getActivitiesByCategory('connection'))
    const gameActivities = computed(() => getActivitiesByCategory('game'))
    const autoActivities = computed(() => getActivitiesByCategory('auto'))
    const dataActivities = computed(() => getActivitiesByCategory('data'))
    const settingsActivities = computed(() => getActivitiesByCategory('settings'))
    const errorActivities = computed(() => getActivitiesByCategory('error'))

    console.log('[✅ ActivityStore] 活动记录存储初始化完成，已注册所有方法')
    console.log('[📊 ActivityStore] 初始活动记录数量:', activities.value.length)

    return {
      // 状态
      activities: readonly(activities),

      // 计算属性
      recentActivities,
      errorCount,
      warningCount,
      successCount,
      connectionActivities,
      gameActivities,
      autoActivities,
      dataActivities,
      settingsActivities,
      errorActivities,

      // 基础方法
      addActivity,
      clearActivities,
      clearActivitiesByType,
      clearActivitiesByCategory,
      getActivitiesByType,
      getActivitiesByCategory,
      cleanupOldActivities,
      getActivityStats,
      markAllAsRead,
      markAsRead,

      // 便捷方法
      addConnectionActivity,
      addGameActivity,
      addAutoFunctionActivity,
      addDataActivity,
      addSettingsActivity,
      addErrorActivity
    }
  },
  {
    persist: true
  }
)
