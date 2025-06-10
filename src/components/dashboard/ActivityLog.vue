<template>
  <Card class="p-6">
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold">最近活动</h3>
          <p class="text-sm text-muted-foreground">最新的活动记录</p>
        </div>
        <div class="flex items-center space-x-2">
          <div class="h-2 w-2 rounded-full bg-green-500 animate-pulse"></div>
          <span class="text-xs text-muted-foreground">实时监控中</span>
        </div>
      </div>

      <div
        class="space-y-3 max-h-80 scrollbar-thin scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar-thumb-slate-400/50 dark:scrollbar-thumb-slate-500/50 scrollbar-track-transparent overflow-y-auto"
      >
        <div
          v-for="activity in activities.slice(0, 8)"
          :key="activity.id"
          class="flex items-start space-x-3 p-2 rounded-lg hover:bg-muted/30 transition-colors"
        >
          <div
            :class="[
              'h-2 w-2 rounded-full mt-2 flex-shrink-0',
              activity.type === 'success'
                ? 'bg-green-500'
                : activity.type === 'info'
                  ? 'bg-blue-500'
                  : activity.type === 'warning'
                    ? 'bg-yellow-500'
                    : 'bg-red-500'
            ]"
          ></div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-foreground leading-tight">
              {{ activity.message }}
            </p>
            <p class="text-xs text-muted-foreground">
              {{ formatRelativeTime(activity.timestamp) }}
            </p>
          </div>
        </div>

        <div v-if="activities.length === 0" class="text-center py-8">
          <Clock class="h-8 w-8 text-muted-foreground mx-auto mb-2" />
          <p class="text-sm text-muted-foreground">暂无活动记录</p>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Card } from '@/components/ui/card'
import { Clock } from 'lucide-vue-next'
import { useFormatters } from '@/hooks/useFormatters'

interface Activity {
  id: string | number
  type: 'success' | 'info' | 'warning' | 'error'
  message: string
  timestamp: number
}

const props = defineProps<{
  activities: Activity[]
}>()

const { formatRelativeTime } = useFormatters()
</script>
