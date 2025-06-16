<template>
  <div
    class="relative flex items-center gap-3 rounded-xl p-2 min-h-[56px] cursor-pointer transition-all duration-200 select-none glass-card"
    :class="[
      isLocal ? 'ring-2 ring-primary' : '',
      selected ? 'ring-2 ring-accent shadow-lg scale-105' : '',
      'hover:ring-2 hover:ring-primary/80 hover:scale-105'
    ]"
    @click="emit('select', player)"
  >
    <!-- 头像 -->
    <div class="w-9 h-9 flex-shrink-0 relative">
      <img
        v-if="player.championId"
        :src="getChampionIconUrl(player.championId)"
        :alt="getChampionName(player.championId)"
        class="w-full h-full rounded-full border-2 border-white/40 object-cover"
      />
      <div v-else class="w-full h-full rounded-full bg-muted flex items-center justify-center">
        <!-- 使用SVG占位图标 -->
        <svg width="28" height="28" viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="14" cy="14" r="14" fill="#e5e7eb" />
          <path d="M14 15c2.761 0 5 2.239 5 5v1H9v-1c0-2.761 2.239-5 5-5Z" fill="#cbd5e1" />
          <circle cx="14" cy="11" r="4" fill="#cbd5e1" />
        </svg>
      </div>
      <span v-if="isLocal" class="absolute -top-1 -right-1 bg-primary text-white text-[10px] px-1 py-0.5 rounded shadow"
        >我</span
      >
    </div>

    <!-- 主信息 -->
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <span class="font-bold text-sm truncate text-primary">{{ player.displayName || '未知召唤师' }}</span>
        <span
          v-if="player.tier"
          class="text-xs rounded bg-accent/80 px-1 py-0.5 text-accent-foreground border border-accent"
          >{{ player.tier }}</span
        >
      </div>
      <div class="flex items-center gap-2 mt-0.5">
        <div class="flex flex-row justify-between w-full items-center">
          <span class="truncate text-xs text-muted-foreground">{{
            player.championId ? getChampionName(player.championId) : '未选英雄'
          }}</span>
          <Badge
            v-if="player.assignedPosition"
            class="ml-2 text-xs"
            :class="positionColorMap[player.assignedPosition] || 'bg-gray-300 text-gray-700'"
          >
            {{ player.assignedPosition }}
          </Badge>
          <Badge v-else class="ml-2 text-xs bg-gray-300 text-gray-700" variant="secondary">未知位置</Badge>
        </div>
      </div>
    </div>

    <!-- 技能 -->
    <div class="flex flex-col gap-1 ml-1">
      <template v-for="(spellId, idx) in [player.spell1Id ?? null, player.spell2Id ?? null]" :key="idx">
        <template v-if="getSpellMeta(spellId).icon">
          <img
            :src="getSpellMeta(spellId).icon"
            :alt="getSpellMeta(spellId).label"
            class="w-5 h-5 rounded"
            loading="lazy"
          />
        </template>
        <template v-else>
          <svg class="w-5 h-5 rounded bg-gray-100" viewBox="0 0 20 20" fill="none">
            <circle cx="10" cy="10" r="10" fill="#e5e7eb" />
            <path d="M10 5v5l3 3" stroke="#cbd5e1" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </template>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getChampionIconUrl, getChampionName, getSpellMeta } from '@/lib'
import { computed, defineEmits } from 'vue'
import { Badge } from '@/components/ui/badge'
import { ChampSelectPlayer } from '@/types/global'

const props = defineProps<{
  player: ChampSelectPlayer
  isLocal?: boolean
  selected?: boolean
}>()

const emit = defineEmits(['select'])

// 位置与颜色映射
const positionColorMap: Record<string, string> = {
  上单: 'bg-blue-500 text-white',
  打野: 'bg-green-500 text-white',
  中单: 'bg-purple-500 text-white',
  ADC: 'bg-yellow-500 text-white',
  辅助: 'bg-pink-500 text-white'
}
</script>

<style scoped>
.glass-card {
  background: rgba(255, 255, 255, 0.18);
  backdrop-filter: blur(8px);
  border: 1.5px solid rgba(255, 255, 255, 0.25);
  box-shadow: 0 2px 8px 0 rgba(0, 0, 0, 0.08);
  transition:
    box-shadow 0.2s,
    border-color 0.2s,
    transform 0.2s;
}
.dark .glass-card {
  background: rgba(30, 32, 40, 0.28);
  border-color: rgba(255, 255, 255, 0.1);
}
</style>
