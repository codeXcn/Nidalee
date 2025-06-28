<template>
  <div class="px-4">
    <!-- 右侧工具栏 -->
    <div class="flex items-center space-x-2">
      <!-- 通知弹出框 -->
      <NotificationHoverCard
        :activities="activities"
        title="系统活动"
        side="bottom"
        align="end"
        @mark-all-read="handleMarkAllRead"
        @view-all="handleViewAllActivities"
      />

      <!-- 刷新按钮 -->
      <button
        class="cursor-pointer p-3 rounded-xl bg-gradient-to-br from-background/80 to-muted/60 backdrop-blur-sm border border-border/50 hover:border-border transition-all duration-200 focus:outline-none shadow-lg hover:shadow-xl group"
        @click="refreshData"
        title="刷新数据"
      >
        <RefreshCw :size="16" class="text-muted-foreground group-hover:text-foreground transition-colors" />
      </button>

      <!-- 设置按钮 -->
      <!-- <button
        class="p-3 rounded-xl bg-gradient-to-br from-background/80 to-muted/60 backdrop-blur-sm border border-border/50 hover:border-border transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:ring-offset-2 shadow-lg hover:shadow-xl group"
        @click="openSettings"
        title="设置"
      >
        <Settings :size="18" class="text-muted-foreground group-hover:text-foreground transition-colors" />
      </button> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { useActivityStore } from '@/stores'
import { RefreshCw, Settings } from 'lucide-vue-next'
const { activities } = storeToRefs(useActivityStore())

// 新增的通知相关方法
const handleMarkAllRead = () => {
  console.log('标记所有活动为已读')
  // 这里可以添加标记所有活动为已读的逻辑
  activities.value.forEach((activity) => {
    activity.read = true
  })
}

const handleViewAllActivities = () => {
  console.log('查看所有活动')
  // 这里可以添加导航到活动页面的逻辑
}

const refreshData = async () => {
  console.log('刷新数据')
  try {
    if (isConnected.value) {
      await fetchSummonerInfo()
      await fetchMatchHistory()
      activityStore.addActivity('info', '数据刷新完成')
    }
  } catch (error) {
    console.error('刷新数据失败:', error)
    activityStore.addActivity('error', '数据刷新失败')
  }
}

const openSettings = () => {
  console.log('打开设置')
  // 这里可以添加导航到设置页面的逻辑
}
</script>

<style></style>
