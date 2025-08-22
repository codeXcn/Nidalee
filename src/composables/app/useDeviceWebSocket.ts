import { useWebSocket } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'
import { useConnectionStore } from '@/stores/core/connectionStore'

export function useDeviceWebSocket() {
  const router = useRouter()
  const ws = ref<ReturnType<typeof useWebSocket> | null>(null)
  const deviceId = ref<string | null>(null)
  const status = ref<'CONNECTING' | 'OPEN' | 'CLOSED' | 'ERROR'>('CONNECTING')
  const lastServerMsg = ref<string | null>(null)
  const lastError = ref<string | null>(null)

  // 引入 Pinia store
  const connectionStore = useConnectionStore()
  onMounted(async () => {
    console.log('useDeviceWebSocket', import.meta.env.VITE_WS_BASE_URL)
    try {
      // 1. 获取唯一设备ID
      const hash = await invoke<string>('get_machine_hash')
      deviceId.value = hash

      // 2. 拼接 WebSocket 地址
      const wsUrl = `${import.meta.env.VITE_WS_BASE_URL}/${hash}`

      // 3. 建立 WebSocket 连接
      const wsInstance = useWebSocket(wsUrl, {
        autoReconnect: {
          retries: 10,
          delay: 3000,
          onFailed() {
            lastError.value = 'WebSocket 多次重连失败'
          }
        },
        immediate: true
      })

      ws.value = wsInstance

      // 4. 监听状态
      wsInstance.ws.value?.addEventListener('open', () => {
        status.value = 'OPEN'
        console.log('WebSocket连接成功')
      })
      wsInstance.ws.value?.addEventListener('close', () => {
        status.value = 'CLOSED'
        console.log('WebSocket连接关闭')
      })
      wsInstance.ws.value?.addEventListener('error', (e) => {
        status.value = 'ERROR'
        lastError.value = 'WebSocket 错误: ' + (e as any)?.toString()
        console.log('WebSocket错误:', e)
      })

      // 5. 监听服务器返回
      wsInstance.ws.value?.addEventListener('message', (event) => {
        lastServerMsg.value = event.data
        try {
          const obj = JSON.parse(event.data)
          if (obj && typeof obj === 'object') {
            console.log('WebSocket响应:', obj)
            if ('code' in obj) {
              if (obj.code === 403) {
                connectionStore.hasAuth = false
                // 直接跳转到 403 页面
                router.replace('/forbidden')
              } else if (obj.code === 201 || (obj.code >= 200 && obj.code < 300)) {
                connectionStore.hasAuth = true
                // 如果当前在 403 页面，可以跳回首页
                if (router.currentRoute.value.path === '/forbidden') {
                  router.replace('/')
                }
              }
            }
          }
        } catch {
          console.log('WebSocket响应:', event.data)
        }
      })
    } catch (e) {
      lastError.value = '获取设备ID或连接WebSocket失败: ' + (e as any)?.toString()
      status.value = 'ERROR'
    }
  })

  return {
    ws,
    deviceId,
    status,
    lastServerMsg,
    lastError
  }
}
