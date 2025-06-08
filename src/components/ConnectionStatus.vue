<template>
  <div
    class="flex items-center gap-3 px-4 py-2 rounded-lg border-2 bg-background/50 backdrop-blur-sm shadow-sm"
  >
    <!-- 连接状态指示器 -->
    <div class="flex items-center gap-2">
      <div
        :class="[
          'h-2 w-2 rounded-full shadow-sm',
          connectionStatus === 'connected'
            ? 'bg-green-500'
            : connectionStatus === 'connecting'
              ? 'bg-yellow-500 animate-pulse'
              : 'bg-red-500'
        ]"
      />
      <span class="text-sm font-medium text-foreground">
        {{ statusText }}
      </span>
    </div>

    <!-- 召唤师信息 -->
    <div
      v-if="connectionStatus === 'connected' && summonerName"
      class="flex items-center gap-2 text-sm"
    >
      <div class="h-4 w-px bg-muted-foreground/30" />
      <span class="text-foreground/80">{{ summonerName }}</span>
    </div>

    <!-- 连接按钮 -->
    <button
      v-if="connectionStatus === 'disconnected'"
      @click="connect"
      class="ml-2 px-3 py-1 text-xs bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors"
    >
      连接
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 连接状态
const connectionStatus = ref<'connected' | 'connecting' | 'disconnected'>('disconnected')
const summonerName = ref<string>('')

// 状态文本
const statusText = computed(() => {
  switch (connectionStatus.value) {
    case 'connected':
      return '已连接'
    case 'connecting':
      return '连接中...'
    case 'disconnected':
    default:
      return '未连接'
  }
})

// 连接到LCU
const connect = async () => {
  connectionStatus.value = 'connecting'

  try {
    // 调用Rust后端连接LCU
    await invoke('connect_to_lcu')

    // 获取召唤师信息
    const summoner: any = await invoke('get_current_summoner')

    connectionStatus.value = 'connected'
    summonerName.value = summoner?.displayName || '未知召唤师'
  } catch (error) {
    console.error('连接LCU失败:', error)
    connectionStatus.value = 'disconnected'
  }
}

// 检查连接状态
const checkConnectionStatus = async () => {
  try {
    const isConnected = await invoke('check_lcu_connection')
    if (isConnected) {
      connectionStatus.value = 'connected'
      const summoner: any = await invoke('get_current_summoner')
      summonerName.value = summoner?.displayName || '未知召唤师'
    } else {
      connectionStatus.value = 'disconnected'
    }
  } catch (error) {
    console.error('检查连接状态失败:', error)
    connectionStatus.value = 'disconnected'
  }
}

// 组件挂载时检查连接状态
onMounted(() => {
  // checkConnectionStatus()
  // 定期检查连接状态
  // const interval = setInterval(checkConnectionStatus, 5000)
  // 组件卸载时清理定时器
  // return () => clearInterval(interval)
})
</script>
