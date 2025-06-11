<template>
  <div :class="['team-panel', color]">
    <h3>{{ title }}</h3>
    <PlayerCard v-for="p in players" :key="p.cellId" :player="p" :color="color" />
  </div>
</template>

<script setup lang="ts">
import { ChampSelectPlayer } from '@/components/analysis/types/champSelect'
import PlayerCard from '@/components/analysis/PlayerCard.vue'

defineProps<{
  title: string
  players: ChampSelectPlayer[] | undefined
  color: 'blue' | 'red'
}>()
</script>

<style scoped>
.team-panel {
  flex: 1;
  background: #f8f9fa;
  border-radius: 8px;
  padding: 12px;
}
.team-panel.blue {
  background: #eaf3ff;
}
.team-panel.red {
  background: #fff1f1;
}
</style>
