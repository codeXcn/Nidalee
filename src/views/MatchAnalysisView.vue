<template>
  <div v-if="isConnected" class="min-h-screen">
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
        <TeamCard :team="session.theirTeam" team-type="enemy" @select="openSummonerDetails" />
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
import { usePlayerListQuery } from '@/composables/useLolApiQuery'
import { useGameStore } from '@/stores/features/gameStore'
import { appContextKey, type AppContext } from '@/types'
import { invoke } from '@tauri-apps/api/core'
import { BarChart3, Info, Lightbulb, Users, X } from 'lucide-vue-next'
const init = ref(false)
const { isConnected } = inject(appContextKey) as AppContext
const { enrichedSession } = useChampSelectSession()
const session = computed(() => {
  if (players.value) {
    const theirTeam = players.value
    const data = { ...enrichedSession.value, theirTeam }
    console.log(data)
    return data
  }
})
const shouldShowMatchAnalysis = ref(false)
// 使用搜索召唤师战绩的钩子
const { fetchSummonerInfo, currentRestult, loading: searchLoading } = useSearchMatches()

// 使用游戏状态 store 来监听状态变化
const gameStore = useGameStore()
const { currentPhase } = storeToRefs(gameStore)
const enabled = computed(() => currentPhase.value === 'InProgress')
const { refetch } = usePlayerListQuery(enabled as any)
const playerList = ref([])
const players = computed(() => {
  if (!playerList.value) return []
  // 先将字符串数组解析为对象数组
  let arr: any[] = []
  try {
    if (typeof playerList.value[0] === 'string') {
      arr = playerList.value.map((s: string) => JSON.parse(s))
    } else {
      arr = playerList.value
    }
  } catch (e) {
    console.error('playerList parse error:', e)
    return []
  }
  // 只取敌方队伍
  return arr
    .filter((p: any) => p.team === 'CHAOS')
    .map((p: any, idx: number) => {
      const spell1Id = p.summonerSpells?.summonerSpellOne?.id || 0
      const spell2Id = p.summonerSpells?.summonerSpellTwo?.id || 0
      const profileIconId = p.profileIconId || 0
      const tier = p.tier || ''
      const rankIcon = tier
        ? `https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-leagues/global/default/images/${tier.toLowerCase()}.png`
        : ''
      const spell1Icon = spell1Id
        ? `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/${spell1Id}.jpg`
        : ''
      const spell2Icon = spell2Id
        ? `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/${spell2Id}.jpg`
        : ''
      const championIcon = p.championId
        ? `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/${p.championId}.png`
        : ''
      const avatar = profileIconId
        ? `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/${profileIconId}.jpg`
        : ''
      return {
        cellId: p.cellId ?? idx,
        puuid: p.puuid,
        summonerId: p.summonerId || '',
        championId: p.championId || 0,
        championName: p.championName || '',
        championPickIntent: p.championPickIntent ?? null,
        selectedSkinId: p.selectedSkinId || p.skinID || 0,
        spell1Id,
        spell2Id,
        assignedPosition: p.assignedPosition || p.position || '',
        displayName: p.riotId || p.displayName || p.summonerName || '',
        tagLine: p.tagLine || '',
        profileIconId,
        tier,
        recentMatches: p.recentMatches ?? null,
        avatar,
        isLocal: p.isLocal ?? false,
        isBot: p.isBot ?? false,
        spell1Icon,
        spell2Icon,
        championIcon,
        rankIcon,
        winRate: p.winRate ?? undefined
      }
    })
})
// 召唤师详情相关 - 必须在 watchEffect 之前声明
const isDetailsOpen = ref(false)
const selectedPlayer = ref<any>(null)
// 添加调试信息和会话监听
watchEffect(async () => {
  const phase = currentPhase.value
  console.log('Current game phase:', phase)
  shouldShowMatchAnalysis.value =
    (!!enrichedSession.value && phase === 'ChampSelect') ||
    phase === 'GameStart' ||
    phase === 'InProgress' ||
    phase === 'WaitingForStats' ||
    phase === 'PreEndOfGame' ||
    phase === 'EndOfGame'
  if (phase === 'InProgress' && !init.value) {
    try {
      const res = await refetch()
      if (res.status === 'success') {
        playerList.value = JSON.parse(res.data) || []
        init.value = !init.value
        console.log('Refetched player list:', res, players.value)
      }
    } catch (error) {
      init.value = false
    }
  } else {
    init.value = false
  }
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
      const summonerInfo = (await invoke('get_summoner_by_id', { id: parseInt(player.summonerId) })) as any
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
