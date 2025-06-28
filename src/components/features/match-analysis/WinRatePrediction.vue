<template>
  <div class="space-y-4">
    <!-- 总体胜率预测 -->
    <div class="text-center p-6 rounded-lg bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20">
      <div class="flex items-center justify-center gap-3 mb-2">
        <TrendingUp v-if="overallWinRate >= 50" class="h-6 w-6 text-green-500 dark:text-green-400" />
        <TrendingDown v-else class="h-6 w-6 text-red-500 dark:text-red-400" />
        <span
          class="text-3xl font-bold"
          :class="overallWinRate >= 50 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'"
        >
          {{ overallWinRate }}%
        </span>
      </div>

      <div class="text-sm text-muted-foreground mb-4">预测胜率</div>

      <div class="w-full bg-muted rounded-full h-3 overflow-hidden">
        <div
          class="h-full transition-all duration-1000 ease-out"
          :class="overallWinRate >= 50 ? 'bg-green-500' : 'bg-red-500'"
          :style="{ width: `${overallWinRate}%` }"
        ></div>
      </div>

      <div class="text-xs text-muted-foreground mt-2">基于队伍实力、阵容匹配和历史数据分析</div>
    </div>

    <!-- 胜率因素分析 -->
    <div class="space-y-3">
      <h4 class="text-sm font-semibold text-muted-foreground">影响因素</h4>

      <div class="space-y-2">
        <div
          v-for="factor in winRateFactors"
          :key="factor.name"
          class="flex items-center justify-between p-3 rounded-lg border"
        >
          <div class="flex items-center gap-3">
            <component
              :is="factor.icon"
              class="h-4 w-4"
              :class="
                factor.impact > 0
                  ? 'text-green-500 dark:text-green-400'
                  : factor.impact < 0
                    ? 'text-red-500 dark:text-red-400'
                    : 'text-muted-foreground'
              "
            />
            <span class="text-sm font-medium text-foreground">{{ factor.name }}</span>
          </div>

          <div class="flex items-center gap-2">
            <div
              class="text-sm"
              :class="
                factor.impact > 0
                  ? 'text-green-600 dark:text-green-400'
                  : factor.impact < 0
                    ? 'text-red-600 dark:text-red-400'
                    : 'text-muted-foreground'
              "
            >
              {{ factor.impact > 0 ? '+' : '' }}{{ factor.impact }}%
            </div>

            <div class="w-12 h-2 bg-muted rounded-full overflow-hidden">
              <div
                class="h-full transition-all duration-500"
                :class="factor.impact > 0 ? 'bg-green-500' : factor.impact < 0 ? 'bg-red-500' : 'bg-muted-foreground'"
                :style="{ width: `${Math.abs(factor.impact) * 2}%` }"
              ></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 时间段胜率 -->
    <div class="space-y-3">
      <h4 class="text-sm font-semibold text-muted-foreground">分时段胜率</h4>

      <div class="grid grid-cols-3 gap-3">
        <div v-for="period in timePeriods" :key="period.name" class="text-center p-3 rounded-lg border">
          <div
            class="text-lg font-bold mb-1"
            :class="period.winRate >= 50 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'"
          >
            {{ period.winRate }}%
          </div>
          <div class="text-xs text-muted-foreground mb-2">{{ period.name }}</div>
          <div class="w-full bg-muted rounded-full h-1.5 overflow-hidden">
            <div
              class="h-full transition-all duration-700"
              :class="period.winRate >= 50 ? 'bg-green-500' : 'bg-red-500'"
              :style="{ width: `${period.winRate}%` }"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <!-- 关键建议 -->
    <div class="p-4 rounded-lg bg-muted/50">
      <h4 class="font-semibold mb-2 flex items-center gap-2">
        <Lightbulb class="h-4 w-4 text-yellow-500 dark:text-yellow-400" />
        提升胜率建议
      </h4>

      <ul class="space-y-1 text-sm text-muted-foreground">
        <li v-for="suggestion in suggestions" :key="suggestion" class="flex items-start gap-2">
          <ArrowRight class="h-3 w-3 mt-0.5 flex-shrink-0 text-primary" />
          <span>{{ suggestion }}</span>
        </li>
      </ul>
    </div>

    <!-- 历史对战记录 -->
    <div class="space-y-3">
      <h4 class="text-sm font-semibold text-muted-foreground">相似阵容记录</h4>

      <div class="grid grid-cols-2 gap-4">
        <div class="p-3 rounded-lg border">
          <div class="text-center">
            <div class="text-lg font-bold text-green-600 dark:text-green-400">{{ historicalData.wins }}</div>
            <div class="text-xs text-muted-foreground">胜利场次</div>
          </div>
        </div>

        <div class="p-3 rounded-lg border">
          <div class="text-center">
            <div class="text-lg font-bold text-red-600 dark:text-red-400">{{ historicalData.losses }}</div>
            <div class="text-xs text-muted-foreground">失败场次</div>
          </div>
        </div>
      </div>

      <div class="text-xs text-center text-muted-foreground">
        基于过去30天相似阵容数据 (样本数: {{ historicalData.total }})
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { TrendingUp, TrendingDown, Users, Shield, Zap, Target, Star, Lightbulb, ArrowRight } from 'lucide-vue-next'

interface Props {
  session: any
}

const props = defineProps<Props>()

// 模拟数据 - 实际应用中需要根据真实数据计算
const overallWinRate = computed(() => 63)

const winRateFactors = computed(() => [
  {
    name: '队伍实力',
    impact: 8,
    icon: Users
  },
  {
    name: '阵容匹配',
    impact: 12,
    icon: Shield
  },
  {
    name: '英雄熟练度',
    impact: -3,
    icon: Star
  },
  {
    name: '版本适应',
    impact: 5,
    icon: Target
  },
  {
    name: '位置匹配',
    impact: -2,
    icon: Zap
  }
])

const timePeriods = computed(() => [
  {
    name: '前期',
    winRate: 68
  },
  {
    name: '中期',
    winRate: 71
  },
  {
    name: '后期',
    winRate: 45
  }
])

const suggestions = computed(() => [
  '发挥前中期优势，避免拖到后期',
  '重点保护核心输出位，确保团战发挥',
  '利用视野控制争夺关键资源',
  '抓住敌方失误时机，建立优势',
  '合理分推，避免无意义团战'
])

const historicalData = computed(() => ({
  wins: 127,
  losses: 83,
  total: 210
}))
</script>
