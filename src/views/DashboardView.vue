<template>
  <div class="space-y-6">
    <!-- 用户信息卡片 -->
    <SummonerCard v-if="summonerInfo" :summoner-info="summonerInfo" :session-duration="sessionDuration" />

    <!-- 顶部统计卡片 -->
    <StatisticsCards
      :is-connected="isConnected"
      :today-matches="todayMatches"
      :win-rate="winRate"
      :enabled-functions-count="enabledFunctionsCount"
      :session-duration="sessionDuration"
      @attempt-connection="attemptConnection"
    />

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 快捷功能 -->
      <QuickActions
        :auto-functions="autoFunctions"
        @toggle-auto-function="toggleAutoFunction"
        @debug="debugLoginInfo"
        @simulate="simulateMatch"
      />

      <!-- 最近活动 -->
      <ActivityLog :activities="activities" />
    </div>

    <!-- 调试信息 -->
    <Card v-if="showDebugInfo && debugInfo" class="p-6">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold">API调试信息</h3>
            <p class="text-sm text-muted-foreground">LCU API响应数据</p>
          </div>
          <Button variant="outline" size="sm" class="text-xs" @click="showDebugInfo = false"> 关闭 </Button>
        </div>

        <div class="space-y-4">
          <div v-for="(value, key) in debugInfo" :key="key" class="space-y-2">
            <h4 class="font-medium text-sm">{{ key }}</h4>
            <pre class="bg-muted p-4 rounded-lg text-xs overflow-x-auto">{{ value }}</pre>
          </div>
        </div>
      </div>
    </Card>

    <!-- 游戏统计 -->
    <GameStats
      :is-connected="isConnected"
      :match-history-loading="matchHistoryLoading"
      :match-statistics="matchStatistics"
      @fetch-match-history="fetchMatchHistory"
    />
  </div>
</template>

<script setup lang="ts">
import { useGameStore } from '@/stores'
import { invoke } from '@tauri-apps/api/core'

// 使用store和监控
const gameStore = useGameStore()
const { attemptConnection } = useGameMonitor()

// 从store中解构响应式状态
const {
  isConnected,
  summonerInfo,
  todayMatches,
  activities,
  autoFunctions,
  winRate,
  enabledFunctionsCount,
  sessionDuration,
  matchStatistics,
  matchHistoryLoading
} = storeToRefs(gameStore)

// 调试状态
const debugInfo = ref<Record<string, unknown> | null>(null)
const showDebugInfo = ref(false)

// 组件挂载时获取游戏版本和对局历史
onMounted(() => {
  // 如果已连接，自动获取对局历史
  if (isConnected.value) {
    fetchMatchHistory()
  }
})

// 监听连接状态变化，自动获取对局历史
watch(isConnected, (newValue) => {
  if (newValue && !matchStatistics.value) {
    fetchMatchHistory()
  }
})

// 方法
const { toggleAutoFunction, simulateMatchResult, fetchMatchHistory } = gameStore

// 调试登录信息
const debugLoginInfo = async (): Promise<void> => {
  try {
    gameStore.addActivity('info', '开始调试API信息...')
    const result = await invoke('debug_login_info')
    debugInfo.value = result as any
    showDebugInfo.value = true
    console.log('调试信息:', result)
    gameStore.addActivity('success', '调试信息获取成功，请查看控制台')
  } catch (error) {
    console.error('调试失败:', error)
    gameStore.addActivity('error', `调试失败: ${error}`)
  }
}

// 模拟对局（用于测试）
const simulateMatch = (): void => {
  const won = Math.random() > 0.5
  simulateMatchResult(won)
}
</script>
