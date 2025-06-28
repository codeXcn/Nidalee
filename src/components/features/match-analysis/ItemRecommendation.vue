<template>
  <Card class="p-4">
    <h3 class="text-lg font-semibold mb-4 flex items-center gap-2 text-foreground">
      <Package class="h-5 w-5 text-amber-500 dark:text-amber-400" />
      装备推荐
    </h3>

    <div v-if="!staticDataLoaded" class="text-center py-4">
      <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary mx-auto mb-2"></div>
      <p class="text-sm text-muted-foreground">加载装备数据中...</p>
    </div>

    <div v-else class="space-y-4">
      <!-- 当前游戏阶段 -->
      <div class="bg-muted/50 rounded-lg p-3">
        <div class="flex items-center justify-between mb-2">
          <h4 class="text-sm font-medium text-foreground">游戏阶段建议</h4>
          <Badge variant="outline" class="text-xs">
            {{ currentGamePhase === 'early' ? '前期' : currentGamePhase === 'mid' ? '中期' : '后期' }}
          </Badge>
        </div>
        <p class="text-xs text-muted-foreground">
          {{ getGamePhaseAdvice() }}
        </p>
      </div>

      <!-- 针对敌方阵容的装备建议 -->
      <div class="space-y-3">
        <h4 class="text-sm font-medium text-foreground">反制装备推荐</h4>

        <!-- 物理防御装备 -->
        <div v-if="enemyPhysicalThreat > 2" class="bg-blue-50 dark:bg-blue-950/30 rounded-lg p-3">
          <div class="flex items-center gap-2 mb-2">
            <Shield class="h-4 w-4 text-blue-600 dark:text-blue-400" />
            <span class="text-sm font-medium text-blue-600 dark:text-blue-400">物理防御装备</span>
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <div
              v-for="item in armorItems.slice(0, 3)"
              :key="item.name"
              class="flex items-center gap-2 p-2 bg-white dark:bg-gray-800 rounded"
            >
              <div class="w-6 h-6 bg-blue-200 dark:bg-blue-800 rounded"></div>
              <span class="text-xs">{{ item.name }}</span>
            </div>
          </div>
          <p class="text-xs text-muted-foreground mt-2">
            敌方有 {{ enemyPhysicalThreat }} 个物理输出英雄，建议优先出护甲装备
          </p>
        </div>

        <!-- 魔法防御装备 -->
        <div v-if="enemyMagicalThreat > 2" class="bg-purple-50 dark:bg-purple-950/30 rounded-lg p-3">
          <div class="flex items-center gap-2 mb-2">
            <Sparkles class="h-4 w-4 text-purple-600 dark:text-purple-400" />
            <span class="text-sm font-medium text-purple-600 dark:text-purple-400">魔法防御装备</span>
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <div
              v-for="item in mrItems.slice(0, 3)"
              :key="item.name"
              class="flex items-center gap-2 p-2 bg-white dark:bg-gray-800 rounded"
            >
              <div class="w-6 h-6 bg-purple-200 dark:bg-purple-800 rounded"></div>
              <span class="text-xs">{{ item.name }}</span>
            </div>
          </div>
          <p class="text-xs text-muted-foreground mt-2">
            敌方有 {{ enemyMagicalThreat }} 个魔法输出英雄，建议出魔抗装备
          </p>
        </div>

        <!-- 通用核心装备 -->
        <div class="bg-amber-50 dark:bg-amber-950/30 rounded-lg p-3">
          <div class="flex items-center gap-2 mb-2">
            <Star class="h-4 w-4 text-amber-600 dark:text-amber-400" />
            <span class="text-sm font-medium text-amber-600 dark:text-amber-400">推荐核心装备</span>
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <div
              v-for="item in coreItems.slice(0, 6)"
              :key="item.name"
              class="flex items-center gap-2 p-2 bg-white dark:bg-gray-800 rounded"
            >
              <div class="w-6 h-6 bg-amber-200 dark:bg-amber-800 rounded"></div>
              <div class="flex-1 min-w-0">
                <p class="text-xs font-medium truncate">{{ item.name }}</p>
                <p class="text-xs text-muted-foreground">{{ item.gold?.total || 0 }}金币</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 经济建议 -->
      <div class="bg-green-50 dark:bg-green-950/30 rounded-lg p-3">
        <div class="flex items-center gap-2 mb-2">
          <DollarSign class="h-4 w-4 text-green-600 dark:text-green-400" />
          <span class="text-sm font-medium text-green-600 dark:text-green-400">经济建议</span>
        </div>
        <div class="space-y-1">
          <p class="text-xs text-muted-foreground">• 前期优先出小件增强对线能力</p>
          <p class="text-xs text-muted-foreground">• 中期尽快成型核心装备参与团战</p>
          <p class="text-xs text-muted-foreground">• 后期根据局势选择攻击或防御装备</p>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { Package, Shield, Sparkles, Star, DollarSign } from 'lucide-vue-next'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useStaticData } from '@/composables/utils/useStaticData'

interface Props {
  session: ChampSelectSession
}

const props = defineProps<Props>()

const { isLoaded: staticDataLoaded, loadStaticData, getRecommendedItems } = useStaticData()

// 当前游戏阶段（可以根据游戏时间或其他因素判断）
const currentGamePhase = computed(() => 'early' as 'early' | 'mid' | 'late')

// 敌方威胁分析
const enemyTeam = computed(() => props.session?.theirTeam || [])

const enemyPhysicalThreat = computed(() => {
  // 分析敌方物理输出威胁（这里使用模拟逻辑）
  return enemyTeam.value.filter((player) => {
    // 这里应该根据英雄类型判断是否为物理输出
    // 暂时使用随机逻辑作为示例
    return Math.random() > 0.5
  }).length
})

const enemyMagicalThreat = computed(() => {
  // 分析敌方魔法输出威胁
  return enemyTeam.value.filter((player) => {
    // 这里应该根据英雄类型判断是否为魔法输出
    return Math.random() > 0.5
  }).length
})

// 装备推荐
const armorItems = computed(() => [
  { name: '布甲鞋', gold: { total: 1100 } },
  { name: '荆棘背心', gold: { total: 800 } },
  { name: '冰霜之心', gold: { total: 2500 } }
])

const mrItems = computed(() => [
  { name: '水银之靴', gold: { total: 1100 } },
  { name: '魔抗斗篷', gold: { total: 450 } },
  { name: '振奋盔甲', gold: { total: 2800 } }
])

const coreItems = computed(() => {
  // 获取通用核心装备（这里使用示例数据）
  return [
    { name: '无尽之刃', gold: { total: 3400 } },
    { name: '狂战士胫甲', gold: { total: 1100 } },
    { name: '幻影之舞', gold: { total: 2600 } },
    { name: '守护天使', gold: { total: 2800 } },
    { name: '破败王者之刃', gold: { total: 3200 } },
    { name: '渴血战斧', gold: { total: 3300 } }
  ]
})

// 游戏阶段建议
const getGamePhaseAdvice = () => {
  switch (currentGamePhase.value) {
    case 'early':
      return '前期专注对线发育，优先购买基础装备和消耗品'
    case 'mid':
      return '中期是团战关键期，确保核心装备已成型'
    case 'late':
      return '后期团战决定胜负，选择合适的输出或坦克装备'
    default:
      return '根据游戏进程调整装备选择'
  }
}

onMounted(() => {
  if (!staticDataLoaded.value) {
    loadStaticData()
  }
})
</script>
