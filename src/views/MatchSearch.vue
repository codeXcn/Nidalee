<template>
  <div class="flex flex-col gap-4">
    <SummonerSearchBox v-model:summoner-name="searchText" @on-search="onSearch" />
    <!-- names数组的tags（shadcn-vue Badge + tailwind） -->
    <div v-if="names.length" class="mb-2 flex gap-2 flex-wrap">
      <Badge
        v-for="(name, idx) in names"
        :key="name"
        :class="[
          'cursor-pointer select-none transition',
          idx === cunrrentIndex ? 'bg-primary text-primary-foreground shadow' : 'bg-muted text-muted-foreground'
        ]"
        @click="cunrrentIndex = idx"
      >
        {{ name }}
      </Badge>
    </div>
    <!-- 用户信息卡片 -->
    <SummonerCard v-if="currentRestult" :summoner-info="currentRestult?.summonerInfo" />

    <!-- 游戏统计 -->
    <GameStats
      v-if="currentRestult"
      :is-connected="isConnected"
      :match-history-loading="loading"
      :match-statistics="currentRestult?.matches"
    />
  </div>
</template>
<script lang="ts" setup>
import { Badge } from '@/components/ui/badge'
import { useSearchMatches } from '@/hooks/useSearchMatches'
import { useGameStore } from '@/stores'

const { onSearch, cunrrentIndex, names, searchText, loading, currentRestult } = useSearchMatches()
console.log(loading.value)
const { isConnected } = storeToRefs(useGameStore())
</script>
