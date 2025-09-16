<template>
  <Card class="p-8 rounded-2xl shadow-lg bg-gradient-to-br">
    <div class="space-y-6">
      <!-- 标题 -->
      <div class="flex items-center space-x-3 mb-2">
        <Zap class="h-6 w-6 text-blue-500 animate-pulse" />
        <h3 class="text-xl font-bold tracking-wide text-blue-700">匹配测试</h3>
      </div>

      <!-- 匹配状态 -->
      <div class="flex flex-col items-center justify-center p-6 rounded-xl border shadow transition-all">
        <div class="flex items-center space-x-3 mb-2">
          <span
            class="h-3 w-3 rounded-full animate-pulse"
            :class="{
              'bg-green-400': matchmakingState?.searchState === 'Searching',
              'bg-yellow-400': matchmakingState?.searchState === 'Found',
              'bg-red-400': !['Searching', 'Found'].includes(matchmakingState?.searchState ?? '')
            }"
          ></span>
          <span class="text-base font-medium text-gray-600">匹配状态</span>
        </div>
        <p
          class="text-2xl font-extrabold tracking-wider"
          :class="{
            'text-green-500': matchmakingState?.searchState === 'Searching',
            'text-yellow-500': matchmakingState?.searchState === 'Found',
            'text-red-500': !['Searching', 'Found'].includes(matchmakingState?.searchState ?? '')
          }"
        >
          {{
            matchmakingState?.searchState === 'Searching'
              ? '匹配中…'
              : matchmakingState?.searchState === 'Found'
                ? '已匹配到！'
                : '未匹配'
          }}
        </p>
      </div>

      <!-- 匹配控制 -->
      <div class="flex flex-col items-center space-y-3">
        <Button
          class="w-40 h-12 text-lg font-semibold rounded-full shadow transition-all"
          :variant="matchmakingState?.searchState === 'Searching' ? 'destructive' : 'default'"
          @click="handleMatchmaking"
        >
          {{ matchmakingState?.searchState === 'Searching' ? '停止匹配' : '开始匹配' }}
        </Button>

        <div v-if="matchmakingState?.searchState === 'Found'" class="flex items-center space-x-4 mt-2">
          <Button variant="default" class="rounded-full px-6" @click="handleAcceptMatch">接受对局</Button>
          <Button variant="destructive" class="rounded-full px-6" @click="handleDeclineMatch">拒绝对局</Button>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Zap } from 'lucide-vue-next'
import { useMatchmaking } from '@/composables'

const { matchmakingState, handleMatchmaking, handleAcceptMatch, handleDeclineMatch } = useMatchmaking()
</script>
