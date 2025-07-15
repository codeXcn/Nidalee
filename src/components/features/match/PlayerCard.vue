<template>
  <div
    class="relative flex items-center gap-3 rounded-xl p-3 min-h-[60px] cursor-pointer select-none group"
    :class="[
      // 基础样式
      'transition-all duration-300 ease-out',
      'hover:shadow-lg hover:-translate-y-0.5',

      // 当前玩家高亮
      isLocal
        ? [
            'bg-gradient-to-r from-primary/10 via-primary/5 to-transparent',
            'border-2 border-primary/40 shadow-primary/20 shadow-md',
            'dark:from-primary/15 dark:via-primary/8 dark:border-primary/50'
          ]
        : ['bg-card/60 backdrop-blur-sm border border-border/60', 'hover:bg-card/80 hover:border-border'],

      // 队伍色彩
      isAlly
        ? ['hover:shadow-blue-500/20', isLocal ? '' : 'hover:border-blue-300/60 dark:hover:border-blue-600/60']
        : ['hover:shadow-red-500/20', isLocal ? '' : 'hover:border-red-300/60 dark:hover:border-red-600/60']
    ]"
    style="will-change: transform, box-shadow"
    @click="emit('select', player)"
  >
    <!-- 头像 -->
    <div class="w-10 h-10 flex-shrink-0 relative">
      <div
        class="w-full h-full rounded-full overflow-hidden ring-2 ring-background group-hover:ring-4 transition-all duration-300"
        :class="isLocal ? 'ring-primary/60 group-hover:ring-primary/80' : 'ring-border group-hover:ring-border'"
      >
        <img
          v-if="player.championId"
          :src="getChampionIconUrl(player.championId)"
          :alt="getChampionName(player.championId)"
          class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-110"
        />
        <div v-else class="w-full h-full bg-muted flex items-center justify-center">
          <!-- 使用SVG占位图标 -->
          <svg width="28" height="28" viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="14" cy="14" r="14" fill="currentColor" class="text-muted-foreground/30" />
            <path
              d="M14 15c2.761 0 5 2.239 5 5v1H9v-1c0-2.761 2.239-5 5-5Z"
              fill="currentColor"
              class="text-muted-foreground/50"
            />
            <circle cx="14" cy="11" r="4" fill="currentColor" class="text-muted-foreground/50" />
          </svg>
        </div>
      </div>

      <!-- 当前玩家标识 -->
      <div
        v-if="isLocal"
        class="absolute -top-1 -right-1 bg-primary text-primary-foreground text-[10px] px-1.5 py-0.5 rounded-full shadow-lg animate-pulse"
      >
        <span class="font-bold">我</span>
      </div>
    </div>

    <!-- 主信息 -->
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <span
          class="font-bold text-sm truncate transition-colors duration-300"
          :class="isLocal ? 'text-primary' : 'text-foreground group-hover:text-foreground'"
        >
          {{ player.displayName || '未知召唤师' }}
        </span>
        <span
          v-if="player.tier"
          class="text-xs rounded-md bg-secondary/80 px-2 py-0.5 text-secondary-foreground border border-secondary transition-all duration-300 group-hover:bg-secondary group-hover:shadow-sm"
        >
          {{ player.tier }}
        </span>
      </div>
      <div class="flex items-center gap-2 mt-1">
        <div class="flex flex-row justify-between w-full items-center">
          <span
            class="truncate text-xs text-muted-foreground transition-colors duration-300 group-hover:text-foreground/80"
          >
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
            class="ml-2 text-xs transition-all duration-300 group-hover:shadow-sm"
            :class="positionColorMap[player.assignedPosition] || 'bg-secondary text-secondary-foreground'"
          >
            {{ player.assignedPosition }}
          </Badge>
          <Badge v-else class="ml-2 text-xs bg-secondary text-secondary-foreground" variant="secondary">未知位置</Badge>
        </div>
      </div>
    </div>

    <!-- 技能 -->
    <div class="flex flex-col gap-1.5 ml-1">
      <template v-for="(spellId, idx) in [player.spell1Id ?? null, player.spell2Id ?? null]" :key="idx">
        <div
          class="w-5 h-5 rounded-md overflow-hidden ring-1 ring-border/60 group-hover:ring-2 transition-all duration-300"
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
</template>

<script setup lang="ts">
import { getChampionIconUrl, getChampionName, getSpellMeta } from '@/lib'

const props = defineProps<{
  player: ChampSelectPlayer
  isLocal?: boolean
  isAlly?: boolean
  selected?: boolean
}>()

const emit = defineEmits(['select'])

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
</script>
