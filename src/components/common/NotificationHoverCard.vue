<template>
  <Popover>
    <PopoverTrigger as-child>
      <button
        class="cursor-pointer relative p-3 rounded-2xl bg-gradient-to-br from-background/80 to-muted/60 backdrop-blur-sm border border-border/50 hover:border-border transition-all duration-200 focus:outline-none shadow-lg hover:shadow-xl group"
        aria-label="查看活动通知"
      >
        <Bell :size="16" class="text-muted-foreground group-hover:text-foreground transition-colors" />
        <!-- 通知数量徽章 -->
        <span
          v-if="unreadCount > 0"
          class="absolute -top-1 -right-2 h-6 w-6 bg-gradient-to-r from-red-500 to-red-600 text-white text-xs font-bold rounded-full flex items-center justify-center animate-bounce shadow-lg border-2 border-background"
        >
          {{ unreadCount > 9 ? '9+' : unreadCount }}
        </span>
        <!-- 脉冲动画环 -->
        <div
          v-if="unreadCount > 0"
          class="absolute -top-1 -right-1 h-3 w-3 bg-red-500 rounded-full animate-ping opacity-20"
        ></div>
      </button>
    </PopoverTrigger>

    <PopoverContent
      :side="side"
      :align="align"
      class="!border-none w-[320px] p-0 shadow-2xl bg-background/95 backdrop-blur-lg rounded-2xl"
    >
      <!-- 头部 -->
      <div class="px-4 py-3 border-b border-border/30 bg-gradient-to-r from-primary/5 to-primary/10 rounded-t-2xl">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <div class="p-1.5 rounded-lg bg-primary/10">
              <Bell :size="15" class="text-primary" />
            </div>
            <h3 class="text-base font-semibold text-foreground">{{ title }}</h3>
          </div>
          <Badge v-if="unreadCount > 0" variant="secondary" class="text-xs bg-red-500/10 text-red-600 border-red-200">
            {{ unreadCount }} 条未读
          </Badge>
        </div>
      </div>

      <!-- 活动内容 -->
      <div class="max-h-[260px] overflow-hidden">
        <div class="px-3 py-2">
          <div class="space-y-2">
            <div
              v-for="activity in activities.slice(0, 6)"
              :key="activity.id"
              class="flex items-start space-x-2 p-2 rounded-lg border border-border/30 bg-card/50 hover:bg-card transition-all duration-200 group"
            >
              <div
                :class="[
                  'h-2 w-2 rounded-full mt-2 flex-shrink-0 transition-all duration-200',
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
                <div class="flex items-center justify-between">
                  <p class="text-sm font-medium text-foreground leading-tight line-clamp-1">
                    {{ activity.message }}
                  </p>
                  <div class="flex items-center space-x-1 ml-2">
                    <span class="text-xs text-muted-foreground">
                      {{ formatRelativeTime(activity.timestamp) }}
                    </span>
                    <div
                      v-if="!activity.read"
                      class="h-2 w-2 bg-blue-500 rounded-full flex-shrink-0 animate-pulse"
                    ></div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="activities.length === 0" class="text-center py-8">
              <div class="p-3 rounded-full bg-muted/50 w-12 h-12 mx-auto mb-2 flex items-center justify-center">
                <Bell class="h-5 w-5 text-muted-foreground" />
              </div>
              <p class="text-sm text-muted-foreground">暂无活动记录</p>
              <p class="text-xs text-muted-foreground mt-1">所有活动将在这里显示</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 底部操作栏 -->
      <div class="px-4 py-2 border-t border-border/30 bg-muted/20 rounded-b-2xl">
        <div class="flex items-center justify-between">
          <button
            v-if="unreadCount > 0"
            class="inline-flex items-center px-2 py-1.5 text-xs text-muted-foreground hover:text-foreground transition-colors rounded-md hover:bg-muted/40"
            @click="handleMarkAllRead"
          >
            <CheckCheck :size="13" class="mr-1" />
            全部标记为已读
          </button>
          <div v-else></div>

          <button
            class="inline-flex items-center px-2 py-1.5 text-xs text-primary hover:text-primary/80 font-medium rounded-md hover:bg-primary/10 transition-all duration-200"
            @click="handleViewAll"
          >
            查看全部活动
            <ArrowRight :size="13" class="ml-1" />
          </button>
        </div>
      </div>
    </PopoverContent>
  </Popover>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Bell, CheckCheck, ArrowRight } from 'lucide-vue-next'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { Badge } from '@/components/ui/badge'

export interface Activity {
  id: string | number
  type: 'success' | 'info' | 'warning' | 'error'
  message: string
  timestamp: number
  read?: boolean
}

const props = withDefaults(
  defineProps<{
    activities: Activity[]
    title?: string
    side?: 'top' | 'right' | 'bottom' | 'left'
    align?: 'start' | 'center' | 'end'
  }>(),
  {
    title: '最近活动',
    side: 'bottom',
    align: 'end'
  }
)

const emit = defineEmits<{
  markAllRead: []
  viewAll: []
}>()

const { formatRelativeTime } = useFormatters()

// 计算未读数量
const unreadCount = computed(() => {
  return props.activities.filter((activity) => !activity.read).length
})

// 处理标记全部已读
const handleMarkAllRead = () => {
  emit('markAllRead')
}

// 处理查看全部
const handleViewAll = () => {
  emit('viewAll')
}
</script>

<style scoped>
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 自定义滚动条样式 */
.custom-scrollbar {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.3) transparent;
}

.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(156, 163, 175, 0.3);
  border-radius: 2px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(156, 163, 175, 0.5);
}
</style>
