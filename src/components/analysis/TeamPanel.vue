<template>
  <Card class="p-6">
    <!-- 搜索框 -->
    <div class="flex items-center gap-2 mb-4">
      <input
        v-model="search"
        type="text"
        placeholder="搜索召唤师名或tag（如 Canyon#95437）"
        class="input input-bordered w-full max-w-xs rounded-lg px-3 py-1 text-sm"
        @keydown.enter="onSearch"
      />
      <button class="btn btn-primary px-4 py-1 rounded-lg" @click="onSearch">查询</button>
    </div>
    <div class="space-y-6">
      <!-- 队伍标题 -->
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold flex items-center">
            <Users class="h-5 w-5 mr-2" :class="[color === 'blue' ? 'text-blue-500' : 'text-red-500']" />
            {{ title }}
          </h3>
          <p class="text-sm text-muted-foreground">队伍信息</p>
        </div>
      </div>

      <!-- 玩家列表 -->
      <div class="grid grid-cols-1 gap-2">
        <PlayerCard
          v-for="player in players"
          :key="player.summonerId + '-' + player.cellId"
          :player="player"
          :is-local="player.cellId === localCellId"
          :selected="!!(selectedPlayer && selectedPlayer.cellId === player.cellId && selectedPlayer.summonerId === player.summonerId)"
          :color="color"
          @select="onSelectPlayer"
        />
      </div>

      <!-- 队伍建议 -->
      <div v-if="suggestions?.length" class="space-y-4 pt-4 border-t">
        <div class="flex items-center justify-between">
          <h4 class="text-sm font-medium flex items-center">
            <Lightbulb class="h-4 w-4 mr-2 text-yellow-500" />
            队伍建议
          </h4>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div v-for="(suggestion, index) in suggestions" :key="index" class="p-3 rounded-lg bg-muted/30">
            <div class="flex items-start space-x-2">
              <div class="p-1 rounded-full" :class="[color === 'blue' ? 'bg-blue-500/10' : 'bg-red-500/10']">
                <Lightbulb class="h-4 w-4" :class="[color === 'blue' ? 'text-blue-500' : 'text-red-500']" />
              </div>
              <p class="text-sm">{{ suggestion }}</p>
            </div>
          </div>
        </div>
      </div>
      <!-- 队伍警告 -->
      <div v-if="warnings?.length" class="space-y-2 pt-2">
        <div class="text-sm text-destructive font-medium">警告：</div>
        <ul class="list-disc pl-5">
          <li v-for="(warning, idx) in warnings" :key="idx" class="text-sm text-destructive">{{ warning }}</li>
        </ul>
      </div>
    </div>
    <!-- 选中队友详细信息弹窗（示例，可自定义更丰富） -->
    <div v-if="selectedPlayer" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-card rounded-xl p-6 shadow-xl w-[340px] relative">
        <button class="absolute top-2 right-2 text-xl" @click="selectedPlayer = null">×</button>
        <div class="font-bold text-lg mb-2">{{ selectedPlayer.displayName }}</div>
        <div class="text-sm text-muted-foreground mb-2">ID: {{ selectedPlayer.summonerId }}</div>
        <div class="text-sm">段位: {{ selectedPlayer.tier || '未知' }}</div>
        <!-- 这里可以扩展显示更多战绩、历史、英雄池等 -->
        <div class="mt-4 text-xs text-muted-foreground">（此处可展示详细战绩、最近比赛等信息）</div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Card } from '@/components/ui/card'
import { Users, Lightbulb } from 'lucide-vue-next'
import PlayerCard from './PlayerCard.vue'
import type { ChampSelectPlayer } from './types/champSelect'
import { inject, ref } from 'vue'

const props = defineProps<{
  title: string
  players: ChampSelectPlayer[]
  color: 'blue' | 'red'
  suggestions?: string[]
  warnings?: string[]
  localPlayerCellId?: number
}>()

const localCellId = props.localPlayerCellId ?? inject('localPlayerCellId', -1)
const selectedPlayer = ref<ChampSelectPlayer | null>(null)
const search = ref('')

function onSelectPlayer(player: ChampSelectPlayer) {
  selectedPlayer.value = player
}
function onSearch() {
  // TODO: 调用后端API查询召唤师信息和战绩
  // emit('search', search.value)
  alert('查询功能待接入后端: ' + search.value)
}
</script>
