<template>
  <Card v-if="summonerInfo" class="overflow-hidden py-0">
    <!-- 头部渐变背景 -->
    <div
      class="p-6 text-white"
      :style="{
        background:
          $colorMode === 'dark'
            ? 'linear-gradient(135deg, var(--color-primary, #f59e42) 0%, #312e81 100%)'
            : 'linear-gradient(135deg, var(--color-primary, #f59e42) 0%, #7c3aed 100%)'
      }"
    >
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
            <p class="text-white/80 text-sm">累计活跃时长</p>
            <p class="text-xl font-bold">{{ sessionStore.formattedTotal }}</p>
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

      <div class="grid grid-cols-1 md:grid-cols-2 gap-10 md:gap-20 justify-items-center items-start">
        <!-- 单人排位 -->
        <div class="space-y-3 flex flex-col items-center min-w-[260px] max-w-[320px] w-full">
          <h4 class="font-medium text-foreground flex items-center justify-center">
            <User class="h-4 w-4 mr-2" />
            单人排位
          </h4>
          <div v-if="summonerInfo.soloRankTier" class="space-y-2">
            <div class="flex items-center space-x-4">
              <img
                v-if="summonerInfo.soloRankTier"
                :src="getTierIconUrl(summonerInfo.soloRankTier)"
                :alt="formatRankTier(summonerInfo.soloRankTier)"
                class="w-20 h-20 rounded-full border-2 shadow-lg transition-all duration-500 breath-glow"
                :style="getRankGlowBreathStyle(summonerInfo.soloRankTier)"
              />
              <div class="flex flex-col justify-center min-w-[140px]">
                <div class="flex items-center gap-2">
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger as-child>
                        <span
                          :class="`px-3 py-1 text-lg font-semibold rounded-2xl select-none shadow-lg border-2 transition-all duration-300 ${getRankColor(summonerInfo.soloRankTier)}`"
                          :style="getBadgeStyle(summonerInfo.soloRankTier)"
                        >
                          <span
                            style="
                              text-shadow:
                                0 2px 8px rgba(0, 0, 0, 0.18),
                                0 1px 0 #fff;
                            "
                            >{{ formatRankTier(summonerInfo.soloRankTier) }}</span
                          >
                        </span>
                      </TooltipTrigger>
                      <TooltipContent side="right">
                        {{ _getTierTooltip(summonerInfo.soloRankTier) }}
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                  <span class="text-base font-medium text-gray-300 tracking-wider ml-1 opacity-80">{{
                    summonerInfo.soloRankDivision
                  }}</span>
                </div>
                <span
                  class="mt-2 inline-flex items-center px-3 py-1 rounded-xl border-2 shadow-md font-semibold text-sm tracking-wide min-w-[110px] max-w-[140px] w-full justify-center"
                  :style="getLpBadgeStyle(summonerInfo.soloRankTier)"
                >
                  <svg class="w-4 h-4 mr-1 opacity-90" fill="none" viewBox="0 0 20 20">
                    <path
                      d="M10 2l2.39 4.84 5.34.78-3.87 3.77.91 5.32L10 13.77l-4.77 2.51.91-5.32-3.87-3.77 5.34-.78L10 2z"
                      fill="currentColor"
                    />
                  </svg>
                  <span class="text-base font-bold mx-1">{{ summonerInfo.soloRankLp }}</span>
                  <span class="ml-1 text-xs font-medium opacity-70">LP</span>
                </span>
              </div>
            </div>
            <div
              v-if="isDashboard"
              class="flex items-center space-x-6 mt-2 border-t border-dashed border-gray-300/40 pt-2"
            >
              <span class="text-green-600 text-base font-bold">{{ summonerInfo.soloRankWins }}胜</span>
              <span class="text-red-500 text-base font-bold">{{ summonerInfo.soloRankLosses }}负</span>
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <span class="text-yellow-600 text-base font-bold flex items-center cursor-pointer">
                      <span class="mr-1">{{
                        getWinRateStyle(getRankWinRate(summonerInfo.soloRankWins, summonerInfo.soloRankLosses)).icon
                      }}</span>
                      胜率 {{ getRankWinRate(summonerInfo.soloRankWins, summonerInfo.soloRankLosses) }}%
                    </span>
                  </TooltipTrigger>
                  <TooltipContent side="right">
                    {{ getWinRateStyle(getRankWinRate(summonerInfo.soloRankWins, summonerInfo.soloRankLosses)).tip }}
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>
          </div>
          <div v-else class="text-sm text-muted-foreground">
            <div class="flex items-center">
              <Shield class="h-4 w-4 mr-2" />
              <span>未定级</span>
            </div>
          </div>
        </div>

        <!-- 灵活排位（镜像对称） -->
        <div class="space-y-3 flex flex-col items-center min-w-[260px] max-w-[320px] w-full">
          <h4 class="font-medium text-foreground flex items-center justify-center">
            <Users class="h-4 w-4 mr-2" />
            灵活排位
          </h4>
          <div v-if="summonerInfo.flexRankTier" class="space-y-2">
            <div class="flex flex-row-reverse items-center space-x-reverse space-x-4 w-full justify-end">
              <img
                v-if="summonerInfo.flexRankTier"
                :src="getTierIconUrl(summonerInfo.flexRankTier)"
                :alt="formatRankTier(summonerInfo.flexRankTier)"
                class="w-20 h-20 rounded-full border-2 shadow-lg transition-all duration-500 breath-glow"
                :style="getRankGlowBreathStyle(summonerInfo.flexRankTier)"
              />
              <div class="flex flex-col justify-center min-w-[140px] items-end text-right">
                <div class="flex flex-row-reverse items-center gap-2">
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger as-child>
                        <span
                          :class="`px-3 py-1 text-lg font-semibold rounded-2xl select-none shadow-lg border-2 transition-all duration-300 ${getRankColor(summonerInfo.flexRankTier)}`"
                          :style="getBadgeStyle(summonerInfo.flexRankTier)"
                        >
                          <span
                            style="
                              text-shadow:
                                0 2px 8px rgba(0, 0, 0, 0.18),
                                0 1px 0 #fff;
                            "
                            >{{ formatRankTier(summonerInfo.flexRankTier) }}</span
                          >
                        </span>
                      </TooltipTrigger>
                      <TooltipContent side="left">
                        {{ _getTierTooltip(summonerInfo.flexRankTier) }}
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                  <span class="text-base font-medium text-gray-300 tracking-wider mr-1 opacity-80">{{
                    summonerInfo.flexRankDivision
                  }}</span>
                </div>
                <span
                  class="mt-2 inline-flex flex-row-reverse items-center px-3 py-1 rounded-xl border-2 shadow-md font-semibold text-sm tracking-wide min-w-[110px] max-w-[140px] w-full justify-center"
                  :style="getLpBadgeStyle(summonerInfo.flexRankTier)"
                >
                  <svg class="w-4 h-4 ml-1 opacity-90" fill="none" viewBox="0 0 20 20">
                    <path
                      d="M10 2l2.39 4.84 5.34.78-3.87 3.77.91 5.32L10 13.77l-4.77 2.51.91-5.32-3.87-3.77 5.34-.78L10 2z"
                      fill="currentColor"
                    />
                  </svg>
                  <span class="text-base font-bold mx-1">{{ summonerInfo.flexRankLp }}</span>
                  <span class="mr-1 text-xs font-medium opacity-70">LP</span>
                </span>
              </div>
            </div>
            <div
              v-if="isDashboard"
              class="flex flex-row-reverse items-center space-x-reverse space-x-6 mt-2 border-t border-dashed border-gray-300/40 pt-2 w-full justify-end"
            >
              <span class="text-green-600 text-base font-bold">{{ summonerInfo.flexRankWins }}胜</span>
              <span class="text-red-500 text-base font-bold">{{ summonerInfo.flexRankLosses }}负</span>
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <span class="text-yellow-600 text-base font-bold flex flex-row-reverse items-center cursor-pointer">
                      <span class="ml-1">{{
                        getWinRateStyle(getRankWinRate(summonerInfo.flexRankWins, summonerInfo.flexRankLosses)).icon
                      }}</span>
                      胜率 {{ getRankWinRate(summonerInfo.flexRankWins, summonerInfo.flexRankLosses) }}%
                    </span>
                  </TooltipTrigger>
                  <TooltipContent side="left">
                    {{ getWinRateStyle(getRankWinRate(summonerInfo.flexRankWins, summonerInfo.flexRankLosses)).tip }}
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>
          </div>
          <div v-else class="text-sm text-muted-foreground w-full flex flex-row-reverse items-center justify-end">
            <Shield class="h-4 w-4 ml-2" />
            <span>未定级</span>
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
import { getTierIconUrl } from '@/lib'
import { Shield, Trophy, User, Users } from 'lucide-vue-next'
import { useRoute } from 'vue-router'
import { useSessionStore } from '@/stores/features/sessionStore'

const route = useRoute()
const isDashboard = computed(() => route.name === 'dashboard')

const props = defineProps<{
  summonerInfo: any
}>()
const { getProfileIconUrl } = useGameAssets()
const { formatChallengePoints } = useFormatters()
const sessionStore = useSessionStore()

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

// 胜率趣味风格（emoji方案）
const getWinRateStyle = (rate: number) => {
  if (rate >= 100) {
    return {
      color: 'text-pink-500',
      icon: '🔥',
      tip: '孩子，你无敌了！'
    }
  } else if (rate >= 70) {
    return {
      color: 'text-yellow-500',
      icon: '👑',
      tip: '原来是高手！'
    }
  } else if (rate >= 50) {
    return {
      color: 'text-green-600',
      icon: '👍',
      tip: '你很稳，保持住！'
    }
  } else {
    return {
      color: 'text-orange-500',
      icon: '💪',
      tip: '努把力，你能行！'
    }
  }
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

// 段位描述Tooltip
const getTierTooltip = (tier: string) => {
  const map: Record<string, string> = {
    IRON: '坚韧黑铁：万丈高楼平地起！',
    BRONZE: '英勇青铜：再接再厉，冲冲冲！',
    SILVER: '不屈白银：稳扎稳打，步步高升！',
    GOLD: '荣耀黄金：离梦想更进一步！',
    PLATINUM: '华贵铂金：高手如云，继续加油！',
    EMERALD: '流光翡翠：新赛季新气象！',
    DIAMOND: '璀璨钻石：你已是高端玩家！',
    MASTER: '超凡大师：巅峰对决，舍我其谁！',
    GRANDMASTER: '傲世宗师：顶尖中的顶尖！',
    CHALLENGER: '最强王者：你就是天选之子！'
  }
  return map[tier] || '加油，段位不是终点，享受游戏乐趣！'
}

const _getTierTooltip = getTierTooltip

// 段位主色发光映射
const rankGlowColorMap: Record<string, string> = {
  IRON: '#6e6e6e',
  BRONZE: '#b87333',
  SILVER: '#bfc1c2',
  GOLD: '#f7c873',
  PLATINUM: '#3fd8e0',
  EMERALD: '#34d399',
  DIAMOND: '#60a5fa',
  MASTER: '#a78bfa',
  GRANDMASTER: '#f87171',
  CHALLENGER: '#ffe066'
}
// 获取rank徽章发光style
const getRankGlowStyle = (tier: string) => {
  const color = rankGlowColorMap[tier] || '#a3a3a3'
  return {
    boxShadow: `0 0 0 2px #fff, 0 0 16px 4px ${color}, 0 0 32px 8px ${color}80`,
    borderColor: color,
    background: '#fff',
    transition: 'box-shadow 0.5s, border-color 0.5s'
  }
}

// 呼吸发光动画style（主色变量）
const getRankGlowBreathStyle = (tier: string) => {
  const color = rankGlowColorMap[tier] || '#a3a3a3'
  return {
    '--glow-color': color,
    '--glow-color-a': color + '80',
    borderColor: color,
    background: '#fff',
    transition: 'box-shadow 0.5s, border-color 0.5s'
  } as any
}

// 段位名badge主色渐变风格
const getBadgeStyle = (tier: string) => {
  const color = rankGlowColorMap[tier] || '#a3a3a3'
  return {
    background: `linear-gradient(135deg, ${color}, ${color}cc, ${color}aa)`, // 主色多层渐变
    color: '#fff',
    boxShadow: `0 3px 12px ${color}55, 0 0 0 1px ${color}`, // 阴影+描边
    border: 'none',
    filter: 'brightness(1.08)',
    letterSpacing: '0.02em',
    fontWeight: '700',
    textShadow: '0 1px 3px rgba(0,0,0,0.4)',
    transition: 'all 0.3s ease'
  }
}
// LP区块渐变风格
const getLpBadgeStyle = (tier: string) => {
  const color = rankGlowColorMap[tier] || '#f7c873'
  return {
    background: `linear-gradient(135deg, #ffffff, #f8fafc, #f1f5f9)`, // 白色到浅灰渐变
    color: color,
    border: `2px solid ${color}`,
    boxShadow: `0 3px 10px ${color}35`,
    filter: 'brightness(1.05)',
    letterSpacing: '0.01em',
    fontWeight: '700',
    textShadow: `0 1px 2px ${color}30`,
    transition: 'all 0.3s ease'
  }
}
</script>

<style scoped>
@keyframes breath-glow {
  0% {
    box-shadow:
      0 0 0 2px #fff,
      0 0 16px 4px var(--glow-color),
      0 0 32px 8px var(--glow-color-a);
  }
  50% {
    box-shadow:
      0 0 0 4px #fff,
      0 0 32px 12px var(--glow-color),
      0 0 64px 16px var(--glow-color-a);
  }
  100% {
    box-shadow:
      0 0 0 2px #fff,
      0 0 16px 4px var(--glow-color),
      0 0 32px 8px var(--glow-color-a);
  }
}
.breath-glow {
  animation: breath-glow 2.4s ease-in-out infinite;
}
</style>
