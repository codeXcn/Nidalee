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
          <!-- 自动连接状态显示 -->
          <div class="flex items-center space-x-2">
            <div v-if="gameStore.gameStatus.connected" class="flex items-center space-x-2 text-green-600 dark:text-green-400">
              <div class="w-2 h-2 bg-green-500 rounded-full"></div>
              <span class="text-sm">已自动连接到游戏</span>
            </div>
            <div v-else class="flex items-center space-x-2 text-orange-600 dark:text-orange-400">
              <div class="w-2 h-2 bg-orange-500 rounded-full animate-pulse"></div>
              <span class="text-sm">等待游戏客户端启动...</span>
            </div>
          </div>

          <!-- 隐藏的手动连接按钮，仅在需要时显示 -->
          <Button
            v-if="false"
            :disabled="gameStore.gameStatus.connecting"
            @click="connectToGame"
            variant="outline"
            size="sm"
          >
            <Gamepad2 class="w-4 h-4 mr-2" />
            手动连接
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
            :class="['w-3 h-3 rounded-full', gameStore.gameStatus.connected ? 'bg-green-500' : 'bg-red-500']"
          />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">
            {{ gameStore.gameStatus.connected ? '在线' : '离线' }}
          </div>
          <p class="text-xs text-muted-foreground">
            {{ gameStore.gameStatus.connected ? '准备就绪' : '游戏未启动' }}
          </p>
          <!-- 显示 lockfile 信息 -->
          <div v-if="gameStore.gameStatus.connected && gameStore.gameStatus.lockfileInfo" class="mt-2 text-xs">
            <p class="text-muted-foreground">进程: {{ gameStore.gameStatus.lockfileInfo.process_name }}</p>
            <p class="text-muted-foreground">PID: {{ gameStore.gameStatus.lockfileInfo.process_id }}</p>
            <p class="text-muted-foreground">端口: {{ gameStore.gameStatus.lockfileInfo.port }}</p>
          </div>
          <!-- 未连接时的提示 -->
          <div v-if="!gameStore.gameStatus.connected" class="mt-2 text-xs">
            <p class="text-orange-600 dark:text-orange-400">请启动英雄联盟客户端，将自动连接</p>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">今日胜场</CardTitle>
          <Trophy class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ gameStore.dailyStats.wins }}</div>
          <p class="text-xs text-muted-foreground">胜率 {{ gameStore.dailyStats.winRate }}%</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">自动功能</CardTitle>
          <Sparkles class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ enabledFeatures }}</div>
          <p class="text-xs text-muted-foreground">项功能启用中</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">活跃时长</CardTitle>
          <Clock class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ sessionTime }}</div>
          <p class="text-xs text-muted-foreground">本次会话</p>
        </CardContent>
      </Card>
    </div>

    <!-- Lockfile 信息展示 -->
    <Card v-if="gameStore.gameStatus.connected && gameStore.gameStatus.lockfileInfo">
      <CardHeader>
        <CardTitle class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <div class="w-2 h-2 bg-green-500 rounded-full" />
            <span>游戏连接信息</span>
          </div>
          <Button
            @click="clearCacheAndRefresh"
            variant="outline"
            size="sm"
            class="text-xs"
          >
            <Database class="w-3 h-3 mr-1" />
            清除缓存
          </Button>
        </CardTitle>
        <CardDescription>当前连接的英雄联盟客户端详细信息</CardDescription>
      </CardHeader>
      <CardContent>
                 <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
           <div class="space-y-2">
             <div class="flex justify-between items-center">
               <span class="text-sm font-medium text-muted-foreground">进程名称:</span>
               <span class="text-sm font-mono bg-muted px-2 py-1 rounded">{{ gameStore.gameStatus.lockfileInfo.process_name }}</span>
             </div>
             <div class="flex justify-between items-center">
               <span class="text-sm font-medium text-muted-foreground">进程ID:</span>
               <span class="text-sm font-mono bg-muted px-2 py-1 rounded">{{ gameStore.gameStatus.lockfileInfo.process_id }}</span>
             </div>
             <div class="flex justify-between items-center">
               <span class="text-sm font-medium text-muted-foreground">连接端口:</span>
               <span class="text-sm font-mono bg-muted px-2 py-1 rounded">{{ gameStore.gameStatus.lockfileInfo.port }}</span>
             </div>
           </div>
           <div class="space-y-2">
             <div class="flex justify-between items-center">
               <span class="text-sm font-medium text-muted-foreground">协议类型:</span>
               <span class="text-sm font-mono bg-muted px-2 py-1 rounded">{{ gameStore.gameStatus.lockfileInfo.protocol }}</span>
             </div>
             <div class="flex justify-between items-center">
               <span class="text-sm font-medium text-muted-foreground">API 地址:</span>
               <span class="text-sm font-mono bg-muted px-2 py-1 rounded">{{ gameStore.gameStatus.lockfileInfo.protocol }}://127.0.0.1:{{ gameStore.gameStatus.lockfileInfo.port }}</span>
             </div>
             <div class="flex justify-between items-center">
               <span class="text-sm font-medium text-muted-foreground">认证令牌:</span>
               <span class="text-sm font-mono bg-muted px-2 py-1 rounded">{{ gameStore.gameStatus.lockfileInfo.token.substring(0, 12) }}...</span>
             </div>
           </div>
         </div>

         <!-- 缓存状态信息 -->
         <div v-if="cacheStatus" class="mt-4 pt-4 border-t">
           <h4 class="text-sm font-medium text-muted-foreground mb-2">路径缓存状态</h4>
           <div class="space-y-2">
             <div class="flex justify-between items-center">
               <span class="text-sm text-muted-foreground">缓存状态:</span>
               <span :class="['text-sm font-mono px-2 py-1 rounded',
                             cacheStatus.hasCachedPath ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' :
                             'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200']">
                 {{ cacheStatus.hasCachedPath ? '已缓存' : '无缓存' }}
               </span>
             </div>
             <div v-if="cacheStatus.hasCachedPath" class="flex justify-between items-center">
               <span class="text-sm text-muted-foreground">缓存路径:</span>
               <span class="text-xs font-mono bg-muted px-2 py-1 rounded max-w-xs truncate" :title="cacheStatus.cachedPath">
                 {{ cacheStatus.cachedPath }}
               </span>
             </div>
             <div v-if="cacheStatus.hasCachedPath" class="flex justify-between items-center">
               <span class="text-sm text-muted-foreground">路径有效:</span>
               <span :class="['text-sm font-mono px-2 py-1 rounded',
                             cacheStatus.isValid ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' :
                             'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200']">
                 {{ cacheStatus.isValid ? '有效' : '无效' }}
               </span>
             </div>
           </div>
         </div>
      </CardContent>
    </Card>

    <!-- 功能快捷操作 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 快速功能 -->
      <Card>
        <CardHeader>
          <CardTitle>快速功能</CardTitle>
          <CardDescription> 常用功能的快捷访问 </CardDescription>
        </CardHeader>
        <CardContent class="space-y-3">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div
                class="w-10 h-10 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center"
              >
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
              <div
                class="w-10 h-10 bg-green-100 dark:bg-green-900 rounded-lg flex items-center justify-center"
              >
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
              <div
                class="w-10 h-10 bg-purple-100 dark:bg-purple-900 rounded-lg flex items-center justify-center"
              >
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
          <CardDescription> 最新的系统活动记录 </CardDescription>
        </CardHeader>
        <CardContent>
          <div class="space-y-4">
            <div
              v-for="activity in recentActivities"
              :key="activity.id"
              class="flex items-start space-x-3"
            >
              <div
                :class="[
                  'w-2 h-2 rounded-full mt-2',
                  activity.type === 'success'
                    ? 'bg-green-500'
                    : activity.type === 'warning'
                      ? 'bg-yellow-500'
                      : 'bg-blue-500'
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
        <CardDescription> 近期游戏表现分析 </CardDescription>
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
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Gamepad2, Trophy, Sparkles, Clock, Zap, Users, BookOpen, BarChart3, Database } from 'lucide-vue-next'
import { useGameStore } from '@/stores/useGameStore'

const gameStore = useGameStore()

const features = ref({
  autoAccept: true,
  autoPick: false,
  autoRunes: true
})

const sessionStartTime = ref(Date.now())
const cacheStatus = ref<any>(null)

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
  const success = await gameStore.connectToGame()

  // 更新最近活动记录
  recentActivities.value.unshift({
    id: Date.now(),
    message: success ? '成功连接到英雄联盟客户端' : '连接失败，请检查游戏客户端',
    type: success ? 'success' : 'warning',
    time: '刚刚'
  })

  // 如果连接成功，更新缓存状态
  if (success) {
    await updateCacheStatus()
  }
}

// 更新缓存状态
async function updateCacheStatus() {
  cacheStatus.value = await gameStore.getCacheStatus()
}

// 清除缓存并刷新
async function clearCacheAndRefresh() {
  const success = await gameStore.clearPathCache()
  if (success) {
    await updateCacheStatus()

    // 更新最近活动记录
    recentActivities.value.unshift({
      id: Date.now(),
      message: '路径缓存已清除',
      type: 'success',
      time: '刚刚'
    })
  }
}

let sessionTimeInterval: any = null

onMounted(async () => {
  // 模拟定时更新会话时间
  sessionTimeInterval = setInterval(() => {
    // 触发响应式更新
    sessionStartTime.value = sessionStartTime.value
  }, 60000)

  // 获取初始缓存状态
  await updateCacheStatus()
})

onUnmounted(() => {
  // 清理定时器
  if (sessionTimeInterval) {
    clearInterval(sessionTimeInterval)
  }
})
</script>
