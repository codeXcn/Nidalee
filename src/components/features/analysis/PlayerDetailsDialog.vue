<template>
  <Dialog :open="open" @update:open="$emit('close')">
    <DialogContent class="max-w-4xl max-h-[80vh] overflow-hidden flex flex-col">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-3">
          <!-- 玩家头像 -->
          <div class="w-12 h-12 rounded-lg overflow-hidden border border-border">
            <img
              v-if="player?.championId"
              :src="getChampionIconUrl(player.championId)"
              :alt="getChampionName(player.championId)"
              class="w-full h-full object-cover"
            />
            <div v-else class="w-full h-full bg-muted flex items-center justify-center">
              <div class="w-6 h-6 bg-muted-foreground/20 rounded" />
            </div>
          </div>

          <div>
            <div class="text-lg font-bold">{{ player?.displayName || '未知召唤师' }}</div>
            <div class="text-sm text-muted-foreground">
              {{ getChampionName(player?.championId) || '未选择英雄' }}
            </div>
          </div>
        </DialogTitle>
        <DialogDescription> 玩家详细信息和最近战绩 </DialogDescription>
      </DialogHeader>

      <div class="flex-1 overflow-y-auto space-y-6">
        <!-- 基础统计 -->
        <div v-if="stats" class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <Card class="p-4 text-center">
            <div class="text-2xl font-bold text-green-600">{{ stats.wins }}</div>
            <div class="text-sm text-muted-foreground">胜场</div>
          </Card>
          <Card class="p-4 text-center">
            <div class="text-2xl font-bold text-red-600">{{ stats.losses }}</div>
            <div class="text-sm text-muted-foreground">负场</div>
          </Card>
          <Card class="p-4 text-center">
            <div class="text-2xl font-bold text-blue-600">
              {{ Math.round((stats.wins / Math.max(1, stats.wins + stats.losses)) * 100) }}%
            </div>
            <div class="text-sm text-muted-foreground">胜率</div>
          </Card>
          <Card class="p-4 text-center">
            <div class="text-2xl font-bold text-purple-600">{{ stats.wins + stats.losses }}</div>
            <div class="text-sm text-muted-foreground">总场次</div>
          </Card>
        </div>

        <!-- 常用英雄 -->
        <div v-if="stats?.favoriteChampions?.length">
          <h3 class="text-lg font-semibold mb-3">常用英雄</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            <Card v-for="champion in stats.favoriteChampions.slice(0, 6)" :key="champion.championId" class="p-3">
              <div class="flex items-center gap-3">
                <div class="w-12 h-12 rounded-lg overflow-hidden border border-border">
                  <img
                    :src="getChampionIconUrl(champion.championId)"
                    :alt="getChampionName(champion.championId)"
                    class="w-full h-full object-cover"
                  />
                </div>
                <div class="flex-1">
                  <div class="font-medium">{{ getChampionName(champion.championId) }}</div>
                  <div class="text-sm text-muted-foreground">
                    {{ champion.games }}场 · {{ Math.round((champion.wins / Math.max(1, champion.games)) * 100) }}% 胜率
                  </div>
                </div>
              </div>
            </Card>
          </div>
        </div>

        <!-- 最近战绩 -->
        <div v-if="stats?.recentPerformance?.length">
          <h3 class="text-lg font-semibold mb-3">最近战绩</h3>
          <div class="space-y-2">
            <Card v-for="(match, index) in stats.recentPerformance.slice(0, 10)" :key="index" class="p-3">
              <div class="flex items-center gap-3">
                <!-- 胜负标识 -->
                <div
                  class="w-8 h-8 rounded-full flex items-center justify-center text-white font-bold text-sm"
                  :class="match.win ? 'bg-green-500' : 'bg-red-500'"
                >
                  {{ match.win ? '胜' : '负' }}
                </div>

                <!-- 英雄信息 -->
                <div class="w-10 h-10 rounded-lg overflow-hidden border border-border">
                  <img
                    v-if="match.championId"
                    :src="getChampionIconUrl(match.championId)"
                    :alt="getChampionName(match.championId)"
                    class="w-full h-full object-cover"
                  />
                  <div v-else class="w-full h-full bg-muted flex items-center justify-center">
                    <span class="text-xs text-muted-foreground">未知</span>
                  </div>
                </div>

                <!-- KDA -->
                <div class="flex-1">
                  <div class="font-medium">{{ match.championName }}</div>
                  <div class="text-sm text-muted-foreground">
                    {{ match.kills }}/{{ match.deaths }}/{{ match.assists }} · KDA
                    {{ ((match.kills + match.assists) / Math.max(1, match.deaths)).toFixed(2) }}
                  </div>
                </div>

                <!-- 游戏时长 -->
                <div class="text-sm text-muted-foreground">{{ Math.floor(match.gameDuration / 60) }}分钟</div>
              </div>
            </Card>
          </div>
        </div>

        <!-- 无数据状态 -->
        <div v-else class="text-center py-8">
          <div class="text-muted-foreground">
            <svg class="w-12 h-12 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
              />
            </svg>
            <p class="text-lg">暂无战绩数据</p>
            <p class="text-sm">可能是机器人或匿名用户</p>
          </div>
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="$emit('close')"> 关闭 </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { getChampionIconUrl, getChampionName } from '@/lib'

// 临时使用函数，应该从 lib 中导入
// const getChampionIdByName = (name: string) => {
//   // 这里应该有一个从英雄名称获取ID的函数
//   // 暂时返回0
//   return 0
// }

defineProps<{
  open: boolean
  player: any
  stats: any
}>()

defineEmits<{
  close: []
}>()
</script>
