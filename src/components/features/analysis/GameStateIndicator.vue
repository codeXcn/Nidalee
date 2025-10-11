<template>
  <div class="text-center space-y-6 max-w-md mx-auto px-6">
    <!-- 状态图标 -->
    <div class="relative">
      <div class="w-20 h-20 mx-auto rounded-full flex items-center justify-center" :class="statusIconClass">
        <div class="w-3 h-3 rounded-full" :class="statusIndicatorClass" :style="{ animation: statusAnimation }"></div>
      </div>
      <!-- 状态图标背景动画 -->
      <div
        v-if="phase === 'InProgress'"
        class="absolute inset-0 w-20 h-20 mx-auto rounded-full border-2 border-primary/20 animate-ping"
      ></div>
    </div>

    <!-- 状态标题和描述 -->
    <div class="space-y-3">
      <h2 class="text-2xl font-bold" :class="statusTitleClass">{{ getStatusTitle }}</h2>
      <p class="text-muted-foreground leading-relaxed">{{ getStatusDescription }}</p>
    </div>

    <!-- 状态徽章 -->
    <div v-if="phase" class="flex items-center justify-center">
      <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full border" :class="statusBadgeClass">
        <div class="w-2 h-2 rounded-full" :class="statusIndicatorClass" :style="{ animation: statusAnimation }"></div>
        <span class="text-sm font-medium">{{ getStatusBadgeText }}</span>
      </div>
    </div>

    <!-- 额外提示信息 -->
    <div v-if="getAdditionalInfo" class="mt-4 p-4 rounded-lg border bg-card/50">
      <p class="text-sm text-muted-foreground">{{ getAdditionalInfo }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  phase: string | null
}>()

// 状态样式计算属性
const statusIndicatorClass = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
      return 'bg-blue-500 dark:bg-blue-400'
    case 'Matchmaking':
      return 'bg-blue-500 dark:bg-blue-400'
    case 'ChampSelect':
      return 'bg-blue-500 dark:bg-blue-400'
    case 'InProgress':
      return 'bg-green-500 dark:bg-green-400'
    case 'EndOfGame':
      return 'bg-gray-500 dark:bg-gray-400'
    default:
      return 'bg-gray-500 dark:bg-gray-400'
  }
})

const statusIconClass = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
      return 'bg-blue-50 dark:bg-blue-950/50 border-2 border-blue-200 dark:border-blue-800'
    case 'Matchmaking':
      return 'bg-blue-50 dark:bg-blue-950/50 border-2 border-blue-200 dark:border-blue-800'
    case 'ChampSelect':
      return 'bg-blue-50 dark:bg-blue-950/50 border-2 border-blue-200 dark:border-blue-800'
    case 'InProgress':
      return 'bg-green-50 dark:bg-green-950/50 border-2 border-green-200 dark:border-green-800'
    case 'EndOfGame':
      return 'bg-gray-50 dark:bg-gray-950/50 border-2 border-gray-200 dark:border-gray-800'
    default:
      return 'bg-gray-50 dark:bg-gray-950/50 border-2 border-gray-200 dark:border-gray-800'
  }
})

const statusTitleClass = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
      return 'text-blue-600 dark:text-blue-400'
    case 'Matchmaking':
      return 'text-blue-600 dark:text-blue-400'
    case 'ChampSelect':
      return 'text-blue-600 dark:text-blue-400'
    case 'InProgress':
      return 'text-green-600 dark:text-green-400'
    case 'EndOfGame':
      return 'text-gray-600 dark:text-gray-400'
    default:
      return 'text-gray-600 dark:text-gray-400'
  }
})

const statusBadgeClass = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
      return 'bg-blue-50 dark:bg-blue-950/50 border-blue-200 dark:border-blue-800 text-blue-700 dark:text-blue-300'
    case 'Matchmaking':
      return 'bg-blue-50 dark:bg-blue-950/50 border-blue-200 dark:border-blue-800 text-blue-700 dark:text-blue-300'
    case 'ChampSelect':
      return 'bg-blue-50 dark:bg-blue-950/50 border-blue-200 dark:border-blue-800 text-blue-700 dark:text-blue-300'
    case 'InProgress':
      return 'bg-green-50 dark:bg-green-950/50 border-green-200 dark:border-green-800 text-green-700 dark:text-green-300'
    case 'EndOfGame':
      return 'bg-gray-50 dark:bg-gray-950/50 border-gray-200 dark:border-gray-800 text-gray-700 dark:text-gray-300'
    default:
      return 'bg-gray-50 dark:bg-gray-950/50 border-gray-200 dark:border-gray-800 text-gray-700 dark:text-gray-300'
  }
})

const statusAnimation = computed(() => {
  const phase = props.phase
  if (phase === 'InProgress') {
    return 'pulse 2s infinite'
  } else if (phase === 'Matchmaking') {
    return 'pulse 1.5s infinite'
  }
  return 'pulse 3s infinite'
})

// 状态标题和描述
const getStatusTitle = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '已进入大厅'
    case 'Matchmaking':
      return '正在匹配中'
    case 'ChampSelect':
      return '正在选择英雄'
    case 'InProgress':
      return '游戏进行中'
    case 'EndOfGame':
      return '对局结束'
    default:
      return '等待客户端连接'
  }
})

const getStatusDescription = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '请选择游戏模式并开始匹配，进入选人后将自动显示队伍信息'
    case 'Matchmaking':
      return '正在为您寻找合适的对手，请耐心等待'
    case 'ChampSelect':
      return '请在客户端中完成英雄与符文配置，本页将展示队伍与战绩信息'
    case 'InProgress':
      return '游戏正在进行中，正在获取实时数据'
    case 'EndOfGame':
      return '比赛结果已出，返回大厅后将恢复大厅视图'
    default:
      return '请启动并登录英雄联盟客户端；进入大厅后将自动切换到选人/对局分析'
  }
})

const getStatusBadgeText = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '房间中'
    case 'Matchmaking':
      return '匹配中'
    case 'ChampSelect':
      return '英雄选择'
    case 'InProgress':
      return '游戏进行中'
    case 'EndOfGame':
      return '游戏结束'
    default:
      return '未连接'
  }
})

const getAdditionalInfo = computed(() => {
  const phase = props.phase
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '💡 提示：选择你喜欢的游戏模式，然后点击"寻找对局"开始匹配'
    case 'Matchmaking':
      return '⏳ 正在为您寻找合适的对手，请耐心等待'
    case 'ChampSelect':
      return '🎯 建议：根据队伍阵容选择合适的英雄与符文配置'
    case 'InProgress':
      return '🎮 游戏正在进行中，实时数据将在连接后显示'
    case 'EndOfGame':
      return '🏆 感谢你的游戏，期待下次对局'
    default:
      return '🎮 请确保英雄联盟客户端正在运行（若已运行但未连接，可稍等片刻或在右上角尝试刷新）'
  }
})
</script>
