<template>
  <div
    class="sticky top-0 z-10 flex items-center justify-between gap-3 px-4 py-3 border-b border-border/50 bg-background/80 backdrop-blur"
  >
    <!-- 左侧：队伍信息 -->
    <div class="flex items-center gap-3 min-w-0">
      <!-- 队伍标识 -->
      <div class="flex items-center gap-2 px-3 py-1.5 rounded-full text-sm font-medium" :class="teamTypeClass">
        <div class="w-2 h-2 rounded-full" :class="teamIndicatorClass" />
        <span>{{ teamTypeName }}</span>
      </div>

      <!-- 分隔线 -->
      <div class="h-4 w-px bg-border/60" />

      <!-- 阶段信息 -->
      <div class="flex items-center gap-2">
        <span class="text-sm text-muted-foreground">阶段</span>
        <div class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium" :class="phaseClass">
          <div class="w-1.5 h-1.5 rounded-full" :class="phaseIndicatorClass" />
          <span>{{ phaseDisplayName }}</span>
        </div>
      </div>

      <!-- 分隔线 -->
      <div class="h-4 w-px bg-border/60" />

      <!-- 队伍人数 -->
      <div class="flex items-center gap-2">
        <span class="text-sm text-muted-foreground">队伍</span>
        <span class="text-sm font-mono font-medium text-foreground">{{ teamCount }}/5</span>
      </div>

      <!-- 数据状态 -->
      <div class="flex items-center gap-1 text-xs" :class="dataStatusClass">
        <div class="w-1.5 h-1.5 rounded-full" :class="dataIndicatorClass" />
        <span>{{ dataStatusText }}</span>
      </div>
    </div>

    <!-- 右侧：操作按钮 -->
    <div class="flex items-center gap-2">
      <!-- 加载状态 -->
      <div v-if="loading" class="flex items-center gap-2 text-xs text-muted-foreground">
        <div class="w-3 h-3 border border-primary/30 border-t-primary rounded-full animate-spin" />
        <span>加载中</span>
      </div>

      <!-- 刷新按钮 -->
      <button
        v-else
        type="button"
        class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
        @click="$emit('refresh')"
      >
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
          />
        </svg>
        <span>刷新</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { GamePhase } from '@/types/match-analysis'

interface Props {
  teamType: 'ally' | 'enemy'
  phase: GamePhase
  teamCount: number
  hasData: boolean
  loading: boolean
}

const props = defineProps<Props>()

defineEmits<{
  refresh: []
}>()

// 队伍类型相关
const teamTypeName = computed(() => {
  return props.teamType === 'ally' ? '我方队伍' : '敌方队伍'
})

const teamTypeClass = computed(() => {
  return props.teamType === 'ally'
    ? 'bg-blue-50 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 ring-1 ring-blue-200 dark:ring-blue-800'
    : 'bg-red-50 dark:bg-red-900/30 text-red-700 dark:text-red-300 ring-1 ring-red-200 dark:ring-red-800'
})

const teamIndicatorClass = computed(() => {
  return props.teamType === 'ally' ? 'bg-blue-500' : 'bg-red-500'
})

// 阶段相关
const phaseDisplayName = computed(() => {
  switch (props.phase) {
    case 'ChampSelect':
      return '选人阶段'
    case 'InProgress':
      return '游戏中'
    case 'Lobby':
      return '大厅'
    case 'Matchmaking':
      return '匹配中'
    case 'EndOfGame':
      return '游戏结束'
    default:
      return '未知'
  }
})

const phaseClass = computed(() => {
  switch (props.phase) {
    case 'ChampSelect':
      return 'bg-purple-50 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300 ring-1 ring-purple-200 dark:ring-purple-800'
    case 'InProgress':
      return 'bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-300 ring-1 ring-green-200 dark:ring-green-800'
    case 'Matchmaking':
      return 'bg-yellow-50 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300 ring-1 ring-yellow-200 dark:ring-yellow-800'
    default:
      return 'bg-gray-50 dark:bg-gray-900/30 text-gray-700 dark:text-gray-300 ring-1 ring-gray-200 dark:ring-gray-800'
  }
})

const phaseIndicatorClass = computed(() => {
  switch (props.phase) {
    case 'ChampSelect':
      return 'bg-purple-500'
    case 'InProgress':
      return 'bg-green-500'
    case 'Matchmaking':
      return 'bg-yellow-500 animate-pulse'
    default:
      return 'bg-gray-500'
  }
})

// 数据状态相关
const dataStatusText = computed(() => {
  if (props.loading) return '加载中'
  if (props.hasData) return '数据就绪'
  if (props.phase === 'ChampSelect' && props.teamType === 'enemy') return '选择中'
  return '无数据'
})

const dataStatusClass = computed(() => {
  if (props.loading) return 'text-yellow-600 dark:text-yellow-400'
  if (props.hasData) return 'text-green-600 dark:text-green-400'
  return 'text-gray-500 dark:text-gray-400'
})

const dataIndicatorClass = computed(() => {
  if (props.loading) return 'bg-yellow-500 animate-pulse'
  if (props.hasData) return 'bg-green-500'
  return 'bg-gray-500'
})
</script>
