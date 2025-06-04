<template>
  <div class="auto-functions-view space-y-6">
    <div>
      <h2 class="text-2xl font-bold">自动功能</h2>
      <p class="text-muted-foreground">配置和管理所有自动化功能</p>
    </div>

    <!-- 快速操作 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <Button @click="enableAll" variant="default">
          <Power class="w-4 h-4 mr-2" />
          全部启用
        </Button>
        <Button @click="disableAll" variant="outline">
          <PowerOff class="w-4 h-4 mr-2" />
          全部禁用
        </Button>
      </div>
      <div class="flex items-center space-x-2">
        <span class="text-sm text-muted-foreground">总计启用:</span>
        <span class="font-bold">{{ enabledCount }}/{{ totalFunctions }}</span>
      </div>
    </div>

    <!-- 功能卡片网格 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <Card v-for="func in functions" :key="func.id" class="transition-all hover:shadow-md">
        <CardHeader class="pb-3">
          <div class="flex items-start justify-between">
            <div class="flex items-center space-x-3">
              <div 
                :class="[
                  'w-10 h-10 rounded-lg flex items-center justify-center',
                  func.enabled ? 'bg-primary text-primary-foreground' : 'bg-muted'
                ]"
              >
                <component :is="func.icon" class="w-5 h-5" />
              </div>
              <div>
                <CardTitle class="text-base">{{ func.name }}</CardTitle>
                <p class="text-sm text-muted-foreground">{{ func.description }}</p>
              </div>
            </div>
            <Switch v-model:checked="func.enabled" />
          </div>
        </CardHeader>
        
        <CardContent v-if="func.enabled" class="pt-0">
          <!-- 自动接受对局配置 -->
          <div v-if="func.id === 'auto-accept'" class="space-y-3">
            <div class="flex items-center justify-between">
              <label class="text-sm font-medium">延迟时间</label>
              <div class="flex items-center space-x-2">
                <input 
                  v-model.number="func.config.delay" 
                  type="range" 
                  min="0" 
                  max="10" 
                  class="w-20"
                />
                <span class="text-sm min-w-0">{{ func.config.delay }}s</span>
              </div>
            </div>
          </div>

          <!-- 自动选择英雄配置 -->
          <div v-else-if="func.id === 'auto-pick'" class="space-y-3">
            <div class="space-y-2">
              <label class="text-sm font-medium">首选英雄</label>
              <select v-model="func.config.championId" class="w-full text-sm border rounded px-2 py-1">
                <option value="">选择英雄</option>
                <option v-for="champ in champions" :key="champ.id" :value="champ.id">
                  {{ champ.name }}
                </option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium">自动锁定</label>
              <Switch v-model:checked="func.config.autoLock" />
            </div>
          </div>

          <!-- 自动符文配置 -->
          <div v-else-if="func.id === 'auto-runes'" class="space-y-3">
            <div class="space-y-2">
              <label class="text-sm font-medium">数据源</label>
              <select v-model="func.config.source" class="w-full text-sm border rounded px-2 py-1">
                <option value="op.gg">OP.GG</option>
                <option value="u.gg">U.GG</option>
                <option value="porofessor">Porofessor</option>
              </select>
            </div>
            <Button @click="updateRunes" size="sm" class="w-full">
              <BookOpen class="w-4 h-4 mr-2" />
              立即更新符文
            </Button>
          </div>

          <!-- 其他功能的简单开关 -->
          <div v-else class="space-y-2">
            <p class="text-xs text-muted-foreground">{{ func.status }}</p>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 高级设置 -->
    <Card>
      <CardHeader>
        <CardTitle>高级设置</CardTitle>
        <CardDescription>
          更详细的自动化配置选项
        </CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-4">
            <h4 class="font-medium">触发条件</h4>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <label class="text-sm">仅在排位赛启用</label>
                <Switch v-model:checked="advancedSettings.rankedOnly" />
              </div>
              <div class="flex items-center justify-between">
                <label class="text-sm">检测游戏模式</label>
                <Switch v-model:checked="advancedSettings.detectGameMode" />
              </div>
              <div class="flex items-center justify-between">
                <label class="text-sm">静音模式运行</label>
                <Switch v-model:checked="advancedSettings.silentMode" />
              </div>
            </div>
          </div>
          
          <div class="space-y-4">
            <h4 class="font-medium">安全设置</h4>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <label class="text-sm">随机延迟</label>
                <Switch v-model:checked="advancedSettings.randomDelay" />
              </div>
              <div class="flex items-center justify-between">
                <label class="text-sm">失败重试</label>
                <Switch v-model:checked="advancedSettings.retryOnFailure" />
              </div>
              <div class="flex items-center justify-between">
                <label class="text-sm">记录操作日志</label>
                <Switch v-model:checked="advancedSettings.logging" />
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 功能状态监控 -->
    <Card>
      <CardHeader>
        <CardTitle>实时状态</CardTitle>
        <CardDescription>
          各功能的运行状态监控
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div v-for="status in functionStatus" :key="status.name" 
               class="flex items-center space-x-3 p-3 border rounded-lg">
            <div 
              :class="[
                'w-3 h-3 rounded-full',
                status.status === 'active' ? 'bg-green-500' : 
                status.status === 'standby' ? 'bg-yellow-500' : 'bg-gray-400'
              ]"
            />
            <div>
              <p class="font-medium text-sm">{{ status.name }}</p>
              <p class="text-xs text-muted-foreground">{{ status.message }}</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { 
  Power, 
  PowerOff, 
  Zap, 
  User, 
  BookOpen, 
  Shield, 
  Eye, 
  Target,
  MessageSquare,
  Settings
} from 'lucide-vue-next'

const functions = ref([
  {
    id: 'auto-accept',
    name: '自动接受对局',
    description: '自动接受匹配邀请',
    icon: Zap,
    enabled: true,
    config: {
      delay: 1
    },
    status: '等待对局邀请'
  },
  {
    id: 'auto-pick',
    name: '自动选择英雄',
    description: '预设英雄自动选择',
    icon: User,
    enabled: false,
    config: {
      championId: 76,
      autoLock: true
    },
    status: '等待选择阶段'
  },
  {
    id: 'auto-ban',
    name: '自动禁用英雄',
    description: '自动禁用指定英雄',
    icon: Shield,
    enabled: true,
    config: {
      championId: 157
    },
    status: '等待禁用阶段'
  },
  {
    id: 'auto-runes',
    name: '自动符文配置',
    description: '根据英雄自动设置符文',
    icon: BookOpen,
    enabled: true,
    config: {
      source: 'op.gg'
    },
    status: '准备就绪'
  },
  {
    id: 'enemy-detection',
    name: '敌方信息检测',
    description: '自动查询敌方玩家信息',
    icon: Eye,
    enabled: false,
    config: {},
    status: '已禁用'
  },
  {
    id: 'dodge-helper',
    name: '躲避助手',
    description: '检测不利对局并提醒',
    icon: Target,
    enabled: false,
    config: {},
    status: '已禁用'
  },
  {
    id: 'auto-message',
    name: '自动消息',
    description: '预设游戏内消息发送',
    icon: MessageSquare,
    enabled: false,
    config: {},
    status: '已禁用'
  },
  {
    id: 'advanced-settings',
    name: '高级配置',
    description: '更多自定义设置',
    icon: Settings,
    enabled: true,
    config: {},
    status: '运行中'
  }
])

const champions = ref([
  { id: 76, name: '尼德丽' },
  { id: 103, name: '阿狸' },
  { id: 84, name: '阿卡丽' },
  { id: 157, name: '亚索' },
  { id: 238, name: '劫' }
])

const advancedSettings = ref({
  rankedOnly: true,
  detectGameMode: true,
  silentMode: false,
  randomDelay: true,
  retryOnFailure: true,
  logging: true
})

const functionStatus = ref([
  {
    name: '自动接受',
    status: 'standby',
    message: '等待邀请'
  },
  {
    name: '自动禁用',
    status: 'standby', 
    message: '等待禁用阶段'
  },
  {
    name: '自动符文',
    status: 'active',
    message: '监控中'
  },
  {
    name: '高级配置',
    status: 'active',
    message: '运行正常'
  }
])

const enabledCount = computed(() => {
  return functions.value.filter(f => f.enabled).length
})

const totalFunctions = computed(() => {
  return functions.value.length
})

function enableAll() {
  functions.value.forEach(func => {
    func.enabled = true
  })
}

function disableAll() {
  functions.value.forEach(func => {
    func.enabled = false
  })
}

function updateRunes() {
  // TODO: 实现更新符文逻辑
  console.log('更新符文')
}
</script> 