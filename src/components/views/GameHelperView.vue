<template>
  <div class="game-helper-view space-y-6">
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold">游戏助手</h2>
        <p class="text-muted-foreground">自动化游戏操作和实时数据分析</p>
      </div>
      <div class="flex items-center space-x-3">
        <Button variant="outline" @click="refreshGameData">
          <RefreshCw class="w-4 h-4 mr-2" />
          刷新数据
        </Button>
        <Button @click="toggleAllFeatures">
          <Power class="w-4 h-4 mr-2" />
          {{ allFeaturesEnabled ? '全部禁用' : '全部启用' }}
        </Button>
      </div>
    </div>

    <!-- 游戏状态 -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center">
          <Activity class="w-5 h-5 mr-2" />
          游戏状态
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="flex items-center space-x-3">
            <div 
              :class="[
                'w-3 h-3 rounded-full',
                gameStatus.connected ? 'bg-green-500' : 'bg-red-500'
              ]"
            />
            <div>
              <p class="font-medium">客户端连接</p>
              <p class="text-sm text-muted-foreground">
                {{ gameStatus.connected ? '已连接' : '未连接' }}
              </p>
            </div>
          </div>
          
          <div class="flex items-center space-x-3">
            <div 
              :class="[
                'w-3 h-3 rounded-full',
                gameStatus.inGame ? 'bg-blue-500' : 'bg-gray-400'
              ]"
            />
            <div>
              <p class="font-medium">游戏状态</p>
              <p class="text-sm text-muted-foreground">
                {{ gameStatus.inGame ? '游戏中' : '大厅' }}
              </p>
            </div>
          </div>
          
          <div class="flex items-center space-x-3">
            <div 
              :class="[
                'w-3 h-3 rounded-full',
                gameStatus.summonerName ? 'bg-green-500' : 'bg-gray-400'
              ]"
            />
            <div>
              <p class="font-medium">召唤师</p>
              <p class="text-sm text-muted-foreground">
                {{ gameStatus.summonerName || '未获取' }}
              </p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 自动功能配置 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 基础自动功能 -->
      <Card>
        <CardHeader>
          <CardTitle>基础功能</CardTitle>
          <CardDescription>
            基本的自动化操作设置
          </CardDescription>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <p class="font-medium">自动接受对局</p>
              <p class="text-sm text-muted-foreground">自动接受匹配邀请</p>
            </div>
            <Switch v-model:checked="config.autoAccept.enabled" />
          </div>
          
          <div v-if="config.autoAccept.enabled" class="ml-4 space-y-3 border-l-2 border-muted pl-4">
            <div class="flex items-center space-x-3">
              <label class="text-sm font-medium min-w-0">延迟时间:</label>
              <div class="flex items-center space-x-2">
                <input 
                  v-model.number="config.autoAccept.delay" 
                  type="number" 
                  min="0" 
                  max="10" 
                  class="w-20 px-2 py-1 text-sm border rounded"
                />
                <span class="text-sm text-muted-foreground">秒</span>
              </div>
            </div>
          </div>

          <Separator />

          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <p class="font-medium">自动选择英雄</p>
              <p class="text-sm text-muted-foreground">预设英雄自动选择</p>
            </div>
            <Switch v-model:checked="config.autoPickChampion.enabled" />
          </div>
          
          <div v-if="config.autoPickChampion.enabled" class="ml-4 space-y-3 border-l-2 border-muted pl-4">
            <div class="space-y-2">
              <label class="text-sm font-medium">主选英雄:</label>
              <select 
                v-model="config.autoPickChampion.championId" 
                class="w-full px-3 py-2 text-sm border rounded-md"
              >
                <option value="">选择英雄</option>
                <option v-for="champion in champions" :key="champion.id" :value="champion.id">
                  {{ champion.name }}
                </option>
              </select>
            </div>
            
            <div class="space-y-2">
              <label class="text-sm font-medium">备选英雄:</label>
              <div class="flex flex-wrap gap-2">
                <Button 
                  v-for="backup in config.autoPickChampion.backupChampions" 
                  :key="backup.id"
                  variant="outline" 
                  size="sm"
                  @click="removeBackupChampion(backup.id)"
                >
                  {{ backup.name }}
                  <X class="w-3 h-3 ml-1" />
                </Button>
                <Button variant="outline" size="sm" @click="showAddBackup = true">
                  <Plus class="w-3 h-3 mr-1" />
                  添加备选
                </Button>
              </div>
            </div>
          </div>

          <Separator />

          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <p class="font-medium">自动禁用英雄</p>
              <p class="text-sm text-muted-foreground">自动禁用指定英雄</p>
            </div>
            <Switch v-model:checked="config.autoBanChampion.enabled" />
          </div>
          
          <div v-if="config.autoBanChampion.enabled" class="ml-4 space-y-3 border-l-2 border-muted pl-4">
            <div class="space-y-2">
              <label class="text-sm font-medium">禁用英雄:</label>
              <select 
                v-model="config.autoBanChampion.championId" 
                class="w-full px-3 py-2 text-sm border rounded-md"
              >
                <option value="">选择英雄</option>
                <option v-for="champion in champions" :key="champion.id" :value="champion.id">
                  {{ champion.name }}
                </option>
              </select>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 高级功能 -->
      <Card>
        <CardHeader>
          <CardTitle>高级功能</CardTitle>
          <CardDescription>
            进阶的辅助功能
          </CardDescription>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <p class="font-medium">自动符文配置</p>
              <p class="text-sm text-muted-foreground">根据英雄自动设置符文</p>
            </div>
            <Switch v-model:checked="config.autoRune.enabled" />
          </div>
          
          <div v-if="config.autoRune.enabled" class="ml-4 space-y-3 border-l-2 border-muted pl-4">
            <div class="space-y-2">
              <label class="text-sm font-medium">数据来源:</label>
              <select 
                v-model="config.autoRune.source" 
                class="w-full px-3 py-2 text-sm border rounded-md"
              >
                <option value="op.gg">OP.GG</option>
                <option value="u.gg">U.GG</option>
                <option value="champion.gg">Champion.GG</option>
              </select>
            </div>
            
            <Button @click="updateRunePage" class="w-full">
              <BookOpen class="w-4 h-4 mr-2" />
              立即更新符文页
            </Button>
          </div>

          <Separator />

          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <p class="font-medium">对局分析</p>
              <p class="text-sm text-muted-foreground">实时分析对局信息</p>
            </div>
            <Switch v-model:checked="config.matchAnalysis.enabled" />
          </div>

          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <p class="font-medium">敌方信息查询</p>
              <p class="text-sm text-muted-foreground">查询敌方玩家数据</p>
            </div>
            <Switch v-model:checked="config.opponentAnalysis.enabled" />
          </div>

          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <p class="font-medium">战术建议</p>
              <p class="text-sm text-muted-foreground">基于阵容的战术提示</p>
            </div>
            <Switch v-model:checked="config.tacticSuggestions.enabled" />
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 实时功能状态 -->
    <Card>
      <CardHeader>
        <CardTitle>功能状态</CardTitle>
        <CardDescription>
          当前启用的自动化功能状态
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div 
            v-for="feature in featureStatus" 
            :key="feature.name"
            class="flex items-center space-x-3 p-3 border rounded-lg"
          >
            <div 
              :class="[
                'w-3 h-3 rounded-full',
                feature.active ? 'bg-green-500' : 'bg-gray-400'
              ]"
            />
            <div>
              <p class="font-medium text-sm">{{ feature.name }}</p>
              <p class="text-xs text-muted-foreground">{{ feature.status }}</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 添加备选英雄对话框 -->
    <Dialog v-model:open="showAddBackup">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>添加备选英雄</DialogTitle>
        </DialogHeader>
        <div class="space-y-4">
          <select 
            v-model="selectedBackupChampion" 
            class="w-full px-3 py-2 border rounded-md"
          >
            <option value="">选择英雄</option>
            <option 
              v-for="champion in availableBackupChampions" 
              :key="champion.id" 
              :value="champion"
            >
              {{ champion.name }}
            </option>
          </select>
        </div>
        <DialogFooter>
          <Button variant="outline" @click="showAddBackup = false">取消</Button>
          <Button @click="addBackupChampion" :disabled="!selectedBackupChampion">
            添加
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Separator } from '@/components/ui/separator'
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { 
  RefreshCw, 
  Power, 
  Activity, 
  BookOpen, 
  Plus, 
  X 
} from 'lucide-vue-next'

const gameStatus = ref({
  connected: true,
  inGame: false,
  summonerName: 'TestSummoner'
})

const config = ref({
  autoAccept: {
    enabled: true,
    delay: 1
  },
  autoPickChampion: {
    enabled: false,
    championId: 76,
    backupChampions: [
      { id: 103, name: "阿狸" },
      { id: 84, name: "阿卡丽" }
    ]
  },
  autoBanChampion: {
    enabled: true,
    championId: 157
  },
  autoRune: {
    enabled: true,
    source: 'op.gg'
  },
  matchAnalysis: {
    enabled: true
  },
  opponentAnalysis: {
    enabled: false
  },
  tacticSuggestions: {
    enabled: true
  }
})

const champions = ref([
  { id: 76, name: "尼德丽" },
  { id: 103, name: "阿狸" },
  { id: 84, name: "阿卡丽" },
  { id: 157, name: "亚索" },
  { id: 238, name: "劫" },
  { id: 11, name: "易" },
  { id: 64, name: "盲僧" }
])

const showAddBackup = ref(false)
const selectedBackupChampion = ref(null)

const allFeaturesEnabled = computed(() => {
  return config.value.autoAccept.enabled && 
         config.value.autoPickChampion.enabled && 
         config.value.autoBanChampion.enabled && 
         config.value.autoRune.enabled
})

const availableBackupChampions = computed(() => {
  const usedIds = config.value.autoPickChampion.backupChampions.map(c => c.id)
  return champions.value.filter(c => !usedIds.includes(c.id))
})

const featureStatus = computed(() => [
  {
    name: '自动接受',
    active: config.value.autoAccept.enabled,
    status: config.value.autoAccept.enabled ? '运行中' : '已禁用'
  },
  {
    name: '自动选英雄',
    active: config.value.autoPickChampion.enabled,
    status: config.value.autoPickChampion.enabled ? '等待选择阶段' : '已禁用'
  },
  {
    name: '自动符文',
    active: config.value.autoRune.enabled,
    status: config.value.autoRune.enabled ? '准备就绪' : '已禁用'
  },
  {
    name: '对局分析',
    active: config.value.matchAnalysis.enabled,
    status: config.value.matchAnalysis.enabled ? '分析中' : '已禁用'
  }
])

function toggleAllFeatures() {
  const enable = !allFeaturesEnabled.value
  config.value.autoAccept.enabled = enable
  config.value.autoPickChampion.enabled = enable
  config.value.autoBanChampion.enabled = enable
  config.value.autoRune.enabled = enable
}

function refreshGameData() {
  // TODO: 实现刷新游戏数据逻辑
  console.log('刷新游戏数据')
}

function updateRunePage() {
  // TODO: 实现更新符文页逻辑
  console.log('更新符文页')
}

function addBackupChampion() {
  if (selectedBackupChampion.value) {
    config.value.autoPickChampion.backupChampions.push(selectedBackupChampion.value)
    selectedBackupChampion.value = null
    showAddBackup.value = false
  }
}

function removeBackupChampion(championId: number) {
  const index = config.value.autoPickChampion.backupChampions.findIndex(c => c.id === championId)
  if (index > -1) {
    config.value.autoPickChampion.backupChampions.splice(index, 1)
  }
}
</script> 