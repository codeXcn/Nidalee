<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
    <!-- 游戏连接状态 -->
    <Card
      :class="[
        'relative p-6 transition-all duration-300',
        isConnected
          ? 'border-l-green-500 bg-green-50/50 dark:bg-green-950/20'
          : 'border-l-red-500 bg-red-50/50 dark:bg-red-950/20'
      ]"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-muted-foreground">游戏连接</p>
          <h2
            :class="[
              'text-2xl font-bold',
              isConnected ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'
            ]"
          >
            {{ isConnected ? '已连接' : '离线' }}
          </h2>
          <p class="text-xs text-muted-foreground mt-1">
            {{ isConnected ? '客户端已就绪' : '等待连接至League客户端' }}
          </p>
        </div>
        <div class="absolute top-4 right-4">
          <div :class="['h-2 w-2 rounded-full', isConnected ? 'bg-green-500' : 'bg-red-500 animate-pulse']"></div>
        </div>
      </div>
      <div class="mt-4">
        <Button v-if="!isConnected" size="sm" variant="outline" class="text-xs" @click="attemptConnection">
          <RefreshCw class="h-3 w-3 mr-1" />
          重新连接
        </Button>
        <div v-else class="text-sm text-green-600 dark:text-green-400">
          <Wifi class="h-3 w-3 inline mr-1" />
          连接正常
        </div>
      </div>
    </Card>

    <!-- 今日对局 -->
    <Card class="relative p-6">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-muted-foreground">今日对局</p>
          <h2 class="text-2xl font-bold">{{ todayMatches.total }}</h2>
          <p class="text-xs text-muted-foreground mt-1">胜率 {{ winRate }}%</p>
        </div>
        <div class="absolute top-4 right-4">
          <TrendingUp class="h-4 w-4 text-muted-foreground" />
        </div>
      </div>
      <div class="mt-4 flex items-center space-x-4 text-sm">
        <div class="flex items-center">
          <div class="h-2 w-2 rounded-full bg-green-500 mr-1"></div>
          <span class="text-green-600 dark:text-green-400">{{ todayMatches.wins }}胜</span>
        </div>
        <div class="flex items-center">
          <div class="h-2 w-2 rounded-full bg-red-500 mr-1"></div>
          <span class="text-red-600 dark:text-red-400">{{ todayMatches.losses }}负</span>
        </div>
      </div>
    </Card>

    <!-- 自动功能 -->
    <Card class="relative p-6">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-muted-foreground">自动功能</p>
          <h2 class="text-2xl font-bold">{{ enabledFunctionsCount }}</h2>
          <p class="text-xs text-muted-foreground mt-1">功能运行中</p>
        </div>
        <div class="absolute top-4 right-4">
          <Settings class="h-4 w-4 text-muted-foreground" />
        </div>
      </div>
      <div class="mt-4 text-sm">
        <div class="flex items-center space-x-1">
          <div :class="['h-2 w-2 rounded-full', enabledFunctionsCount > 0 ? 'bg-green-500' : 'bg-gray-400']"></div>
          <span :class="[enabledFunctionsCount > 0 ? 'text-green-600 dark:text-green-400' : 'text-muted-foreground']">
            {{ enabledFunctionsCount > 0 ? '自动化已启用' : '所有功能已停用' }}
          </span>
        </div>
      </div>
    </Card>

    <!-- 活跃时长 -->
    <Card class="relative p-6">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-muted-foreground">活跃时长</p>
          <h2 class="text-2xl font-bold">{{ sessionDuration }}</h2>
          <p class="text-xs text-muted-foreground mt-1">本次会话</p>
        </div>
        <div class="absolute top-4 right-4">
          <Clock class="h-4 w-4 text-muted-foreground" />
        </div>
      </div>
      <div class="mt-4 text-sm text-muted-foreground">
        <div class="flex items-center">
          <Play class="h-3 w-3 mr-1" />
          <span>{{ formatTime(new Date()) }}</span>
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Settings, Clock, TrendingUp, RefreshCw, Wifi, Play } from 'lucide-vue-next'
import { useFormatters } from '@/hooks/useFormatters'

const props = defineProps<{
  isConnected: boolean
  todayMatches: {
    total: number
    wins: number
    losses: number
  }
  winRate: number
  enabledFunctionsCount: number
  sessionDuration: string
}>()

const emit = defineEmits<{
  (e: 'attempt-connection'): void
}>()

const { formatTime } = useFormatters()

const attemptConnection = () => {
  emit('attempt-connection')
}
</script>
