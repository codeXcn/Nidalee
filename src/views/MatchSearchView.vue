<template>
  <div class="flex flex-col gap-4">
    <SummonerSearchBox v-model:summoner-name="searchText" @on-search="onSearch" />
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
    <SummonerCard v-if="currentRestult" :summoner-info="currentRestult?.summonerInfo" />
    <GameStats
      v-if="currentRestult"
      :is-connected="isConnected"
      :match-history-loading="loading"
      :match-statistics="currentRestult?.matches"
    />
  </div>
</template>
<script lang="ts" setup>
import { appContextKey, type AppContext } from '@/types'

const { isConnected } = inject(appContextKey) as AppContext

const { onSearch, cunrrentIndex, names, searchText, loading, currentRestult } = useSearchMatches()
</script>
