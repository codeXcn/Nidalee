<template>
  <Card :class="cardClass" :style="`animation-delay: ${animationDelay}ms`">
    <div class="flex items-center justify-between mb-4 lg:mb-6">
      <div class="flex items-center gap-3">
        <div :class="dotClass"></div>
        <h3 :class="titleClass">{{ title }}</h3>
      </div>
      <Badge :variant="'outline'" :class="badgeClass"> {{ team.length }} 人 </Badge>
    </div>
    <div class="space-y-2 lg:space-y-3">
      <PlayerCard
        v-for="(player, index) in team"
        :key="player.summonerId + '-' + player.cellId"
        :player="player"
        :is-local="localPlayerCellId ? player.cellId === localPlayerCellId : false"
        :is-ally="teamType === 'ally'"
        @select="$emit('select', player)"
        class="cursor-pointer animate-in fade-in-0 duration-500"
        :class="playerCardClass"
        :style="`animation-delay: ${playerDelay + index * 100}ms;`"
      />
    </div>
    <div class="mt-4 lg:mt-6 pt-4 border-t border-border">
      <TeamStats :team="team" :team-type="teamType" />
    </div>
  </Card>
</template>

<script setup lang="ts">
import TeamStats from '@/components/features/match-analysis/TeamStats.vue'
import PlayerCard from '@/components/features/match/PlayerCard.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Card from '@/components/ui/card/Card.vue'
import { computed } from 'vue'

declare interface ChampSelectPlayer {
  cellId: number
  summonerId?: string
  championId: number
  championPickIntent?: number
  selectedSkinId?: number
  spell1Id?: number
  spell2Id?: number
  assignedPosition?: string
  displayName?: string
  tier?: string
  winRate?: number
}

const props = defineProps<{
  team: ChampSelectPlayer[]
  teamType: 'ally' | 'enemy'
  localPlayerCellId?: number | null
}>()

const emit = defineEmits(['select'])

const isAlly = computed(() => props.teamType === 'ally')

const cardClass = computed(
  () =>
    `p-4 lg:p-6 animate-in fade-in-0 duration-500 ${isAlly.value ? 'slide-in-from-left-4' : 'slide-in-from-right-4'}`
)
const animationDelay = computed(() => (isAlly.value ? 100 : 200))
const playerDelay = computed(() => (isAlly.value ? 200 : 300))
const dotClass = computed(
  () => `w-3 h-3 lg:w-4 lg:h-4 rounded-full animate-pulse ${isAlly.value ? 'bg-blue-500' : 'bg-red-500'}`
)
const titleClass = computed(
  () =>
    `text-lg lg:text-xl font-bold ${isAlly.value ? 'text-blue-600 dark:text-blue-400' : 'text-red-600 dark:text-red-400'}`
)
const badgeClass = computed(() =>
  isAlly.value
    ? 'text-blue-600 dark:text-blue-400 border-blue-300 dark:border-blue-600'
    : 'text-red-600 dark:text-red-400 border-red-300 dark:border-red-600'
)
const playerCardClass = computed(() => (isAlly.value ? 'slide-in-from-left-2' : 'slide-in-from-right-2'))
const title = computed(() => (isAlly.value ? '我方队伍' : '敌方队伍'))
</script>
