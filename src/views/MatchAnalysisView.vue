<template>
  <div class="duel-analysis">
    <ScorePanel :myScore="session?.myScore" :theirScore="session?.theirScore" />
    <div class="teams-row">
      <TeamPanel :title="'蓝方队伍'" :players="session?.myTeam" color="blue" />
      <TeamPanel :title="'红方队伍'" :players="session?.theirTeam" color="red" />
    </div>
    <SuggestionPanel :suggestions="session?.suggestions" :warnings="session?.warnings" />
  </div>
</template>

<script setup lang="ts">
import { useChampSelectSession } from '@/components/analysis/composables/useChampSelectSession'

const { session } = useChampSelectSession()
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
