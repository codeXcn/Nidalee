<template>
  <div class="bg-card rounded-xl border p-6">
    <div class="flex items-center justify-between mb-4">
      <div>
        <h3 class="text-lg font-semibold">活动记录管理</h3>
        <p class="text-sm text-muted-foreground">查看和管理应用活动记录</p>
      </div>
      <Badge variant="outline">{{ activities.length }} 条记录</Badge>
    </div>

    <!-- 快速统计 -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
      <div class="text-center p-3 bg-green-50 rounded-lg">
        <div class="text-2xl font-bold text-green-600">{{ successCount }}</div>
        <div class="text-xs text-green-600">成功</div>
      </div>
      <div class="text-center p-3 bg-blue-50 rounded-lg">
        <div class="text-2xl font-bold text-blue-600">{{ infoCount }}</div>
        <div class="text-xs text-blue-600">信息</div>
      </div>
      <div class="text-center p-3 bg-yellow-50 rounded-lg">
        <div class="text-2xl font-bold text-yellow-600">{{ warningCount }}</div>
        <div class="text-xs text-yellow-600">警告</div>
      </div>
      <div class="text-center p-3 bg-red-50 rounded-lg">
        <div class="text-2xl font-bold text-red-600">{{ errorCount }}</div>
        <div class="text-xs text-red-600">错误</div>
      </div>
    </div>

    <!-- 分类统计 -->
    <div class="space-y-3 mb-6">
      <h4 class="text-sm font-medium text-muted-foreground">按分类统计</h4>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
        <div
          v-for="(count, category) in categoryStats"
          :key="category"
          class="flex items-center justify-between p-2 bg-muted/30 rounded-lg"
        >
          <span class="text-xs">{{ ActivityCategoryLabels[category as ActivityCategory] }}</span>
          <Badge variant="secondary" class="text-xs">{{ count }}</Badge>
        </div>
      </div>
    </div>

    <!-- 最近活动 -->
    <div class="space-y-3 mb-6">
      <div class="flex items-center justify-between">
        <h4 class="text-sm font-medium text-muted-foreground">最近活动</h4>
        <Button variant="ghost" size="sm" @click="showAllActivities = !showAllActivities">
          {{ showAllActivities ? '收起' : '查看全部' }}
          <ChevronDown :class="['ml-1 h-3 w-3 transition-transform', showAllActivities && 'rotate-180']" />
        </Button>
      </div>

      <div class="space-y-2 max-h-60 overflow-y-auto">
        <div
          v-for="activity in displayedActivities"
          :key="activity.id"
          class="flex items-start space-x-3 p-3 bg-muted/20 rounded-lg"
        >
          <div
            :class="[
              'flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold',
              getTypeStyle(activity.type)
            ]"
          >
            {{ getTypeIcon(activity.type) }}
          </div>
          <div class="flex-1 min-w-0">
            <div class="flex items-center space-x-2 mb-1">
              <Badge variant="outline" class="text-xs">
                {{ ActivityCategoryLabels[activity.category] }}
              </Badge>
              <span class="text-xs text-muted-foreground">
                {{ formatRelativeTime(activity.timestamp) }}
              </span>
            </div>
            <p class="text-sm truncate">{{ activity.message }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex items-center justify-between pt-4 border-t">
      <div class="flex items-center space-x-2">
        <Button variant="outline" size="sm" @click="clearByType('error')"> 清除错误 </Button>
        <Button variant="outline" size="sm" @click="clearByType('warning')"> 清除警告 </Button>
      </div>

      <div class="flex items-center space-x-2">
        <Button variant="outline" size="sm" @click="exportActivities">
          <Download class="h-4 w-4 mr-2" />
          导出
        </Button>
        <Button variant="destructive" size="sm" @click="showClearDialog = true">
          <Trash2 class="h-4 w-4 mr-2" />
          清空全部
        </Button>
      </div>
    </div>

    <!-- 清空确认对话框 -->
    <AlertDialog v-model:open="showClearDialog">
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>确认清空</AlertDialogTitle>
          <AlertDialogDescription> 此操作将删除所有活动记录，且无法恢复。确定要继续吗？ </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>取消</AlertDialogCancel>
          <AlertDialogAction @click="clearAllActivities"> 确认清空 </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { ChevronDown, Download, Trash2 } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle
} from '@/components/ui/alert-dialog'
import type { Activity, ActivityType, ActivityCategory } from '@/types'
import { ActivityCategoryLabels, ActivityTypeStyles } from '@/types'
import { useFormatters } from '@/composables'
import { useActivityStore } from '@/stores/activityStore'

// 使用 activity store
const activityStore = useActivityStore()
const { formatRelativeTime } = useFormatters()

// 响应式状态
const showAllActivities = ref(false)
const showClearDialog = ref(false)

// 计算属性
const activities = computed(() => activityStore.activities)

const successCount = computed(() => activityStore.successCount)
const warningCount = computed(() => activityStore.warningCount)
const errorCount = computed(() => activityStore.errorCount)
const infoCount = computed(() => activities.value.filter((a) => a.type === 'info').length)

// 按分类统计
const categoryStats = computed(() => {
  const stats: Record<string, number> = {}
  activities.value.forEach((activity) => {
    stats[activity.category] = (stats[activity.category] || 0) + 1
  })
  return stats
})

// 显示的活动列表
const displayedActivities = computed(() => {
  const recentActivities = activities.value.slice(0, 5)
  return showAllActivities.value ? activities.value : recentActivities
})

// 获取类型样式
const getTypeStyle = (type: ActivityType) => {
  const style = ActivityTypeStyles[type]
  return `${style.color} ${style.bgColor}`
}

// 获取类型图标
const getTypeIcon = (type: ActivityType) => {
  return ActivityTypeStyles[type].icon
}

// 按类型清除
const clearByType = (type: ActivityType) => {
  activityStore.clearActivitiesByType(type)
}

// 清空所有活动
const clearAllActivities = () => {
  activityStore.clearActivities()
  showClearDialog.value = false
}

// 导出活动记录
const exportActivities = () => {
  const data = activities.value.map((activity) => ({
    时间: new Date(activity.timestamp).toLocaleString('zh-CN'),
    类型: activity.type,
    分类: ActivityCategoryLabels[activity.category],
    消息: activity.message
  }))

  const jsonStr = JSON.stringify(data, null, 2)
  const blob = new Blob([jsonStr], { type: 'application/json' })
  const url = URL.createObjectURL(blob)

  const a = document.createElement('a')
  a.href = url
  a.download = `活动记录_${new Date().toISOString().split('T')[0]}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)

  // 记录导出操作
  activityStore.addActivity('info', '活动记录已导出', 'system')
}
</script>
