<template>
  <div class="min-h-screen bg-background">
    <!-- 通知组件已移除，使用 toast 替代 -->
    <div class="max-w-7xl mx-auto">
      <!-- 标题和状态 -->
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between mb-8 pb-6 border-b border-border">
        <div class="space-y-2">
          <h2 class="text-3xl font-bold text-foreground tracking-tight">自动功能配置</h2>
          <p class="text-muted-foreground text-lg">配置各种自动化游戏功能和执行延迟</p>
        </div>
        <div class="flex items-center gap-4 mt-4 sm:mt-0">
          <Badge variant="secondary" class="px-4 py-2 text-sm font-medium bg-muted/50 hover:bg-muted transition-colors">
            已启用: {{ enabledFunctionsCount }} 项
          </Badge>
          <Button
            variant="destructive"
            size="default"
            @click="handleDisableAll"
            :disabled="!isAnyFunctionEnabled"
            class="gap-2 transition-all hover:bg-destructive/90 disabled:opacity-50"
          >
            <X class="h-4 w-4" />
            全部禁用
          </Button>
        </div>
      </div>

      <!-- 功能卡片网格 -->
      <div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
        <!-- 自动接受对局 -->
        <div class="transition-transform hover:scale-[1.02]">
          <FunctionCard
            title="自动接受对局"
            description="自动接受匹配到的对局，避免错过游戏机会"
            v-model:enabled="autoFunctions.acceptMatch.enabled"
            v-model:delay="autoFunctions.acceptMatch.delay"
          />
        </div>

        <!-- 自动选择英雄 -->
        <div class="transition-transform hover:scale-[1.02]">
          <ChampionFunctionCard
            title="自动选择英雄"
            description="在选择阶段自动选择指定英雄，快速完成英雄选择"
            v-model:enabled="autoFunctions.selectChampion.enabled"
            v-model:delay="autoFunctions.selectChampion.delay"
            :champion-info="autoFunctions.selectChampion.championInfo"
            :show-risk-warning="true"
            risk-warning-text="⚠️ 自动选择英雄可能被游戏检测识别，存在封号风险，请谨慎使用并合理设置延迟"
            @champion-select="handleChampionSelect"
            @champion-clear="handleClearSelect"
          />
        </div>

        <!-- 自动符文配置 -->
        <div class="transition-transform hover:scale-[1.02]">
          <FunctionCard
            title="自动符文配置"
            description="根据选择的英雄自动配置最优符文页面"
            v-model:enabled="autoFunctions.runeConfig.enabled"
            v-model:delay="autoFunctions.runeConfig.delay"
          />
        </div>

        <!-- 自动禁用英雄 -->
        <div class="transition-transform hover:scale-[1.02]">
          <ChampionFunctionCard
            title="自动禁用英雄"
            description="在禁用阶段自动禁用指定英雄，防止对手选择"
            v-model:enabled="autoFunctions.banChampion.enabled"
            v-model:delay="autoFunctions.banChampion.delay"
            :champion-info="autoFunctions.banChampion.championInfo"
            :show-risk-warning="true"
            risk-warning-text="⚠️ 自动禁用英雄可能被游戏检测识别，存在封号风险，请谨慎使用并合理设置延迟"
            @champion-select="handleChampionBan"
            @champion-clear="handleClearBan"
          />
        </div>
      </div>

      <!-- 功能状态摘要 -->
      <div v-if="isAnyFunctionEnabled" class="mt-8 p-6 bg-muted/30 rounded-lg border border-border">
        <h3 class="text-lg font-semibold text-foreground mb-4">已启用功能</h3>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <div
            v-for="func in enabledFunctions"
            :key="func.key"
            class="flex items-center gap-3 p-3 bg-card rounded-lg border border-border"
          >
            <div class="h-2 w-2 bg-green-500 rounded-full animate-pulse"></div>
            <span class="text-sm font-medium text-foreground">{{ func.name }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ChampionInfo } from '@/stores/autoFunctionStore'
import { useAutoFunctionStore } from '@/stores/autoFunctionStore'
import { CheckCircle, Info, X, XCircle } from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const autoFunctionStore = useAutoFunctionStore()

// 计算属性 - 使用 toRef 确保响应式
const autoFunctions = computed(() => {
  const functions = autoFunctionStore.autoFunctions
  console.log('[AutoFunctionsView] autoFunctions computed:', functions)
  return functions
})

// 添加更详细的响应式监听
watch(
  () => autoFunctionStore.autoFunctions,
  (newVal, oldVal) => {
    console.log('[AutoFunctionsView] Store autoFunctions changed:', {
      old: oldVal,
      new: newVal,
      acceptMatch: {
        old: oldVal?.acceptMatch,
        new: newVal?.acceptMatch
      },
      selectChampion: {
        old: oldVal?.selectChampion,
        new: newVal?.selectChampion
      },
      runeConfig: {
        old: oldVal?.runeConfig,
        new: newVal?.runeConfig
      },
      banChampion: {
        old: oldVal?.banChampion,
        new: newVal?.banChampion
      }
    })
  },
  { deep: true, immediate: true }
)

// 监听每个功能的enabled状态变化
watch(
  () => autoFunctionStore.autoFunctions.acceptMatch.enabled,
  (newVal, oldVal) => {
    console.log('[AutoFunctionsView] acceptMatch.enabled changed:', { from: oldVal, to: newVal })
  },
  { immediate: true }
)

watch(
  () => autoFunctionStore.autoFunctions.selectChampion.enabled,
  (newVal, oldVal) => {
    console.log('[AutoFunctionsView] selectChampion.enabled changed:', { from: oldVal, to: newVal })
  },
  { immediate: true }
)

watch(
  () => autoFunctionStore.autoFunctions.runeConfig.enabled,
  (newVal, oldVal) => {
    console.log('[AutoFunctionsView] runeConfig.enabled changed:', { from: oldVal, to: newVal })
  },
  { immediate: true }
)

watch(
  () => autoFunctionStore.autoFunctions.banChampion.enabled,
  (newVal, oldVal) => {
    console.log('[AutoFunctionsView] banChampion.enabled changed:', { from: oldVal, to: newVal })
  },
  { immediate: true }
)

const enabledFunctionsCount = computed(() => autoFunctionStore.enabledFunctionsCount)
const isAnyFunctionEnabled = computed(() => autoFunctionStore.isAnyFunctionEnabled)
const enabledFunctions = computed(() => autoFunctionStore.enabledFunctions)

// 禁用所有功能
const handleDisableAll = async () => {
  console.log('Disabling all functions')
  autoFunctionStore.disableAllFunctions()
}

// 英雄选择处理
const handleChampionSelect = async (championInfo: ChampionInfo) => {
  console.log('Selecting champion:', championInfo)
  autoFunctionStore.setChampionSelect(championInfo)
}

const handleClearSelect = async () => {
  console.log('Clearing champion select')
  autoFunctionStore.clearChampionSelect()
}

// 英雄禁用处理
const handleChampionBan = async (championInfo: ChampionInfo) => {
  console.log('Banning champion:', championInfo)
  autoFunctionStore.setChampionBan(championInfo)
}

const handleClearBan = async () => {
  console.log('Clearing champion ban')
  autoFunctionStore.clearChampionBan()
}

// 调试信息
onMounted(() => {
  console.log('[AutoFunctionsView] Component mounted')
  console.log('[AutoFunctionsView] Initial store functions:', autoFunctions.value)
  console.log('[AutoFunctionsView] Initial enabled count:', enabledFunctionsCount.value)
})

console.log('[AutoFunctionsView] Component setup')
console.log('[AutoFunctionsView] Store functions at setup:', autoFunctions.value)
console.log('[AutoFunctionsView] Enabled count at setup:', enabledFunctionsCount.value)
</script>
