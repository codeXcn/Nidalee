<template>
  <Card class="p-4">
    <h3 class="text-lg font-semibold mb-4 flex items-center gap-2 text-foreground">
      <Zap class="h-5 w-5 text-purple-500 dark:text-purple-400" />
      符文分析
    </h3>

    <div v-if="!staticDataLoaded" class="text-center py-4">
      <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary mx-auto mb-2"></div>
      <p class="text-sm text-muted-foreground">加载符文数据中...</p>
    </div>

    <div v-else class="space-y-4">
      <!-- 我方符文分析 -->
      <div class="space-y-2">
        <h4 class="text-sm font-medium text-blue-600 dark:text-blue-400">我方符文搭配</h4>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
          <div
            v-for="player in allyTeam"
            :key="player.cellId"
            class="flex items-center gap-2 p-2 rounded-md bg-blue-50 dark:bg-blue-950/30"
          >
            <div class="w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center">
              <span class="text-xs font-bold text-blue-600 dark:text-blue-400">
                {{ player.displayName?.charAt(0)?.toUpperCase() || '?' }}
              </span>
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-xs font-medium truncate">{{ player.displayName || '未知' }}</p>
              <div class="flex items-center gap-1">
                <div v-if="getPlayerMainRune(player)" class="flex items-center gap-1">
                  <div class="w-4 h-4 bg-purple-500 rounded-sm"></div>
                  <span class="text-xs text-muted-foreground">
                    {{ getPlayerMainRune(player)?.name || '未知符文' }}
                  </span>
                </div>
                <span v-else class="text-xs text-muted-foreground">未选择符文</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 敌方符文分析 -->
      <div class="space-y-2">
        <h4 class="text-sm font-medium text-red-600 dark:text-red-400">敌方符文搭配</h4>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
          <div
            v-for="player in enemyTeam"
            :key="player.cellId"
            class="flex items-center gap-2 p-2 rounded-md bg-red-50 dark:bg-red-950/30"
          >
            <div class="w-8 h-8 rounded-full bg-red-100 dark:bg-red-900 flex items-center justify-center">
              <span class="text-xs font-bold text-red-600 dark:text-red-400">
                {{ player.displayName?.charAt(0)?.toUpperCase() || '?' }}
              </span>
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-xs font-medium truncate">{{ player.displayName || '未知' }}</p>
              <div class="flex items-center gap-1">
                <div v-if="getPlayerMainRune(player)" class="flex items-center gap-1">
                  <div class="w-4 h-4 bg-purple-500 rounded-sm"></div>
                  <span class="text-xs text-muted-foreground">
                    {{ getPlayerMainRune(player)?.name || '未知符文' }}
                  </span>
                </div>
                <span v-else class="text-xs text-muted-foreground">未选择符文</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 符文趋势分析 -->
      <div class="bg-muted/50 rounded-lg p-3 space-y-2">
        <h5 class="text-sm font-medium text-foreground">符文趋势分析</h5>
        <div class="grid grid-cols-2 gap-4">
          <div class="text-center">
            <p class="text-lg font-bold text-blue-600 dark:text-blue-400">{{ allyRuneDistribution.domination }}</p>
            <p class="text-xs text-muted-foreground">我方主宰系</p>
          </div>
          <div class="text-center">
            <p class="text-lg font-bold text-red-600 dark:text-red-400">{{ enemyRuneDistribution.domination }}</p>
            <p class="text-xs text-muted-foreground">敌方主宰系</p>
          </div>
        </div>
        <div class="text-center">
          <p class="text-xs text-muted-foreground">
            {{ getRuneAdvice() }}
          </p>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { Zap } from 'lucide-vue-next'
import { Card } from '@/components/ui/card'
import { useStaticData } from '@/composables/utils/useStaticData'

interface Props {
  session: ChampSelectSession
}

const props = defineProps<Props>()

const {
  isLoaded: staticDataLoaded,
  loadStaticData,
  getRuneById,
  getRecommendedRunes
} = useStaticData()

// 队伍数据
const allyTeam = computed(() => props.session?.myTeam || [])
const enemyTeam = computed(() => props.session?.theirTeam || [])

// 获取玩家主符文（模拟数据，实际需要从API获取）
const getPlayerMainRune = (player: any) => {
  // 这里应该从玩家数据中获取实际的符文ID
  // 现在使用模拟数据
  const mockRuneId = 8112 // 电刑
  return getRuneById(mockRuneId)
}

// 符文分布统计
const allyRuneDistribution = computed(() => {
  return {
    domination: allyTeam.value.filter(player => {
      const rune = getPlayerMainRune(player)
      return rune?.tree === '主宰'
    }).length
  }
})

const enemyRuneDistribution = computed(() => {
  return {
    domination: enemyTeam.value.filter(player => {
      const rune = getPlayerMainRune(player)
      return rune?.tree === '主宰'
    }).length
  }
})

// 符文建议
const getRuneAdvice = () => {
  const allyDom = allyRuneDistribution.value.domination
  const enemyDom = enemyRuneDistribution.value.domination

  if (allyDom > enemyDom) {
    return '我方爆发能力更强，适合主动发起团战'
  } else if (enemyDom > allyDom) {
    return '敌方爆发能力更强，注意避免被先手'
  } else {
    return '双方爆发能力相当，注意团战站位'
  }
}

onMounted(() => {
  if (!staticDataLoaded.value) {
    loadStaticData()
  }
})
</script>
