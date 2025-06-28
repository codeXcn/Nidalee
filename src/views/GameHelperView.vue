<template>
  <ClientDisconnected v-if="!isConnected" :onRetry="tryReconnect" />
  <div v-else class="min-h-screen bg-background">
    <div class="max-w-7xl mx-auto">
      <!-- 游戏助手组件 -->
      <GameHelper />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useConnectStore } from '@/stores'
const { isConnected } = storeToRefs(useConnectStore())
</script>
