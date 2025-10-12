<template>
  <Card class="border-none px-0.5 py-0">
    <div class="space-y-0.5">
      <CompactPlayerCard
        v-for="(player, index) in teamData.players"
        :key="(player.displayName || player.summonerId || player.puuid || index) + '-' + index"
        :player="player"
        :player-stats="getPlayerStats(index)"
        :is-local="player.cellId === teamData.localPlayerCellId"
        @select="$emit('select-player', player, getPlayerStats(index))"
      />
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Card } from '@/components/ui/card'
import CompactPlayerCard from './CompactPlayerCard.vue'
import { computed, watchEffect } from 'vue'

const props = withDefaults(
  defineProps<{
    teamData: {
      players: any[]
      localPlayerCellId?: number
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

// 监控 props.teamStats 的变化
watchEffect(() => {
  console.log(`[TeamAnalysisCard] ${props.teamType} teamStats 更新:`, {
    length: props.teamStats?.length,
    isArray: Array.isArray(props.teamStats),
    stats: props.teamStats,
    players: props.teamData.players.map((p, i) => ({ index: i, name: p.displayName, cellId: p.cellId }))
  })
})

// 🔥 性能优化：预先匹配所有玩家的战绩，避免重复计算
const playerStatsMap = computed(() => {
  if (!props.teamStats || props.teamStats.length === 0) {
    return new Map()
  }

  const map = new Map()

  props.teamData.players.forEach((player, index) => {
    if (!player) return

    // 通过 displayName、puuid 或 cellId 匹配战绩
    const matchedStats = props.teamStats!.find((stats) => {
      if (!stats) return false

      // 优先通过 displayName 匹配
      if (stats.displayName && player.displayName) {
        return stats.displayName.toLowerCase() === player.displayName.toLowerCase()
      }

      // 备选：通过 puuid 匹配
      if ((stats as any).puuid && player.puuid) {
        return (stats as any).puuid === player.puuid
      }

      // 备选：通过 cellId 匹配
      if ((stats as any).cellId !== undefined && player.cellId !== undefined) {
        return (stats as any).cellId === player.cellId
      }

      return false
    })

    if (matchedStats) {
      map.set(index, matchedStats)
      console.log(`[TeamAnalysisCard] ✅ 玩家 "${player.displayName}" (index=${index}) 匹配到战绩`)
    } else {
      console.log(`[TeamAnalysisCard] ❌ 玩家 "${player.displayName}" (index=${index}) 未找到匹配的战绩`)
    }
  })

  return map
})

// 🔥 优化后：直接从缓存的 Map 中获取（不会重复计算）
const getPlayerStats = (index: number) => {
  return playerStatsMap.value.get(index) || null
}
</script>
