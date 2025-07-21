<template>
  <Card class="p-4 lg:p-6">
    <div class="flex items-center justify-between mb-4 lg:mb-6">
      <div class="flex items-center gap-3">
        <div :class="dotClass"></div>
        <h3 :class="titleClass">{{ title }}</h3>
      </div>
      <Badge :variant="'outline'" :class="badgeClass"> {{ team.length }} 人 </Badge>
    </div>
    <div class="space-y-2 lg:space-y-3">
      <PlayerCard
        v-for="player in team"
        :key="player.summonerId + '-' + player.cellId"
        :player="player"
        :is-local="localPlayerCellId ? player.cellId === localPlayerCellId : false"
        :is-ally="teamType === 'ally'"
        @select="$emit('select', player)"
        class="cursor-pointer"
      />
    </div>
    <!-- <div class="mt-4 lg:mt-6 pt-4 border-t border-border">
      <TeamStats :team="team" :team-type="teamType" />
    </div> -->
  </Card>
</template>

<script setup lang="ts">
// 定义一个通用的接口，用于描述任何可以在卡片中显示的玩家信息
// 这使得 TeamCard 可以接收来自不同阶段（选人、游戏中）的玩家数据
interface PlayerDisplayInfo {
  summonerId: string | number // 在游戏中可能是 summonerName(string)，选人时是 summonerId(number)
  cellId?: number // 游戏内可能没有
  displayName: string
  championIcon?: string
  [key: string]: any // 允许其他属性，以兼容原始的 ChampSelectPlayer 等类型
}

const props = withDefaults(
  defineProps<{
    team: PlayerDisplayInfo[]
    teamType: 'ally' | 'enemy'
    localPlayerCellId?: number | null
  }>(),
  {
    team: () => []
  }
)

defineEmits(['select'])

const isAlly = computed(() => props.teamType === 'ally')

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
const title = computed(() => (isAlly.value ? '我方队伍' : '敌方队伍'))
</script>
