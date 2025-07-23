<template>
  <div class="min-h-screen bg-background">
    <div class="max-w-7xl mx-auto">
      <!-- 游戏助手组件 -->
      <GameHelper />
    </div>
  </div>
</template>

<script setup lang="ts">
import { appContextKey, type AppContext } from '@/types'

const { isConnected } = inject(appContextKey) as AppContext

const tryReconnect = () => {
  // 这里可以添加重连逻辑
  console.log('Attempting to reconnect...')
}
</script>
