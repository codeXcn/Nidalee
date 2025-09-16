<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
    <Card
      :class="[
        'relative p-6 rounded-xl shadow-sm border-l-4 transition-all duration-300',
        isConnected
          ? 'border-l-green-400 bg-green-50/60 dark:bg-green-950/10'
          : 'border-l-red-400 bg-red-50/60 dark:bg-red-950/10'
      ]"
    >
      <div>
        <p class="text-sm font-medium text-muted-foreground flex items-center mb-1">
          <Wifi class="h-4 w-4 mr-1 text-green-400" />游戏连接
        </p>
        <h2
          class="text-2xl font-extrabold tracking-tight leading-tight"
          :class="isConnected ? 'text-green-500 dark:text-green-400' : 'text-red-500 dark:text-red-400'"
        >
          {{ isConnected ? '已连接' : '离线' }}
        </h2>
        <p class="text-xs text-muted-foreground mt-1">
          {{ isConnected ? '客户端已就绪' : '等待连接至League客户端' }}
        </p>
      </div>
      <div class="mt-4 flex items-center text-xs text-muted-foreground">
        <Wifi class="h-3 w-3 mr-1" />
        <span>{{ isConnected ? '连接正常' : '未连接' }}</span>
      </div>
    </Card>

    <!-- 今日对局 -->
    <Card class="relative p-6 rounded-xl shadow-sm border-l-4 border-l-blue-400 bg-blue-50/60 dark:bg-blue-950/10">
      <div>
        <p class="text-sm font-medium text-muted-foreground flex items-center mb-1">
          <TrendingUp class="h-4 w-4 mr-1 text-blue-400" />今日对局
        </p>
        <h2 class="text-2xl font-extrabold text-blue-500 dark:text-blue-400 tracking-tight leading-tight">
          {{ todayMatches.total }}
        </h2>
        <p class="text-xs text-muted-foreground mt-1">胜率 {{ winRate }}%</p>
      </div>
      <div class="mt-4 flex items-center space-x-4 text-xs text-muted-foreground">
        <div class="flex items-center">
          <div class="h-2 w-2 rounded-full bg-green-500 mr-1"></div>
          <span class="text-green-500 dark:text-green-400">{{ todayMatches.wins }}胜</span>
        </div>
        <div class="flex items-center">
          <div class="h-2 w-2 rounded-full bg-red-500 mr-1"></div>
          <span class="text-red-500 dark:text-red-400">{{ todayMatches.losses }}负</span>
        </div>
      </div>
    </Card>

    <!-- 自动功能 -->
    <Card
      class="relative p-6 rounded-xl shadow-sm border-l-4 border-l-purple-400 bg-purple-50/60 dark:bg-purple-950/10"
    >
      <div>
        <p class="text-sm font-medium text-muted-foreground flex items-center mb-1">
          <Settings class="h-4 w-4 mr-1 text-purple-400" />自动功能
        </p>
        <h2 class="text-2xl font-extrabold text-purple-500 dark:text-purple-400 tracking-tight leading-tight">
          {{ enabledFunctionsCount }}
        </h2>
        <p class="text-xs text-muted-foreground mt-1">功能运行中</p>
      </div>
      <div class="mt-4 text-xs text-muted-foreground flex items-center space-x-1">
        <div :class="['h-2 w-2 rounded-full', enabledFunctionsCount > 0 ? 'bg-green-400' : 'bg-gray-300']"></div>
        <span :class="[enabledFunctionsCount > 0 ? 'text-green-500 dark:text-green-400' : 'text-muted-foreground']">
          {{ enabledFunctionsCount > 0 ? '自动化已启用' : '所有功能已停用' }}
        </span>
      </div>
    </Card>

    <!-- 活跃时长 -->
    <Card class="relative p-6 rounded-xl shadow-sm border-l-4 border-l-pink-400 bg-pink-50/60 dark:bg-pink-950/10">
      <div>
        <p class="text-sm font-medium text-muted-foreground flex items-center mb-1">
          <Clock class="h-4 w-4 mr-1 text-pink-400" />活跃时长
        </p>
        <h2 class="text-2xl font-extrabold text-pink-500 dark:text-pink-400 tracking-tight leading-tight">
          {{ sessionStore.formattedTotal }}
        </h2>
        <p class="text-xs text-muted-foreground mt-1">累计</p>
      </div>
      <div class="mt-4 text-xs text-muted-foreground flex items-center">
        <Play class="h-3 w-3 mr-1" />
        <span>{{ formatTime(new Date()) }}</span>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { Clock, Play, Settings, TrendingUp, Wifi } from 'lucide-vue-next'
import { useSessionStore } from '@/stores/features/sessionStore'

defineProps<{
  isConnected: boolean
  todayMatches: {
    total: number
    wins: number
    losses: number
  }
  winRate: number
  enabledFunctionsCount: number
}>()

defineEmits<{
  (e: 'attempt-connection'): void
}>()

const { formatTime } = useFormatters()
const sessionStore = useSessionStore()
</script>
