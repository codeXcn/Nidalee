<template>
  <div
    class="flex items-center gap-3 px-4 py-2 rounded-lg border-2 bg-background/50 backdrop-blur-sm shadow-sm"
  >
    <!-- 连接状态指示器 -->
    <div class="flex items-center gap-2">
      <div
        :class="[
          'h-2 w-2 rounded-full shadow-sm',
          connectionStatus === 'connected'
            ? 'bg-green-500'
            : connectionStatus === 'connecting'
              ? 'bg-yellow-500 animate-pulse'
              : 'bg-red-500'
        ]"
      />
      <span class="text-sm font-medium text-foreground">
        {{ isConnected ? summonerInfo?.displayName || '未知召唤师' : statusText }}
      </span>
    </div>

    <!-- 等级和段位信息 -->
    <div
      v-if="isConnected && summonerInfo"
      class="flex items-center gap-2 text-sm"
    >
      <div class="h-4 w-px bg-muted-foreground/30" />
      <span class="text-foreground/80">等级 {{ summonerInfo.summonerLevel }}</span>
      <span v-if="summonerInfo.soloRankTier" class="text-foreground/80">
        {{ formatRankTier(summonerInfo.soloRankTier) }} {{ summonerInfo.soloRankDivision }}
      </span>
    </div>

    <!-- 连接按钮 -->
    <button
      v-if="!isConnected && !isConnecting"
      @click="attemptConnection"
      class="ml-2 px-3 py-1 text-xs bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors"
    >
      连接
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useGameStore } from '@/stores/gameStore'
import { useGameMonitor } from '@/composables/useGameMonitor'

// 使用store和监控
const gameStore = useGameStore()
const { attemptConnection } = useGameMonitor()

// 从store中解构响应式状态
const { isConnected, isConnecting, summonerInfo, connectionStatus } = storeToRefs(gameStore)

// 状态文本
const statusText = computed(() => {
  switch (connectionStatus.value) {
    case 'connected':
      return '已连接'
    case 'connecting':
      return '连接中...'
    case 'disconnected':
    default:
      return '未连接'
  }
})

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
