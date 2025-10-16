<template>
  <div class="flex flex-col gap-4">
    <SummonerCard v-if="isConnected" :summoner-info="summonerInfo" is-dashboard />
    <StatisticsCards
      :is-connected="isConnected"
      :today-matches="todayMatches"
      :win-rate="winRate"
      :enabled-functions-count="enabledFunctionsCount"
    />
    <GameStats
      :is-connected="isConnected"
      :match-history-loading="matchHistoryLoading"
      :match-statistics="matchStatistics"
      :selected-queue-id="selectedQueueId"
      @fetch-match-history="handleFetchMatchHistory"
      @queue-change="handleQueueChange"
    />
  </div>
</template>

<script setup lang="ts">
import { appContextKey } from '@/types'

const { fetchMatchHistory } = inject(appContextKey) as { fetchMatchHistory: (queueId?: number | null) => void }
const dataStore = useDataStore()
const connectionStore = useConnectionStore()
const activityLogger = useActivityLogger()
const autoFunctionStore = useAutoFunctionStore()

const { summonerInfo, matchStatistics, isDataLoading } = storeToRefs(dataStore)
const { isConnected } = storeToRefs(connectionStore)
const { enabledFunctionsCount } = storeToRefs(autoFunctionStore)

// 当前选中的队列ID（null = 全部模式）
const selectedQueueId = ref<number | null>(null)

// ✅ 直接从后端获取已计算好的数据
const todayMatches = computed(() => ({
  total: matchStatistics.value?.todayGames || 0,
  wins: matchStatistics.value?.todayWins || 0,
  losses: (matchStatistics.value?.todayGames || 0) - (matchStatistics.value?.todayWins || 0)
}))

const winRate = computed(() => matchStatistics.value?.winRate || 0)

const handleFetchMatchHistory = () => {
  activityLogger.log.info('手动刷新对局历史', 'data')
  fetchMatchHistory(selectedQueueId.value)
}

const handleQueueChange = (queueId: number | null) => {
  selectedQueueId.value = queueId
  activityLogger.log.info(`切换队列类型: ${queueId || '全部'}`, 'data')
  fetchMatchHistory(queueId)
}

const matchHistoryLoading = computed(() => isDataLoading.value)
</script>
