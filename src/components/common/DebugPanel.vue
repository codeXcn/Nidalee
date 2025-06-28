<template>
  <div v-if="isDevelopment" class="fixed bottom-4 right-4 z-50">
    <div class="bg-background/95 backdrop-blur-sm border border-border rounded-lg shadow-lg p-4 max-w-sm">
      <div class="flex items-center justify-between mb-3">
        <h3 class="text-sm font-semibold">🛠️ 调试面板</h3>
        <Button variant="ghost" size="sm" @click="togglePanel">
          {{ isExpanded ? '收起' : '展开' }}
        </Button>
      </div>
      
      <div v-if="isExpanded" class="space-y-3">
        <div class="text-xs space-y-2">
          <div class="font-medium">日志过滤器:</div>
          <div class="flex flex-wrap gap-1">
            <Badge 
              v-for="filter in logFilters" 
              :key="filter.key"
              :variant="filter.active ? 'default' : 'outline'"
              class="cursor-pointer text-xs"
              @click="toggleFilter(filter.key)"
            >
              {{ filter.label }}
            </Badge>
          </div>
        </div>
        
        <div class="text-xs space-y-2">
          <div class="font-medium">快捷操作:</div>
          <div class="flex flex-col gap-1">
            <Button size="sm" variant="outline" @click="clearConsole">
              清空控制台
            </Button>
            <Button size="sm" variant="outline" @click="showCurrentPhase">
              显示当前阶段
            </Button>
            <Button size="sm" variant="outline" @click="showAutoConfig">
              显示自动配置
            </Button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { useGameStatusStore } from '@/stores'
import { useAutoFunctionStore } from '@/stores/autoFunctionStore'

const isDevelopment = ref(import.meta.env.DEV)
const isExpanded = ref(false)
const gameStatusStore = useGameStatusStore()
const autoFunctionStore = useAutoFunctionStore()

const logFilters = ref([
  { key: 'gamePhase', label: '🎮 阶段', active: true },
  { key: 'autoFunction', label: '🤖 自动', active: true },
  { key: 'champSelect', label: '⭐ 选人', active: true },
  { key: 'autoPick', label: '🎯 自动选', active: true },
  { key: 'autoBan', label: '🚫 自动禁', active: true },
  { key: 'store', label: '📊 Store', active: true }
])

const togglePanel = () => {
  isExpanded.value = !isExpanded.value
}

const toggleFilter = (key: string) => {
  const filter = logFilters.value.find(f => f.key === key)
  if (filter) {
    filter.active = !filter.active
    updateConsoleFilter()
  }
}

const updateConsoleFilter = () => {
  // 这里可以实现控制台日志过滤逻辑
  const activeFilters = logFilters.value.filter(f => f.active).map(f => f.key)
  console.log('🔧 活跃的日志过滤器:', activeFilters)
}

const clearConsole = () => {
  console.clear()
  console.log('🧹 控制台已清空')
  console.log('📋 日志说明:')
  console.log('  🎮 = 游戏阶段管理')
  console.log('  🤖 = 自动功能处理')
  console.log('  ⭐ = 选人操作')
  console.log('  🚫 = 禁用操作')
  console.log('  🔍 = 数据查找')
  console.log('  📊 = 状态管理')
  console.log('  📡 = API调用')
  console.log('  ✅ = 成功操作')
  console.log('  ❌ = 错误信息')
  console.log('  ⚠️ = 警告信息')
  console.log('==========================================\n')
}

const showCurrentPhase = () => {
  console.log('\n📊 当前游戏状态:')
  console.log('  阶段:', gameStatusStore.currentPhase)
  console.log('  是否在游戏中:', gameStatusStore.isInGame)
  console.log('  是否在选人:', gameStatusStore.isInChampSelect)
  console.log('  是否在房间:', gameStatusStore.isInLobby)
  console.log('==========================================\n')
}

const showAutoConfig = () => {
  console.log('\n🤖 自动功能配置:')
  const { autoFunctions } = autoFunctionStore
  console.log('  自动接受对局:', {
    enabled: autoFunctions.acceptMatch.enabled,
    delay: autoFunctions.acceptMatch.delay
  })
  console.log('  自动选择英雄:', {
    enabled: autoFunctions.selectChampion.enabled,
    championId: autoFunctions.selectChampion.championId,
    championName: autoFunctions.selectChampion.championInfo?.name,
    delay: autoFunctions.selectChampion.delay
  })
  console.log('  自动禁用英雄:', {
    enabled: autoFunctions.banChampion.enabled,
    championId: autoFunctions.banChampion.championId,
    championName: autoFunctions.banChampion.championInfo?.name,
    delay: autoFunctions.banChampion.delay
  })
  console.log('  自动符文配置:', {
    enabled: autoFunctions.runeConfig.enabled,
    delay: autoFunctions.runeConfig.delay
  })
  console.log('==========================================\n')
}

// 初始化时显示说明
onMounted(() => {
  if (isDevelopment.value) {
    console.log('🛠️ 调试面板已加载')
    console.log('   右下角可以看到调试面板，点击展开查看更多选项')
    clearConsole()
  }
})
</script>
