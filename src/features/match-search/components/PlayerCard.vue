<template>
  <div
    class="relative group bg-gradient-to-br from-card/80 via-card/60 to-card/40 backdrop-blur-sm border border-border/40 rounded-xl p-4 transition-all duration-300 hover:shadow-lg hover:shadow-primary/10 hover:border-primary/30"
    style="will-change: transform, box-shadow"
    @click="emit('select', player)"
  >
    <div class="flex items-start justify-between mb-4">
      <div class="flex items-center gap-3">
        <!-- 英雄头像 -->
        <div class="relative">
          <div
            class="w-12 h-12 rounded-full overflow-hidden ring-2 ring-background/80 group-hover:ring-primary/40 transition-all duration-300"
          >
            <img
              v-if="player.championId"
              :src="getChampionIconUrl(player.championId)"
              :alt="getChampionName(player.championId)"
              class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-110"
            />
            <div v-else class="w-full h-full bg-gradient-to-br from-muted to-muted/60 flex items-center justify-center">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="12" cy="12" r="12" fill="currentColor" class="text-muted-foreground/30" />
                <path
                  d="M12 13c2.761 0 5 2.239 5 5v1H7v-1c0-2.761 2.239-5 5-5Z"
                  fill="currentColor"
                  class="text-muted-foreground/50"
                />
                <circle cx="12" cy="9" r="3" fill="currentColor" class="text-muted-foreground/50" />
              </svg>
            </div>
          </div>

          <!-- 当前玩家标识 -->
          <div
            v-if="isLocal"
            class="absolute -top-1 -right-1 bg-gradient-to-r from-primary to-primary/80 text-primary-foreground text-[10px] px-2 py-0.5 rounded-full shadow-lg animate-pulse font-bold"
          >
            我
          </div>
        </div>

        <!-- 玩家信息 -->
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2 mb-1">
            <h3 class="font-bold text-foreground truncate">
              {{ player.displayName || '未知召唤师' }}
            </h3>
            <div
              v-if="player.tier"
              class="px-2 py-0.5 text-xs font-medium rounded-full bg-gradient-to-r from-yellow-500/20 to-yellow-600/20 text-yellow-600 border border-yellow-500/30"
            >
              {{ player.tier }}
            </div>
          </div>

          <div class="flex items-center gap-2">
            <span class="text-sm text-muted-foreground">
              {{
                player.championName
                  ? player.championName
                  : player.championId
                    ? getChampionName(player.championId)
                    : '未选英雄'
              }}
            </span>
            <Badge
              v-if="player.assignedPosition"
              class="text-xs font-medium"
              :class="
                positionColorMap[player.assignedPosition.toUpperCase()] || 'bg-secondary text-secondary-foreground'
              "
            >
              {{ player.assignedPosition.toUpperCase() }}
            </Badge>
            <Badge v-else class="text-xs bg-secondary text-secondary-foreground" variant="secondary">未知位置</Badge>
          </div>
        </div>
      </div>

      <!-- 召唤师技能 -->
      <div class="flex flex-col gap-1">
        <template v-for="(spellId, idx) in [player.spell1Id ?? null, player.spell2Id ?? null]" :key="idx">
          <div
            class="w-6 h-6 rounded-md overflow-hidden ring-1 ring-border/60 group-hover:ring-primary/40 transition-all duration-300"
          >
            <template v-if="getSpellMeta(spellId).icon">
              <img
                :src="getSpellMeta(spellId).icon"
                :alt="getSpellMeta(spellId).label"
                class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-110"
                loading="lazy"
              />
            </template>
            <template v-else>
              <div class="w-full h-full bg-muted flex items-center justify-center">
                <svg class="w-3 h-3" viewBox="0 0 20 20" fill="none">
                  <circle cx="10" cy="10" r="10" fill="currentColor" class="text-muted-foreground/30" />
                  <path
                    d="M10 5v5l3 3"
                    stroke="currentColor"
                    class="text-muted-foreground/50"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                </svg>
              </div>
            </template>
          </div>
        </template>
      </div>
    </div>

    <!-- 战绩统计区域 -->
    <div v-if="playerStats" class="space-y-3">
      <!-- 核心数据卡片 -->
      <div class="grid grid-cols-3 gap-2">
        <!-- KDA -->
        <div
          class="bg-gradient-to-br from-green-500/10 to-green-600/10 border border-green-500/20 rounded-lg p-2 text-center"
        >
          <div class="text-xs text-muted-foreground mb-1">KDA</div>
          <div class="flex items-center justify-center gap-1 text-sm font-medium">
            <span class="text-green-500">{{ playerStats.avgKills?.toFixed(1) || '0' }}</span>
            <span class="text-muted-foreground">/</span>
            <span class="text-red-500">{{ playerStats.avgDeaths?.toFixed(1) || '0' }}</span>
            <span class="text-muted-foreground">/</span>
            <span class="text-blue-500">{{ playerStats.avgAssists?.toFixed(1) || '0' }}</span>
          </div>
        </div>

        <!-- 胜率 -->
        <div
          class="bg-gradient-to-br from-purple-500/10 to-purple-600/10 border border-purple-500/20 rounded-lg p-2 text-center"
        >
          <div class="text-xs text-muted-foreground mb-1">胜率</div>
          <div :class="getWinRateColor(playerStats.winRate)" class="text-sm font-bold">
            {{ playerStats.winRate?.toFixed(1) || '0' }}%
          </div>
        </div>

        <!-- 总场次 -->
        <div
          class="bg-gradient-to-br from-orange-500/10 to-orange-600/10 border border-orange-500/20 rounded-lg p-2 text-center"
        >
          <div class="text-xs text-muted-foreground mb-1">场次</div>
          <div class="text-sm font-bold text-orange-500">
            {{ playerStats.totalGames || 0 }}
          </div>
        </div>
      </div>

      <!-- 常用英雄 -->
      <div v-if="playerStats.favoriteChampions?.length" class="space-y-2">
        <div class="flex items-center gap-2">
          <div class="w-4 h-4 rounded-full bg-gradient-to-r from-blue-500 to-purple-500"></div>
          <span class="text-xs font-medium text-muted-foreground">常用英雄</span>
        </div>
        <div class="flex gap-2">
          <div
            v-for="champion in playerStats.favoriteChampions.slice(0, 3)"
            :key="champion.championId"
            class="relative group/champion"
            :title="`${getChampionName(champion.championId)} (${champion.gamesPlayed}场, ${champion.winRate}%胜率)`"
          >
            <div
              class="w-8 h-8 rounded-lg overflow-hidden ring-1 ring-border/60 group-hover/champion:ring-primary/40 transition-all duration-300"
            >
              <img
                :src="getChampionIconUrl(champion.championId)"
                :alt="getChampionName(champion.championId)"
                class="w-full h-full object-cover transition-transform duration-300 group-hover/champion:scale-110"
              />
            </div>
            <div
              class="absolute -bottom-1 -right-1 w-3 h-3 rounded-full text-[6px] flex items-center justify-center font-bold bg-background border border-border"
            >
              {{ champion.gamesPlayed }}
            </div>
          </div>
          <span v-if="playerStats.favoriteChampions.length > 3" class="text-xs text-muted-foreground self-center">
            +{{ playerStats.favoriteChampions.length - 3 }}
          </span>
        </div>
      </div>

      <!-- 最近战绩 -->
      <div v-if="playerStats?.recentPerformance?.length" class="space-y-2">
        <div class="flex items-center gap-2">
          <div class="w-4 h-4 rounded-full bg-gradient-to-r from-green-500 to-blue-500"></div>
          <span class="text-xs font-medium text-muted-foreground">最近战绩</span>
          <span class="text-xs text-muted-foreground">({{ playerStats.recentPerformance.length }}场)</span>
        </div>
        <div class="space-y-1.5 max-h-20 overflow-y-auto">
          <div
            v-for="match in playerStats.recentPerformance.slice(0, 2)"
            :key="match.gameId"
            class="flex items-center gap-2 p-2 rounded-lg bg-muted/30 border border-border/50 hover:bg-muted/50 transition-colors"
          >
            <!-- 英雄头像 -->
            <div class="w-6 h-6 rounded-md overflow-hidden ring-1 ring-border/60 flex-shrink-0">
              <img
                :src="getChampionIconUrl(match.championId)"
                :alt="getChampionName(match.championId)"
                class="w-full h-full object-cover"
              />
            </div>

            <!-- 对局信息 -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 mb-0.5">
                <!-- 胜负标识 -->
                <div
                  class="w-3 h-3 rounded-full text-[8px] flex items-center justify-center font-bold flex-shrink-0"
                  :class="match.win ? 'bg-green-500 text-white' : 'bg-red-500 text-white'"
                >
                  {{ match.win ? '胜' : '负' }}
                </div>

                <!-- 英雄名称 -->
                <span class="text-xs text-foreground font-medium truncate">
                  {{ getChampionName(match.championId) }}
                </span>
              </div>

              <!-- KDA和游戏时长 -->
              <div class="flex items-center gap-2">
                <div class="flex items-center gap-1">
                  <span class="text-xs text-green-500">{{ match.kills || 0 }}</span>
                  <span class="text-xs text-muted-foreground">/</span>
                  <span class="text-xs text-red-500">{{ match.deaths || 0 }}</span>
                  <span class="text-xs text-muted-foreground">/</span>
                  <span class="text-xs text-blue-500">{{ match.assists || 0 }}</span>
                </div>

                <span v-if="match.gameDuration" class="text-xs text-muted-foreground">
                  {{ Math.floor(match.gameDuration / 60) }}分{{ match.gameDuration % 60 }}秒
                </span>
              </div>
            </div>

            <!-- 游戏模式 -->
            <div
              v-if="match.queueId"
              class="text-xs text-muted-foreground bg-muted px-1.5 py-0.5 rounded flex-shrink-0"
            >
              {{ getQueueName(match.queueId) }}
            </div>
          </div>

          <!-- 更多对局提示 -->
          <div v-if="playerStats.recentPerformance.length > 2" class="text-center py-1">
            <span class="text-xs text-muted-foreground bg-muted px-2 py-1 rounded">
              还有 {{ playerStats.recentPerformance.length - 2 }} 场对局
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- 调试信息 -->
    <div v-if="playerStats && !playerStats.recentPerformance?.length" class="mt-2 text-xs text-muted-foreground">
      调试: recentPerformance 为空或不存在
    </div>
  </div>
</template>

<script setup lang="ts">
import { getChampionIconUrl, getChampionName, getSpellMeta, getQueueName } from '@/lib'

const props = defineProps<{
  player: PlayerDisplayInfo
  isLocal?: boolean
  isAlly?: boolean
  selected?: boolean
  playerStats?: any // 新增：玩家战绩数据
}>()

const emit = defineEmits(['select'])

// 调试：输出playerStats数据
watchEffect(() => {
  if (props.playerStats) {
    console.log('PlayerCard playerStats:', props.playerStats)
    console.log('recentPerformance:', props.playerStats.recentPerformance)
  }
})

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
