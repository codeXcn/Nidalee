<template>
  <Dialog :open="visible" @update:open="$emit('update:visible', $event)">
    <DialogContent class="!max-w-[80vw] w-[80vw] bg-background text-foreground">
      <DialogHeader>
        <DialogTitle>游戏详细信息</DialogTitle>
        <DialogDescription v-if="selectedGame">
          {{ selectedGame.champion_name }} - {{ formatGameMode(selectedGame.game_mode as string) }} -
          {{ formatRelativeTime(selectedGame.game_creation as number) }}
        </DialogDescription>
      </DialogHeader>

      <ScrollArea class="max-h-[60vh] border-none will-change-auto">
        <!-- 加载状态 -->
        <div v-if="loading" class="flex items-center justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
          <span class="ml-3 text-muted-foreground">正在加载游戏详细信息...</span>
        </div>

        <!-- 详细信息内容 -->
        <div v-else-if="gameDetailData" class="space-y-6 px-2.5 will-change-auto">
          <!-- 基本游戏信息 -->
          <Card>
            <div class="p-4">
              <h4 class="font-semibold mb-3">基本信息</h4>
              <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                <div>
                  <span class="text-muted-foreground">游戏ID:</span>
                  <p class="font-mono">{{ gameDetailData.gameId }}</p>
                </div>
                <div>
                  <span class="text-muted-foreground">游戏时长:</span>
                  <p>{{ formatDuration(gameDetailData.gameDuration || 0) }}</p>
                </div>
                <div>
                  <span class="text-muted-foreground">地图:</span>
                  <p>{{ getMapName(gameDetailData.mapId) }}</p>
                </div>
                <div>
                  <span class="text-muted-foreground">游戏模式:</span>
                  <p>{{ formatGameMode(gameDetailData.gameMode || '') }}</p>
                </div>
                <div>
                  <span class="text-muted-foreground">队列类型:</span>
                  <p>{{ getQueueName(gameDetailData.queueId || 0) }}</p>
                </div>
                <div>
                  <span class="text-muted-foreground">游戏版本:</span>
                  <p class="text-xs font-mono">{{ gameDetailData.gameVersion }}</p>
                </div>
              </div>
            </div>
          </Card>

          <!-- 队伍信息 -->
          <div class="grid grid-cols-1 gap-6">
            <!-- 蓝队 -->
            <Card class="bg-blue-50/50 dark:bg-blue-950/30">
              <div
                class="px-4 py-2 flex items-center font-bold text-blue-700 dark:text-blue-200 border-b border-blue-200 dark:border-blue-800"
              >
                <span class="mr-2">蓝队 ({{ getTeamResult('100') }})</span>
                <span class="ml-auto text-xs font-normal flex items-center">
                  击杀: {{ gameDetailData?.blueTeamStats?.kills || 0 }} | 经济:
                  {{ formatNumber(gameDetailData?.blueTeamStats?.gold_earned || 0) }} | 伤害:
                  {{ formatNumber(gameDetailData?.blueTeamStats?.total_damage_dealt_to_champions || 0) }}
                  | 视野: {{ gameDetailData?.blueTeamStats?.vision_score || 0 }} | BAN:
                  <span
                    v-for="ban in getTeamBans('100', gameDetailData?.teams)"
                    :key="ban.championId"
                    class="inline-block mx-0.5"
                  >
                    <img
                      :src="getChampionIconUrl(ban.championId)"
                      class="h-6 w-6 rounded"
                      :title="getChampionName(ban.championId)"
                    />
                  </span>
                </span>
              </div>
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead v-for="column in columns" :key="column.key" :class="column.class">{{
                      column.label
                    }}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  <TableRow
                    v-for="participant in getTeamParticipants('100', gameDetailData)"
                    :key="participant.participantId"
                  >
                    <TableCell class="flex items-center space-x-2">
                      <img
                        :src="getProfileIconUrl(getPlayerProfileIcon(participant.participantId, gameDetailData))"
                        class="h-8 w-8 rounded-full"
                      />
                      <span class="font-medium truncate">{{
                        getPlayerDisplayName(participant.participantId, gameDetailData)
                      }}</span>
                      <img v-if="participant.rankTier" :src="getRankIconUrl(participant.rankTier)" class="h-6 w-6" />
                    </TableCell>
                    <TableCell class="relative">
                      <div class="flex items-center gap-1">
                        <div class="relative">
                          <img
                            :src="getChampionIconUrl(participant.championId)"
                            class="h-8 w-8"
                            :title="participant.championName"
                          />
                          <span
                            class="absolute -bottom-1 -right-1 bg-gray-900/75 text-white text-[10px] min-w-[16px] h-4 flex items-center justify-center rounded"
                          >
                            {{ participant.stats?.champLevel || '?' }}
                          </span>
                        </div>
                        <span class="text-sm font-medium">{{ participant.championName }}</span>
                      </div>
                    </TableCell>
                    <TableCell>
                      <div class="flex items-center justify-center gap-1 w-full">
                        <img
                          v-for="i in itemSlots"
                          :key="i"
                          :src="getItemIconUrl((participant.stats?.[`item${i}`] as number) || 0, gameVersion)"
                          class="h-6 w-6 rounded bg-gray-100 dark:bg-gray-800"
                          :style="{ opacity: participant.stats?.[`item${i}`] ? 1 : 0.3 }"
                          :alt="participant.stats?.[`item${i}`] ? `装备 ${participant.stats[`item${i}`]}` : '空装备槽'"
                        />
                      </div>
                    </TableCell>
                    <TableCell class="text-center">
                      {{ participant.stats?.kills }}/{{ participant.stats?.deaths }}/{{ participant.stats?.assists }}
                    </TableCell>
                    <TableCell class="text-center text-yellow-700 dark:text-yellow-300">
                      {{ formatNumber(participant.stats?.goldEarned || 0) }}
                    </TableCell>
                    <TableCell class="text-center text-blue-700 dark:text-blue-300">
                      {{ formatNumber(participant.stats?.totalDamageDealtToChampions || 0) }}
                    </TableCell>
                    <TableCell class="text-center font-bold text-lg text-indigo-700 dark:text-indigo-300">
                      {{ participant.score || '-' }}
                    </TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </Card>

            <!-- 红队 -->
            <Card class="bg-red-50/50 dark:bg-red-950/30">
              <div
                class="px-4 py-2 flex items-center font-bold text-red-700 dark:text-red-200 border-b border-red-200 dark:border-red-800"
              >
                <span class="mr-2">红队 ({{ getTeamResult('200') }})</span>
                <span class="ml-auto text-xs font-normal flex items-center">
                  击杀: {{ gameDetailData?.redTeamStats?.kills || 0 }} | 经济:
                  {{ formatNumber(gameDetailData?.redTeamStats?.gold_earned || 0) }} | 伤害:
                  {{ formatNumber(gameDetailData?.redTeamStats?.total_damage_dealt_to_champions || 0) }}
                  | 视野: {{ gameDetailData?.redTeamStats?.vision_score || 0 }} | BAN:
                  <span
                    v-for="ban in getTeamBans('200', gameDetailData?.teams)"
                    :key="ban.championId"
                    class="inline-block mx-0.5"
                  >
                    <img
                      :src="getChampionIconUrl(ban.championId)"
                      class="h-6 w-6 rounded"
                      :title="getChampionName(ban.championId)"
                    />
                  </span>
                </span>
              </div>
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead v-for="column in columns" :key="column.key" :class="column.class">{{
                      column.label
                    }}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  <TableRow
                    v-for="participant in getTeamParticipants('200', gameDetailData)"
                    :key="participant.participantId"
                  >
                    <TableCell class="flex items-center space-x-2">
                      <img
                        :src="getProfileIconUrl(getPlayerProfileIcon(participant.participantId, gameDetailData))"
                        class="h-8 w-8 rounded-full"
                      />
                      <span class="font-medium truncate">{{
                        getPlayerDisplayName(participant.participantId, gameDetailData)
                      }}</span>
                      <img v-if="participant.rankTier" :src="getRankIconUrl(participant.rankTier)" class="h-6 w-6" />
                    </TableCell>
                    <TableCell class="relative">
                      <div class="flex items-center gap-1">
                        <div class="relative">
                          <img
                            :src="getChampionIconUrl(participant.championId)"
                            class="h-8 w-8"
                            :title="participant.championName"
                          />
                          <span
                            class="absolute -bottom-1 -right-1 bg-gray-900/75 text-white text-[10px] min-w-[16px] h-4 flex items-center justify-center rounded"
                          >
                            {{ participant.stats?.champLevel || '?' }}
                          </span>
                        </div>
                        <span class="text-sm font-medium">
                          {{ participant.championName }}
                        </span>
                      </div>
                    </TableCell>
                    <TableCell>
                      <div class="flex items-center justify-center gap-1 w-full">
                        <img
                          v-for="i in itemSlots"
                          :key="i"
                          :src="getItemIconUrl((participant.stats?.[`item${i}`] as number) || 0, gameVersion)"
                          class="h-6 w-6 rounded bg-gray-100 dark:bg-gray-800"
                          :style="{
                            opacity: participant.stats?.[`item${i}`] ? 1 : 0.3
                          }"
                          :alt="participant.stats?.[`item${i}`] ? `装备 ${participant.stats[`item${i}`]}` : '空装备槽'"
                        />
                      </div>
                    </TableCell>
                    <TableCell class="text-center">
                      {{ participant.stats?.kills }}/{{ participant.stats?.deaths }}/{{ participant.stats?.assists }}
                    </TableCell>
                    <TableCell class="text-center text-yellow-700 dark:text-yellow-300">
                      {{ formatNumber(participant.stats?.goldEarned || 0) }}
                    </TableCell>
                    <TableCell class="text-center text-blue-700 dark:text-blue-300">
                      {{ formatNumber(participant.stats?.totalDamageDealtToChampions || 0) }}
                    </TableCell>
                    <TableCell class="text-center font-bold text-lg text-indigo-700 dark:text-indigo-300">
                      {{ participant.score || '-' }}
                    </TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </Card>
          </div>

          <!-- 单项最佳 -->
          <div class="flex gap-4 mt-4">
            <Card class="flex-1 p-4 text-center">
              <img :src="getChampionIconUrl(gameDetailData.bestPlayerChampionId as number)" class="h-10 w-10 mx-auto" />
              <div class="font-bold text-lg mt-2">{{ gameDetailData.maxDamage }}</div>
              <div class="text-xs text-muted-foreground">最高英雄伤害</div>
            </Card>
            <Card class="flex-1 p-4 text-center">
              <img :src="getChampionIconUrl(gameDetailData.maxTankChampionId as number)" class="h-10 w-10 mx-auto" />
              <div class="font-bold text-lg mt-2">{{ gameDetailData.maxTank }}</div>
              <div class="text-xs text-muted-foreground">最高承受伤害</div>
            </Card>
            <Card class="flex-1 p-4 text-center">
              <img :src="getChampionIconUrl(gameDetailData.maxStreakChampionId as number)" class="h-10 w-10 mx-auto" />
              <div class="font-bold text-lg mt-2">{{ gameDetailData.maxStreak }}</div>
              <div class="text-xs text-muted-foreground">最多连杀</div>
            </Card>
          </div>
        </div>
      </ScrollArea>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import {
  getChampionIconUrl,
  getChampionName,
  getItemIconUrl,
  getMapName,
  getProfileIconUrl,
  getQueueName,
  getRankIconUrl
} from '@/lib'
import { useActivityStore, useAppSessionStore } from '@/stores'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  selectedGame: RecentGame | null
}>()

const visible = defineModel<boolean>('visible')

const activityStore = useActivityStore()
const appSessionStore = useAppSessionStore()
const { formatGameMode, formatRelativeTime, formatDuration } = useFormatters()

const loading = ref(false)
const gameDetailData = ref<GameDetailData | null>(null)

// 获取游戏版本
const { gameVersion } = storeToRefs(appSessionStore)

// 监听游戏数据变化
watch(
  () => props.selectedGame,
  async (newGame) => {
    if (newGame) {
      loading.value = true
      try {
        const result = await invoke<GameDetailData>('get_game_detail', {
          gameId: newGame.game_id
        })
        gameDetailData.value = result
      } catch (err) {
        console.error('获取游戏详细信息失败:', err)
        activityStore.addActivity('error', `获取游戏详细信息失败: ${err}`)
      } finally {
        loading.value = false
      }
    }
  }
)

// 表格列定义
const columns = ref<Column[]>([
  { key: 'summoner', label: '召唤师', class: 'w-[200px]' },
  { key: 'champion', label: '英雄/等级', class: 'w-[120px]' },
  { key: 'items', label: '装备', class: 'w-[250px] text-center' },
  { key: 'kda', label: 'KDA', class: 'w-[100px] text-center' },
  { key: 'gold', label: '经济', class: 'w-[100px] text-center' },
  { key: 'damage', label: '伤害', class: 'w-[100px] text-center' },
  { key: 'score', label: '评分', class: 'w-[80px] text-center' }
])

// 装备槽位
const itemSlots = [0, 1, 2, 3, 4, 5, 6]

// 其他辅助函数
const getTeamResult = (teamId: string): string => {
  if (!gameDetailData.value?.teams) return ''
  const team = gameDetailData.value.teams.find((t) => t.teamId.toString() === teamId)
  if (!team) return ''
  return team.win === 'Win' ? '胜方' : '败方'
}

const getTeamParticipants = (teamId: string, gameDetail: GameDetailData) => {
  if (!gameDetail?.participants) return []
  return gameDetail.participants.filter((p) => p.teamId.toString() === teamId)
}

const getTeamBans = (teamId: string, teams: any[]) => {
  if (!teams) return []
  const team = teams.find((t) => t.teamId.toString() === teamId)
  return team?.bans || []
}

const getPlayerProfileIcon = (participantId: number, gameDetail: GameDetailData): number => {
  const identity = gameDetail.participantIdentities?.find((id) => id.participantId === participantId)
  return identity?.player?.profileIcon || 0
}

const getPlayerDisplayName = (participantId: number, gameDetail: GameDetailData): string => {
  const identity = gameDetail.participantIdentities?.find((id) => id.participantId === participantId)
  if (!identity?.player) return '未知玩家'

  const { gameName, tagLine, summonerName } = identity.player
  if (gameName && tagLine) {
    return `${gameName}#${tagLine}`
  }
  return summonerName || '未知玩家'
}

const formatNumber = (num: number): string => {
  return num?.toLocaleString() || '0'
}

// const getRankIconUrl = (tier: string): string => {
//   if (!tier) return ''
//   const tierLower = tier.toLowerCase()
//   return `https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-leagues/global/default/images/gold.png`
// }

// const getItemIconUrl = (itemId: unknown): string => {
//   if (!itemId || typeof itemId !== 'number' || itemId === 0) {
//     // 使用SVG问号占位
//     return 'data:image/svg+xml;utf8,<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg"><rect width="32" height="32" rx="6" fill="%23e5e7eb"/><text x="16" y="22" text-anchor="middle" font-size="20" fill="%239ca3af" font-family="Arial, Helvetica, sans-serif">?</text></svg>'
//   }
//   return `https://ddragon.leagueoflegends.com/cdn/14.23.1/img/item/${itemId}.png`
// }
</script>
