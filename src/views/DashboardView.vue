<template>
  <div v-if="isConnected" class="flex flex-col gap-4">
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

    <!-- 游戏统计 -->
    <GameStats
      :is-connected="isConnected"
      :match-history-loading="matchHistoryLoading"
      :match-statistics="matchStatistics"
      @fetch-match-history="handleFetchMatchHistory"
    />
  </div>
  <ClientDisconnected v-else />
</template>

<script setup lang="ts">
import { useActivityLogger } from '@/composables/utils/useActivityLogger'
import { useAutoFunctionStore } from '@/stores/autoFunctionStore'
import { useConnectionStore } from '@/stores/core/connectionStore'
import { useDataStore } from '@/stores/core/dataStore'
import { appContextKey } from '@/types'
const { fetchMatchHistory } = inject(appContextKey) as { fetchMatchHistory: () => void }
// 使用新的stores
const dataStore = useDataStore()
const connectionStore = useConnectionStore()
const activityLogger = useActivityLogger()
const autoFunctionStore = useAutoFunctionStore()

// 从stores中获取响应式状态
const { summonerInfo, matchHistory, matchStatistics, isDataLoading } = storeToRefs(dataStore)
const { isConnected } = storeToRefs(connectionStore)
const { enabledFunctionsCount } = storeToRefs(autoFunctionStore)

// 计算属性
const todayMatches = computed(() => {
  const today = new Date().toDateString()
  const todayMatches = matchHistory.value.filter((match) => new Date(match.gameCreation).toDateString() === today)
  const wins = todayMatches.filter((match) => match.win).length
  const losses = todayMatches.length - wins

  return {
    total: todayMatches.length,
    wins,
    losses
  }
})
// 包装带日志的手动刷新方法
const handleFetchMatchHistory = async () => {
  activityLogger.log.info('手动刷新对局历史', 'data')
  await fetchMatchHistory()
}
const winRate = computed(() => {
  if (matchHistory.value.length === 0) return 0
  const wins = matchHistory.value.filter((match) => match.win).length
  return Math.round((wins / matchHistory.value.length) * 100)
})

// 使用明确的加载状态，而不是依赖"未加载完成"
const matchHistoryLoading = computed(() => isDataLoading.value)

// 模拟会话持续时间（暂时保留，后续可以移到专门的session store）
const sessionDuration = ref('0分钟')

// 监听连接状态变化
watchEffect(() => {
  console.log('连接状态:', isConnected.value)
})
</script>
