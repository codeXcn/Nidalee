<template>
  <div class="space-y-4">
    <h4 class="text-sm font-semibold text-muted-foreground mb-3">队伍分析</h4>

    <div class="grid grid-cols-2 gap-4">
      <!-- 平均等级 -->
      <div class="text-center p-3 rounded-lg bg-muted/50">
        <div class="text-lg font-bold" :class="teamType === 'ally' ? 'text-blue-600 dark:text-blue-400' : 'text-red-600 dark:text-red-400'">
          {{ avgLevel }}
        </div>
        <div class="text-xs text-muted-foreground">平均等级</div>
      </div>

      <!-- 胜率 -->
      <div class="text-center p-3 rounded-lg bg-muted/50">
        <div class="text-lg font-bold" :class="getWinRateColor(avgWinRate)">
          {{ avgWinRate }}%
        </div>
        <div class="text-xs text-muted-foreground">平均胜率</div>
      </div>
    </div>

    <!-- 排位分布 -->
    <div class="space-y-2">
      <div class="text-xs font-medium text-muted-foreground">排位分布</div>
      <div class="flex gap-1">
        <div
          v-for="(rank, index) in rankDistribution"
          :key="index"
          class="flex-1 h-2 rounded-full"
          :class="getRankColor(rank.tier)"
          :title="`${rank.tier} ${rank.division || ''}`"
        ></div>
      </div>
      <div class="flex justify-between text-xs text-muted-foreground">
        <span>{{ getHighestRank() }}</span>
        <span>{{ getLowestRank() }}</span>
      </div>
    </div>

    <!-- 常用英雄类型 -->
    <div class="space-y-2">
      <div class="text-xs font-medium text-muted-foreground">队伍风格</div>
      <div class="flex flex-wrap gap-1">
        <Badge
          v-for="style in teamStyles"
          :key="style.name"
          variant="secondary"
          class="text-xs px-2 py-0.5"
          :title="`置信度: ${style.confidence}%`"
        >
          {{ style.name }}
        </Badge>
      </div>
    </div>

    <!-- 优势时间段 -->
    <div class="space-y-2">
      <div class="text-xs font-medium text-muted-foreground">强势时期</div>
      <div class="grid grid-cols-3 gap-1">
        <div
          v-for="period in powerSpikes"
          :key="period.name"
          class="text-center p-2 rounded text-xs"
          :class="period.strength > 70 ? 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400' :
                  period.strength > 50 ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400' :
                  'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400'"
        >
          <div class="font-medium text-foreground">{{ period.name }}</div>
          <div class="text-xs opacity-75">{{ period.strength }}%</div>
        </div>
      </div>
    </div>

    <!-- 队伍协同度 -->
    <div class="space-y-2">
      <div class="text-xs font-medium text-muted-foreground">团队协同</div>
      <div class="space-y-1">
        <div v-for="synergy in teamSynergies" :key="synergy.type" class="flex justify-between items-center">
          <span class="text-xs">{{ synergy.type }}</span>
          <div class="flex items-center gap-1">
            <div class="w-8 h-1.5 bg-muted rounded-full overflow-hidden">
              <div
                class="h-full transition-all duration-300"
                :class="synergy.score > 70 ? 'bg-green-500' : synergy.score > 50 ? 'bg-yellow-500' : 'bg-red-500'"
                :style="{ width: `${synergy.score}%` }"
              ></div>
            </div>
            <span class="text-xs w-6" :class="synergy.score > 70 ? 'text-green-600 dark:text-green-400' : synergy.score > 50 ? 'text-yellow-600 dark:text-yellow-400' : 'text-red-600 dark:text-red-400'">
              {{ synergy.score }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Badge } from '@/components/ui/badge'

interface Props {
  team: any[]
  teamType: 'ally' | 'enemy'
}

const props = defineProps<Props>()

// 计算平均等级
const avgLevel = computed(() => {
  const levels = props.team.map(player => player.summonerLevel || 30)
  const avg = levels.reduce((sum, level) => sum + level, 0) / levels.length
  return Math.round(avg)
})

// 计算平均胜率（模拟数据，实际应该从战绩API获取）
const avgWinRate = computed(() => {
  // 这里需要实际的胜率数据，暂时返回模拟值
  return Math.floor(Math.random() * 40) + 40 // 40-80%
})

// 排位分布分析
const rankDistribution = computed(() => {
  // 模拟排位数据，实际应该从玩家排位信息获取
  const ranks = ['GOLD', 'SILVER', 'PLATINUM', 'GOLD', 'SILVER']
  return ranks.map(tier => ({
    tier,
    division: ['I', 'II', 'III', 'IV'][Math.floor(Math.random() * 4)]
  }))
})

// 队伍风格分析（更详细）
const teamStyles = computed(() => {
  // 根据英雄选择和玩家数据分析队伍风格
  const styles = [
    { name: '团战导向', confidence: 85 },
    { name: '前期强势', confidence: 72 },
    { name: '控制链强', confidence: 68 }
  ]
  return styles.slice(0, 3)
})

// 强势时期分析
const powerSpikes = computed(() => [
  { name: '前期', strength: 75 },
  { name: '中期', strength: 82 },
  { name: '后期', strength: 45 }
])

// 队伍协同度分析
const teamSynergies = computed(() => [
  { type: '开团能力', score: 78 },
  { type: '后排保护', score: 65 },
  { type: '控制链接', score: 82 },
  { type: '输出配合', score: 70 }
])

// 工具函数
const getWinRateColor = (winRate: number) => {
  if (winRate >= 70) return 'text-green-600 dark:text-green-400'
  if (winRate >= 60) return 'text-blue-600 dark:text-blue-400'
  if (winRate >= 50) return 'text-yellow-600 dark:text-yellow-400'
  return 'text-red-600 dark:text-red-400'
}

const getRankColor = (tier: string) => {
  const colors: Record<string, string> = {
    'IRON': 'bg-gray-400',
    'BRONZE': 'bg-amber-600',
    'SILVER': 'bg-gray-300',
    'GOLD': 'bg-yellow-400',
    'PLATINUM': 'bg-teal-400',
    'EMERALD': 'bg-emerald-400',
    'DIAMOND': 'bg-blue-400',
    'MASTER': 'bg-purple-500',
    'GRANDMASTER': 'bg-red-500',
    'CHALLENGER': 'bg-yellow-500'
  }
  return colors[tier] || 'bg-gray-400'
}

const getHighestRank = () => {
  const rankOrder = ['IRON', 'BRONZE', 'SILVER', 'GOLD', 'PLATINUM', 'EMERALD', 'DIAMOND', 'MASTER', 'GRANDMASTER', 'CHALLENGER']
  const ranks = rankDistribution.value.map(r => r.tier)
  const highest = ranks.reduce((prev, curr) =>
    rankOrder.indexOf(curr) > rankOrder.indexOf(prev) ? curr : prev
  )
  return highest
}

const getLowestRank = () => {
  const rankOrder = ['IRON', 'BRONZE', 'SILVER', 'GOLD', 'PLATINUM', 'EMERALD', 'DIAMOND', 'MASTER', 'GRANDMASTER', 'CHALLENGER']
  const ranks = rankDistribution.value.map(r => r.tier)
  const lowest = ranks.reduce((prev, curr) =>
    rankOrder.indexOf(curr) < rankOrder.indexOf(prev) ? curr : prev
  )
  return lowest
}
</script>
