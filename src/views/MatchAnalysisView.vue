<template>
  <div class="min-h-screen">
    <!-- 匹配中时显示匹配面板 -->
    <div v-if="currentPhase === 'Matchmaking'" class="w-full max-w-4xl mx-auto p-6">
      <MatchmakingPanel />
    </div>

    <!-- 对局分析页面 -->
    <div v-else-if="shouldShowMatchAnalysis" class="w-full max-w-full mx-auto">
      <TeamDataManager
        :session="session"
        :current-phase="currentPhase"
        :summoner-stats="summonerStats"
        :their-team-stats="theirTeamStats"
        :has-live-client-data="hasLiveClientData"
        :cached-champ-select-data="cachedChampSelectData"
        @toggle-filter="showFilterDialog = true"
        @open-summoner-details="handleSummonerDetails"
      />
    </div>

    <!-- 默认状态显示 -->
    <div v-else class="flex items-center justify-center h-screen bg-background">
      <div class="text-center space-y-6 max-w-md mx-auto px-6">
        <!-- 状态图标 -->
        <div class="relative">
          <div class="w-20 h-20 mx-auto rounded-full flex items-center justify-center" :class="statusIconClass">
            <div
              class="w-3 h-3 rounded-full"
              :class="statusIndicatorClass"
              :style="{ animation: statusAnimation }"
            ></div>
          </div>
          <!-- 状态图标背景动画 -->
          <div
            v-if="currentPhase === 'InProgress'"
            class="absolute inset-0 w-20 h-20 mx-auto rounded-full border-2 border-primary/20 animate-ping"
          ></div>
        </div>

        <!-- 状态标题和描述 -->
        <div class="space-y-3">
          <h2 class="text-2xl font-bold" :class="statusTitleClass">{{ getStatusTitle }}</h2>
          <p class="text-muted-foreground leading-relaxed">{{ getStatusDescription }}</p>
        </div>

        <!-- 状态徽章 -->
        <div v-if="currentPhase" class="flex items-center justify-center">
          <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full border" :class="statusBadgeClass">
            <div
              class="w-2 h-2 rounded-full"
              :class="statusIndicatorClass"
              :style="{ animation: statusAnimation }"
            ></div>
            <span class="text-sm font-medium">{{ getStatusBadgeText }}</span>
          </div>
        </div>

        <!-- 额外提示信息 -->
        <div v-if="getAdditionalInfo" class="mt-4 p-4 rounded-lg border bg-card/50">
          <p class="text-sm text-muted-foreground">{{ getAdditionalInfo }}</p>
        </div>
      </div>
    </div>

    <!-- 过滤对话框 -->
    <!-- <MatchFilterDialog v-model:open="showFilterDialog" /> -->

    <!-- 召唤师详情抽屉 -->
    <SummonerDetailsDialog
      v-if="selectedSummoner"
      v-model:open="showSummonerDetails"
      :summoner="selectedSummoner"
      :match-history="selectedMatchHistory"
      @close="closeSummonerDetails"
    />

    <!-- 加载状态 -->
    <div v-if="isLoading" class="fixed inset-0 bg-background/80 backdrop-blur-sm flex items-center justify-center z-50">
      <div class="flex flex-col items-center gap-4">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
        <p class="text-sm text-muted-foreground">{{ loadingMessage }}</p>
      </div>
    </div>

    <!-- 错误状态 -->
    <div v-if="hasErrors" class="fixed bottom-4 right-4 z-50">
      <div class="bg-destructive text-destructive-foreground rounded-lg p-4 max-w-sm">
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
              clip-rule="evenodd"
            />
          </svg>
          <span class="font-medium">发生错误</span>
        </div>
        <p class="text-sm mt-1">{{ recentErrors[0]?.message }}</p>
        <button @click="clearErrors" class="text-xs underline mt-2">清除错误</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, watchEffect, onMounted, onUnmounted } from 'vue'
import { useGameStore } from '@/stores/features/gameStore'
import { useDataStore } from '@/stores/core/dataStore'
import { fetchChampionSummary } from '@/lib/dataApi'
import { useLiveClientManager } from '@/composables/game/useLiveClientManager'
import { useMatchHistoryFetcher } from '@/composables/game/useMatchHistoryFetcher'
import { useErrorHandler } from '@/composables/utils/useErrorHandler'
import { useGameEvents } from '@/composables/utils/useEventBus'
import { usePerformanceMonitor } from '@/composables/utils/usePerformanceMonitor'
import TeamDataManager from '@/components/features/game/TeamDataManager.vue'
import MatchmakingPanel from '@/components/features/game/MatchmakingPanel.vue'
// import MatchFilterDialog from '@/components/features/match/MatchFilterDialog.vue'
import SummonerDetailsDialog from '@/components/features/match/SummonerDetailsDialog.vue'
// import type { EnrichedChampSelectPlayer, EnrichedLivePlayer } from '@/types/handle.d'

// Store 和状态管理
const gameStore = useGameStore()
const dataStore = useDataStore()

// Composables
const liveClientManager = useLiveClientManager()
const matchHistoryFetcher = useMatchHistoryFetcher()
const errorHandler = useErrorHandler()
const { emitGamePhaseChanged } = useGameEvents()
const { measureAsync } = usePerformanceMonitor()

// 响应式状态
const showFilterDialog = ref(false)
const showSummonerDetails = ref(false)
const selectedSummoner = ref<any>(null)
const selectedMatchHistory = ref<any>(null)

// 缓存状态（内存级，用于阶段切换空窗期）
const cachedChampSelectData = ref<{
  myTeam: any[]
  theirTeam: any[]
  session: any
} | null>(null)

// 计算属性
const currentPhase = computed(() => gameStore.currentPhase)
const enrichedSession = computed(() => gameStore.champSelectSession)
const isConnected = computed(() => true) // TODO: 需要从 gameStore 获取真实的连接状态

// 会话数据 - 智能合并逻辑
const session = computed(() => {
  // 如果有 LiveClient 数据，优先使用
  if (Array.isArray(liveClientManager.players.value) && liveClientManager.players.value.length > 0) {
    const theirTeam = liveClientManager.players.value
    // 在游戏阶段，需要从 LiveClient 数据中获取我方队伍信息
    const myTeam =
      liveClientManager.myTeamPlayers.value.length > 0
        ? liveClientManager.myTeamPlayers.value
        : enrichedSession.value?.myTeam || []
    const base = enrichedSession.value ? enrichedSession.value : {}
    const data = {
      ...base,
      myTeam,
      theirTeam
    }
    console.log('session (LiveClient data)', data)
    return data
  }

  // 如果有选人阶段数据，使用它
  if (enrichedSession.value) {
    // 为选人阶段数据添加 displayName 字段
    const enrichedData = {
      ...enrichedSession.value,
      myTeam:
        enrichedSession.value.myTeam?.map((p: any) => ({
          ...p,
          displayName: p.gameName && p.tagLine ? `${p.gameName}#${p.tagLine}` : p.summonerId || '未知召唤师'
        })) || [],
      theirTeam:
        enrichedSession.value.theirTeam?.map((p: any) => ({
          ...p,
          displayName: p.gameName && p.tagLine ? `${p.gameName}#${p.tagLine}` : p.summonerId || '未知召唤师'
        })) || []
    }
    console.log('session (ChampSelect data)', enrichedData)
    return enrichedData
  }

  // 在游戏阶段切换时，如果 enrichedSession 被清空，使用缓存的数据
  if (currentPhase.value === 'InProgress' && cachedChampSelectData.value) {
    console.log('session (cached data)', cachedChampSelectData.value)
    return cachedChampSelectData.value.session
  }

  return null
})

// 是否显示对局分析
const shouldShowMatchAnalysis = computed(() => {
  const phase = currentPhase.value
  const isInProgressWithData =
    phase === 'InProgress' &&
    (liveClientManager.myTeamPlayers.value.length > 0 || liveClientManager.players.value.length > 0)

  return (
    (!!enrichedSession.value && phase === 'ChampSelect') ||
    phase === 'GameStart' ||
    isInProgressWithData ||
    (phase === 'InProgress' && !!cachedChampSelectData.value)
  )
})

// LiveClient 数据状态
const hasLiveClientData = computed(() => liveClientManager.hasData.value)

// 战绩数据
const summonerStats = computed(() => matchHistoryFetcher.summonerStats.value)
const theirTeamStats = computed(() => matchHistoryFetcher.theirTeamStats.value)

// 状态样式计算属性
const statusIndicatorClass = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
      return 'bg-blue-500 dark:bg-blue-400'
    case 'ReadyCheck':
      return 'bg-orange-500 dark:bg-orange-400'
    case 'ChampSelect':
      return 'bg-blue-500 dark:bg-blue-400'
    case 'GameStart':
      return 'bg-purple-500 dark:bg-purple-400'
    case 'InProgress':
      return hasLiveClientData.value ? 'bg-green-500 dark:bg-green-400' : 'bg-yellow-500 dark:bg-yellow-400'
    case 'WaitingForStats':
      return 'bg-indigo-500 dark:bg-indigo-400'
    case 'EndOfGame':
      return 'bg-gray-500 dark:bg-gray-400'
    default:
      return 'bg-gray-500 dark:bg-gray-400'
  }
})

const statusIconClass = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
      return 'bg-blue-50 dark:bg-blue-950/50 border-2 border-blue-200 dark:border-blue-800'
    case 'ReadyCheck':
      return 'bg-orange-50 dark:bg-orange-950/50 border-2 border-orange-200 dark:border-orange-800'
    case 'ChampSelect':
      return 'bg-blue-50 dark:bg-blue-950/50 border-2 border-blue-200 dark:border-blue-800'
    case 'GameStart':
      return 'bg-purple-50 dark:bg-purple-950/50 border-2 border-purple-200 dark:border-purple-800'
    case 'InProgress':
      return hasLiveClientData.value
        ? 'bg-green-50 dark:bg-green-950/50 border-2 border-green-200 dark:border-green-800'
        : 'bg-yellow-50 dark:bg-yellow-950/50 border-2 border-yellow-200 dark:border-yellow-800'
    case 'WaitingForStats':
      return 'bg-indigo-50 dark:bg-indigo-950/50 border-2 border-indigo-200 dark:border-indigo-800'
    case 'EndOfGame':
      return 'bg-gray-50 dark:bg-gray-950/50 border-2 border-gray-200 dark:border-gray-800'
    default:
      return 'bg-gray-50 dark:bg-gray-950/50 border-2 border-gray-200 dark:border-gray-800'
  }
})

const statusTitleClass = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
      return 'text-blue-600 dark:text-blue-400'
    case 'ReadyCheck':
      return 'text-orange-600 dark:text-orange-400'
    case 'ChampSelect':
      return 'text-blue-600 dark:text-blue-400'
    case 'GameStart':
      return 'text-purple-600 dark:text-purple-400'
    case 'InProgress':
      return hasLiveClientData.value ? 'text-green-600 dark:text-green-400' : 'text-yellow-600 dark:text-yellow-400'
    case 'WaitingForStats':
      return 'text-indigo-600 dark:text-indigo-400'
    case 'EndOfGame':
      return 'text-gray-600 dark:text-gray-400'
    default:
      return 'text-gray-600 dark:text-gray-400'
  }
})

const statusBadgeClass = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
      return 'bg-blue-50 dark:bg-blue-950/50 border-blue-200 dark:border-blue-800 text-blue-700 dark:text-blue-300'
    case 'ReadyCheck':
      return 'bg-orange-50 dark:bg-orange-950/50 border-orange-200 dark:border-orange-800 text-orange-700 dark:text-orange-300'
    case 'ChampSelect':
      return 'bg-blue-50 dark:bg-blue-950/50 border-blue-200 dark:border-blue-800 text-blue-700 dark:text-blue-300'
    case 'GameStart':
      return 'bg-purple-50 dark:bg-purple-950/50 border-purple-200 dark:border-purple-800 text-purple-700 dark:text-purple-300'
    case 'InProgress':
      return hasLiveClientData.value
        ? 'bg-green-50 dark:bg-green-950/50 border-green-200 dark:border-green-800 text-green-700 dark:text-green-300'
        : 'bg-yellow-50 dark:bg-yellow-950/50 border-yellow-200 dark:border-yellow-800 text-yellow-700 dark:text-yellow-300'
    case 'WaitingForStats':
      return 'bg-indigo-50 dark:bg-indigo-950/50 border-indigo-200 dark:border-indigo-800 text-indigo-700 dark:text-indigo-300'
    case 'EndOfGame':
      return 'bg-gray-50 dark:bg-gray-950/50 border-gray-200 dark:border-gray-800 text-gray-700 dark:text-gray-300'
    default:
      return 'bg-gray-50 dark:bg-gray-950/50 border-gray-200 dark:border-gray-800 text-gray-700 dark:text-gray-300'
  }
})

const statusAnimation = computed(() => {
  const phase = currentPhase.value
  if (phase === 'InProgress' && !hasLiveClientData.value) {
    return 'pulse 2s infinite'
  } else if (phase === 'ReadyCheck') {
    return 'pulse 1.5s infinite'
  }
  return 'pulse 3s infinite'
})

// 状态标题和描述
const getStatusTitle = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '已进入大厅'
    case 'ReadyCheck':
      return '对局确认中'
    case 'ChampSelect':
      return '正在选择英雄'
    case 'GameStart':
      return '游戏加载中'
    case 'InProgress':
      if (hasLiveClientData.value) {
        return '游戏进行中（实时分析已连接）'
      } else if (cachedChampSelectData.value) {
        return '游戏进行中（正在获取实时数据）'
      } else {
        return '游戏进行中（正在连接）'
      }
    case 'WaitingForStats':
      return '结算中（生成战报）'
    case 'EndOfGame':
      return '对局结束'
    default:
      return '等待客户端连接'
  }
})

const getStatusDescription = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '请选择游戏模式并开始匹配，进入选人后将自动显示队伍信息'
    case 'ReadyCheck':
      return '请在客户端中确认准备状态，以免错过对局'
    case 'ChampSelect':
      return '请在客户端中完成英雄与符文配置，本页将展示队伍与战绩信息'
    case 'GameStart':
      return '游戏正在加载，请稍候，进入游戏后将自动连接实时数据'
    case 'InProgress':
      if (!hasLiveClientData.value && cachedChampSelectData.value) {
        return '已进入游戏，正在连接实时数据…'
      } else if (!hasLiveClientData.value) {
        return '正在检测与游戏客户端的连接，请确保游戏窗口已启动且未最小化'
      } else {
        return '已连接实时数据，正在更新对局分析'
      }
    case 'WaitingForStats':
      return '游戏已结束，正在获取比赛统计数据'
    case 'EndOfGame':
      return '比赛结果已出，返回大厅后将恢复大厅视图'
    default:
      return '请启动并登录英雄联盟客户端；进入大厅后将自动切换到选人/对局分析'
  }
})

const getStatusBadgeText = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '房间中'
    case 'ReadyCheck':
      return '准备确认'
    case 'ChampSelect':
      return '英雄选择'
    case 'GameStart':
      return '游戏开始'
    case 'InProgress':
      return '游戏进行中'
    case 'WaitingForStats':
      return '等待统计'
    case 'EndOfGame':
      return '游戏结束'
    default:
      return '未连接'
  }
})

const getAdditionalInfo = computed(() => {
  const phase = currentPhase.value
  switch (phase) {
    case 'Lobby':
    case 'None':
      return '💡 提示：选择你喜欢的游戏模式，然后点击“寻找对局”开始匹配'
    case 'ReadyCheck':
      return '⚡ 请及时确认准备状态，避免错过游戏'
    case 'ChampSelect':
      return '🎯 建议：根据队伍阵容选择合适的英雄与符文配置'
    case 'GameStart':
      return '⏳ 游戏正在加载，请耐心等待'
    case 'InProgress':
      if (!hasLiveClientData.value) {
        return '🔗 正在连接游戏客户端，请确保游戏正在运行且未被防火墙拦截'
      }
      return null
    case 'WaitingForStats':
      return '📊 正在获取详细的比赛统计数据，请稍候'
    case 'EndOfGame':
      return '🏆 感谢你的游戏，期待下次对局'
    default:
      return '🎮 请确保英雄联盟客户端正在运行（若已运行但未连接，可稍等片刻或在右上角尝试刷新）'
  }
})

// 加载状态
const isLoading = computed(() => liveClientManager.isLoading.value || matchHistoryFetcher.isLoading.value)
const loadingMessage = computed(() => {
  if (liveClientManager.isLoading.value) return '正在获取游戏数据...'
  if (matchHistoryFetcher.isLoading.value) return '正在获取战绩数据...'
  return '加载中...'
})

// 错误状态
const hasErrors = computed(() => errorHandler.hasErrors.value)
const recentErrors = computed(() => errorHandler.getRecentErrors(1))

// 加载英雄数据
const loadChampions = async () => {
  if (dataStore.champions.length > 0) {
    console.log('[MatchAnalysis] 英雄数据已存在，跳过加载')
    return
  }

  try {
    await measureAsync('load-champions', async () => {
      const response = await fetchChampionSummary()
      if (response.data) {
        dataStore.setChampions(response.data)
        console.log('[MatchAnalysis] 英雄数据加载完成:', response.data.length)
      }
    })
  } catch (error) {
    errorHandler.handleError(error instanceof Error ? error : String(error), '加载英雄数据')
  }
}

// 获取本地玩家名称（优先当前会话，回退缓存的选人阶段数据）
const getLocalPlayerName = (): string | undefined => {
  // 当前会话
  if (enrichedSession.value?.localPlayerCellId !== undefined) {
    const localPlayer = enrichedSession.value.myTeam?.find(
      (p: any) => p.cellId === enrichedSession.value.localPlayerCellId
    )
    return localPlayer?.displayName || localPlayer?.summonerId
  }
  // 回退缓存（进入 InProgress 后 enrichedSession 可能为空）
  if (cachedChampSelectData.value?.session?.localPlayerCellId !== undefined) {
    const session = cachedChampSelectData.value.session
    const localPlayer = session.myTeam?.find((p: any) => p.cellId === session.localPlayerCellId)
    return (
      localPlayer?.displayName ||
      (localPlayer?.gameName && localPlayer?.tagLine ? `${localPlayer.gameName}#${localPlayer.tagLine}` : undefined)
    )
  }
  return undefined
}

// 处理召唤师详情
const handleSummonerDetails = (summoner: any, matchHistory: any) => {
  selectedSummoner.value = summoner
  selectedMatchHistory.value = matchHistory
  showSummonerDetails.value = true
}

// 关闭召唤师详情
const closeSummonerDetails = () => {
  showSummonerDetails.value = false
  selectedSummoner.value = null
  selectedMatchHistory.value = null
}

// 清除错误
const clearErrors = () => {
  errorHandler.clearErrors()
}

// 监听游戏阶段变化
watchEffect(async () => {
  const phase = currentPhase.value
  console.log('Current game phase:', phase)

  // 发布游戏阶段变化事件
  emitGamePhaseChanged(phase)

  // 缓存选人阶段的数据，避免阶段切换时的空窗期
  if (phase === 'ChampSelect' && enrichedSession.value) {
    // 为缓存数据也添加 displayName 字段
    const enrichedData = {
      ...enrichedSession.value,
      myTeam:
        enrichedSession.value.myTeam?.map((p: any) => ({
          ...p,
          displayName: p.gameName && p.tagLine ? `${p.gameName}#${p.tagLine}` : p.summonerId || '未知召唤师'
        })) || [],
      theirTeam:
        enrichedSession.value.theirTeam?.map((p: any) => ({
          ...p,
          displayName: p.gameName && p.tagLine ? `${p.gameName}#${p.tagLine}` : p.summonerId || '未知召唤师'
        })) || []
    }

    cachedChampSelectData.value = {
      myTeam: enrichedData.myTeam,
      theirTeam: enrichedData.theirTeam,
      session: enrichedData
    }
    console.log('[Cache] 缓存选人阶段数据:', cachedChampSelectData.value)
  }

  // 在游戏进行中时启动 LiveClient 事件监听
  if (phase === 'InProgress') {
    console.log('[LiveClient] 游戏进入进行中阶段，启动事件监听')
    liveClientManager.resetState()
    const localPlayerName = getLocalPlayerName()
    // 刷新后首帧立即强制拉取一次，快速填充双方信息
    await liveClientManager.getLivePlayers(localPlayerName)
    await liveClientManager.startLiveClientAvailabilityCheck(localPlayerName)
  } else {
    // 不在游戏进行中时停止监听并重置状态
    liveClientManager.resetState()

    // 如果离开游戏阶段，清除缓存
    if (phase !== 'ChampSelect' && phase !== 'GameStart') {
      cachedChampSelectData.value = null
      console.log('[Cache] 清除选人阶段缓存')
    }
  }

  // 选人阶段获取战绩数据：注意不要用 `!summonerStats.value` 做判断（空数组也为 truthy）
  if (isConnected.value && enrichedSession.value && phase === 'ChampSelect') {
    console.log('enrichedSession', enrichedSession.value)
    if (!Array.isArray(enrichedSession.value.myTeam)) console.log('myTeam', enrichedSession.value.myTeam)

    try {
      await errorHandler.withRetry(
        async () => {
          // 获取我方队伍战绩
          const myTeamPlayers =
            enrichedSession.value.myTeam?.map((p: any, index: number) => {
              const qName = p.gameName && p.tagLine ? `${p.gameName}#${p.tagLine}` : ''
              const isUnqueryable = p.nameVisibilityType === 'HIDDEN' || !qName
              return {
                summonerName: qName,
                isBot: isUnqueryable,
                index
              }
            }) || []

          // 获取敌方队伍战绩
          const theirTeamPlayers =
            enrichedSession.value.theirTeam?.map((p: any, index: number) => {
              const qName = p.gameName && p.tagLine ? `${p.gameName}#${p.tagLine}` : ''
              const isUnqueryable = p.nameVisibilityType === 'HIDDEN' || !qName
              return {
                summonerName: qName,
                isBot: isUnqueryable,
                index
              }
            }) || []

          await matchHistoryFetcher.fetchTeamMatchHistory(myTeamPlayers, theirTeamPlayers)
        },
        { maxRetries: 2 },
        '获取战绩'
      )
    } catch (error) {
      errorHandler.handleError(error instanceof Error ? error : String(error), '获取战绩')
    }
  }
})

// 监听 LiveClient 数据变化，获取战绩
watch(
  () => liveClientManager.hasData.value,
  async (hasData) => {
    if (hasData && currentPhase.value === 'InProgress') {
      console.log('[MatchAnalysis] LiveClient 数据可用，开始获取战绩')

      try {
        await errorHandler.withRetry(
          async () => {
            const resolveQueryName = (name: string | undefined, team: 'my' | 'enemy'): string => {
              if (!name) return ''
              if (name.includes('#')) return name
              // 从缓存的选人阶段数据推断（仅限我方，敌方通常不可见）
              if (team === 'my' && cachedChampSelectData.value?.myTeam?.length) {
                const found = cachedChampSelectData.value.myTeam.find((pm: any) => {
                  const gn = pm?.gameName
                  return typeof gn === 'string' && gn.length > 0 && gn === name
                })
                if (found && found.gameName && found.tagLine) {
                  return `${found.gameName}#${found.tagLine}`
                }
              }
              // 回退：直接使用原始名称尝试查询（服务器可能可解析）
              return name
            }

            const myTeamInputs = liveClientManager.myTeamPlayers.value.map((p: any, index: number) => {
              const qName = resolveQueryName(p.displayName, 'my')
              return {
                summonerName: qName,
                isBot: p.isBot,
                index
              }
            })

            const enemyTeamInputs = liveClientManager.players.value.map((p: any, index: number) => {
              const qName = resolveQueryName(p.displayName, 'enemy')
              return {
                summonerName: qName,
                isBot: p.isBot,
                index
              }
            })

            await matchHistoryFetcher.fetchTeamMatchHistory(myTeamInputs, enemyTeamInputs)
          },
          { maxRetries: 2 },
          '获取战绩'
        )
      } catch (error) {
        errorHandler.handleError(error instanceof Error ? error : String(error), '获取战绩')
      }
    }
  }
)

// 组件挂载
onMounted(async () => {
  console.log('[MatchAnalysis] 组件挂载，开始初始化')

  try {
    await loadChampions()
    console.log('[MatchAnalysis] 初始化完成')
  } catch (error) {
    errorHandler.handleError(error instanceof Error ? error : String(error), '组件初始化')
  }
})

// 组件卸载
onUnmounted(() => {
  console.log('[MatchAnalysis] 组件卸载，清理资源')
  liveClientManager.resetState()
  matchHistoryFetcher.clearCache()
})
</script>
