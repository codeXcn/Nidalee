<template>
  <Card class="border-none px-1 py-0">
    <!-- 使用Card展示每个召唤师 -->
    <div class="space-y-1">
      <div
        v-for="(player, index) in teamData.players"
        :key="(player.displayName || player.summonerId || player.puuid || index) + '-' + index"
        class="w-full bg-transparent"
      >
        <Card
          class="px-1 py-2 border group cursor-pointer p-0"
          :class="!isQueryable(player) ? 'opacity-60 grayscale' : ''"
          @click="$emit('select-player', player, getPlayerStats(index))"
        >
          <CardContent class="p-1">
            <div class="flex items-center gap-3">
              <!-- 英雄头像 -->
              <div class="relative flex-shrink-0">
                <div class="w-12 h-12 rounded-lg overflow-hidden border border-border">
                  <img
                    v-if="player.championId"
                    :src="getChampionIconUrl(player.championId)"
                    :alt="getChampionName(player.championId)"
                    class="w-full h-full object-cover"
                  />
                  <div v-else class="w-full h-full bg-muted flex items-center justify-center">
                    <div class="w-6 h-6 bg-muted-foreground/20 rounded" />
                  </div>
                </div>

                <!-- 召唤师技能 -->
                <div class="absolute -bottom-1 -right-1 flex gap-0.5">
                  <div
                    v-for="spellId in [player.spell1Id, player.spell2Id]"
                    :key="spellId"
                    class="w-4 h-4 rounded border border-border/60 bg-background overflow-hidden"
                  >
                    <img
                      v-if="spellId && getSpellMeta(spellId)"
                      :src="getSpellMeta(spellId)?.icon"
                      :alt="getSpellMeta(spellId)?.label"
                      class="w-full h-full object-cover"
                    />
                  </div>
                </div>
              </div>

              <!-- 玩家信息 -->
              <div class="flex-1 min-w-0">
                <div class="flex flex-col gap-0.5">
                  <div class="flex items-center gap-1">
                    <h3 class="text-sm font-bold text-foreground truncate max-w-24">
                      {{ player.displayName || '未知召唤师' }}
                    </h3>
                    <!-- 不可查询原因标签 -->
                    <template v-if="!isQueryable(player)">
                      <div
                        class="px-1.5 py-0.5 text-[10px] font-medium rounded-full bg-muted text-muted-foreground border border-border/60"
                      >
                        {{ getUnqueryableReason(player) }}
                      </div>
                    </template>
                    <div
                      v-if="player.tier"
                      class="px-2 py-0.5 text-xs font-medium rounded-full bg-gradient-to-r from-yellow-500/20 to-yellow-600/20 text-yellow-600 border border-yellow-500/30 flex-shrink-0"
                    >
                      {{ player.tier }}
                    </div>
                  </div>
                  <div class="flex items-center gap-1">
                    <span class="text-xs text-muted-foreground">
                      {{ getChampionName(player.championId) || '未选择英雄' }}
                    </span>
                    <!-- 位置图标 -->
                    <div
                      v-if="getPositionName(index)"
                      class="text-xs px-1 py-0.5 bg-muted/50 rounded text-muted-foreground"
                    >
                      {{ getPositionName(index) }}
                    </div>
                  </div>
                </div>
              </div>

              <!-- 战绩统计 -->
              <div v-if="getPlayerStats(index)" class="flex-shrink-0">
                <div class="text-right space-y-1">
                  <!-- 胜率 -->
                  <div class="text-xs font-medium">
                    <span class="text-green-600">{{ getPlayerStats(index).wins }}胜</span>
                    <span class="text-muted-foreground">/</span>
                    <span class="text-red-600">{{ getPlayerStats(index).losses }}负</span>
                    <div class="text-xs text-muted-foreground">
                      {{
                        Math.round(
                          (getPlayerStats(index).wins /
                            Math.max(1, getPlayerStats(index).wins + getPlayerStats(index).losses)) *
                            100
                        )
                      }}%
                    </div>
                  </div>

                  <!-- 最近表现 -->
                  <div v-if="getPlayerStats(index).recentPerformance?.length" class="flex gap-0.5">
                    <div
                      v-for="(match, matchIndex) in getPlayerStats(index).recentPerformance.slice(0, 5)"
                      :key="matchIndex"
                      class="w-2 h-2 rounded-full text-[8px] flex items-center justify-center font-bold flex-shrink-0"
                      :class="match.win ? 'bg-green-500' : 'bg-red-500'"
                      :title="`${match.win ? '胜' : '负'} - ${match.championName} ${match.kills}/${match.deaths}/${match.assists}`"
                    />
                  </div>
                </div>
              </div>
            </div>

            <!-- 常用英雄 -->
            <div v-if="getPlayerStats(index)?.favoriteChampions?.length" class="mt-2 pt-2 border-t border-border/30">
              <div class="flex items-center gap-1">
                <div class="flex items-center gap-0.5">
                  <div class="w-2 h-2 rounded-full bg-gradient-to-r from-purple-500 to-pink-500"></div>
                  <span class="text-xs text-muted-foreground">常用</span>
                </div>
                <div class="flex gap-1">
                  <div
                    v-for="champion in getPlayerStats(index).favoriteChampions.slice(0, 3)"
                    :key="champion.championId"
                    class="flex flex-col items-center gap-0.5"
                  >
                    <div class="w-6 h-6 rounded overflow-hidden ring-1 ring-border/60 shadow-sm">
                      <img
                        :src="getChampionIconUrl(champion.championId)"
                        :alt="getChampionName(champion.championId)"
                        class="w-full h-full object-cover"
                      />
                    </div>
                    <div class="text-center">
                      <div class="text-[9px] text-muted-foreground leading-none">
                        {{ Math.round((champion.wins / Math.max(1, champion.games)) * 100) }}%
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- No Stats Data -->
            <div v-else-if="isQueryable(player)" class="text-center py-2">
              <div class="text-muted-foreground text-xs">
                <svg class="w-4 h-4 mx-auto mb-1 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                  ></path>
                </svg>
                <p class="text-xs">暂无战绩数据</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Card, CardContent } from '@/components/ui/card'
import { getChampionIconUrl, getChampionName, getSpellMeta } from '@/lib'

const props = withDefaults(
  defineProps<{
    teamData: {
      players: any[]
    }
    teamStats?: any[]
    teamType: 'ally' | 'enemy'
    localPlayerCellId?: number | null
  }>(),
  {
    teamStats: () => []
  }
)

defineEmits<{
  'select-player': [player: any, stats: any]
}>()

// 判断玩家是否可查询
const isQueryable = (player: any) => {
  return !(player?.isBot === true || player?.nameVisibilityType === 'HIDDEN' || !player?.displayName)
}

// 获取不可查询原因
const getUnqueryableReason = (player: any) => {
  if (player?.isBot === true) return '机器人'
  if (player?.nameVisibilityType === 'HIDDEN') return '匿名'
  return '身份不可识别'
}

// 根据索引获取玩家战绩数据
const getPlayerStats = (index: number) => {
  if (props.teamStats && props.teamStats[index]) {
    const player = props.teamData.players[index]
    const stats = props.teamStats[index]
    console.log(`TeamAnalysisCard: 玩家"${player?.displayName}" (索引${index}) 的战绩数据:`, stats)
    return stats
  }
  return null
}

// 获取位置名称
const getPositionName = (index: number) => {
  const positions = ['上路', '打野', '中路', 'ADC', '辅助']
  return positions[index] || ''
}
</script>
