<template>
  <div v-if="showStatus" class="fixed top-4 right-4 z-50">
    <div
      class="flex items-center gap-2 px-3 py-2 rounded-lg shadow-lg transition-all duration-300"
      :class="statusClass"
    >
      <!-- 状态图标 -->
      <div class="w-2 h-2 rounded-full" :class="indicatorClass"></div>

      <!-- 状态文本 -->
      <span class="text-sm font-medium">{{ statusText }}</span>

      <!-- 重连按钮 -->
      <button
        v-if="!isOnline && !isReconnecting"
        @click="reconnect"
        class="ml-2 p-1 rounded hover:bg-black/10 transition-colors"
        title="重新连接"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
          />
        </svg>
      </button>

      <!-- 关闭按钮 -->
      <button @click="dismiss" class="ml-1 p-1 rounded hover:bg-black/10 transition-colors" title="关闭">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  autoHide?: boolean
  hideDelay?: number
}

const props = withDefaults(defineProps<Props>(), {
  autoHide: true,
  hideDelay: 3000
})

const emit = defineEmits<{
  'status-change': [isOnline: boolean]
  reconnect: []
}>()

// 状态管理
const isOnline = ref(true)
const isReconnecting = ref(false)
const showStatus = ref(false)
const lastOnlineTime = ref(Date.now())
const connectionErrors = ref(0)

// 错误处理
const errorHandler = useErrorHandler()

// 网络状态检测
let statusCheckInterval: NodeJS.Timeout | null = null
let hideTimer: NodeJS.Timeout | null = null

// 计算属性
const statusText = computed(() => {
  if (isReconnecting.value) {
    return '重新连接中...'
  }
  if (!isOnline.value) {
    return '网络连接断开'
  }
  if (connectionErrors.value > 0) {
    return `连接不稳定 (${connectionErrors.value} 次错误)`
  }
  return '网络连接正常'
})

const statusClass = computed(() => {
  if (isReconnecting.value) {
    return 'bg-yellow-500 text-white'
  }
  if (!isOnline.value) {
    return 'bg-red-500 text-white'
  }
  if (connectionErrors.value > 0) {
    return 'bg-orange-500 text-white'
  }
  return 'bg-green-500 text-white'
})

const indicatorClass = computed(() => {
  if (isReconnecting.value) {
    return 'animate-pulse bg-white'
  }
  if (!isOnline.value) {
    return 'bg-white'
  }
  if (connectionErrors.value > 0) {
    return 'animate-pulse bg-white'
  }
  return 'bg-white'
})

// 网络状态监听
const setupNetworkListeners = () => {
  // 监听在线/离线事件
  window.addEventListener('online', handleOnline)
  window.addEventListener('offline', handleOffline)

  // 定期检查连接状态
  statusCheckInterval = setInterval(checkConnectionStatus, 5000)
}

const cleanupNetworkListeners = () => {
  window.removeEventListener('online', handleOnline)
  window.removeEventListener('offline', handleOffline)

  if (statusCheckInterval) {
    clearInterval(statusCheckInterval)
    statusCheckInterval = null
  }

  if (hideTimer) {
    clearTimeout(hideTimer)
    hideTimer = null
  }
}

// 网络事件处理
const handleOnline = () => {
  console.log('[NetworkStatus] 网络已连接')
  isOnline.value = true
  isReconnecting.value = false
  connectionErrors.value = 0
  lastOnlineTime.value = Date.now()

  showStatus.value = true
  emit('status-change', true)

  // 自动隐藏状态提示
  if (props.autoHide) {
    scheduleHide()
  }
}

const handleOffline = () => {
  console.log('[NetworkStatus] 网络已断开')
  isOnline.value = false
  showStatus.value = true
  emit('status-change', false)
}

// 检查连接状态
const checkConnectionStatus = async () => {
  if (!isOnline.value) return

  try {
    // 尝试访问一个轻量级的端点来检查连接
    const response = await fetch('/api/health', {
      method: 'HEAD',
      mode: 'no-cors',
      cache: 'no-cache'
    })

    // 如果请求成功，重置错误计数
    connectionErrors.value = 0
  } catch (error) {
    console.warn('[NetworkStatus] 连接检查失败:', error)
    connectionErrors.value++

    // 如果错误次数过多，标记为离线
    if (connectionErrors.value >= 3) {
      isOnline.value = false
      showStatus.value = true
      emit('status-change', false)
    }
  }
}

// 重新连接
const reconnect = async () => {
  if (isReconnecting.value) return

  isReconnecting.value = true
  showStatus.value = true

  try {
    // 等待一段时间后重试
    await new Promise((resolve) => setTimeout(resolve, 2000))

    // 重新检查连接状态
    await checkConnectionStatus()

    if (isOnline.value) {
      connectionErrors.value = 0
      console.log('[NetworkStatus] 重新连接成功')
      emit('reconnect')

      if (props.autoHide) {
        scheduleHide()
      }
    }
  } catch (error) {
    console.error('[NetworkStatus] 重新连接失败:', error)
    errorHandler.handleError(error as Error, '网络重连')
  } finally {
    isReconnecting.value = false
  }
}

// 关闭状态提示
const dismiss = () => {
  showStatus.value = false
}

// 安排自动隐藏
const scheduleHide = () => {
  if (hideTimer) {
    clearTimeout(hideTimer)
  }

  hideTimer = setTimeout(() => {
    if (isOnline.value && connectionErrors.value === 0) {
      showStatus.value = false
    }
  }, props.hideDelay)
}

// 组件挂载
onMounted(() => {
  setupNetworkListeners()

  // 初始检查
  checkConnectionStatus()
})

// 组件卸载
onUnmounted(() => {
  cleanupNetworkListeners()
})
</script>
