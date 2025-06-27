<template>
  <div class="space-y-6">
    <!-- 用户信息卡片 -->
    <SummonerCard v-if="isConnected" :summoner-info="summonerInfo" :session-duration="sessionDuration" />

    <!-- 顶部统计卡片 -->
    <StatisticsCards
      :is-connected="isConnected"
      :today-matches="todayMatches"
      :win-rate="winRate"
      :enabled-functions-count="enabledFunctionsCount"
      :session-duration="sessionDuration"
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
import { useAutoFunctionManager } from '@/composables/game/useAutoFunctionManager'
import {
  useSummonerStore,
  useMatchStatisticsStore,
  useConnectStore,
  useActivityStore,
  useAutoFunctionStore,
  useAppSessionStore
} from '@/stores'
import { invoke } from '@tauri-apps/api/core'

// 直接使用各个 store
const summonerStore = useSummonerStore()
const matchStatisticsStore = useMatchStatisticsStore()
const connectionStore = useConnectStore()
const activityStore = useActivityStore()
const autoFunctionStore = useAutoFunctionStore()
const appSessionStore = useAppSessionStore()
watchEffect(() => {
  // 监听连接状态变化
  console.log('连接状态:', connectionStore.isConnected)
})
// 使用自动功能管理器
const autoFunctionManager = useAutoFunctionManager()

// 从各个 store 中解构响应式状态
const { summonerInfo } = storeToRefs(summonerStore)
const { todayMatches, matchStatistics, matchHistoryLoading, winRate } = storeToRefs(matchStatisticsStore)
const { isConnected } = storeToRefs(connectionStore)
const { activities } = storeToRefs(activityStore)
const { autoFunctions, enabledFunctionsCount } = storeToRefs(autoFunctionStore)
const { sessionDuration } = storeToRefs(appSessionStore)

// 调试状态
const debugInfo = ref<Record<string, unknown> | null>(null)
const showDebugInfo = ref(false)



// 方法
const toggleAutoFunction = (key: keyof typeof autoFunctions.value) => {
  autoFunctionManager.handleAutoFunctionToggle(key)
}

const simulateMatchResult = (won: boolean) => {
  matchStatisticsStore.simulateMatchResult(won)
  activityStore.addActivity(won ? 'success' : 'info', won ? '对局胜利！' : '对局结束')
}

const fetchMatchHistory = inject('fetchMatchHistory', async () => {
  try {
    await matchStatisticsStore.fetchMatchHistory()
  } catch (error) {
    console.error('获取对局历史失败:', error)
  }
})
const fetchSummonerInfo = inject('fetchSummonerInfo', async () => {
  try {
    await summonerStore.fetchSummonerInfo()
  } catch (error) {
    console.error('获取召唤师信息失败:', error)
  }
})
</script>
