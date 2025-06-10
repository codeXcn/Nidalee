<template>
  <Card v-if="summonerInfo" class="overflow-hidden py-0">
    <!-- 头部渐变背景 -->
    <div class="bg-gradient-to-br from-blue-500 via-purple-600 to-indigo-700 p-6 text-white">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <!-- 头像 -->
          <div class="relative">
            <div class="h-20 w-20 rounded-full bg-white/20 backdrop-blur-sm border-2 border-white/30 overflow-hidden">
              <img
                v-if="summonerInfo.profileIconId && !imageLoadError"
                :src="getProfileIconUrl(summonerInfo.profileIconId)"
                :alt="`${summonerInfo.displayName} 的头像`"
                class="w-full h-full object-cover transition-opacity duration-300"
                :class="{ 'opacity-0': imageLoading }"
                @error="handleImageError"
                @load="handleImageLoad"
              />

              <!-- 加载中的骨架屏 -->
              <div
                v-if="imageLoading && summonerInfo.profileIconId && !imageLoadError"
                class="absolute inset-0 w-full h-full flex items-center justify-center"
              >
                <div class="w-6 h-6 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
              </div>

              <!-- 备用显示（无头像ID或加载失败时） -->
              <div
                v-if="!summonerInfo.profileIconId || imageLoadError || (!imageLoading && imageLoadError)"
                class="w-full h-full flex items-center justify-center text-white font-bold text-2xl"
              >
                {{ summonerInfo.displayName }}
              </div>
            </div>
            <div
              class="absolute -bottom-1 -right-1 h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center text-white text-sm font-bold border-2 border-white"
            >
              {{ summonerInfo.summonerLevel }}
            </div>
            <!-- 挑战水晶等级 -->
            <div
              v-if="summonerInfo.challengeCrystalLevel"
              class="absolute -top-1 -left-1 h-6 w-6 rounded-full bg-yellow-500 flex items-center justify-center text-white text-xs font-bold border border-white"
            >
              {{ getChallengeIcon(summonerInfo.challengeCrystalLevel) }}
            </div>
          </div>

          <!-- 基本信息 -->
          <div>
            <h2 class="text-2xl font-bold text-white">{{ summonerInfo.displayName }}</h2>
            <p class="text-white/80">等级 {{ summonerInfo.summonerLevel }} 召唤师</p>
            <div class="flex items-center space-x-2 mt-2">
              <div class="h-2 w-2 rounded-full bg-green-400 animate-pulse"></div>
              <span class="text-white/90 font-medium">已连接</span>
              <span v-if="summonerInfo.availability" class="text-white/70 text-sm">
                • {{ formatAvailability(summonerInfo.availability) }}
              </span>
            </div>
          </div>
        </div>

        <!-- 挑战点数和会话时长 -->
        <div class="text-right text-white">
          <div v-if="summonerInfo.challengePoints" class="mb-2">
            <p class="text-white/80 text-sm">挑战点数</p>
            <p class="text-xl font-bold">
              {{ formatChallengePoints(summonerInfo.challengePoints) }}
            </p>
          </div>
          <div>
            <p class="text-white/80 text-sm">会话时长</p>
            <p class="text-xl font-bold">{{ sessionDuration }}</p>
          </div>
        </div>
      </div>

      <!-- 经验条 -->
      <div v-if="summonerInfo.percentCompleteForNextLevel" class="mt-4">
        <div class="flex justify-between text-white/80 text-sm mb-1">
          <span>升级进度</span>
          <span>{{ summonerInfo.percentCompleteForNextLevel }}%</span>
        </div>
        <div class="w-full bg-white/20 rounded-full h-2">
          <div
            class="bg-white rounded-full h-2 transition-all duration-300"
            :style="{ width: `${summonerInfo.percentCompleteForNextLevel}%` }"
          ></div>
        </div>
        <div class="flex justify-between text-white/60 text-xs mt-1">
          <span>{{ summonerInfo.xpSinceLastLevel }} XP</span>
          <span>还需 {{ summonerInfo.xpUntilNextLevel }} XP</span>
        </div>
      </div>
    </div>

    <!-- 排位信息部分 -->
    <div class="p-6 bg-background">
      <h3 class="text-lg font-semibold mb-4 flex items-center">
        <Trophy class="h-5 w-5 mr-2 text-yellow-500" />
        排位统计
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- 单人排位 -->
        <div class="space-y-3">
          <h4 class="font-medium text-foreground flex items-center">
            <User class="h-4 w-4 mr-2" />
            单人排位
          </h4>
          <div v-if="summonerInfo.soloRankTier" class="space-y-2">
            <div class="flex items-center space-x-3">
              <div class="flex items-center space-x-3">
                <div :class="['px-3 py-1.5 rounded-lg text-sm font-bold', getRankColor(summonerInfo.soloRankTier)]">
                  {{ formatRankTier(summonerInfo.soloRankTier) }}
                  {{ summonerInfo.soloRankDivision }}
                </div>
                <span class="text-sm text-muted-foreground">{{ summonerInfo.soloRankLP }} LP</span>
              </div>
            </div>
            <div class="flex items-center space-x-4 text-sm">
              <span class="text-green-600 dark:text-green-400 font-medium">{{ summonerInfo.soloRankWins }}胜</span>
              <span class="text-red-600 dark:text-red-400 font-medium">{{ summonerInfo.soloRankLosses }}负</span>
              <span class="text-muted-foreground">
                胜率 {{ getRankWinRate(summonerInfo.soloRankWins, summonerInfo.soloRankLosses) }}%
              </span>
            </div>
          </div>
          <div v-else class="text-sm text-muted-foreground">
            <div class="flex items-center">
              <Shield class="h-4 w-4 mr-2" />
              <span>未定级</span>
            </div>
          </div>
        </div>

        <!-- 灵活排位 -->
        <div class="space-y-3">
          <h4 class="font-medium text-foreground flex items-center">
            <Users class="h-4 w-4 mr-2" />
            灵活排位
          </h4>
          <div v-if="summonerInfo.flexRankTier" class="space-y-2">
            <div class="flex items-center space-x-3">
              <div class="flex items-center space-x-3">
                <div :class="['px-3 py-1.5 rounded-lg text-sm font-bold', getRankColor(summonerInfo.flexRankTier)]">
                  {{ formatRankTier(summonerInfo.flexRankTier) }}
                  {{ summonerInfo.flexRankDivision }}
                </div>
                <span class="text-sm text-muted-foreground">{{ summonerInfo.flexRankLP }} LP</span>
              </div>
            </div>
            <div class="flex items-center space-x-4 text-sm">
              <span class="text-green-600 dark:text-green-400 font-medium">{{ summonerInfo.flexRankWins }}胜</span>
              <span class="text-red-600 dark:text-red-400 font-medium">{{ summonerInfo.flexRankLosses }}负</span>
              <span class="text-muted-foreground">
                胜率 {{ getRankWinRate(summonerInfo.flexRankWins, summonerInfo.flexRankLosses) }}%
              </span>
            </div>
          </div>
          <div v-else class="text-sm text-muted-foreground">
            <div class="flex items-center">
              <Shield class="h-4 w-4 mr-2" />
              <span>未定级</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 游戏状态和历史最高 -->
      <div class="flex items-center justify-between mt-6 pt-4 border-t border-border">
        <div v-if="summonerInfo.gameStatus" class="flex items-center space-x-2">
          <div class="h-2 w-2 rounded-full bg-green-500 animate-pulse"></div>
          <span :class="['px-3 py-1 rounded-full text-sm font-medium', getGameStatusColor(summonerInfo.gameStatus)]">
            {{ formatGameStatus(summonerInfo.gameStatus) }}
          </span>
        </div>

        <div v-if="summonerInfo.highestRankThisSeason" class="text-sm text-muted-foreground">
          赛季最高: {{ formatRankTier(summonerInfo.highestRankThisSeason) }}
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Card } from '@/components/ui/card'
import { User, Users, Shield, Trophy } from 'lucide-vue-next'
import { useFormatters } from '@/hooks/useFormatters'
import { useGameAssets } from '@/hooks/useGameAssets'

const props = defineProps<{
  summonerInfo: any
  sessionDuration: string
}>()

const { getProfileIconUrl } = useGameAssets()
const { formatChallengePoints } = useFormatters()

// 头像相关状态
const imageLoadError = ref(false)
const imageLoading = ref(true)

// 处理图片加载错误
const handleImageError = (event: Event): void => {
  console.warn('头像加载失败:', event)
  imageLoadError.value = true
  imageLoading.value = false
}

// 处理图片加载成功
const handleImageLoad = (): void => {
  imageLoadError.value = false
  imageLoading.value = false
}

// 获取挑战水晶图标
const getChallengeIcon = (level: string): string => {
  const iconMap: Record<string, string> = {
    IRON: '🥉',
    BRONZE: '🥉',
    SILVER: '🥈',
    GOLD: '🥇',
    PLATINUM: '💎',
    DIAMOND: '💎',
    MASTER: '👑',
    GRANDMASTER: '👑',
    CHALLENGER: '⭐'
  }
  return iconMap[level] || '🏆'
}

// 格式化排位等级
const formatRankTier = (tier: string): string => {
  const tierMap: Record<string, string> = {
    IRON: '坚韧黑铁',
    BRONZE: '英勇青铜',
    SILVER: '不屈白银',
    GOLD: '荣耀黄金',
    PLATINUM: '华贵铂金',
    EMERALD: '流光翡翠',
    DIAMOND: '璀璨钻石',
    MASTER: '超凡大师',
    GRANDMASTER: '傲世宗师',
    CHALLENGER: '最强王者'
  }
  return tierMap[tier] || tier
}

// 获取排位颜色
const getRankColor = (tier: string): string => {
  const colorMap: Record<string, string> = {
    IRON: 'bg-zinc-500/20 text-zinc-600 dark:text-zinc-400',
    BRONZE: 'bg-orange-500/20 text-orange-600 dark:text-orange-400',
    SILVER: 'bg-slate-500/20 text-slate-600 dark:text-slate-400',
    GOLD: 'bg-yellow-500/20 text-yellow-600 dark:text-yellow-400',
    PLATINUM: 'bg-cyan-500/20 text-cyan-600 dark:text-cyan-400',
    EMERALD: 'bg-emerald-500/20 text-emerald-600 dark:text-emerald-400',
    DIAMOND: 'bg-blue-500/20 text-blue-600 dark:text-blue-400',
    MASTER: 'bg-purple-500/20 text-purple-600 dark:text-purple-400',
    GRANDMASTER: 'bg-red-500/20 text-red-600 dark:text-red-400',
    CHALLENGER: 'bg-yellow-500/20 text-yellow-600 dark:text-yellow-400'
  }
  return colorMap[tier] || 'bg-gray-500/20 text-gray-600 dark:text-gray-400'
}

// 计算胜率
const getRankWinRate = (wins?: number, losses?: number): number => {
  if (!wins && !losses) return 0
  const totalGames = (wins || 0) + (losses || 0)
  if (totalGames === 0) return 0
  return Math.round(((wins || 0) / totalGames) * 100)
}

// 格式化游戏状态
const formatGameStatus = (status: string): string => {
  const statusMap: Record<string, string> = {
    hosting_RANKED_SOLO_5x5: '排位单双',
    hosting_NORMAL: '匹配模式',
    hosting_ARAM: '大乱斗',
    inGame: '游戏中',
    outOfGame: '客户端',
    away: '离开',
    online: '在线'
  }
  return statusMap[status] || status
}

// 获取游戏状态颜色
const getGameStatusColor = (status: string): string => {
  if (status.includes('hosting') || status === 'inGame') {
    return 'bg-green-500/20 text-green-600 dark:text-green-400'
  }
  if (status === 'away') {
    return 'bg-yellow-500/20 text-yellow-600 dark:text-yellow-400'
  }
  return 'bg-blue-500/20 text-blue-600 dark:text-blue-400'
}

// 格式化可用性状态
const formatAvailability = (availability: string): string => {
  const availabilityMap: Record<string, string> = {
    chat: '可聊天',
    away: '离开',
    dnd: '勿扰',
    online: '在线',
    mobile: '手机在线',
    offline: '离线'
  }
  return availabilityMap[availability] || availability
}
</script>
