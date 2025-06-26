<template>
  <div class="flex items-center gap-3 px-4 py-2 rounded-lg border-2 bg-background/50 backdrop-blur-sm shadow-sm">
    <!-- 连接状态指示器 -->
    <div class="flex items-center gap-2">
      <div :class="['animate-pulse h-2 w-2 rounded-full shadow-sm', isConnected ? 'bg-green-500' : 'bg-red-500']" />
      <span class="text-sm font-medium text-foreground">
        {{ isConnected ? summonerInfo?.displayName || '未知召唤师' : '未连接' }}
      </span>
    </div>

    <!-- 等级和段位信息 -->
    <div v-if="isConnected && summonerInfo" class="flex items-center gap-2 text-sm">
      <div class="h-4 w-px bg-muted-foreground/30" />
      <span class="text-foreground/80">等级 {{ summonerInfo.summonerLevel }}</span>
      <span v-if="summonerInfo.soloRankTier" class="text-foreground/80">
        {{ formatRankTier(summonerInfo.soloRankTier) }} {{ summonerInfo.soloRankDivision }}
      </span>
    </div>

    <!-- 连接/刷新按钮 -->
    <button
      @click="refreshConnection"
      :class="[
        'ml-2 px-3 py-1 text-xs rounded hover:bg-primary/90 transition-colors',
        isConnected 
          ? 'bg-green-600 text-white hover:bg-green-700' 
          : 'bg-primary text-primary-foreground'
      ]"
    >
      {{ isConnected ? '刷新' : '重新连接' }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { useSummonerStore, useMatchStatisticsStore, useConnectionStore } from '@/stores'

// 直接从 store 获取状态
const summonerStore = useSummonerStore()
const matchStatisticsStore = useMatchStatisticsStore()
const connectionStore = useConnectionStore()

// 从store中解构响应式状态
const { summonerInfo } = storeToRefs(summonerStore)
const { isConnected } = storeToRefs(connectionStore)

// 手动刷新
const refreshConnection = async () => {
  try {
    await summonerStore.fetchSummonerInfo()
    await matchStatisticsStore.fetchMatchHistory()
  } catch (error) {
    console.error('手动刷新失败:', error)
  }
}

// 格式化段位
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
</script>
