<template>
  <div class="space-y-6">
    <!-- 召唤师基本信息 -->
    <div class="flex items-start gap-4 p-4 rounded-lg bg-muted/50">
      <div
        class="w-16 h-16 rounded-lg bg-gradient-to-br from-primary/20 to-primary/10 flex items-center justify-center"
      >
        <span class="text-2xl font-bold text-primary">
          {{ summonerName?.charAt(0)?.toUpperCase() || '?' }}
        </span>
      </div>

      <div class="flex-1 min-w-0">
        <h3 class="text-xl font-bold mb-1 text-foreground">{{ summonerName || '未知召唤师' }}</h3>
        <div class="flex items-center gap-2 mb-2">
          <Badge variant="outline" class="text-xs"> 等级 {{ summonerLevel || 1 }} </Badge>
          <Badge v-if="rankedTier" :variant="getRankedVariant(rankedTier)" class="text-xs">
            {{ getRankedDisplay(rankedTier, rankedDivision) }}
          </Badge>
        </div>
        <p class="text-sm text-muted-foreground">召唤师 ID: {{ summonerId }}</p>
      </div>
    </div>

    <!-- 排位信息 -->
    <Card class="p-4">
      <h4 class="font-semibold mb-3 flex items-center gap-2">
        <Trophy class="h-4 w-4" />
        排位信息
      </h4>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- 单/双排 -->
        <div class="p-3 rounded-lg border">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-medium text-foreground">单/双排</span>
            <Badge v-if="soloRank" :variant="getRankedVariant(soloRank.tier)">
              {{ getRankedDisplay(soloRank.tier, soloRank.rank) }}
            </Badge>
          </div>

          <div v-if="soloRank" class="space-y-1">
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">胜率</span>
              <span :class="getWinRateColor(soloRank.winRate)"> {{ soloRank.winRate }}% </span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">胜/负</span>
              <span>{{ soloRank.wins }}/{{ soloRank.losses }}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">LP</span>
              <span>{{ soloRank.leaguePoints }} LP</span>
            </div>
          </div>

          <div v-else class="text-sm text-muted-foreground">未进行排位</div>
        </div>

        <!-- 灵活排位 -->
        <div class="p-3 rounded-lg border">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-medium text-foreground">灵活排位</span>
            <Badge v-if="flexRank" :variant="getRankedVariant(flexRank.tier)">
              {{ getRankedDisplay(flexRank.tier, flexRank.rank) }}
            </Badge>
          </div>

          <div v-if="flexRank" class="space-y-1">
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">胜率</span>
              <span :class="getWinRateColor(flexRank.winRate)"> {{ flexRank.winRate }}% </span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">胜/负</span>
              <span>{{ flexRank.wins }}/{{ flexRank.losses }}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">LP</span>
              <span>{{ flexRank.leaguePoints }} LP</span>
            </div>
          </div>

          <div v-else class="text-sm text-muted-foreground">未进行排位</div>
        </div>
      </div>
    </Card>

    <!-- 常用英雄 -->
    <Card class="p-4">
      <h4 class="font-semibold mb-3 flex items-center gap-2">
        <Star class="h-4 w-4" />
        常用英雄
      </h4>

      <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
        <div
          v-for="champion in topChampions"
          :key="champion.championId"
          class="flex items-center gap-3 p-3 rounded-lg border hover:bg-muted/50 transition-colors"
        >
          <div class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
            <span class="text-xs font-bold text-primary">{{ champion.name?.charAt(0) || '?' }}</span>
          </div>

          <div class="flex-1 min-w-0">
            <div class="font-medium text-sm truncate text-foreground">{{ champion.name || '未知英雄' }}</div>
            <div class="text-xs text-muted-foreground">{{ champion.gamesPlayed }} 场 · {{ champion.winRate }}%</div>
          </div>
        </div>
      </div>

      <div v-if="topChampions.length === 0" class="text-center py-8 text-muted-foreground">
        <User class="h-8 w-8 mx-auto mb-2 opacity-50" />
        <p class="text-sm">暂无英雄数据</p>
      </div>
    </Card>

    <!-- 最近战绩 -->
    <Card class="p-4">
      <h4 class="font-semibold mb-3 flex items-center gap-2">
        <History class="h-4 w-4" />
        最近战绩
      </h4>

      <div class="space-y-2">
        <div
          v-for="match in recentMatches"
          :key="match.gameId"
          class="flex items-center gap-3 p-3 rounded-lg border"
          :class="
            match.win
              ? 'bg-green-50 dark:bg-green-950/20 border-green-200 dark:border-green-800'
              : 'bg-red-50 dark:bg-red-950/20 border-red-200 dark:border-red-800'
          "
        >
          <div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center">
            <span class="text-xs font-bold text-primary">{{ match.championName?.charAt(0) || '?' }}</span>
          </div>

          <div class="flex-1">
            <div class="flex items-center gap-2 mb-1">
              <span class="font-medium text-sm text-foreground">{{ match.championName || '未知英雄' }}</span>
              <Badge :variant="match.win ? 'default' : 'destructive'" class="text-xs">
                {{ match.win ? '胜利' : '失败' }}
              </Badge>
            </div>

            <div class="flex items-center gap-4 text-xs text-muted-foreground">
              <span>{{ match.kda.kills }}/{{ match.kda.deaths }}/{{ match.kda.assists }}</span>
              <span>{{ formatGameDuration(match.gameDuration) }}</span>
              <span>{{ formatGameMode(match.gameMode) }}</span>
            </div>
          </div>

          <div class="text-right">
            <div
              class="text-sm font-medium"
              :class="match.win ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'"
            >
              KDA {{ calculateKDA(match.kda) }}
            </div>
            <div class="text-xs text-muted-foreground">
              {{ formatRelativeTime(match.gameCreation) }}
            </div>
          </div>
        </div>
      </div>

      <div v-if="recentMatches.length === 0" class="text-center py-8 text-muted-foreground">
        <History class="h-8 w-8 mx-auto mb-2 opacity-50" />
        <p class="text-sm">暂无战绩数据</p>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { Trophy, Star, User, History } from 'lucide-vue-next'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

interface Props {
  summonerId: string | number
  showHeader?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showHeader: true
})

// 模拟数据 - 在实际应用中这些应该从 API 获取
const summonerName = ref('示例召唤师')
const summonerLevel = ref(156)
const rankedTier = ref('GOLD')
const rankedDivision = ref('II')

const soloRank = ref({
  tier: 'GOLD',
  rank: 'II',
  leaguePoints: 65,
  wins: 128,
  losses: 95,
  winRate: 57
})

const flexRank = ref({
  tier: 'SILVER',
  rank: 'I',
  leaguePoints: 42,
  wins: 35,
  losses: 28,
  winRate: 56
})

const topChampions = ref([
  { championId: 1, name: '艾希', gamesPlayed: 25, winRate: 68 },
  { championId: 2, name: '金克丝', gamesPlayed: 18, winRate: 61 },
  { championId: 3, name: '卡莎', gamesPlayed: 15, winRate: 53 },
  { championId: 4, name: '薇恩', gamesPlayed: 12, winRate: 75 },
  { championId: 5, name: '德莱文', gamesPlayed: 8, winRate: 50 },
  { championId: 6, name: '卢锡安', gamesPlayed: 6, winRate: 67 }
])

const recentMatches = ref([
  {
    gameId: 1,
    championName: '艾希',
    win: true,
    kda: { kills: 8, deaths: 2, assists: 12 },
    gameDuration: 1890000, // 31.5 minutes in ms
    gameMode: 'CLASSIC',
    gameCreation: Date.now() - 3600000 // 1 hour ago
  },
  {
    gameId: 2,
    championName: '金克丝',
    win: false,
    kda: { kills: 5, deaths: 7, assists: 6 },
    gameDuration: 2280000, // 38 minutes in ms
    gameMode: 'CLASSIC',
    gameCreation: Date.now() - 7200000 // 2 hours ago
  },
  {
    gameId: 3,
    championName: '卡莎',
    win: true,
    kda: { kills: 12, deaths: 3, assists: 8 },
    gameDuration: 1560000, // 26 minutes in ms
    gameMode: 'CLASSIC',
    gameCreation: Date.now() - 10800000 // 3 hours ago
  }
])

// 工具函数
const getRankedVariant = (tier: string) => {
  const tierMap: Record<string, string> = {
    IRON: 'outline',
    BRONZE: 'secondary',
    SILVER: 'outline',
    GOLD: 'default',
    PLATINUM: 'default',
    EMERALD: 'default',
    DIAMOND: 'default',
    MASTER: 'destructive',
    GRANDMASTER: 'destructive',
    CHALLENGER: 'destructive'
  }
  return tierMap[tier] || 'outline'
}

const getRankedDisplay = (tier: string, division?: string) => {
  const tierNames: Record<string, string> = {
    IRON: '坚韧黑铁',
    BRONZE: '英勇黄铜',
    SILVER: '不屈白银',
    GOLD: '荣耀黄金',
    PLATINUM: '华贵铂金',
    EMERALD: '流光翡翠',
    DIAMOND: '璀璨钻石',
    MASTER: '超凡大师',
    GRANDMASTER: '傲世宗师',
    CHALLENGER: '最强王者'
  }

  const divisionNames: Record<string, string> = {
    I: 'I',
    II: 'II',
    III: 'III',
    IV: 'IV'
  }

  const tierName = tierNames[tier] || tier
  const divisionName = division ? divisionNames[division] : ''

  return division ? `${tierName} ${divisionName}` : tierName
}

const getWinRateColor = (winRate: number) => {
  if (winRate >= 60) return 'text-green-600 dark:text-green-400'
  if (winRate >= 50) return 'text-yellow-600 dark:text-yellow-400'
  return 'text-red-600 dark:text-red-400'
}

const calculateKDA = (kda: { kills: number; deaths: number; assists: number }) => {
  const ratio = kda.deaths === 0 ? kda.kills + kda.assists : (kda.kills + kda.assists) / kda.deaths
  return ratio.toFixed(1)
}

const formatGameDuration = (duration: number) => {
  const minutes = Math.floor(duration / 60000)
  const seconds = Math.floor((duration % 60000) / 1000)
  return `${minutes}:${seconds.toString().padStart(2, '0')}`
}

const formatGameMode = (mode: string) => {
  const modeMap: Record<string, string> = {
    CLASSIC: '召唤师峡谷',
    ARAM: '极地大乱斗',
    URF: '无限火力',
    ONEFORALL: '克隆大作战'
  }
  return modeMap[mode] || mode
}

const formatRelativeTime = (timestamp: number) => {
  const now = Date.now()
  const diff = now - timestamp
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(hours / 24)

  if (days > 0) return `${days}天前`
  if (hours > 0) return `${hours}小时前`
  return '刚刚'
}

onMounted(() => {
  // 在实际应用中，这里应该根据 summonerId 获取数据
  console.log('Loading summoner details for:', props.summonerId)
})
</script>
