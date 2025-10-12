<template>
  <div class="min-h-screen bg-background">
    <!-- 匹配中状态 -->
    <div v-if="currentPhase === 'Matchmaking'" class="w-full max-w-4xl mx-auto p-6">
      <MatchmakingPanel />
    </div>

    <!-- 对局分析主界面 -->
    <div v-else-if="shouldShowAnalysis" class="w-full max-w-full mx-auto">
      <div class="flex gap-1 h-screen max-h-screen overflow-hidden">
        <!-- 左侧：我方队伍 -->
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
              v-if="hasMyTeamData"
              :team-data="myTeamData!"
              :team-stats="myTeamStats"
              team-type="ally"
              @select-player="handlePlayerDetails"
            />
          </div>
        </div>

        <!-- 分割线 -->
        <div class="w-px bg-border/50"></div>

        <!-- 右侧：敌方队伍 -->
        <div class="flex-1 flex flex-col min-w-0">
          <AnalysisHeader
            team-type="enemy"
            :phase="currentPhase"
            :team-count="getEnemyTeamCount()"
            :has-data="hasEnemyTeamData"
            :loading="isEnemyTeamLoading"
          />

          <div class="flex-1 overflow-y-auto">
            <!-- 统一使用 TeamAnalysisCard 组件 -->
            <TeamAnalysisCard
              v-if="hasEnemyTeamData"
              :team-data="enemyTeamData!"
              :team-stats="enemyTeamStats"
              team-type="enemy"
              @select-player="handlePlayerDetails"
            />

            <!-- 如果没有敌方队伍数据，显示简单的英雄选择列表作为降级方案 -->
            <!-- <EnemyChampionPicks v-else-if="currentPhase === 'ChampSelect'" :champion-picks="enemyChampionPicks" /> -->
          </div>
        </div>
      </div>
    </div>

    <!-- 默认状态 -->
    <div v-else class="flex items-center justify-center h-screen bg-background">
      <GameStateIndicator :phase="currentPhase" />
    </div>

    <!-- 召唤师详情弹窗 -->
    <SummonerDetailsDialog
      v-if="selectedPlayer"
      :open="showPlayerDetails"
      :summoner="selectedPlayer"
      :summoner-result="currentRestult"
      :loading="summonerLoading"
      @close="closePlayerDetails"
    />

    <!-- 加载状态 -->
    <div v-if="isLoading" class="fixed inset-0 bg-background/80 backdrop-blur-sm flex items-center justify-center z-50">
      <div class="flex flex-col items-center gap-4">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
        <p class="text-sm text-muted-foreground">{{ loadingMessage }}</p>
      </div>
    </div>

    <!-- 错误状态 -->
    <div v-if="hasErrors" class="fixed bottom-4 right-4 z-50">
      <div class="bg-destructive text-destructive-foreground rounded-lg p-4 max-w-sm">
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
              clip-rule="evenodd"
            />
          </svg>
          <span class="font-medium">发生错误</span>
        </div>
        <p class="text-sm mt-1">{{ recentErrors[0]?.message }}</p>
        <button @click="clearErrors" class="text-xs underline mt-2">清除错误</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { storeToRefs } from 'pinia'
import { useMatchAnalysisStore } from '@/stores/features/matchAnalysisStore'
import { useSearchMatches } from '@/composables/game/useSearchMatches'
import MatchmakingPanel from '@/components/features/game/MatchmakingPanel.vue'

console.log('[MatchAnalysisViewV2] 🎬 组件正在创建...')

// 子组件
import AnalysisHeader from '@/components/features/analysis/AnalysisHeader.vue'
import TeamAnalysisCard from '@/components/features/analysis/TeamAnalysisCard.vue'
import EnemyChampionPicks from '@/components/features/analysis/EnemyChampionPicks.vue'
import GameStateIndicator from '@/components/features/analysis/GameStateIndicator.vue'
import SummonerDetailsDialog from '@/components/features/match/SummonerDetailsDialog.vue'

// 使用 Pinia Store
const matchAnalysisStore = useMatchAnalysisStore()
const {
  currentPhase,
  isLoading,
  isEnemyTeamLoading,
  myTeamData,
  myTeamStats,
  enemyTeamData,
  enemyTeamStats,
  enemyChampionPicks,
  shouldShowAnalysis,
  hasMyTeamData,
  hasEnemyTeamData
} = storeToRefs(matchAnalysisStore)

// 🎯 完全依赖后端缓存和实时推送

// 🎯 简化：只启动对局分析系统，移除复杂的缓存逻辑
import { useMatchAnalysisInit } from '@/composables/game/core/useMatchAnalysisInit'

useMatchAnalysisInit() // 启动分析系统

onMounted(() => {
  console.log('[MatchAnalysisViewV2] ✅ 组件已挂载')
})

onBeforeUnmount(() => {
  console.log('[MatchAnalysisViewV2] 🔴 组件即将卸载')
})

// 使用召唤师详情查询
const { fetchSummonerInfo, currentRestult, loading: summonerLoading } = useSearchMatches()

// 玩家详情状态
const selectedPlayer = ref<any>(null)
const selectedPlayerStats = ref<any>(null)
const showPlayerDetails = ref(false)

// 错误状态 (模拟 - 应该从 useMatchAnalysisManagerV3 中获取)
const hasErrors = ref(false)
const recentErrors = ref<{ message: string }[]>([])

// 计算属性
const getEnemyTeamCount = () => {
  if (currentPhase.value === 'ChampSelect') {
    return enemyChampionPicks.value.length
  } else if (hasEnemyTeamData.value) {
    return enemyTeamData.value!.players.length
  }
  return 0
}

// 加载消息
const loadingMessage = computed(() => {
  if (isLoading.value) return '正在获取对局数据...'
  return '加载中...'
})

// 事件处理
const handlePlayerDetails = async (player: any, stats: any) => {
  selectedPlayer.value = player
  selectedPlayerStats.value = stats
  showPlayerDetails.value = true

  // 如果有玩家显示名称，尝试获取详细信息
  if (player.displayName && player.displayName !== '未知召唤师') {
    await fetchSummonerInfo([player.displayName])
  }
}

const closePlayerDetails = () => {
  selectedPlayer.value = null
  selectedPlayerStats.value = null
  showPlayerDetails.value = false
  // 可以选择是否重置召唤师查询结果
  // currentRestult.value = null
}

const clearErrors = () => {
  hasErrors.value = false
  recentErrors.value = []
}
</script>

<style scoped>
.min-h-screen {
  min-height: 100vh;
}
</style>
