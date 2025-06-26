<!--
  Store 重构测试组件
  用于验证新的模块化 store 是否正常工作
-->
<template>
  <div class="store-test p-4 space-y-4">
    <h2 class="text-xl font-bold">Store 重构测试</h2>

    <!-- 连接状态测试 -->
    <div class="card p-4 border rounded">
      <h3 class="font-semibold mb-2">连接状态</h3>
      <p>连接状态: {{ isConnected ? '已连接' : '未连接' }}</p>
      <p>认证信息: {{ authInfo ? '已获取' : '未获取' }}</p>
      <button @click="testConnection" class="btn btn-primary mt-2">测试连接</button>
    </div>

    <!-- 召唤师信息测试 -->
    <div class="card p-4 border rounded">
      <h3 class="font-semibold mb-2">召唤师信息</h3>
      <p>显示名称: {{ displayName || '未获取' }}</p>
      <p>召唤师等级: {{ summonerLevel || '未获取' }}</p>
      <p>加载中: {{ summonerLoading ? '是' : '否' }}</p>
      <button @click="testSummoner" class="btn btn-primary mt-2">获取召唤师信息</button>
    </div>

    <!-- 活动记录测试 -->
    <div class="card p-4 border rounded">
      <h3 class="font-semibold mb-2">活动记录</h3>
      <p>总记录数: {{ activities.length }}</p>
      <p>错误数: {{ errorCount }}</p>
      <p>警告数: {{ warningCount }}</p>
      <button @click="testActivity" class="btn btn-primary mt-2">添加测试活动</button>
      <div class="mt-2 max-h-32 overflow-y-auto">
        <div
          v-for="activity in recentActivities"
          :key="activity.id"
          :class="['text-sm p-1', getActivityClass(activity.type)]"
        >
          [{{ activity.type }}] {{ activity.message }}
        </div>
      </div>
    </div>

    <!-- 自动功能测试 -->
    <div class="card p-4 border rounded">
      <h3 class="font-semibold mb-2">自动功能</h3>
      <p>已启用功能数: {{ enabledFunctionsCount }}</p>
      <div class="space-y-1 mt-2">
        <label v-for="(enabled, key) in autoFunctions" :key="key" class="flex items-center space-x-2">
          <input type="checkbox" :checked="enabled" @change="toggleFunction(key)" />
          <span>{{ getFunctionDisplayName(key) }}</span>
        </label>
      </div>
    </div>

    <!-- 对局统计测试 -->
    <div class="card p-4 border rounded">
      <h3 class="font-semibold mb-2">对局统计</h3>
      <p>今日胜率: {{ winRate }}%</p>
      <p>今日对局: {{ todayMatches.total }}场</p>
      <p>胜利: {{ todayMatches.wins }}场</p>
      <p>失败: {{ todayMatches.losses }}场</p>
      <div class="mt-2 space-x-2">
        <button @click="simulateWin" class="btn btn-success">模拟胜利</button>
        <button @click="simulateLoss" class="btn btn-error">模拟失败</button>
      </div>
    </div>

    <!-- 应用会话测试 -->
    <div class="card p-4 border rounded">
      <h3 class="font-semibold mb-2">应用会话</h3>
      <p>会话时长: {{ sessionDuration }}</p>
      <p>游戏版本: {{ gameVersion }}</p>
      <button @click="updateDuration" class="btn btn-primary mt-2">更新时长</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import {
  useConnectionStore,
  useSummonerStore,
  useActivityStore,
  useAutoFunctionStore,
  useMatchStatisticsStore,
  useAppSessionStore
} from '@/stores'

// 各个 store 实例
const connectionStore = useConnectionStore()
const summonerStore = useSummonerStore()
const activityStore = useActivityStore()
const autoFunctionStore = useAutoFunctionStore()
const matchStatisticsStore = useMatchStatisticsStore()
const appSessionStore = useAppSessionStore()

// 响应式状态
const { isConnected, authInfo } = storeToRefs(connectionStore)
const { displayName, summonerLevel, loading: summonerLoading } = storeToRefs(summonerStore)
const { activities, recentActivities, errorCount, warningCount } = storeToRefs(activityStore)
const { autoFunctions, enabledFunctionsCount } = storeToRefs(autoFunctionStore)
const { todayMatches, winRate } = storeToRefs(matchStatisticsStore)
const { sessionDuration, gameVersion } = storeToRefs(appSessionStore)

// 测试方法
const testConnection = async () => {
  activityStore.addActivity('info', '开始测试连接...')
  await connectionStore.checkConnection()
}

const testSummoner = async () => {
  activityStore.addActivity('info', '开始获取召唤师信息...')
  try {
    await summonerStore.fetchSummonerInfo()
    activityStore.addActivity('success', '召唤师信息获取成功')
  } catch (error) {
    activityStore.addActivity('error', '召唤师信息获取失败')
  }
}

const testActivity = () => {
  const messages = ['这是一条测试信息', '这是一条成功消息', '这是一条警告消息', '这是一条错误消息']
  const types: Array<Activity['type']> = ['info', 'success', 'warning', 'error']

  const randomIndex = Math.floor(Math.random() * messages.length)
  activityStore.addActivity(types[randomIndex], messages[randomIndex])
}

const toggleFunction = (key: string) => {
  const result = autoFunctionStore.toggleAutoFunction(key as keyof typeof autoFunctions.value)
  activityStore.addActivity('info', `${result.name} ${result.enabled ? '已启用' : '已禁用'}`)
}

const simulateWin = () => {
  matchStatisticsStore.simulateMatchResult(true)
  activityStore.addActivity('success', '模拟对局胜利')
}

const simulateLoss = () => {
  matchStatisticsStore.simulateMatchResult(false)
  activityStore.addActivity('info', '模拟对局失败')
}

const updateDuration = () => {
  appSessionStore.updateSessionDuration()
  activityStore.addActivity('info', '会话时长已更新')
}

// 辅助方法
const getActivityClass = (type: Activity['type']) => {
  const classes = {
    info: 'text-blue-600',
    success: 'text-green-600',
    warning: 'text-yellow-600',
    error: 'text-red-600'
  }
  return classes[type] || 'text-gray-600'
}

const getFunctionDisplayName = (key: string) => {
  const names: Record<string, string> = {
    acceptMatch: '自动接受对局',
    selectChampion: '自动选择英雄',
    runeConfig: '自动符文配置',
    banChampion: '自动禁用英雄'
  }
  return names[key] || key
}

// 初始化时添加测试活动
onMounted(() => {
  activityStore.addActivity('info', 'Store 重构测试组件已加载')
})
</script>

<style scoped>
.btn {
  @apply px-3 py-1 rounded text-sm font-medium transition-colors;
}

.btn-primary {
  @apply bg-blue-500 text-white hover:bg-blue-600;
}

.btn-success {
  @apply bg-green-500 text-white hover:bg-green-600;
}

.btn-error {
  @apply bg-red-500 text-white hover:bg-red-600;
}

.card {
  @apply bg-white shadow-sm;
}
</style>
