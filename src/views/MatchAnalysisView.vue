<template>
  <div class="container mx-auto p-4 space-y-4">
    <!-- 加载状态 -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
        <p class="mt-4 text-muted-foreground">正在加载对局数据...</p>
      </div>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="flex items-center justify-center h-64">
      <div class="text-center">
        <AlertCircle class="h-12 w-12 text-destructive mx-auto" />
        <p class="mt-4 text-destructive">{{ error }}</p>
        <Button @click="retry" class="mt-4">重试</Button>
      </div>
    </div>

    <!-- 主要内容 -->
    <div v-else-if="session" class="space-y-4">
      <!-- 比分面板 -->
      <ScorePanel :my-score="session.myScore" :their-score="session.theirScore" />

      <!-- 队伍对称分布 -->
      <div class="flex justify-center items-start gap-8 mt-8">
        <!-- 我方队伍 -->
        <div class="flex flex-col gap-4">
          <PlayerCard
            v-for="player in session.myTeam"
            :key="player.summonerId + '-' + player.cellId"
            :player="player"
            :is-local="player.cellId === session.localPlayerCellId"
            class="w-60 min-w-[240px]"
          />
        </div>
        <!-- VS分割 -->
        <div class="flex flex-col items-center justify-center">
          <span class="text-2xl font-bold text-gray-400">VS</span>
        </div>
        <!-- 敌方队伍 -->
        <div class="flex flex-col gap-4">
          <PlayerCard
            v-for="player in session.theirTeam"
            :key="player.summonerId + '-' + player.cellId"
            :player="player"
            class="w-60 min-w-[240px]"
          />
        </div>
      </div>

      <!-- 建议面板 -->
      <!-- <SuggestionPanel
        :team-comparison="session.analysis?.teamComparison"
        :recommendations="session.analysis?.recommendations"
        :strategies="session.analysis?.strategies"
      /> -->
    </div>

    <!-- 无数据状态 -->
    <div v-else class="flex items-center justify-center h-64">
      <div class="text-center">
        <Info class="h-12 w-12 text-muted-foreground mx-auto" />
        <p class="mt-4 text-muted-foreground">等待进入选人阶段...</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AlertCircle, Info } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import ScorePanel from '@/components/analysis/ScorePanel.vue'
import TeamPanel from '@/components/analysis/TeamPanel.vue'
import SuggestionPanel from '@/components/analysis/SuggestionPanel.vue'
import { useChampSelectSession } from '@/components/analysis/composables/useChampSelectSession'
import { computed } from 'vue'
import PlayerCard from '@/components/analysis/PlayerCard.vue'

const { session, loading, error, retry } = useChampSelectSession()

// 判断 theirTeam 是否有真实玩家（summonerId 不是 '0'）
const hasRealEnemy = computed(() => {
  if (!session.value) return false
  return session.value.theirTeam.some((p) => p.summonerId && p.summonerId !== '0')
})
</script>

<style scoped>
.duel-analysis {
  padding: 20px;
}
.teams-row {
  display: flex;
  gap: 16px;
  margin-top: 20px;
}
</style>
