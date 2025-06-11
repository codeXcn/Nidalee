<template>
  <div class="suggestion-panel">
    <div class="suggestion-section good">
      <div class="section-title"><span class="icon">💡</span> 战术建议</div>
      <ul>
        <li v-if="!suggestions || suggestions.length === 0" class="empty">暂无建议</li>
        <li v-for="(item, idx) in suggestions" :key="idx" class="good-item">
          <span class="good-dot">🟢</span> {{ item }}
        </li>
      </ul>
    </div>
    <div class="suggestion-section warning">
      <div class="section-title"><span class="icon">⚠️</span> 注意事项</div>
      <ul>
        <li v-if="!warnings || warnings.length === 0" class="empty">暂无注意事项</li>
        <li v-for="(item, idx) in warnings" :key="idx" class="warn-item">
          <span class="warn-dot">🔴</span> {{ item }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ suggestions?: string[]; warnings?: string[] }>()
</script>

<style scoped>
.suggestion-panel {
  margin-top: 32px;
  display: flex;
  gap: 32px;
}
.suggestion-section {
  flex: 1;
  background: #f7f9fa;
  border-radius: 8px;
  padding: 16px 20px;
}
.section-title {
  font-weight: bold;
  margin-bottom: 8px;
  font-size: 15px;
  display: flex;
  align-items: center;
  gap: 4px;
}
.good-item,
.warn-item {
  margin-bottom: 4px;
}
.good-dot {
  color: #43a047;
  margin-right: 4px;
}
.warn-dot {
  color: #e53935;
  margin-right: 4px;
}
.empty {
  color: #aaa;
  font-size: 13px;
}
</style>
