<template>
  <div class="w-full h-full relative">
    <!-- 使用 Transition 实现平滑切换 -->
    <Transition name="fade" mode="out-in">
      <!-- Main Analysis View -->
      <div v-if="shouldShowAnalysis && hasMyTeamData && isDataReady" key="analysis" class="w-full max-w-full mx-auto">
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
      <GameStatusHub v-else key="status" />
    </Transition>

    <!-- Summoner Details Dialog -->
    <SummonerDetailsDialog
      v-if="selectedPlayer"
      :open="showPlayerDetails"
      :summoner="selectedPlayer"
      :summoner-result="currentRestult"
      :loading="summonerLoading"
      @close="closePlayerDetails"
    />
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMatchAnalysisStore } from './store'

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

// 🎨 平滑切换逻辑：添加短暂延迟，确保数据准备完毕再显示
const isDataReady = ref(false)

watch(
  () => shouldShowAnalysis.value && hasMyTeamData.value,
  (shouldShow) => {
    if (shouldShow) {
      // 数据加载完成后，延迟 150ms 再显示，避免闪烁
      isDataReady.value = false
      setTimeout(() => {
        isDataReady.value = true
      }, 150)
    } else {
      isDataReady.value = false
    }
  },
  { immediate: true }
)

onMounted(async () => {
  if (currentPhase.value === 'ChampSelect' || currentPhase.value === 'InProgress') {
    console.log('[MatchAnalysisView] 🔄 尝试从缓存恢复对局分析数据...')

    try {
      const cachedData = await invoke<TeamAnalysisData | null>('get_cached_analysis_data')

      if (cachedData) {
        console.log('[MatchAnalysisViewV2] ✅ 成功恢复缓存数据')
        matchAnalysisStore.setTeamAnalysisData(cachedData)
        // 注意：不要手动设置 isDataReady，让 watch 自动处理
        // watch 会在数据更新后自动触发，并添加 150ms 延迟
      } else {
        console.log('[MatchAnalysisViewV2] ⚠️ 缓存中没有数据，等待 WebSocket 事件')
      }
    } catch (error) {
      console.error('[MatchAnalysisViewV2] ❌ 恢复缓存数据失败:', error)
      // 失败不影响正常流程，等待 WebSocket 事件即可
    }
  }
})

onBeforeUnmount(() => {
  console.log('[MatchAnalysisViewV2] 🔴 组件即将卸载，清理数据')
  // matchAnalysisStore.clearAllData()
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

/* 淡入淡出过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.fade-enter-to,
.fade-leave-from {
  opacity: 1;
}
</style>
