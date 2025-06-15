import { ref, onMounted, onUnmounted } from 'vue'
import { useGameStore } from '@/stores/gameStore'

export function useGameMonitor() {
  const gameStore = useGameStore()
  const isMonitoring = ref(false)
  const monitorInterval = ref<number | null>(null)
  const sessionInterval = ref<number | null>(null)

  // 启动监控
  const startMonitoring = () => {
    if (isMonitoring.value) return

    isMonitoring.value = true
    gameStore.addActivity('info', '开始监控游戏状态')

    // 立即检查一次连接
    gameStore.checkConnection()

    // 设置定期检查连接状态（每3秒）
    monitorInterval.value = window.setInterval(() => {
      gameStore.checkConnection()
    }, 3000)

    // 设置会话时长更新（每秒）
    sessionInterval.value = window.setInterval(() => {
      gameStore.updateSessionDuration()
    }, 1000)
  }

  // 停止监控
  const stopMonitoring = () => {
    if (!isMonitoring.value) return

    isMonitoring.value = false
    gameStore.addActivity('info', '停止监控游戏状态')

    if (monitorInterval.value) {
      clearInterval(monitorInterval.value)
      monitorInterval.value = null
    }

    if (sessionInterval.value) {
      clearInterval(sessionInterval.value)
      sessionInterval.value = null
    }
  }

  // 重新开始监控
  const restartMonitoring = () => {
    stopMonitoring()
    // setTimeout(() => {
    //   startMonitoring()
    // }, 1000)
  }

  // 尝试连接
  const attemptConnection = async () => {
    await gameStore.connect()
  }

  // 组件挂载时自动开始监控
  // onMounted(() => {
  //   startMonitoring()
  // })

  // 组件卸载时清理
  onUnmounted(() => {
    stopMonitoring()
  })

  return {
    isMonitoring,
    startMonitoring,
    stopMonitoring,
    restartMonitoring,
    attemptConnection
  }
}
