<template>
  <Card class="p-6">
    <div class="space-y-4">
      <div class="flex items-center space-x-2">
        <Zap class="h-5 w-5 text-blue-500" />
        <h3 class="text-lg font-semibold">匹配测试</h3>
      </div>

      <!-- 匹配状态 -->
      <div class="p-4 rounded-lg border">
        <div class="space-y-2">
          <div class="flex items-center space-x-2">
            <div
              class="h-2 w-2 rounded-full"
              :class="{
                'bg-green-500': matchmakingState?.searchState === 'Searching',
                'bg-red-500': matchmakingState?.searchState !== 'Searching'
              }"
            ></div>
            <p class="text-sm font-medium">匹配状态</p>
          </div>
          <p
            class="text-lg font-bold"
            :class="{
              'text-green-500': matchmakingState?.searchState === 'Searching',
              'text-red-500': matchmakingState?.searchState !== 'Searching'
            }"
          >
            {{
              matchmakingState?.searchState === 'Searching'
                ? '匹配中'
                : matchmakingState?.searchState === 'Found'
                  ? '已匹配到'
                  : '未匹配'
            }}
          </p>
        </div>
      </div>

      <!-- 匹配控制 -->
      <div class="flex items-center space-x-4">
        <Button
          :variant="matchmakingState?.searchState === 'Searching' ? 'destructive' : 'default'"
          @click="handleMatchmaking"
        >
          {{ matchmakingState?.searchState === 'Searching' ? '停止匹配' : '开始匹配' }}
        </Button>

        <div v-if="matchmakingState?.searchState === 'Found'" class="flex items-center space-x-4">
          <Button variant="default" @click="handleAcceptMatch"> 接受对局 </Button>
          <Button variant="destructive" @click="handleDeclineMatch"> 拒绝对局 </Button>
        </div>
      </div>

      <!-- 对局信息 -->
      <div v-if="matchInfo" class="p-4 rounded-lg border">
        <h4 class="font-medium mb-2">当前对局信息</h4>
        <div class="space-y-2">
          <p>对局ID: {{ matchInfo.matchId }}</p>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <h5 class="font-medium mb-1">我方队伍</h5>
              <div
                v-for="player in matchInfo.players.filter((p: PlayerInfo) => p.teamId === 1)"
                :key="player.summonerName"
              >
                {{ player.summonerName }}
              </div>
            </div>
            <div>
              <h5 class="font-medium mb-1">对方队伍</h5>
              <div
                v-for="player in matchInfo.players.filter((p: PlayerInfo) => p.teamId === 2)"
                :key="player.summonerName"
              >
                {{ player.summonerName }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Zap } from 'lucide-vue-next'
import { useMatchmaking, type PlayerInfo } from '@/components/game-helper/composables/useMatchmaking'
const { matchmakingState, matchInfo, handleMatchmaking, handleAcceptMatch, handleDeclineMatch } = useMatchmaking()
</script>
