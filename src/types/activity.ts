// 活动记录相关类型定义

export type ActivityType = 'success' | 'info' | 'warning' | 'error'

export type ActivityCategory =
  | 'connection' // 连接状态
  | 'game' // 游戏流程
  | 'auto' // 自动功能
  | 'data' // 数据更新
  | 'settings' // 用户设置
  | 'error' // 错误异常
  | 'system' // 系统事件

export interface Activity {
  id: string
  type: ActivityType
  message: string
  timestamp: number
  category: ActivityCategory
  read?: boolean
}

// 活动分类的显示名称映射
export const ActivityCategoryLabels: Record<ActivityCategory, string> = {
  connection: '连接状态',
  game: '游戏流程',
  auto: '自动功能',
  data: '数据更新',
  settings: '用户设置',
  error: '错误异常',
  system: '系统事件'
}

// 活动类型的显示样式映射
export const ActivityTypeStyles: Record<ActivityType, { color: string; bgColor: string; icon: string }> = {
  success: {
    color: 'text-green-600',
    bgColor: 'bg-green-50',
    icon: '✓'
  },
  info: {
    color: 'text-blue-600',
    bgColor: 'bg-blue-50',
    icon: 'ℹ'
  },
  warning: {
    color: 'text-yellow-600',
    bgColor: 'bg-yellow-50',
    icon: '⚠'
  },
  error: {
    color: 'text-red-600',
    bgColor: 'bg-red-50',
    icon: '✕'
  }
}

// 活动分类的颜色主题
export const ActivityCategoryColors: Record<ActivityCategory, string> = {
  connection: 'border-blue-200 text-blue-700',
  game: 'border-green-200 text-green-700',
  auto: 'border-purple-200 text-purple-700',
  data: 'border-orange-200 text-orange-700',
  settings: 'border-gray-200 text-gray-700',
  error: 'border-red-200 text-red-700',
  system: 'border-indigo-200 text-indigo-700'
}
