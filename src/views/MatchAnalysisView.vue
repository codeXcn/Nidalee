<template>
  <div class="min-h-screen">
    <!-- 主要内容 -->
    <div v-if="session && shouldShowMatchAnalysis" class="w-full max-w-7xl mx-auto space-y-6">
      <!-- 队伍分析卡片 -->
      <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 lg:gap-8">
        <!-- 分层递进动画：动画只加在子卡片上 -->
        <TeamCard
          :team="session.myTeam"
          team-type="ally"
          :local-player-cell-id="session.localPlayerCellId"
          @select="openSummonerDetails"
        />
        <template v-if="session.theirTeam && session.theirTeam.length">
          <TeamCard :team="session.theirTeam" team-type="enemy" @select="openSummonerDetails" />
        </template>
        <template v-else>
          <div class="text-center text-muted-foreground mt-2">敌方队伍将在进入游戏后显示</div>
        </template>
      </div>
    </div>

    <!-- 无数据状态 -->
    <div v-else class="flex items-center justify-center h-64">
      <div class="text-center">
        <Info class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
        <h3 class="text-lg font-semibold mb-2 text-foreground">
          {{ getStatusTitle() }}
        </h3>
        <p class="text-muted-foreground px-4">
          {{ getStatusDescription() }}
        </p>
      </div>
    </div>
    <!-- 召唤师详情抽屉 -->
    <Sheet v-model:open="isDetailsOpen">
      <SheetContent class="w-[500px] sm:w-[700px] lg:w-[900px] xl:w-[1000px] overflow-y-auto p-0">
        <div
          class="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b border-border p-6"
        >
          <SheetHeader>
            <div class="flex items-center justify-between">
              <SheetTitle class="flex items-center gap-4 text-left">
                <div v-if="currentRestult" class="flex items-center gap-4">
                  <!-- 使用查询到的召唤师信息 -->
                  <div
                    class="w-14 h-14 rounded-full bg-gradient-to-br from-primary/20 to-primary/10 flex items-center justify-center ring-2 ring-primary/20"
                  >
                    <span class="text-lg font-bold text-primary">{{
                      currentRestult.displayName?.charAt(0)?.toUpperCase() || '?'
                    }}</span>
                  </div>
                  <div>
                    <h3 class="text-xl font-bold text-foreground">{{ currentRestult.displayName || '未知召唤师' }}</h3>
                    <p class="text-sm text-muted-foreground">召唤师详情与战绩分析</p>
                  </div>
                </div>
                <div v-else-if="selectedPlayer" class="flex items-center gap-4">
                  <!-- 查询中或失败时显示原始玩家信息 -->
                  <div
                    class="w-14 h-14 rounded-full bg-gradient-to-br from-primary/20 to-primary/10 flex items-center justify-center ring-2 ring-primary/20"
                  >
                    <span class="text-lg font-bold text-primary">{{
                      selectedPlayer.displayName?.charAt(0)?.toUpperCase() || '?'
                    }}</span>
                  </div>
                  <div>
                    <h3 class="text-xl font-bold text-foreground">{{ selectedPlayer.displayName || '未知召唤师' }}</h3>
                    <p class="text-sm text-muted-foreground">召唤师详情与战绩分析</p>
                  </div>
                </div>
              </SheetTitle>

              <SheetClose
                class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-secondary text-muted-foreground hover:text-foreground"
              >
                <X class="h-4 w-4" />
                <span class="sr-only">关闭</span>
              </SheetClose>
            </div>
          </SheetHeader>
        </div>

        <div class="p-6 pt-4 space-y-6">
          <!-- 加载状态 -->
          <div v-if="searchLoading" class="flex items-center justify-center py-8">
            <div class="flex items-center gap-3">
              <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary"></div>
              <span class="text-muted-foreground">正在查询召唤师战绩...</span>
            </div>
          </div>

          <!-- 战绩数据 -->
          <div v-else-if="currentRestult" class="space-y-6">
            <!-- 召唤师信息卡片 -->
            <SummonerCard :summoner-info="currentRestult.summonerInfo" />

            <!-- 游戏统计 -->
            <GameStats :is-connected="true" :match-history-loading="false" :match-statistics="currentRestult.matches" />
          </div>

          <!-- 无数据状态 -->
          <div v-else class="flex items-center justify-center py-8">
            <div class="text-center">
              <Info class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
              <h3 class="text-lg font-semibold mb-2 text-foreground">暂无战绩数据</h3>
              <p class="text-muted-foreground">未能获取到该召唤师的战绩信息</p>
            </div>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  </div>
</template>

<script setup lang="ts">
import { useChampSelectSession } from '@/composables'
import { useSearchMatches } from '@/composables/game/useSearchMatches'
import { usePlayerListQuery } from '@/composables/useLolApiQuery'
import { useGameStore } from '@/stores/features/gameStore'
import { appContextKey, type AppContext } from '@/types'
import { Info, X } from 'lucide-vue-next'
import { useGameAssets } from '@/composables/game/useGameAssets'
import { useDataStore } from '@/stores/core/dataStore'

const { isConnected } = inject(appContextKey) as AppContext
const { enrichedSession } = useChampSelectSession()

const shouldShowMatchAnalysis = ref(false)
// 使用搜索召唤师战绩的钩子
const { fetchSummonerInfo, currentRestult, loading: searchLoading } = useSearchMatches()

// 使用游戏状态 store 来监听状态变化
const gameStore = useGameStore()
const { currentPhase } = storeToRefs(gameStore)
const enabled = computed(() => currentPhase.value === 'InProgress')
const { data: playerList, refetch } = usePlayerListQuery(enabled)

const { getChampionIconUrl } = useGameAssets()
const dataStore = useDataStore()

const session = computed(() => {
  if (players.value) {
    const theirTeam = players.value
    const data = { ...enrichedSession.value, theirTeam }
    return data
  } else {
    return enrichedSession.value
  }
})
const players = computed(() => {
  if (!Array.isArray(playerList.value)) {
    return []
  }
  console.log('players', playerList.value)
  // 只取敌方队伍
  return playerList.value
    .filter((p: LiveClientPlayer) => p.team === 'CHAOS')
    .map((p: LiveClientPlayer): EnrichedLivePlayer => {
      // 从 championName 映射到 championId
      const champion = Object.values(dataStore.champions).find((c) => c.name === p.championName)
      const championId = champion ? parseInt(champion.key, 10) : 0

      // 在这里进行字段的适配和转换
      return {
        displayName: p.summonerName,
        championName: p.championName,
        team: p.team,
        isBot: p.isBot,
        isLocal: false, // 敌方队伍
        championIcon: getChampionIconUrl(championId)
      }
    })
})
// 召唤师详情相关 - 必须在 watchEffect 之前声明
const isDetailsOpen = ref(false)
// 注意：selectedPlayer 的类型现在可能需要更通用，因为它可能来自选人阶段或游戏中
const selectedPlayer = ref<EnrichedChampSelectPlayer | EnrichedLivePlayer | null>(null)

const openSummonerDetails = async (player: EnrichedChampSelectPlayer | EnrichedLivePlayer) => {
  selectedPlayer.value = player
  isDetailsOpen.value = true
  if (player.displayName && player.displayName !== '未知玩家' && player.displayName !== '未知召唤师') {
    await fetchSummonerInfo([player.displayName])
  }
}

// const { data: builds, refetch: refetchBuilds } = useBuildsByAliasQuery(source.value, champion.value)
// const { data: builds } = useQuery({
//   queryKey: ['builds-by-alias', source, champion],
//   queryFn: () => invoke<BuildSection>('get_builds_by_alias', { source, champion }),
//   enabled: computed(() => !!source && !!champion) // 有参数才请求
// })
// watch(
//   () => builds.value,
//   (val) => {
//     console.log('builds', val, allRunes.value)
//   },
//   { deep: true }
// )

watchEffect(async () => {
  const phase = currentPhase.value
  console.log('Current game phase:', phase)
  shouldShowMatchAnalysis.value =
    (!!enrichedSession.value && phase === 'ChampSelect') || phase === 'GameStart' || phase === 'InProgress'

  if (phase === 'InProgress' && isConnected.value) {
    console.log('in progress, refetching player list')
    await refetch()
  }
})

// 根据游戏阶段获取状态标题
const getStatusTitle = () => {
  switch (currentPhase.value) {
    case 'None':
      return '暂无游戏活动'
    case 'Lobby':
      return '房间等待中'
    case 'Matchmaking':
      return '正在匹配中'
    case 'ReadyCheck':
      return '确认对局'
    case 'ChampSelect':
      return '正在加载选人阶段数据，请稍候...'
    case 'GameStart':
      return '游戏即将开始'
    case 'InProgress':
      return '游戏进行中'
    case 'WaitingForStats':
      return '等待结算'
    case 'PreEndOfGame':
    case 'EndOfGame':
      return '游戏结束'
    case 'TerminatedInError':
      return '游戏异常终止'
    default:
      return '进入游戏以查看对局分析'
  }
}

// 获取状态描述
const getStatusDescription = () => {
  switch (currentPhase.value) {
    case 'None':
      return '当前没有进行任何游戏活动。启动英雄联盟客户端并开始匹配以查看对局分析。'
    case 'Lobby':
      return '正在房间中等待，准备开始匹配对手。'
    case 'Matchmaking':
      return '正在寻找合适的对手，找到后将进入选人界面。'
    case 'ReadyCheck':
      return '找到对局！请在客户端中确认准备状态。'
    case 'ChampSelect':
      return '正在加载选人阶段数据，请稍候...'
    case 'GameStart':
      return '选人阶段结束，游戏即将加载。'
    case 'InProgress':
      return '游戏正在进行中，祝你好运！'
    case 'WaitingForStats':
      return '游戏已结束，正在等待战绩统计。'
    case 'PreEndOfGame':
    case 'EndOfGame':
      return '游戏已结束，可以查看战绩详情。'
    case 'TerminatedInError':
      return '游戏因错误而终止，请重新开始匹配。'
    default:
      return '开始游戏匹配以查看详细的对局分析数据。'
  }
}
</script>
