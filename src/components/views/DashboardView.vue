<template>
  <div class="dashboard space-y-6">
    <!-- 欢迎区域 -->
    <div class="welcome-section">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-2xl font-bold">欢迎回来!</h2>
          <p class="text-muted-foreground mt-1">准备开始你的英雄联盟征程</p>
        </div>
        <div class="flex items-center space-x-4">
          <Button @click="connectToGame" :disabled="gameStatus.connecting">
            <Gamepad2 class="w-4 h-4 mr-2" />
            {{ gameStatus.connected ? '已连接' : '连接游戏' }}
          </Button>
        </div>
      </div>
    </div>

    <!-- 状态卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">游戏连接</CardTitle>
          <div 
            :class="[
              'w-3 h-3 rounded-full',
              gameStatus.connected ? 'bg-green-500' : 'bg-red-500'
            ]"
          />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">
            {{ gameStatus.connected ? '在线' : '离线' }}
          </div>
          <p class="text-xs text-muted-foreground">
            {{ gameStatus.connected ? '准备就绪' : '等待连接' }}
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">今日胜场</CardTitle>
          <Trophy class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ dailyStats.wins }}</div>
          <p class="text-xs text-muted-foreground">
            胜率 {{ dailyStats.winRate }}%
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">自动功能</CardTitle>
          <Sparkles class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ enabledFeatures }}</div>
          <p class="text-xs text-muted-foreground">
            项功能启用中
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">活跃时长</CardTitle>
          <Clock class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ sessionTime }}</div>
          <p class="text-xs text-muted-foreground">
            本次会话
          </p>
        </CardContent>
      </Card>
    </div>

    <!-- 功能快捷操作 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 快速功能 -->
      <Card>
        <CardHeader>
          <CardTitle>快速功能</CardTitle>
          <CardDescription>
            常用功能的快捷访问
          </CardDescription>
        </CardHeader>
        <CardContent class="space-y-3">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center">
                <Zap class="w-5 h-5 text-blue-600 dark:text-blue-400" />
              </div>
              <div>
                <p class="font-medium">自动接受对局</p>
                <p class="text-sm text-muted-foreground">自动接受匹配邀请</p>
              </div>
            </div>
            <Switch v-model:checked="features.autoAccept" />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 bg-green-100 dark:bg-green-900 rounded-lg flex items-center justify-center">
                <Users class="w-5 h-5 text-green-600 dark:text-green-400" />
              </div>
              <div>
                <p class="font-medium">自动选择英雄</p>
                <p class="text-sm text-muted-foreground">预设英雄自动选择</p>
              </div>
            </div>
            <Switch v-model:checked="features.autoPick" />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 bg-purple-100 dark:bg-purple-900 rounded-lg flex items-center justify-center">
                <BookOpen class="w-5 h-5 text-purple-600 dark:text-purple-400" />
              </div>
              <div>
                <p class="font-medium">自动符文配置</p>
                <p class="text-sm text-muted-foreground">根据英雄自动设置符文</p>
              </div>
            </div>
            <Switch v-model:checked="features.autoRunes" />
          </div>
        </CardContent>
      </Card>

      <!-- 最近活动 -->
      <Card>
        <CardHeader>
          <CardTitle>最近活动</CardTitle>
          <CardDescription>
            最新的系统活动记录
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div class="space-y-4">
            <div v-for="activity in recentActivities" :key="activity.id" class="flex items-start space-x-3">
              <div 
                :class="[
                  'w-2 h-2 rounded-full mt-2',
                  activity.type === 'success' ? 'bg-green-500' : 
                  activity.type === 'warning' ? 'bg-yellow-500' : 
                  'bg-blue-500'
                ]"
              />
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium">{{ activity.message }}</p>
                <p class="text-xs text-muted-foreground">{{ activity.time }}</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 统计图表 -->
    <Card>
      <CardHeader>
        <CardTitle>游戏统计</CardTitle>
        <CardDescription>
          近期游戏表现分析
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="h-64 flex items-center justify-center text-muted-foreground">
          <div class="text-center">
            <BarChart3 class="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>统计图表功能开发中...</p>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { 
  Gamepad2, 
  Trophy, 
  Sparkles, 
  Clock, 
  Zap, 
  Users, 
  BookOpen,
  BarChart3
} from 'lucide-vue-next'

const gameStatus = ref({
  connected: false,
  connecting: false
})

const dailyStats = ref({
  wins: 7,
  losses: 3,
  winRate: 70
})

const features = ref({
  autoAccept: true,
  autoPick: false,
  autoRunes: true
})

const sessionStartTime = ref(Date.now())

const recentActivities = ref([
  {
    id: 1,
    message: '自动接受对局成功',
    type: 'success',
    time: '2分钟前'
  },
  {
    id: 2,
    message: '检测到新的英雄联盟客户端',
    type: 'info',
    time: '5分钟前'
  },
  {
    id: 3,
    message: '符文页配置已更新',
    type: 'success',
    time: '10分钟前'
  },
  {
    id: 4,
    message: '连接丢失，正在重连...',
    type: 'warning',
    time: '15分钟前'
  }
])

const enabledFeatures = computed(() => {
  return Object.values(features.value).filter(Boolean).length
})

const sessionTime = computed(() => {
  const minutes = Math.floor((Date.now() - sessionStartTime.value) / 60000)
  const hours = Math.floor(minutes / 60)
  if (hours > 0) {
    return `${hours}h ${minutes % 60}m`
  }
  return `${minutes}m`
})

async function connectToGame() {
  gameStatus.value.connecting = true
  try {
    // TODO: 实现实际的游戏连接逻辑
    await new Promise(resolve => setTimeout(resolve, 2000))
    gameStatus.value.connected = true
    
    recentActivities.value.unshift({
      id: Date.now(),
      message: '成功连接到英雄联盟客户端',
      type: 'success',
      time: '刚刚'
    })
  } catch (error) {
    recentActivities.value.unshift({
      id: Date.now(),
      message: '连接失败，请检查游戏客户端',
      type: 'warning',
      time: '刚刚'
    })
  } finally {
    gameStatus.value.connecting = false
  }
}

onMounted(() => {
  // 模拟定时更新会话时间
  setInterval(() => {
    // 触发响应式更新
    sessionStartTime.value = sessionStartTime.value
  }, 60000)
})
</script> 