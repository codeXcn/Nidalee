<template>
  <div class="min-h-screen flex flex-col justify-center items-center bg-white">
    <!-- 加载状态 -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
        <p class="mt-4 text-muted-foreground">正在加载对局数据...</p>
      </div>
    </div>

    <!-- 主要内容 -->
    <div v-else-if="session" class="w-full max-w-5xl space-y-8">
      <!-- 比分面板 -->
      <ScorePanel :my-score="session.myScore" :their-score="session.theirScore" />

      <!-- 队伍对称分布 -->
      <div class="flex justify-center items-center gap-16 mt-8">
        <!-- 我方队伍 -->
        <div class="flex flex-col gap-6">
          <PlayerCard
            v-for="player in session.myTeam"
            :key="player.summonerId + '-' + player.cellId"
            :player="player"
            :is-local="player.cellId === session.localPlayerCellId"
            class="w-64"
          />
        </div>
        <!-- VS分割 -->
        <div class="flex flex-col items-center justify-center">
          <span class="text-4xl font-extrabold text-gray-300 tracking-widest mb-2">VS</span>
        </div>
        <!-- 敌方队伍 -->
        <div class="flex flex-col gap-6">
          <PlayerCard
            v-for="player in session.theirTeam"
            :key="player.summonerId + '-' + player.cellId"
            :player="player"
            class="w-64"
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
import { useChampSelectSession } from '@/components/analysis/composables/useChampSelectSession'
import { Info } from 'lucide-vue-next'

const { session, loading } = useChampSelectSession()
watchEffect(() => console.log(session.value))
</script>

<style scoped>
/* 你可以根据需要微调间距和宽度 */
</style>
