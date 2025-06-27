<template>
  <Card class="p-6">
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold flex items-center">
            <BarChart class="h-5 w-5 mr-2 text-blue-500" />
            游戏统计
          </h3>
          <p class="text-sm text-muted-foreground">近期游戏数据概览</p>
        </div>
        <Button
          :disabled="!isConnected || matchHistoryLoading"
          variant="outline"
          size="sm"
          @click="$emit('fetch-match-history')"
        >
          <RefreshCw :class="['h-4 w-4 mr-2', { 'animate-spin': matchHistoryLoading }]" />
          {{ matchHistoryLoading ? '加载中...' : '刷新数据' }}
        </Button>
      </div>

      <!-- 加载状态 -->
      <div v-if="matchHistoryLoading" class="flex items-center justify-center py-16">
        <div class="text-center">
          <Loader2 class="h-12 w-12 animate-spin text-blue-500 mx-auto mb-4" />
          <p class="text-lg font-medium text-muted-foreground">正在分析对局数据...</p>
          <p class="text-sm text-muted-foreground">请稍候，这可能需要几秒钟</p>
        </div>
      </div>

      <!-- 未连接状态 -->
      <div v-else-if="!isConnected" class="flex items-center justify-center py-16">
        <div class="text-center">
          <Wifi class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
          <p class="text-lg font-medium text-muted-foreground">需要连接到League客户端</p>
          <p class="text-sm text-muted-foreground">连接后即可查看详细的游戏统计</p>
        </div>
      </div>

      <!-- 无数据状态 -->
      <div v-else-if="!matchStatistics" class="flex items-center justify-center py-16">
        <div class="text-center">
          <BarChart class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
          <p class="text-lg font-medium text-muted-foreground">暂无统计数据</p>
          <p class="text-sm text-muted-foreground">点击"刷新数据"获取最新的游戏统计</p>
        </div>
      </div>

      <!-- 统计数据展示 -->
      <div v-else class="space-y-6">
        <!-- 总体数据概览 -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="text-center p-4 rounded-lg bg-muted/30">
            <Trophy class="h-8 w-8 text-yellow-500 mx-auto mb-2" />
            <p class="text-2xl font-bold text-foreground">{{ matchStatistics.total_games }}</p>
            <p class="text-sm text-muted-foreground">总对局</p>
          </div>
          <div class="text-center p-4 rounded-lg bg-green-500/10">
            <Award class="h-8 w-8 text-green-500 mx-auto mb-2" />
            <p class="text-2xl font-bold text-green-600 dark:text-green-400">
              {{ matchStatistics.wins }}
            </p>
            <p class="text-sm text-muted-foreground">胜场</p>
          </div>
          <div class="text-center p-4 rounded-lg bg-red-500/10">
            <Target class="h-8 w-8 text-red-500 mx-auto mb-2" />
            <p class="text-2xl font-bold text-red-600 dark:text-red-400">
              {{ matchStatistics.losses }}
            </p>
            <p class="text-sm text-muted-foreground">负场</p>
          </div>
          <div class="text-center p-4 rounded-lg bg-blue-500/10">
            <TrendingUp class="h-8 w-8 text-blue-500 mx-auto mb-2" />
            <p class="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {{ matchStatistics.win_rate.toFixed(1) }}%
            </p>
            <p class="text-sm text-muted-foreground">胜率</p>
          </div>
        </div>

        <!-- KDA统计 -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-4">
            <h4 class="font-semibold flex items-center">
              <Swords class="h-4 w-4 mr-2 text-red-500" />
              KDA统计
            </h4>
            <div class="grid grid-cols-3 gap-4">
              <div class="text-center p-3 rounded-lg border">
                <p class="text-lg font-bold text-foreground">
                  {{ matchStatistics.avg_kills.toFixed(1) }}
                </p>
                <p class="text-xs text-muted-foreground">平均击杀</p>
              </div>
              <div class="text-center p-3 rounded-lg border">
                <p class="text-lg font-bold text-foreground">
                  {{ matchStatistics.avg_deaths.toFixed(1) }}
                </p>
                <p class="text-xs text-muted-foreground">平均死亡</p>
              </div>
              <div class="text-center p-3 rounded-lg border">
                <p class="text-lg font-bold text-foreground">
                  {{ matchStatistics.avg_assists.toFixed(1) }}
                </p>
                <p class="text-xs text-muted-foreground">平均助攻</p>
              </div>
            </div>
            <div class="text-center p-3 rounded-lg bg-purple-500/10">
              <p class="text-xl font-bold text-purple-600 dark:text-purple-400">
                {{ matchStatistics.avg_kda.toFixed(2) }}
              </p>
              <p class="text-sm text-muted-foreground">平均KDA</p>
            </div>
            <!-- 召唤师特征分析 -->
            <SummonerTraits :match-statistics="matchStatistics" />
          </div>

          <!-- 常用英雄 -->
          <div class="space-y-4">
            <h4 class="font-semibold flex items-center">
              <Star class="h-4 w-4 mr-2 text-yellow-500" />
              常用英雄
            </h4>
            <div class="space-y-2">
              <div
                v-for="champion in matchStatistics.favorite_champions.slice(0, 5)"
                :key="champion.champion_name"
                class="flex items-center justify-between p-2 rounded-lg border"
              >
                <div class="flex items-center space-x-2">
                  <div class="h-8 w-8 rounded-full bg-blue-500/20 flex items-center justify-center">
                    <span class="text-xs font-bold">{{ getChampionName(champion.champion_id).charAt(0) }}</span>
                  </div>
                  <div>
                    <p class="font-medium text-sm">{{ getChampionName(champion.champion_id) }}</p>
                    <p class="text-xs text-muted-foreground">{{ champion.games_played }}场</p>
                  </div>
                </div>
                <div class="text-right">
                  <p
                    class="text-sm font-bold"
                    :class="[
                      champion.win_rate >= 60
                        ? 'text-green-600 dark:text-green-400'
                        : champion.win_rate >= 50
                          ? 'text-yellow-600 dark:text-yellow-400'
                          : 'text-red-600 dark:text-red-400'
                    ]"
                  >
                    {{ champion.win_rate.toFixed(0) }}%
                  </p>
                  <p class="text-xs text-muted-foreground">{{ champion.wins }}胜</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 最近对局 -->
        <div class="space-y-4" v-if="matchStatistics.recent_performance.length > 0">
          <h4 class="font-semibold flex items-center">
            <Calendar class="h-4 w-4 mr-2 text-blue-500" />
            最近对局
          </h4>
          <div class="grid gap-3" style="grid-template-columns: repeat(auto-fit, minmax(260px, 1fr))">
            <div
              v-for="game in matchStatistics.recent_performance.slice(0, 10)"
              :key="game.game_creation"
              :class="[
                game.win
                  ? 'bg-green-500/10 border-green-500/30 hover:border-green-500/50'
                  : 'bg-red-500/10 border-red-500/30 hover:border-red-500/50'
              ]"
              class="p-3 rounded-lg border cursor-pointer transition-all duration-200 hover:shadow-md hover:scale-[1.02] relative"
              @click="openGameDetail(game)"
            >
              <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">{{ getChampionName(game.champion_id) }}</span>
                <Badge :variant="game.win ? 'default' : 'destructive'" class="text-xs">
                  {{ game.win ? '胜利' : '失败' }}
                </Badge>
              </div>
              <div class="flex items-center justify-between text-sm mb-1">
                <span class="font-mono">{{ game.kills }}/{{ game.deaths }}/{{ game.assists }}</span>
                <span class="text-muted-foreground">{{ formatGameTime(game.game_duration) }}</span>
              </div>
              <div class="flex items-center text-xs text-gray-400 mt-1">
                <Clock class="w-3 h-3 mr-1" />
                <span>{{ formatRelativeTime(game.game_creation) }}</span>
              </div>
              <div class="text-xs text-muted-foreground mt-1">
                {{ getQueueName(game.queue_id) }}
              </div>
              <div
                class="absolute right-2 bottom-2 rotate-12 px-3 py-0.5 rounded-full shadow text-xs font-bold select-none flex items-center gap-1"
                :class="{
                  'bg-green-500 text-white':
                    game.performance_rating.includes('超神') || game.performance_rating.includes('亮眼'),
                  'bg-yellow-400 text-white': game.performance_rating.includes('不错'),
                  'bg-red-500 text-white': game.performance_rating.includes('需要加油'),
                  'bg-purple-500 text-white':
                    game.performance_rating.includes('五杀') || game.performance_rating.includes('四杀')
                }"
              >
                <Award v-if="game.performance_rating.includes('超神')" class="w-4 h-4" />
                <Star v-else-if="game.performance_rating.includes('亮眼')" class="w-4 h-4" />
                <Flame
                  v-else-if="game.performance_rating.includes('五杀') || game.performance_rating.includes('四杀')"
                  class="w-4 h-4"
                />
                <Smile v-else-if="game.performance_rating.includes('不错')" class="w-4 h-4" />
                <Meh v-else-if="game.performance_rating.includes('一般')" class="w-4 h-4" />
                <AlertCircle v-else-if="game.performance_rating.includes('需要加油')" class="w-4 h-4" />
                <span>{{ game.performance_rating }}</span>
              </div>
            </div>
          </div>
        </div>
        <div v-else class="text-center text-muted-foreground py-8">
          <div class="text-3xl mb-2">🎮</div>
          <p>暂无对局记录</p>
        </div>
      </div>
    </div>
  </Card>
  <GameDetailDialog v-model:visible="dialogOpen" :selectedGame="selectedGame" />
</template>

<script setup lang="ts">
import { getChampionName } from '@/lib'
import {
  AlertCircle,
  Award,
  BarChart,
  Calendar,
  Clock,
  Flame,
  Loader2,
  Meh,
  RefreshCw,
  Smile,
  Star,
  Swords,
  Target,
  TrendingUp,
  Trophy,
  Wifi
} from 'lucide-vue-next'
const dialogOpen = ref(false)
const selectedGame = ref(null)

const openGameDetail = (game: any) => {
  selectedGame.value = game
  console.log(game)
  dialogOpen.value = true
}
const props = defineProps<{
  isConnected: boolean
  matchHistoryLoading: boolean
  matchStatistics: any
}>()
const emit = defineEmits<{
  (e: 'fetch-match-history'): void
  (e: 'open-game-detail', game: any): void
}>()

const { formatGameMode, formatGameTime, formatRelativeTime } = useFormatters()

const queueMap: Record<number, string> = {
  420: '单双排', // 峡谷之巅/召唤师峡谷Ranked Solo/Duo
  430: '匹配模式', // 召唤师峡谷普通匹配
  440: '灵活组排', // 召唤师峡谷Ranked Flex
  450: '极地大乱斗', // ARAM
  400: '灵活匹配', // 召唤师峡谷团队匹配
  700: '克隆模式', // Clash/Clones
  900: '无限火力', // URF
  1020: '奥德赛：浩劫', // Odyssey
  830: '新手人机', // Co-op vs. AI Intro
  840: '初级人机', // Co-op vs. AI Beginner
  850: '中级人机', // Co-op vs. AI Intermediate
  2020: '灵能特工', // Arena
  1700: '斗魂竞技场' // Arena 2v2v2v2
}
const getQueueName = (queueId: number) => queueMap[queueId] || '其它模式'
</script>
