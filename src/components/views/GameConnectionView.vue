<template>
  <div class="game-connection-view">
    <!-- 连接状态卡片 -->
    <div class="connection-card">
      <div class="connection-header">
        <h2 class="text-xl font-bold">游戏连接</h2>
        <div class="status-badge" :class="statusClass">
          {{ statusText }}
        </div>
      </div>

      <div class="connection-content">
        <!-- 连接信息 -->
        <div v-if="gameStore.isConnected" class="connection-info">
          <div class="info-grid">
            <div class="info-item">
              <span class="label">端口</span>
              <span class="value">{{ gameStore.lcuConfig?.port }}</span>
            </div>
            <div class="info-item">
              <span class="label">状态</span>
              <span class="value">已连接</span>
            </div>
          </div>
        </div>

        <!-- 未连接状态 -->
        <div v-else class="disconnected-info">
          <p class="text-gray-600 dark:text-gray-400 mb-4">
            {{
              gameStore.gamePath
                ? '点击连接按钮连接到游戏客户端'
                : '请先设置游戏路径或确保游戏客户端正在运行'
            }}
          </p>
        </div>

        <!-- 操作按钮 -->
        <div class="action-buttons">
          <button
            @click="handleConnect"
            :disabled="gameStore.gameStatus.connecting"
            class="btn btn-primary"
          >
            <LoadingIcon v-if="gameStore.gameStatus.connecting" class="w-4 h-4" />
            <WifiIcon v-else class="w-4 h-4" />
            {{
              gameStore.gameStatus.connecting
                ? '连接中...'
                : gameStore.isConnected
                  ? '重新连接'
                  : '连接游戏'
            }}
          </button>

          <button
            v-if="gameStore.isConnected"
            @click="gameStore.disconnectFromGame"
            class="btn btn-secondary"
          >
            <WifiSlashIcon class="w-4 h-4" />
            断开连接
          </button>

          <button @click="openSettings" class="btn btn-secondary">
            <CogIcon class="w-4 h-4" />
            设置
          </button>
        </div>
      </div>
    </div>

    <!-- 游戏路径快速设置 -->
    <div v-if="!gameStore.gamePath" class="quick-setup-card">
      <div class="setup-header">
        <h3 class="text-lg font-semibold">快速设置</h3>
        <p class="text-sm text-gray-600 dark:text-gray-400">首次使用请设置游戏路径</p>
      </div>

      <div class="setup-content">
        <button @click="openFileDialog" :disabled="selectingPath" class="btn btn-primary w-full">
          <FolderOpenIcon v-if="!selectingPath" class="w-4 h-4" />
          <LoadingIcon v-else class="w-4 h-4" />
          {{ selectingPath ? '选择中...' : '选择游戏安装目录' }}
        </button>

        <div class="setup-hint">
          <p class="text-xs text-gray-500 dark:text-gray-400">通常位于：D:\WeGameApps\英雄联盟</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useGameStore } from '@/stores/useGameStore'
import { WifiIcon, WifiSlashIcon, CogIcon, FolderOpenIcon } from '@heroicons/vue/24/outline'
import { LoadingIcon } from '@/components/ui/icons'

// 定义事件
const emit = defineEmits<{
  openSettings: []
}>()

const gameStore = useGameStore()
const selectingPath = ref(false)

// 连接状态
const statusClass = computed(() => {
  if (gameStore.gameStatus.connecting) return 'connecting'
  if (gameStore.isConnected) return 'connected'
  return 'disconnected'
})

const statusText = computed(() => {
  if (gameStore.gameStatus.connecting) return '连接中'
  if (gameStore.isConnected) return '已连接'
  return '未连接'
})

// 组件挂载时加载游戏路径
onMounted(() => {
  gameStore.loadGamePath()
})

// 处理连接
async function handleConnect() {
  const success = await gameStore.connectToGame()

  if (!success && !gameStore.gamePath) {
    gameStore.addNotification({
      message: '连接失败，请设置正确的游戏路径',
      type: 'warning'
    })
  }
}

// 打开文件选择器
async function openFileDialog() {
  selectingPath.value = true

  try {
    const { open } = await import('@tauri-apps/api/dialog')

    const selectedPath = await open({
      directory: true,
      multiple: false,
      title: '选择英雄联盟安装目录'
    })

    if (selectedPath && typeof selectedPath === 'string') {
      gameStore.setGamePath(selectedPath)
      gameStore.addNotification({
        message: '游戏路径设置成功，可以尝试连接了',
        type: 'success'
      })
    }
  } catch (error) {
    console.error('打开文件对话框失败:', error)
    gameStore.addNotification({
      message: '打开文件选择器失败',
      type: 'error'
    })
  } finally {
    selectingPath.value = false
  }
}

// 打开设置
function openSettings() {
  emit('openSettings')
}
</script>

<style scoped>
.game-connection-view {
  @apply p-6;
}

.connection-card {
  @apply bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-6;
}

.connection-header {
  @apply flex items-center justify-between mb-6;
}

.status-badge {
  @apply px-3 py-1 rounded-full text-sm font-medium;
}

.status-badge.connected {
  @apply bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200;
}

.status-badge.connecting {
  @apply bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200 animate-pulse;
}

.status-badge.disconnected {
  @apply bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200;
}

.connection-content {
  @apply space-y-4;
}

.connection-info {
  @apply p-4 bg-gray-50 dark:bg-gray-700 rounded-lg;
}

.info-grid {
  @apply grid grid-cols-2 gap-4;
}

.info-item {
  @apply flex flex-col;
}

.label {
  @apply text-sm text-gray-600 dark:text-gray-400;
}

.value {
  @apply font-mono text-sm font-medium text-gray-900 dark:text-gray-100;
}

.disconnected-info {
  @apply text-center py-4;
}

.action-buttons {
  @apply flex gap-3 flex-wrap;
}

.btn {
  @apply px-4 py-2 rounded-md font-medium text-sm transition-colors
         flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed;
}

.btn-primary {
  @apply bg-blue-600 hover:bg-blue-700 text-white;
}

.btn-secondary {
  @apply bg-gray-200 hover:bg-gray-300 text-gray-700 
         dark:bg-gray-600 dark:hover:bg-gray-700 dark:text-gray-200;
}

.quick-setup-card {
  @apply bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-6;
}

.setup-header {
  @apply mb-4;
}

.setup-content {
  @apply space-y-3;
}

.setup-hint {
  @apply text-center;
}
</style>
