<template>
  <div class="w-full max-w-2xl mx-auto">
    <!-- 主状态卡片 -->
    <Card
      class="p-6 rounded-2xl shadow-lg bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800 border-slate-200 dark:border-slate-700"
    >
      <div class="space-y-6">
        <!-- 标题区域 -->
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-3">
            <div class="relative group">
              <Search
                class="h-6 w-6 text-primary animate-pulse group-hover:scale-110 transition-transform duration-200"
              />
              <div
                class="absolute -top-1 -right-1 w-3 h-3 rounded-full animate-ping group-hover:animate-pulse"
                :class="{
                  'bg-green-500': matchmakingState?.searchState === 'Searching',
                  'bg-yellow-500': matchmakingState?.searchState === 'Found',
                  'bg-red-500': !['Searching', 'Found'].includes(matchmakingState?.searchState ?? '')
                }"
              ></div>
            </div>
            <div>
              <h3 class="text-xl font-bold text-foreground">匹配状态</h3>
              <p class="text-sm text-muted-foreground">正在寻找合适的对手</p>
            </div>
          </div>
          <Badge
            :variant="matchmakingState?.searchState === 'Searching' ? 'default' : 'secondary'"
            class="text-sm font-medium"
          >
            {{ getStatusText() }}
          </Badge>
        </div>

        <!-- 匹配中状态 -->
        <div v-if="matchmakingState?.searchState === 'Searching'" class="space-y-6">
          <!-- 时间信息网格 -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <!-- 预估等待时间 -->
            <Card
              v-if="matchmakingState?.estimatedQueueTime"
              class="p-4 bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800 hover:shadow-md hover:scale-105 transition-all duration-200 cursor-default"
            >
              <div class="text-center">
                <div class="flex items-center justify-center mb-2">
                  <Clock class="h-4 w-4 text-blue-600 dark:text-blue-400 mr-2" />
                  <span class="text-sm font-medium text-blue-700 dark:text-blue-300">预估等待时间</span>
                </div>
                <div class="text-2xl font-bold text-blue-600 dark:text-blue-400 mb-1">
                  {{ formatTime(matchmakingState.estimatedQueueTime) }}
                </div>
                <div class="text-xs text-blue-500 dark:text-blue-400">基于当前队列情况估算</div>
              </div>
            </Card>

            <!-- 实际等待时间 -->
            <Card
              class="p-4 bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800 hover:shadow-md hover:scale-105 transition-all duration-200 cursor-default"
            >
              <div class="text-center">
                <div class="flex items-center justify-center mb-2">
                  <Timer class="h-4 w-4 text-green-600 dark:text-green-400 mr-2" />
                  <span class="text-sm font-medium text-green-700 dark:text-green-300">已等待时间</span>
                </div>
                <div class="text-2xl font-bold text-green-600 dark:text-green-400 mb-1">
                  {{ formatTime(actualWaitTime) }}
                </div>
                <div class="text-xs text-green-500 dark:text-green-400">从开始匹配时计算</div>
              </div>
            </Card>
          </div>

          <!-- 进度条 -->
          <div class="space-y-2">
            <div class="flex justify-between text-sm text-muted-foreground">
              <span>匹配进度</span>
              <span>{{ Math.round(waitProgress) }}%</span>
            </div>
            <div class="w-full bg-muted rounded-full h-2 overflow-hidden">
              <div
                class="h-full bg-gradient-to-r from-primary to-primary/80 rounded-full transition-all duration-1000 ease-out"
                :style="{ width: `${waitProgress}%` }"
              ></div>
            </div>
          </div>

          <!-- 匹配提示 -->
          <div class="text-center p-4 bg-muted/50 rounded-lg">
            <div class="flex items-center justify-center space-x-2 text-muted-foreground">
              <Loader2 class="h-4 w-4 animate-spin" />
              <span class="text-sm">正在为您寻找实力相当的对手...</span>
            </div>
          </div>
        </div>

        <!-- 匹配成功状态 -->
        <div v-else-if="matchmakingState?.searchState === 'Found'" class="text-center space-y-4">
          <div class="p-6 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-xl">
            <CheckCircle class="h-12 w-12 text-green-500 mx-auto mb-3" />
            <h4 class="text-lg font-semibold text-green-700 dark:text-green-300 mb-2">匹配成功！</h4>
            <p class="text-sm text-green-600 dark:text-green-400">已为您找到合适的对手，请确认是否接受对局</p>
          </div>
        </div>

        <!-- 未匹配状态 -->
        <div v-else class="text-center space-y-4">
          <div class="p-6 bg-muted/50 rounded-xl">
            <Search class="h-12 w-12 text-muted-foreground mx-auto mb-3" />
            <h4 class="text-lg font-semibold text-foreground mb-2">准备匹配</h4>
            <p class="text-sm text-muted-foreground">点击下方按钮开始寻找对手</p>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex flex-col items-center space-y-3">
          <Button
            class="w-full max-w-xs h-12 text-lg font-semibold transition-all duration-200"
            :class="{
              'hover:bg-destructive/90 hover:scale-105 hover:shadow-lg': matchmakingState?.searchState === 'Searching',
              'hover:bg-primary/90 hover:scale-105 hover:shadow-lg': matchmakingState?.searchState !== 'Searching'
            }"
            :variant="matchmakingState?.searchState === 'Searching' ? 'destructive' : 'default'"
            @click="handleMatchmaking"
          >
            <Search v-if="matchmakingState?.searchState !== 'Searching'" class="h-4 w-4 mr-2" />
            <X v-else class="h-4 w-4 mr-2" />
            {{ getButtonText() }}
          </Button>

          <div v-if="matchmakingState?.searchState === 'Found'" class="flex items-center space-x-3 w-full max-w-xs">
            <Button
              variant="default"
              class="flex-1 transition-all duration-200 hover:bg-primary/90 hover:scale-105 hover:shadow-lg"
              @click="handleAcceptMatch"
            >
              <Check class="h-4 w-4 mr-2" />
              接受对局
            </Button>
            <Button
              variant="destructive"
              class="flex-1 transition-all duration-200 hover:bg-destructive/90 hover:scale-105 hover:shadow-lg"
              @click="handleDeclineMatch"
            >
              <X class="h-4 w-4 mr-2" />
              拒绝对局
            </Button>
          </div>
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { Search, Clock, Timer, Loader2, CheckCircle, Check, X } from 'lucide-vue-next'
import { useMatchmaking } from '@/composables'

const { matchmakingState, handleMatchmaking, handleAcceptMatch, handleDeclineMatch } = useMatchmaking()
console.log('[MatchmakingPanel] matchmakingState', matchmakingState.value)

// 匹配开始时间
const matchmakingStartTime = ref<Date | null>(null)

// 监听匹配状态变化
watch(
  () => matchmakingState.value?.searchState,
  (newState, oldState) => {
    if (newState === 'Searching' && oldState !== 'Searching') {
      // 开始匹配
      matchmakingStartTime.value = new Date()
    } else if (newState !== 'Searching' && oldState === 'Searching') {
      // 停止匹配
      matchmakingStartTime.value = null
    }
  }
)

// 实际等待时间（秒）
const actualWaitTime = computed(() => {
  if (!matchmakingStartTime.value || matchmakingState.value?.searchState !== 'Searching') {
    return 0
  }
  return Math.floor((Date.now() - matchmakingStartTime.value.getTime()) / 1000)
})

// 等待进度百分比
const waitProgress = computed(() => {
  if (!matchmakingState.value?.estimatedQueueTime || actualWaitTime.value === 0) {
    return 0
  }
  const progress = (actualWaitTime.value / matchmakingState.value.estimatedQueueTime) * 100
  return Math.min(progress, 100)
})

// 获取状态文本
const getStatusText = () => {
  switch (matchmakingState.value?.searchState) {
    case 'Searching':
      return '匹配中'
    case 'Found':
      return '已匹配'
    default:
      return '未匹配'
  }
}

// 获取按钮文本
const getButtonText = () => {
  switch (matchmakingState.value?.searchState) {
    case 'Searching':
      return '取消匹配'
    case 'Found':
      return '等待确认'
    default:
      return '开始匹配'
  }
}

// 格式化时间显示
const formatTime = (timeValue: number) => {
  // 确保是正数
  const seconds = Math.max(0, Math.floor(timeValue))

  if (seconds < 60) {
    return `${seconds}秒`
  } else if (seconds < 3600) {
    const minutes = Math.floor(seconds / 60)
    const remainingSeconds = seconds % 60
    if (remainingSeconds === 0) {
      return `${minutes}分钟`
    }
    return `${minutes}分${remainingSeconds}秒`
  } else {
    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    if (minutes === 0) {
      return `${hours}小时`
    }
    return `${hours}小时${minutes}分钟`
  }
}
</script>
