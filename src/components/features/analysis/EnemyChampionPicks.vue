<template>
  <div class="p-4">
    <div class="mb-4">
      <h3 class="text-lg font-semibold text-foreground mb-2">敌方英雄选择</h3>
      <p class="text-sm text-muted-foreground">选人阶段 - 实时更新敌方英雄选择情况</p>
    </div>

    <div class="space-y-3">
      <div
        v-for="(pick, index) in championPicks"
        :key="`enemy-pick-${pick.cellId}-${index}`"
        class="flex items-center p-3 rounded-lg border border-border/50 bg-card/30 hover:bg-card/50 transition-colors"
      >
        <!-- 位置指示器 -->
        <div
          class="flex items-center justify-center w-8 h-8 rounded-full bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 text-sm font-bold mr-3"
        >
          {{ index + 1 }}
        </div>

        <!-- 英雄头像 -->
        <div class="w-12 h-12 rounded-full overflow-hidden border-2 border-border mr-3 flex-shrink-0 relative">
          <!-- 已选择的英雄 -->
          <img
            v-if="pick.championId"
            :src="getChampionIconUrl(pick.championId)"
            :alt="getChampionName(pick.championId)"
            class="w-full h-full object-cover"
          />
          <!-- 预选英雄（半透明显示） -->
          <img
            v-else-if="pick.championPickIntent"
            :src="getChampionIconUrl(pick.championPickIntent)"
            :alt="getChampionName(pick.championPickIntent)"
            class="w-full h-full object-cover opacity-50"
            :title="`预选: ${getChampionName(pick.championPickIntent)}`"
          />
          <!-- 未选择 -->
          <div v-else class="w-full h-full bg-muted flex items-center justify-center text-muted-foreground">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </div>
          <!-- 预选指示器 -->
          <div
            v-if="!pick.championId && pick.championPickIntent"
            class="absolute inset-0 flex items-center justify-center bg-black/40"
          >
            <span class="text-xs text-white font-bold">预选</span>
          </div>
        </div>

        <!-- 英雄信息 -->
        <div class="flex-1">
          <div class="font-medium text-foreground">
            {{
              pick.championId
                ? getChampionName(pick.championId)
                : pick.championPickIntent
                  ? `预选: ${getChampionName(pick.championPickIntent)}`
                  : '未选择'
            }}
          </div>
          <div class="text-sm text-muted-foreground">
            {{ getPositionName(index) }}
          </div>
        </div>

        <!-- 状态指示器 -->
        <div class="flex items-center gap-2">
          <div
            v-if="pick.championId"
            class="px-2 py-1 rounded-full text-xs font-medium bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400"
          >
            已选择
          </div>
          <div
            v-else-if="pick.championPickIntent"
            class="px-2 py-1 rounded-full text-xs font-medium bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 animate-pulse"
          >
            预选中
          </div>
          <div
            v-else
            class="px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-400 animate-pulse"
          >
            选择中
          </div>
        </div>
      </div>
    </div>

    <!-- 选择进度 -->
    <div class="mt-6 p-3 rounded-lg bg-muted/30">
      <div class="flex justify-between text-sm text-muted-foreground mb-2">
        <span>选择进度</span>
        <span>{{ selectedCount }}/{{ championPicks.length }}</span>
      </div>
      <div class="w-full bg-muted rounded-full h-2">
        <div
          class="h-full bg-gradient-to-r from-blue-500 to-purple-500 rounded-full transition-all duration-300"
          :style="{ width: `${progressPercent}%` }"
        />
      </div>
    </div>

    <!-- 提示信息 -->
    <div class="mt-4 text-center text-xs text-muted-foreground">
      <p>💡 敌方完整信息将在游戏开始后获取</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { getChampionIconUrl, getChampionName } from '@/lib'

interface EnemyChampionPick {
  cellId: number
  championId: number | null
  championPickIntent?: number | null // 预选英雄
}

interface Props {
  championPicks: EnemyChampionPick[]
}

const props = defineProps<Props>()

// 位置名称映射
const getPositionName = (index: number): string => {
  const positions = ['上单', '打野', '中单', 'ADC', '辅助']
  return positions[index] || `位置${index + 1}`
}

// 计算选择进度
const selectedCount = computed(() => {
  return props.championPicks.filter((pick) => pick.championId !== null).length
})

const progressPercent = computed(() => {
  if (props.championPicks.length === 0) return 0
  return (selectedCount.value / props.championPicks.length) * 100
})
</script>
