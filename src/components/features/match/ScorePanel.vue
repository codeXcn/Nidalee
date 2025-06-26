<template>
  <div class="score-panel">
    <div class="score-title">对局状态</div>
    <div class="score-row">
      <span class="score blue">{{ myScore ?? '-' }}</span>
      <span class="vs">VS</span>
      <span class="score red">{{ theirScore ?? '-' }}</span>
    </div>
    <div class="score-labels">
      <span>我方分数</span>
      <span>敌方分数</span>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ myScore?: number; theirScore?: number }>()
</script>

<style scoped>
.score-panel {
  text-align: center;
  margin-bottom: 24px;
}
.score-title {
  font-size: 18px;
  font-weight: bold;
  margin-bottom: 8px;
}
.score-row {
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 32px;
  font-weight: bold;
  margin-bottom: 4px;
}
.score.blue {
  color: #1976d2;
}
.score.red {
  color: #e53935;
}
.vs {
  margin: 0 16px;
  color: #888;
}
.score-labels {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: #888;
  max-width: 220px;
  margin: 0 auto;
}
</style>
