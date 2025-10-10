<template>
  <div class="flex gap-3 h-screen max-h-screen overflow-hidden">
    <!-- 左侧：我方队伍 -->
    <div class="flex-1 flex flex-col min-w-0">
      <GamePhaseHandler
        :current-phase="currentPhase"
        :team-count="session?.myTeam?.length || 0"
        :show-filter-button="true"
        :has-live-client-data="hasLiveClientData"
        :cached-champ-select-data="cachedChampSelectData"
        @toggle-filter="$emit('toggle-filter')"
      />

      <div
        class="flex-1 scrollbar-thin scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar-thumb-slate-400/50 dark:scrollbar-thumb-slate-500/50 scrollbar-track-transparent overflow-y-auto scroll-smooth"
      >
        <TeamCard
          v-if="session && session.myTeam"
          :team="session.myTeam"
          team-type="ally"
          :local-player-cell-id="session.localPlayerCellId"
          :summoner-stats="summonerStats || []"
          :hide-unqueryable="false"
          @select="openSummonerDetails"
        />
      </div>
    </div>

    <!-- 分割线 -->
    <div class="w-px bg-border/50"></div>

    <!-- 右侧：敌方队伍 -->
    <div class="flex-1 flex flex-col min-w-0">
      <GamePhaseHandler
        :current-phase="currentPhase"
        :team-count="session?.theirTeam?.length || 0"
        :show-filter-button="false"
        :has-live-client-data="hasLiveClientData"
        :cached-champ-select-data="cachedChampSelectData"
      />

      <div
        class="flex-1 scrollbar-thin scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar-thumb-slate-400/50 dark:scrollbar-thumb-slate-500/50 scrollbar-track-transparent overflow-y-auto scroll-smooth"
      >
        <TeamCard
          v-if="session && session.theirTeam"
          :team="session.theirTeam"
          team-type="enemy"
          :summoner-stats="theirTeamStats || []"
          :hide-unqueryable="false"
          @select="openSummonerDetails"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import GamePhaseHandler from './GamePhaseHandler.vue'
import TeamCard from '@/components/features/match/TeamCard.vue'
// import type { EnrichedChampSelectPlayer, EnrichedLivePlayer } from '@/types/handle.d'

interface Props {
  session: any
  currentPhase: string | null
  summonerStats: any[]
  theirTeamStats: any[]
  hasLiveClientData: boolean
  cachedChampSelectData: any
}

defineProps<Props>()

const emit = defineEmits<{
  'toggle-filter': []
  'open-summoner-details': [summoner: any, matchHistory: any]
}>()

// 召唤师详情相关状态
const selectedSummoner = ref<any>(null)
const selectedMatchHistory = ref<any>(null)

// 打开召唤师详情
const openSummonerDetails = (summoner: any, matchHistory: any) => {
  selectedSummoner.value = summoner
  selectedMatchHistory.value = matchHistory
  emit('open-summoner-details', summoner, matchHistory)
}

// 关闭逻辑交由父组件处理
</script>
