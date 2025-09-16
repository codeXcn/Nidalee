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
      @fetch-match-history="handleFetchMatchHistory"
    />
  </div>
</template>

<script setup lang="ts">
import { useActivityLogger } from '@/composables/utils/useActivityLogger'
import { useAutoFunctionStore } from '@/stores/autoFunctionStore'
import { useConnectionStore } from '@/stores/core/connectionStore'
import { useDataStore } from '@/stores/core/dataStore'
import { appContextKey } from '@/types'
const { fetchMatchHistory } = inject(appContextKey) as { fetchMatchHistory: () => void }
const dataStore = useDataStore()
const connectionStore = useConnectionStore()
const activityLogger = useActivityLogger()
const autoFunctionStore = useAutoFunctionStore()

const { summonerInfo, matchHistory, matchStatistics, isDataLoading } = storeToRefs(dataStore)
const { isConnected } = storeToRefs(connectionStore)
const { enabledFunctionsCount } = storeToRefs(autoFunctionStore)

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
const handleFetchMatchHistory = () => {
  activityLogger.log.info('手动刷新对局历史', 'data')
  fetchMatchHistory()
}
const winRate = computed(() => {
  if (matchHistory.value.length === 0) return 0
  const wins = matchHistory.value.filter((match) => match.win).length
  return Math.round((wins / matchHistory.value.length) * 100)
})

// 使用明确的加载状态，而不是依赖"未加载完成"
const matchHistoryLoading = computed(() => isDataLoading.value)
</script>
