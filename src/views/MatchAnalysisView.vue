<template>
  <div class="min-h-screen">
    <div v-if="session && shouldShowMatchAnalysis" class="w-full max-w-full mx-auto">
      <!-- 左右分屏布局 -->
      <div class="flex gap-3 h-screen max-h-screen overflow-hidden">
        <!-- 左侧：我方队伍 -->
        <div class="flex-1 flex flex-col min-w-0">
          <div class="flex items-center justify-between mb-0">
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded-full bg-blue-500 animate-pulse"></div>
              <h2 class="text-lg font-bold text-blue-600 dark:text-blue-400">我方队伍</h2>
            </div>
            <Badge
              variant="outline"
              class="text-blue-600 dark:text-blue-400 border-blue-300 dark:border-blue-600 text-xs"
            >
              {{ session.myTeam?.length || 0 }} 人
            </Badge>
          </div>

          <div
            class="flex-1 scrollbar-thin scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar-thumb-slate-400/50 dark:scrollbar-thumb-slate-500/50 scrollbar-track-transparent overflow-y-auto scroll-smooth"
          >
            <TeamCard
              v-if="session.myTeam"
              :team="session.myTeam"
              team-type="ally"
              :local-player-cell-id="session.localPlayerCellId"
              :summoner-stats="summonerStats || []"
              @select="openSummonerDetails"
            />
          </div>
        </div>

        <!-- 分割线 -->
        <div class="w-px bg-border/50"></div>

        <!-- 右侧：敌方队伍 -->
        <div class="flex-1 flex flex-col min-w-0">
          <div class="flex items-center justify-between mb-0">
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded-full bg-red-500 animate-pulse"></div>
              <h2 class="text-lg font-bold text-red-600 dark:text-red-400">敌方队伍</h2>
              <!-- 添加提示信息 -->
              <div v-if="currentPhase === 'ChampSelect'" class="text-center text-muted-foreground mb-0">
                <div class="flex items-center justify-center gap-2 text-xs bg-muted/50 rounded-lg p-2">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    ></path>
                  </svg>
                  敌方队伍信息将在进入游戏后实时更新
                </div>
              </div>
            </div>

            <Badge variant="outline" class="text-red-600 dark:text-red-400 border-red-300 dark:border-red-600 text-xs">
              {{ session.theirTeam?.length || 0 }} 人
            </Badge>
          </div>

          <div
            class="flex-1 scrollbar-thin scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar-thumb-slate-400/50 dark:scrollbar-thumb-slate-500/50 scrollbar-track-transparent overflow-y-auto scroll-smooth"
          >
            <template v-if="session.theirTeam && session.theirTeam.length">
              <TeamCard
                :team="session.theirTeam"
                team-type="enemy"
                :summoner-stats="theirTeamStats || []"
                @select="openSummonerDetails"
              />
            </template>
            <template v-else>
              <div class="text-center text-muted-foreground mt-8">
                <svg class="w-12 h-12 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                  ></path>
                </svg>
                <p class="text-sm">敌方队伍将在进入游戏后显示</p>
              </div>
            </template>
          </div>
        </div>
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
    <!-- 召唤师详情 -->
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
import { invoke } from '@tauri-apps/api/core'

const { isConnected } = inject(appContextKey) as AppContext
const { enrichedSession } = useChampSelectSession()

const shouldShowMatchAnalysis = ref(false)
// 使用搜索召唤师战绩的钩子
const {
  summonerStats,
  fetchSummonerInfo,
  currentRestult,
  loading: searchLoading,
  getRencentMatchesByPuuid
} = useSearchMatches()

// 使用游戏状态 store 来监听状态变化
const gameStore = useGameStore()
const { currentPhase } = storeToRefs(gameStore)
const enabled = computed(() => currentPhase.value === 'InProgress')
const { data: playerList, refetch } = usePlayerListQuery(enabled)
const theirTeamStats = ref<MatchStatistics[] | null>(null)
const { getChampionIconUrl } = useGameAssets()
const dataStore = useDataStore()

const session = computed<ChampSelectSession>(() => {
  if (Array.isArray(players.value) && players.value.length > 0) {
    const theirTeam = players.value
    const data = { ...enrichedSession.value, theirTeam }
    console.log('session', data)
    return data
  } else {
    return enrichedSession.value || null
  }
})

const players = ref<EnrichedLivePlayer[]>([])

// 使用 watch 监听 playerList 变化
watch(
  () => playerList.value,
  async (newPlayerList) => {
    if (!Array.isArray(newPlayerList)) {
      players.value = []
      return
    }

    console.log('players', newPlayerList)

    // 修复：正确识别队伍，而不是简单假设 CHAOS 是敌方
    // 需要根据本地玩家信息来确定哪个队伍是敌方
    let enemyTeam: LiveClientPlayer[] = []

    if (enrichedSession.value?.localPlayerCellId !== undefined) {
      // 如果有选人阶段的会话信息，使用它来确定队伍
      const localPlayer = enrichedSession.value.myTeam?.find(
        (p: EnrichedChampSelectPlayer) => p.cellId === enrichedSession.value.localPlayerCellId
      )
      if (localPlayer) {
        // 根据本地玩家的队伍信息来确定敌方队伍
        // 这里需要根据实际情况调整逻辑
        const localPlayerName = localPlayer.displayName || localPlayer.summonerId

        // 在实时玩家列表中查找本地玩家，确定其队伍
        const localPlayerInGame = newPlayerList.find((p: LiveClientPlayer) => p.summonerName === localPlayerName)
        if (localPlayerInGame) {
          const localTeam = localPlayerInGame.team
          // 敌方队伍就是与本地玩家不同的队伍
          enemyTeam = newPlayerList.filter((p: LiveClientPlayer) => p.team !== localTeam)
          console.log(`本地玩家 ${localPlayerName} 在队伍 ${localTeam}，敌方队伍:`, enemyTeam)
        } else {
          // 如果找不到本地玩家，回退到原来的逻辑
          console.warn('无法找到本地玩家，使用回退逻辑')
          enemyTeam = newPlayerList.filter((p: LiveClientPlayer) => p.team === 'CHAOS')
        }
      } else {
        // 回退逻辑
        enemyTeam = newPlayerList.filter((p: LiveClientPlayer) => p.team === 'CHAOS')
      }
    } else {
      // 没有选人阶段信息时的回退逻辑
      enemyTeam = newPlayerList.filter((p: LiveClientPlayer) => p.team === 'CHAOS')
    }

    const names = enemyTeam.map((p: LiveClientPlayer) => p.summonerName)

    try {
      const matches = await invoke<SummonerWithMatches[]>('get_summoners_and_histories', { names, count: 6 })
      theirTeamStats.value = matches.map((m) => m.matches)
      console.log('matches', theirTeamStats.value, summonerStats.value)
    } catch (error) {
      console.error('获取敌方队伍战绩失败:', error)
      theirTeamStats.value = []
    }

    // 只取敌方队伍
    players.value = enemyTeam.map((p: LiveClientPlayer): EnrichedLivePlayer => {
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
  }
)
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
watchEffect(async () => {
  const phase = currentPhase.value
  console.log('Current game phase:', phase)
  shouldShowMatchAnalysis.value =
    (!!enrichedSession.value && phase === 'ChampSelect') || phase === 'GameStart' || phase === 'InProgress'
  if (isConnected.value && !summonerStats.value && enrichedSession.value) {
    console.log('enrichedSession', enrichedSession.value)
    if (!Array.isArray(enrichedSession.value.myTeam)) console.log('myTeam', enrichedSession.value.myTeam)
    const ids = enrichedSession.value.myTeam?.map((p: EnrichedLivePlayer) => p.puuid).filter(Boolean) || []
    console.log('获取战绩的puuid列表:', ids)
    await getRencentMatchesByPuuid(ids, 6)
    if (summonerStats.value) {
      console.log('Fetched summoner stats:', summonerStats.value)
    }
  }
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
      return '当前没有进行任何游戏活动。开始游戏以查看对局分析。'
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
