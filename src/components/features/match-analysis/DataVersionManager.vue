<template>
  <Card class="p-4">
    <h3 class="text-lg font-semibold mb-4 flex items-center gap-2 text-foreground">
      <Database class="h-5 w-5 text-indigo-500 dark:text-indigo-400" />
      数据版本管理
    </h3>

    <div class="space-y-4">
      <!-- 当前版本信息 -->
      <div class="bg-muted/50 rounded-lg p-3 space-y-2">
        <div class="flex items-center justify-between">
          <span class="text-sm font-medium text-foreground">当前数据版本</span>
          <Badge variant="outline" class="text-indigo-600 dark:text-indigo-400">
            {{ currentVersion }}
          </Badge>
        </div>

        <div class="flex items-center justify-between">
          <span class="text-sm text-muted-foreground">最后更新时间</span>
          <span class="text-sm text-muted-foreground">
            {{ lastUpdateTime ? formatDate(lastUpdateTime) : '未知' }}
          </span>
        </div>

        <div class="flex items-center justify-between">
          <span class="text-sm text-muted-foreground">数据状态</span>
          <div class="flex items-center gap-2">
            <div :class="['w-2 h-2 rounded-full', isLoaded ? 'bg-green-500' : 'bg-red-500']"></div>
            <span class="text-sm text-muted-foreground">
              {{ isLoaded ? '已加载' : '未加载' }}
            </span>
          </div>
        </div>
      </div>

      <!-- 数据源说明 -->
      <div class="bg-blue-50 dark:bg-blue-950/30 rounded-lg p-3 space-y-2">
        <h4 class="text-sm font-medium text-blue-600 dark:text-blue-400">数据来源</h4>
        <div class="space-y-1 text-xs text-blue-600 dark:text-blue-400">
          <div class="flex items-center gap-2">
            <Globe class="h-3 w-3" />
            <span>符文 & 装备: Riot Data Dragon API</span>
          </div>
          <div class="flex items-center gap-2">
            <HardDrive class="h-3 w-3" />
            <span>队列 & 皮肤: 本地静态文件</span>
          </div>
        </div>
        <p class="text-xs text-blue-600/80 dark:text-blue-400/80">自动同步最新游戏版本，确保数据准确性</p>
      </div>

      <!-- 更新控制 -->
      <div class="flex items-center justify-between">
        <div class="space-y-1">
          <p class="text-sm font-medium text-foreground">手动更新</p>
          <p class="text-xs text-muted-foreground">强制从服务器获取最新数据</p>
        </div>

        <Button @click="handleManualUpdate" :disabled="isUpdating" size="sm" variant="outline">
          <RefreshCw :class="['h-4 w-4 mr-2', isUpdating && 'animate-spin']" />
          {{ isUpdating ? '更新中...' : '立即更新' }}
        </Button>
      </div>

      <!-- 更新状态提示 -->
      <div
        v-if="shouldUpdateData"
        class="bg-amber-50 dark:bg-amber-950/30 rounded-lg p-3 border border-amber-200 dark:border-amber-800"
      >
        <div class="flex items-center gap-2 text-amber-600 dark:text-amber-400">
          <AlertTriangle class="h-4 w-4" />
          <span class="text-sm font-medium">建议更新数据</span>
        </div>
        <p class="text-xs text-amber-600/80 dark:text-amber-400/80 mt-1">
          数据已超过24小时未更新，建议刷新以获取最新信息
        </p>
      </div>

      <!-- 离线提示 -->
      <div
        v-if="!navigator.onLine"
        class="bg-red-50 dark:bg-red-950/30 rounded-lg p-3 border border-red-200 dark:border-red-800"
      >
        <div class="flex items-center gap-2 text-red-600 dark:text-red-400">
          <WifiOff class="h-4 w-4" />
          <span class="text-sm font-medium">离线模式</span>
        </div>
        <p class="text-xs text-red-600/80 dark:text-red-400/80 mt-1">当前处于离线状态，使用缓存数据</p>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Database, Globe, HardDrive, RefreshCw, AlertTriangle, WifiOff } from 'lucide-vue-next'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { useStaticData } from '@/composables/utils/useStaticData'

const { isLoaded, currentVersion, lastUpdateTime, shouldUpdateData, loadStaticData } = useStaticData()

const isUpdating = ref(false)

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  }).format(date)
}

const handleManualUpdate = async () => {
  isUpdating.value = true
  try {
    await loadStaticData(true) // 强制更新
    console.log('Manual update completed')
  } catch (error) {
    console.error('Manual update failed:', error)
  } finally {
    isUpdating.value = false
  }
}
</script>
