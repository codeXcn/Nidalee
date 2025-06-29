<template>
  <div v-if="isConnected" class="min-h-screen bg-background">
    <div class="max-w-7xl mx-auto">
      <!-- 游戏助手组件 -->
      <GameHelper />
    </div>
  </div>
  <ClientDisconnected v-else :onRetry="tryReconnect" />
</template>

<script setup lang="ts">
import { appContextKey } from '@/types'

const { isConnected } = inject(appContextKey)
</script>
