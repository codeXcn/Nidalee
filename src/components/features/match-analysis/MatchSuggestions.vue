<template>
  <div class="space-y-4">
    <!-- 核心建议 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <!-- 早期游戏建议 -->
      <div class="p-4 rounded-lg border border-green-200 bg-green-50 dark:bg-green-950/20 dark:border-green-800">
        <div class="flex items-center gap-2 mb-3">
          <Clock class="h-4 w-4 text-green-600 dark:text-green-400" />
          <h4 class="font-semibold text-green-800 dark:text-green-400">前期策略</h4>
        </div>
        <ul class="space-y-1 text-sm text-green-700 dark:text-green-300">
          <li v-for="suggestion in earlySuggestions" :key="suggestion" class="flex items-start gap-2">
            <ArrowRight class="h-3 w-3 mt-0.5 flex-shrink-0" />
            <span>{{ suggestion }}</span>
          </li>
        </ul>
      </div>

      <!-- 团战建议 -->
      <div class="p-4 rounded-lg border border-blue-200 bg-blue-50 dark:bg-blue-950/20 dark:border-blue-800">
        <div class="flex items-center gap-2 mb-3">
          <Users class="h-4 w-4 text-blue-600 dark:text-blue-400" />
          <h4 class="font-semibold text-blue-800 dark:text-blue-400">团战要点</h4>
        </div>
        <ul class="space-y-1 text-sm text-blue-700 dark:text-blue-300">
          <li v-for="suggestion in teamfightSuggestions" :key="suggestion" class="flex items-start gap-2">
            <ArrowRight class="h-3 w-3 mt-0.5 flex-shrink-0" />
            <span>{{ suggestion }}</span>
          </li>
        </ul>
      </div>

      <!-- 后期建议 -->
      <div class="p-4 rounded-lg border border-purple-200 bg-purple-50 dark:bg-purple-950/20 dark:border-purple-800">
        <div class="flex items-center gap-2 mb-3">
          <Target class="h-4 w-4 text-purple-600 dark:text-purple-400" />
          <h4 class="font-semibold text-purple-800 dark:text-purple-400">后期重点</h4>
        </div>
        <ul class="space-y-1 text-sm text-purple-700 dark:text-purple-300">
          <li v-for="suggestion in lateSuggestions" :key="suggestion" class="flex items-start gap-2">
            <ArrowRight class="h-3 w-3 mt-0.5 flex-shrink-0" />
            <span>{{ suggestion }}</span>
          </li>
        </ul>
      </div>
    </div>

    <!-- 威胁分析 -->
    <div class="mt-6">
      <h4 class="text-sm font-semibold text-muted-foreground mb-3">重点关注</h4>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- 威胁目标 -->
        <div class="p-3 rounded-lg border border-red-200 bg-red-50 dark:bg-red-950/20 dark:border-red-800">
          <div class="flex items-center gap-2 mb-2">
            <AlertTriangle class="h-4 w-4 text-red-600 dark:text-red-400" />
            <span class="text-sm font-medium text-red-800 dark:text-red-400">高威胁目标</span>
          </div>
          <div class="flex flex-wrap gap-2">
            <Badge v-for="threat in threats" :key="threat" variant="destructive" class="text-xs">
              {{ threat }}
            </Badge>
          </div>
        </div>

        <!-- 机会目标 -->
        <div class="p-3 rounded-lg border border-green-200 bg-green-50 dark:bg-green-950/20 dark:border-green-800">
          <div class="flex items-center gap-2 mb-2">
            <TrendingUp class="h-4 w-4 text-green-600 dark:text-green-400" />
            <span class="text-sm font-medium text-green-800 dark:text-green-400">机会目标</span>
          </div>
          <div class="flex flex-wrap gap-2">
            <Badge
              v-for="opportunity in opportunities"
              :key="opportunity"
              class="text-xs bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100"
            >
              {{ opportunity }}
            </Badge>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Clock, Users, Target, ArrowRight, AlertTriangle, TrendingUp } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'

interface Props {
  session: any
}

const props = defineProps<Props>()

// 前期建议
const earlySuggestions = computed(() => {
  return ['优先抢占河道视野', '保护己方打野发育', '寻找Gank机会', '控制小龙视野']
})

// 团战建议
const teamfightSuggestions = computed(() => {
  return ['优先击杀敌方C位', '保护己方核心输出', '合理分配技能释放', '控制团战节奏']
})

// 后期建议
const lateSuggestions = computed(() => {
  return ['控制大龙和远古龙', '避免单独行动', '集体推进压力', '寻找决胜团战']
})

// 威胁分析
const threats = computed(() => {
  // 分析敌方威胁程度高的角色
  const enemyTeam = props.session?.theirTeam || []
  return enemyTeam.slice(0, 2).map((player) => player.summonerName || `玩家${player.cellId}`)
})

// 机会分析
const opportunities = computed(() => {
  // 分析可以针对的目标
  const enemyTeam = props.session?.theirTeam || []
  return enemyTeam.slice(-2).map((player) => player.summonerName || `玩家${player.cellId}`)
})
</script>
