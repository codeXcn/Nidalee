<template>
  <div class="player-card">
    <span class="avatar">{{ player.displayName?.[0] || '?' }}</span>
    <div class="info">
      <div>{{ player.displayName || 'Player' }}</div>
      <div class="desc">
        {{ player.assignedPosition || '-' }} | {{ player.tier || '-' }}
        <span v-if="player.winRate">胜率: {{ player.winRate }}%</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ChampSelectPlayer } from '@/components/analysis/types/champSelect'
defineProps<{ player: ChampSelectPlayer; color: 'blue' | 'red' }>()
</script>

<style scoped>
.player-card {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
  background: #fff;
  border-radius: 6px;
  padding: 8px;
}
.avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #bcd;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 10px;
  font-weight: bold;
}
.info .desc {
  font-size: 12px;
  color: #888;
}
</style>
