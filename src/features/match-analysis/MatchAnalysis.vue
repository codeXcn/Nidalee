<template>
  <!-- Main Analysis View -->
  <div v-if="shouldShowAnalysis && hasMyTeamData" class="w-full max-w-full mx-auto">
    <div class="flex gap-1 h-screen max-h-screen overflow-hidden">
      <!-- Ally Team -->
      <div class="flex-1 flex flex-col min-w-0">
        <AnalysisHeader
          team-type="ally"
          :phase="currentPhase"
          :team-count="myTeamData?.players.length || 0"
          :has-data="hasMyTeamData"
          :loading="isLoading"
        />
        <div class="flex-1 overflow-y-auto">
          <TeamAnalysisCard
            :team-data="myTeamData!"
            :team-stats="myTeamStats"
            team-type="ally"
            @select-player="handlePlayerDetails"
          />
        </div>
      </div>

      <div class="w-px bg-border/50"></div>

      <!-- Enemy Team -->
      <div class="flex-1 flex flex-col min-w-0">
        <AnalysisHeader
          team-type="enemy"
          :phase="currentPhase"
          :team-count="enemyTeamData?.players.length || 0"
          :has-data="hasEnemyTeamData"
          :loading="isEnemyTeamLoading"
        />
        <div class="flex-1 overflow-y-auto">
          <TeamAnalysisCard
            :team-data="enemyTeamData!"
            :team-stats="enemyTeamStats"
            team-type="enemy"
            @select-player="handlePlayerDetails"
          />
        </div>
      </div>
    </div>
  </div>

  <!-- Pre-Analysis Status Hub -->
  <GameStatusHub v-else />

  <!-- Summoner Details Dialog -->
  <SummonerDetailsDialog
    v-if="selectedPlayer"
    :open="showPlayerDetails"
    :summoner="selectedPlayer"
    :summoner-result="currentRestult"
    :loading="summonerLoading"
    @close="closePlayerDetails"
  />
</template>

<script setup lang="ts">
import { useMatchAnalysisStore } from './store'

console.log('[MatchAnalysisViewV2] 🎬 组件正在创建...')

// Use Pinia Store
const matchAnalysisStore = useMatchAnalysisStore()
const {
  currentPhase,
  isLoading,
  isEnemyTeamLoading,
  myTeamData,
  myTeamStats,
  enemyTeamData,
  enemyTeamStats,
  shouldShowAnalysis,
  hasMyTeamData,
  hasEnemyTeamData
} = storeToRefs(matchAnalysisStore)

// 注释：敌方英雄选择现在由 team-analysis-data 事件自动更新
// 不再需要手动监听 gameStore.champSelectSession

onMounted(() => {
  console.log('[MatchAnalysisViewV2] ✅ 组件已挂载')
})

onBeforeUnmount(() => {
  console.log('[MatchAnalysisViewV2] 🔴 组件即将卸载，清理数据')
  matchAnalysisStore.clearAllData()
})

// Summoner details logic
const { fetchSummonerInfo, currentRestult, loading: summonerLoading } = useSearchMatches()
const selectedPlayer = ref<any>(null)
const showPlayerDetails = ref(false)

const handlePlayerDetails = async (player: any) => {
  selectedPlayer.value = player
  showPlayerDetails.value = true
  if (player.displayName && player.displayName !== '未知召唤师') {
    await fetchSummonerInfo([player.displayName])
  }
}

const closePlayerDetails = () => {
  selectedPlayer.value = null
  showPlayerDetails.value = false
}
</script>

<style scoped>
.min-h-screen {
  min-height: 100vh;
}
</style>
