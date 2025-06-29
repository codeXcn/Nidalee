<template>
  <div v-if="isConnected" class="min-h-screen">
    <!-- 主要内容 -->
    <div v-if="session && shouldShowMatchAnalysis" class="w-full max-w-7xl mx-auto space-y-6">
      <!-- 队伍分析卡片 -->
      <div
        class="grid grid-cols-1 xl:grid-cols-2 gap-6 lg:gap-8 animate-in fade-in-0 duration-700 slide-in-from-bottom-4"
      >
        <!-- 我方队伍 -->
        <Card
          class="p- case 'ChampSelect': return '正在加载选人阶段数据，请稍候...' case 'GameStart': return '选人阶段结束，游戏即将加载。' case 'InProgress': return '游戏正在进行中，可以查看队伍分析。' case 'WaitingForStats': return '游戏已结束，正在等待战绩统计。' case 'PreEndOfGame': case 'EndOfGame': return '游戏已结束，可以查看本局详细数据。'imate-in fade-in-0 duration-500 slide-in-from-left-4"
          style="animation-delay: 100ms"
        >
          <div class="flex items-center justify-between mb-4 lg:mb-6">
            <div class="flex items-center gap-3">
              <div class="w-3 h-3 lg:w-4 lg:h-4 bg-blue-500 rounded-full animate-pulse"></div>
              <h3 class="text-lg lg:text-xl font-bold text-blue-600 dark:text-blue-400">我方队伍</h3>
            </div>
            <Badge variant="outline" class="text-blue-600 dark:text-blue-400 border-blue-300 dark:border-blue-600">
              {{ session.myTeam.length }} 人
            </Badge>
          </div>

          <div class="space-y-2 lg:space-y-3">
            <PlayerCard
              v-for="(player, index) in session.myTeam"
              :key="player.summonerId + '-' + player.cellId"
              :player="player"
              :is-local="player.cellId === session.localPlayerCellId"
              :is-ally="true"
              @select="openSummonerDetails"
              class="cursor-pointer animate-in fade-in-0 duration-500 slide-in-from-left-2"
              :style="`animation-delay: ${200 + index * 100}ms;`"
            />
          </div>

          <!-- 队伍统计信息 -->
          <div class="mt-4 lg:mt-6 pt-4 border-t border-border">
            <TeamStats :team="session.myTeam" team-type="ally" />
          </div>
        </Card>

        <!-- 敌方队伍 -->
        <Card class="p-4 lg:p-6 animate-in fade-in-0 duration-500 slide-in-from-right-4" style="animation-delay: 200ms">
          <div class="flex items-center justify-between mb-4 lg:mb-6">
            <div class="flex items-center gap-3">
              <div class="w-3 h-3 lg:w-4 lg:h-4 bg-red-500 rounded-full animate-pulse"></div>
              <h3 class="text-lg lg:text-xl font-bold text-red-600 dark:text-red-400">敌方队伍</h3>
            </div>
            <Badge variant="outline" class="text-red-600 dark:text-red-400 border-red-300 dark:border-red-600">
              {{ session.theirTeam.length }} 人
            </Badge>
          </div>

          <div class="space-y-2 lg:space-y-3">
            <PlayerCard
              v-for="(player, index) in session.theirTeam"
              :key="player.summonerId + '-' + player.cellId"
              :player="player"
              :is-ally="false"
              @select="openSummonerDetails"
              class="cursor-pointer animate-in fade-in-0 duration-500 slide-in-from-right-2"
              :style="`animation-delay: ${300 + index * 100}ms;`"
            />
          </div>

          <!-- 队伍统计信息 -->
          <div class="mt-4 lg:mt-6 pt-4 border-t border-border">
            <TeamStats :team="session.theirTeam" team-type="enemy" />
          </div>
        </Card>
      </div>

      <!-- 对局建议 -->
      <Card class="p-4 lg:p-6 animate-in fade-in-0 duration-500 slide-in-from-bottom-3" style="animation-delay: 800ms">
        <h3 class="text-lg font-semibold mb-4 flex items-center gap-2 text-foreground">
          <Lightbulb class="h-5 w-5 text-yellow-500 dark:text-yellow-400" />
          对局建议
        </h3>
        <MatchSuggestions :session="session" />
      </Card>

      <!-- 补充功能区域 -->
      <div
        class="grid grid-cols-1 lg:grid-cols-2 gap-6 animate-in fade-in-0 duration-500 slide-in-from-bottom-4"
        style="animation-delay: 1000ms"
      >
        <!-- 阵容分析 -->
        <Card class="p-4 lg:p-6">
          <h3 class="text-lg font-semibold mb-4 flex items-center gap-2 text-foreground">
            <Users class="h-5 w-5 text-blue-500 dark:text-blue-400" />
            阵容分析
          </h3>
          <TeamComposition :session="session" />
        </Card>

        <!-- 胜率预测 -->
        <Card class="p-4 lg:p-6">
          <h3 class="text-lg font-semibold mb-4 flex items-center gap-2 text-foreground">
            <BarChart3 class="h-5 w-5 text-green-500 dark:text-green-400" />
            胜率预测
          </h3>
          <WinRatePrediction :session="session" />
        </Card>
      </div>

      <!-- 静态数据增强功能区域 -->
      <div
        class="grid grid-cols-1 lg:grid-cols-2 gap-6 animate-in fade-in-0 duration-500 slide-in-from-bottom-5"
        style="animation-delay: 1200ms"
      >
        <!-- 符文分析 -->
        <RuneAnalysis :session="session" />

        <!-- 装备推荐 -->
        <ItemRecommendation :session="session" />
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
  <ClientDisconnected v-else />
</template>

<script setup lang="ts">
import { useChampSelectSession } from '@/composables'
import { useSearchMatches } from '@/composables/game/useSearchMatches'
import { useGameStatusStore } from '@/stores'
import { appContextKey } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { BarChart3, Info, Lightbulb, Users, X } from 'lucide-vue-next'

const { isConnected } = inject(appContextKey)
const { session: rawSession, enrichedSession, loading } = useChampSelectSession()
const session = computed(() => enrichedSession.value)
const shouldShowMatchAnalysis = ref(false)
// 使用搜索召唤师战绩的钩子
const { fetchSummonerInfo, currentRestult, loading: searchLoading } = useSearchMatches()

// 使用游戏状态 store 来监听状态变化
const gameStatusStore = useGameStatusStore()
const { currentPhase, isInChampSelect } = storeToRefs(gameStatusStore)
// 召唤师详情相关 - 必须在 watchEffect 之前声明
const isDetailsOpen = ref(false)
const selectedPlayer = ref<any>(null)
// 添加调试信息和会话监听
watchEffect(() => {
  console.log('Session data:', session.value)
  const phase = currentPhase.value
  console.log('Current game phase:', phase)
  shouldShowMatchAnalysis.value = !!session.value &&
    phase === 'ChampSelect' ||
    phase === 'GameStart' ||
    phase === 'InProgress' ||
    phase === 'WaitingForStats' ||
    phase === 'PreEndOfGame' ||
    phase === 'EndOfGame'

})

const openSummonerDetails = async (player: any) => {
  selectedPlayer.value = player
  isDetailsOpen.value = true
  // 首先尝试使用 displayName 搜索
  if (player.displayName && player.displayName !== '未知玩家' && player.displayName !== '未知召唤师') {
    console.log('Searching match history by displayName:', player.displayName)
    try {
      await fetchSummonerInfo([player.displayName])
      console.log('Search by displayName completed, result:', currentRestult.value)
    } catch (error) {
      console.error('Failed to fetch summoner data by displayName:', error)
    }
  }
  // 如果 displayName 不可用或搜索失败，尝试通过 summonerId 获取召唤师信息
  else if (player.summonerId && player.summonerId !== '0') {
    console.log('Trying to get summoner info by summonerId:', player.summonerId)
    try {
      // 使用 summonerId 获取召唤师基本信息
      const summonerInfo = await invoke('get_summoner_by_id', { id: parseInt(player.summonerId) })
      console.log('Summoner info by ID:', summonerInfo)

      if (summonerInfo && summonerInfo.displayName) {
        console.log('Got summoner displayName:', summonerInfo.displayName)
        // 使用获取到的 displayName 搜索战绩
        await fetchSummonerInfo([summonerInfo.displayName])
        console.log('Search by fetched displayName completed, result:', currentRestult.value)
      }
    } catch (error) {
      console.error('Failed to fetch summoner info by ID:', error)
    }
  } else {
    console.log('No valid identifier found for summoner search')
  }
}

// 获取状态标题
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
