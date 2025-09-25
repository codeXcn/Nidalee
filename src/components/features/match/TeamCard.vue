<template>
  <div>
    <!-- <div class="flex items-center justify-between mb-2">
      <div class="flex items-center gap-3">
        <div :class="dotClass"></div>
        <h3 :class="titleClass">{{ title }}</h3>
      </div>
      <Badge :variant="'outline'" :class="badgeClass"> {{ team.length }} 人 </Badge>
    </div> -->

    <!-- 使用Card展示每个召唤师 -->
    <div>
      <div
        v-for="(player, index) in team"
        :key="player.summonerId + '-' + player.cellId"
        class="w-full bg-transparent mb-1"
      >
        <Card
          class="p-2 border group cursor-pointer transition-all duration-150 hover:border-primary hover:bg-primary/5"
          @click="$emit('select', player)"
        >
          <CardContent class="p-0">
            <!-- 点击提示 -->
            <!-- <div class="absolute top-1 right-1 opacity-0 group-hover:opacity-100 transition-opacity">
              <div class="bg-primary/10 text-primary text-xs px-1.5 py-0.5 rounded-full">点击查看</div>
            </div> -->
            <!-- 第一行：基本信息 -->
            <div class="flex items-center mb-1 gap-2">
              <!-- 左侧：头像和基本信息 -->
              <div class="flex items-center gap-2">
                <!-- Champion Avatar -->
                <div class="relative">
                  <div class="w-8 h-8 rounded-full overflow-hidden ring-1 ring-background/60">
                    <img
                      v-if="player.championId"
                      :src="getChampionIconUrl(player.championId)"
                      :alt="getChampionName(player.championId)"
                      class="w-full h-full object-cover"
                    />
                    <div v-else class="w-full h-full bg-muted flex items-center justify-center text-muted-foreground">
                      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                        ></path>
                      </svg>
                    </div>
                  </div>
                  <div
                    v-if="localPlayerCellId ? player.cellId === localPlayerCellId : false"
                    class="absolute -top-1 -right-1 bg-gradient-to-r from-primary to-primary/80 text-primary-foreground text-[11px] px-1 py-0.5 rounded-full shadow animate-pulse font-bold"
                  >
                    我
                  </div>
                </div>

                <!-- Player Info -->
                <div class="flex flex-col gap-0.5">
                  <div class="flex items-center gap-1">
                    <h3 class="text-sm font-bold text-foreground truncate max-w-24">
                      {{ player.displayName || '未知召唤师' }}
                    </h3>
                    <div
                      v-if="player.tier"
                      class="px-2 py-0.5 text-xs font-medium rounded-full bg-gradient-to-r from-yellow-500/20 to-yellow-600/20 text-yellow-600 border border-yellow-500/30 flex-shrink-0"
                    >
                      {{ player.tier }}
                    </div>
                  </div>
                  <div class="flex items-center gap-1">
                    <span class="text-xs text-muted-foreground">{{
                      player.championId ? getChampionName(player.championId) : '未选英雄'
                    }}</span>
                    <Badge
                      v-if="player.assignedPosition"
                      class="text-[10px] font-medium px-1"
                      :class="
                        positionColorMap[player.assignedPosition.toUpperCase()] ||
                        'bg-secondary text-secondary-foreground'
                      "
                      >{{ player.assignedPosition.toUpperCase() }}</Badge
                    >
                    <!-- <Badge v-else class="text-[10px] bg-secondary text-secondary-foreground px-1" variant="secondary"
                      >未知位置</Badge
                    > -->
                    <div class="flex gap-1">
                      <template v-for="(spellId, idx) in [player.spell1Id ?? null, player.spell2Id ?? null]" :key="idx">
                        <div class="w-5 h-5 rounded overflow-hidden ring-1 ring-border/60">
                          <img
                            v-if="spellId"
                            :src="getSpellMeta(spellId)?.icon"
                            :alt="getSpellMeta(spellId)?.label"
                            class="w-full h-full object-cover"
                          />
                          <div
                            v-else
                            class="w-full h-full bg-muted flex items-center justify-center text-muted-foreground"
                          >
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 10V3L4 14h7v7l9-11h-7z"
                              ></path>
                            </svg>
                          </div>
                        </div>
                      </template>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 中间：统计数据 -->
              <div v-if="getPlayerStats(index)" class="flex-1 flex gap-1">
                <!-- <div
                  class="flex items-center gap-1 bg-gradient-to-r from-green-500/10 to-green-600/10 border border-green-500/20 rounded px-1 py-1"
                >
                  <div class="text-center">
                    <div class="text-xs font-bold text-green-600">
                      {{ getPlayerStats(index).avgKills.toFixed(1) }}
                    </div>
                    <div class="text-[11px] text-muted-foreground">击杀</div>
                  </div>
                  <div class="text-center">
                    <div class="text-xs font-bold text-red-500">{{ getPlayerStats(index).avgDeaths.toFixed(1) }}</div>
                    <div class="text-[11px] text-muted-foreground">死亡</div>
                  </div>
                  <div class="text-center">
                    <div class="text-xs font-bold text-blue-500">
                      {{ getPlayerStats(index).avgAssists.toFixed(1) }}
                    </div>
                    <div class="text-[11px] text-muted-foreground">助攻</div>
                  </div>
                </div> -->

                <div class="flex items-center gap-1">
                  <div class="text-center">
                    <div class="text-xs font-bold" :class="getWinRateColor(getPlayerStats(index).winRate)">
                      {{ getPlayerStats(index).winRate.toFixed(1) }}%
                    </div>
                    <div class="text-[10px] text-muted-foreground">胜率</div>
                  </div>
                  <div class="text-center">
                    <div class="text-xs font-bold text-orange-500">{{ getPlayerStats(index).totalGames }}</div>
                    <div class="text-[10px] text-muted-foreground">场次</div>
                  </div>
                </div>

                <div class="flex items-center gap-1">
                  <div class="text-center">
                    <div class="text-xs font-bold text-blue-600">{{ getPlayerStats(index).avgKda.toFixed(2) }}</div>
                    <div class="text-[10px] text-muted-foreground">KDA</div>
                  </div>
                </div>
              </div>

              <!-- 右侧：常用英雄 -->
              <div v-if="getPlayerStats(index)?.favoriteChampions?.length" class="flex items-center gap-1">
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
                      <div class="text-[10px] text-muted-foreground">{{ champion.gamesPlayed }}场</div>
                      <!-- <div class="text-[11px] font-medium" :class="getWinRateColor(champion.winRate)">
                        {{ champion.winRate.toFixed(1) }}%
                      </div> -->
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 第二行：最近战绩 -->
            <div v-if="getPlayerStats(index)?.recentPerformance?.length">
              <!-- <div class="flex items-center gap-1">
                <div class="w-2 h-2 rounded-full bg-gradient-to-r from-green-500 to-blue-500"></div>
                <span class="text-xs font-medium text-muted-foreground">最近战绩</span>
              </div> -->
              <div class="grid grid-cols-3 gap-1">
                <div
                  v-for="match in getPlayerStats(index).recentPerformance.slice(0, 6)"
                  :key="match.gameId"
                  class="flex items-center gap-1 p-1 rounded bg-muted/30 border border-border/50 hover:bg-muted/50 transition-colors"
                >
                  <!-- 英雄头像 -->
                  <div class="w-6 h-6 rounded overflow-hidden ring-1 ring-border/60 flex-shrink-0">
                    <img
                      :src="getChampionIconUrl(match.championId)"
                      :alt="getChampionName(match.championId)"
                      class="w-full h-full object-cover"
                    />
                  </div>

                  <!-- 对局信息 -->
                  <div class="flex-1 min-w-0">
                    <!-- 英雄名称 -->
                    <!-- <div class="flex items-center gap-0.5 mb-0.5">
                      <span class="text-xs text-foreground font-medium truncate">
                        {{ getChampionName(match.championId) }}
                      </span>
                    </div> -->

                    <!-- KDA和游戏时长 -->
                    <div class="flex items-center gap-1">
                      <div class="flex items-center gap-0.5">
                        <span class="text-xs text-green-500 font-medium">{{ match.kills }}</span>
                        <span class="text-xs text-muted-foreground">/</span>
                        <span class="text-xs text-red-500 font-medium">{{ match.deaths }}</span>
                        <span class="text-xs text-muted-foreground">/</span>
                        <span class="text-xs text-blue-500 font-medium">{{ match.assists }}</span>
                      </div>

                      <!-- <span class="text-xs text-muted-foreground">
                        {{ Math.floor(match.gameDuration / 60) }}分{{ match.gameDuration % 60 }}秒
                      </span> -->
                    </div>
                  </div>

                  <!-- 右侧：胜负和比赛类型 -->
                  <div class="flex gap-0.5">
                    <!-- 比赛类型 -->
                    <span class="text-[10px] text-muted-foreground bg-muted px-1 py-0.5 rounded">
                      {{ getQueueName(match.queueId) }}
                    </span>
                    <!-- 胜负标识 -->
                    <div
                      class="w-4 h-4 rounded-full text-[11px] flex items-center justify-center font-bold flex-shrink-0 shadow-sm"
                      :class="match.win ? 'bg-green-500 text-white' : 'bg-red-500 text-white'"
                    >
                      {{ match.win ? '胜' : '负' }}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- No Stats Data -->
            <div v-else class="text-center py-2">
              <div class="text-muted-foreground">
                <svg class="w-8 h-8 mx-auto mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                  ></path>
                </svg>
                <p class="text-sm">暂无战绩数据</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Card, CardContent } from '@/components/ui/card'
import { getChampionIconUrl, getChampionName, getSpellMeta, getQueueName } from '@/lib'

const props = withDefaults(
  defineProps<{
    team: ChampSelectPlayer[]
    teamType: 'ally' | 'enemy'
    localPlayerCellId?: number | null
    summonerStats?: any[] // 新增：召唤师战绩数据数组
  }>(),
  {
    team: () => [],
    summonerStats: () => []
  }
)

defineEmits(['select'])

const isAlly = computed(() => props.teamType === 'ally')

const dotClass = computed(
  () => `w-3 h-3 lg:w-4 lg:h-4 rounded-full animate-pulse ${isAlly.value ? 'bg-blue-500' : 'bg-red-500'}`
)
const titleClass = computed(
  () =>
    `text-lg lg:text-xl font-bold ${isAlly.value ? 'text-blue-600 dark:text-blue-400' : 'text-red-600 dark:text-red-400'}`
)
const badgeClass = computed(() =>
  isAlly.value
    ? 'text-blue-600 dark:text-blue-400 border-blue-300 dark:border-blue-600'
    : 'text-red-600 dark:text-red-400 border-red-300 dark:border-red-600'
)
const title = computed(() => (isAlly.value ? '我方队伍' : '敌方队伍'))

// 根据索引获取玩家战绩数据
const getPlayerStats = (index: number) => {
  if (props.summonerStats && props.summonerStats[index]) {
    const player = props.team[index]
    const stats = props.summonerStats[index]
    console.log(`TeamCard: 玩家"${player?.displayName}" (索引${index}) 的战绩数据:`, stats)
    console.log(`TeamCard: recentPerformance长度:`, stats.recentPerformance)
    return stats
  }
  const player = props.team[index]
  console.log(`TeamCard: 玩家"${player?.displayName}" (索引${index}) 没有战绩数据`)
  return null
}

// 位置与颜色映射 - 更现代的配色
const positionColorMap: Record<string, string> = {
  上单: 'bg-blue-500 text-white shadow-blue-500/20',
  打野: 'bg-green-500 text-white shadow-green-500/20',
  中单: 'bg-purple-500 text-white shadow-purple-500/20',
  ADC: 'bg-orange-500 text-white shadow-orange-500/20',
  辅助: 'bg-pink-500 text-white shadow-pink-500/20',
  // 英文位置
  TOP: 'bg-blue-500 text-white shadow-blue-500/20',
  JUNGLE: 'bg-green-500 text-white shadow-green-500/20',
  MIDDLE: 'bg-purple-500 text-white shadow-purple-500/20',
  BOTTOM: 'bg-orange-500 text-white shadow-orange-500/20',
  UTILITY: 'bg-pink-500 text-white shadow-pink-500/20'
}

// 胜率颜色映射
const getWinRateColor = (winRate: number) => {
  if (winRate >= 60) return 'text-green-500'
  if (winRate >= 50) return 'text-yellow-500'
  return 'text-red-500'
}

// 统一使用公共的队列名称映射（基于 queueId）
</script>
