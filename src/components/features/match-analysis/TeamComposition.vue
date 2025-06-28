<template>
  <div class="space-y-4">
    <!-- 阵容类型分析 -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- 我方阵容 -->
      <div class="p-4 rounded-lg border border-blue-200 bg-blue-50 dark:bg-blue-950/20">
        <h4 class="font-semibold text-blue-800 dark:text-blue-400 mb-3">我方阵容</h4>
        <div class="space-y-2">
          <div v-for="comp in allyComposition" :key="comp.type" class="flex justify-between items-center">
            <span class="text-sm text-blue-700 dark:text-blue-300">{{ comp.type }}</span>
            <div class="flex items-center gap-2">
              <div class="w-12 h-2 bg-blue-200 dark:bg-blue-800 rounded-full overflow-hidden">
                <div class="h-full bg-blue-500 transition-all duration-300" :style="{ width: `${comp.score}%` }"></div>
              </div>
              <span class="text-xs font-medium text-blue-600 dark:text-blue-400 w-8">{{ comp.score }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 敌方阵容 -->
      <div class="p-4 rounded-lg border border-red-200 bg-red-50 dark:bg-red-950/20">
        <h4 class="font-semibold text-red-800 dark:text-red-400 mb-3">敌方阵容</h4>
        <div class="space-y-2">
          <div v-for="comp in enemyComposition" :key="comp.type" class="flex justify-between items-center">
            <span class="text-sm text-red-700 dark:text-red-300">{{ comp.type }}</span>
            <div class="flex items-center gap-2">
              <div class="w-12 h-2 bg-red-200 dark:bg-red-800 rounded-full overflow-hidden">
                <div class="h-full bg-red-500 transition-all duration-300" :style="{ width: `${comp.score}%` }"></div>
              </div>
              <span class="text-xs font-medium text-red-600 dark:text-red-400 w-8">{{ comp.score }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 阵容对比 -->
    <div class="space-y-3">
      <h4 class="text-sm font-semibold text-muted-foreground">阵容对比</h4>

      <div class="space-y-2">
        <div v-for="comparison in compositionComparison" :key="comparison.aspect">
          <div class="flex justify-between items-center mb-1">
            <span class="text-sm font-medium text-foreground">{{ comparison.aspect }}</span>
            <span
              class="text-xs font-semibold"
              :class="
                comparison.advantage === 'ally'
                  ? 'text-blue-600 dark:text-blue-400'
                  : comparison.advantage === 'enemy'
                    ? 'text-red-600 dark:text-red-400'
                    : 'text-muted-foreground'
              "
            >
              {{ getAdvantageText(comparison.advantage) }}
            </span>
          </div>

          <div class="relative h-3 bg-muted rounded-full overflow-hidden">
            <!-- 我方优势 -->
            <div
              v-if="comparison.allyScore > comparison.enemyScore"
              class="absolute left-1/2 top-0 h-full bg-blue-500 rounded-r-full transition-all duration-500"
              :style="{ width: `${(comparison.allyScore - comparison.enemyScore) / 2}%` }"
            ></div>

            <!-- 敌方优势 -->
            <div
              v-else-if="comparison.enemyScore > comparison.allyScore"
              class="absolute right-1/2 top-0 h-full bg-red-500 rounded-l-full transition-all duration-500"
              :style="{ width: `${(comparison.enemyScore - comparison.allyScore) / 2}%` }"
            ></div>

            <!-- 中心线 -->
            <div class="absolute left-1/2 top-0 w-0.5 h-full bg-border transform -translate-x-0.5"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- 战术建议 -->
    <div class="p-4 rounded-lg bg-muted/50">
      <h4 class="font-semibold mb-2 flex items-center gap-2">
        <Target class="h-4 w-4 text-primary" />
        战术建议
      </h4>

      <div class="space-y-2">
        <div v-for="tactic in tactics" :key="tactic.phase" class="text-sm">
          <span class="font-medium text-primary">{{ tactic.phase }}：</span>
          <span class="text-muted-foreground">{{ tactic.recommendation }}</span>
        </div>
      </div>
    </div>

    <!-- 关键时间点 -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
      <div v-for="timing in keyTimings" :key="timing.phase" class="text-center p-3 rounded-lg border">
        <div
          class="text-lg font-bold"
          :class="
            timing.advantage === 'ally'
              ? 'text-blue-600 dark:text-blue-400'
              : timing.advantage === 'enemy'
                ? 'text-red-600 dark:text-red-400'
                : 'text-muted-foreground'
          "
        >
          {{ timing.time }}
        </div>
        <div class="text-xs text-muted-foreground mb-1">{{ timing.phase }}</div>
        <div
          class="text-xs"
          :class="
            timing.advantage === 'ally'
              ? 'text-blue-600 dark:text-blue-400'
              : timing.advantage === 'enemy'
                ? 'text-red-600 dark:text-red-400'
                : 'text-muted-foreground'
          "
        >
          {{ timing.advantage === 'ally' ? '我方优势' : timing.advantage === 'enemy' ? '敌方优势' : '平衡' }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Target } from 'lucide-vue-next'

interface Props {
  session: any
}

const props = defineProps<Props>()

// 模拟数据 - 在实际应用中应该根据英雄数据计算
const allyComposition = computed(() => [
  { type: '爆发伤害', score: 75 },
  { type: '持续伤害', score: 60 },
  { type: '控制能力', score: 85 },
  { type: '坦度', score: 70 },
  { type: '机动性', score: 55 }
])

const enemyComposition = computed(() => [
  { type: '爆发伤害', score: 80 },
  { type: '持续伤害', score: 90 },
  { type: '控制能力', score: 65 },
  { type: '坦度', score: 85 },
  { type: '机动性', score: 70 }
])

const compositionComparison = computed(() => [
  {
    aspect: '前期战斗力',
    allyScore: 70,
    enemyScore: 65,
    advantage: 'ally' as 'ally' | 'enemy' | 'balanced'
  },
  {
    aspect: '中期团战',
    allyScore: 80,
    enemyScore: 75,
    advantage: 'ally' as 'ally' | 'enemy' | 'balanced'
  },
  {
    aspect: '后期输出',
    allyScore: 65,
    enemyScore: 85,
    advantage: 'enemy' as 'ally' | 'enemy' | 'balanced'
  },
  {
    aspect: 'Gank能力',
    allyScore: 75,
    enemyScore: 70,
    advantage: 'ally' as 'ally' | 'enemy' | 'balanced'
  },
  {
    aspect: '推塔速度',
    allyScore: 60,
    enemyScore: 80,
    advantage: 'enemy' as 'ally' | 'enemy' | 'balanced'
  }
])

const tactics = computed(() => [
  {
    phase: '前期 (0-15分钟)',
    recommendation: '利用控制优势进行主动Gank，建立经济领先'
  },
  {
    phase: '中期 (15-25分钟)',
    recommendation: '集结团战，发挥阵容控制链优势'
  },
  {
    phase: '后期 (25分钟+)',
    recommendation: '避免拖延，速战速决，防止敌方后期发力'
  }
])

const keyTimings = computed(() => [
  {
    phase: '前期',
    time: '0-15分',
    advantage: 'ally' as 'ally' | 'enemy' | 'balanced'
  },
  {
    phase: '中期',
    time: '15-25分',
    advantage: 'ally' as 'ally' | 'enemy' | 'balanced'
  },
  {
    phase: '后期',
    time: '25分+',
    advantage: 'enemy' as 'ally' | 'enemy' | 'balanced'
  }
])

const getAdvantageText = (advantage: 'ally' | 'enemy' | 'balanced') => {
  switch (advantage) {
    case 'ally':
      return '我方优势'
    case 'enemy':
      return '敌方优势'
    default:
      return '平衡'
  }
}
</script>
