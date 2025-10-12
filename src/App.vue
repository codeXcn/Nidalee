<script setup lang="ts">
import { Toaster } from 'vue-sonner'
import 'vue-sonner/style.css'
import { appContextKey } from './types'
import ClientDisconnected from './components/common/ClientDisconnected.vue'
import { listen } from '@tauri-apps/api/event'
const { isDark, checkConnection, isConnected, fetchMatchHistory } = useApp()
const theme = computed(() => (isDark.value ? 'dark' : 'light'))
// 提供方法给子组件使用
provide(appContextKey, {
  checkConnection,
  fetchMatchHistory,
  isConnected,
  isDark
})
const transitions = ['fade', 'slide-fade', 'scale', 'slide-up']
const currentTransition = ref(transitions[0])
const randomTransition = () => {
  const index = Math.floor(Math.random() * transitions.length)
  currentTransition.value = transitions[index]
}
const handleRouteChange = () => {
  randomTransition()
}
const route = useRoute()

// 临时添加：监听后端 LCU WS 原始事件，便于联调观察
// let unlistenLcuWs: (() => void) | null = null
// let unlistenConnectionState: (() => void) | null = null

// onMounted(async () => {
//   console.log('[LCU-WS] 开始监听 lcu-ws 事件...')
//   try {
//     // 监听 LCU WebSocket 事件
//     unlistenLcuWs = await listen<string>('lcu-ws', (e) => {
//       try {
//         const data = JSON.parse(e.payload)
//         if (Array.isArray(data) && data.length >= 3) {
//           const [messageType, eventType, payload] = data
//           if (messageType === 8 && eventType === 'OnJsonApiEvent') {
//             // 只显示重要事件的日志
//             const importantEvents = [
//               '/lol-gameflow/v1/gameflow-phase',
//               '/lol-gameflow/v1/session',
//               '/lol-champ-select/v1/session',
//               '/lol-lobby/v2/lobby',
//               '/lol-matchmaking/v1/search'
//             ]

//             if (importantEvents.includes(payload.uri)) {
//               console.log(`[LCU-WS] ${payload.uri}:`, payload.eventType, payload.data)
//             }
//           }
//         }
//       } catch {
//         // 静默处理解析错误
//       }
//     })

//     // 监听连接状态变化
//     unlistenConnectionState = await listen('connection-state-changed', (e) => {
//       console.log('[LCU-WS] 连接状态变化:', e.payload)
//     })

//     console.log('[LCU-WS] 监听器注册成功')
//   } catch (e) {
//     console.error('[LCU-WS] 监听失败:', e)
//   }
// })
// onUnmounted(() => {
//   if (unlistenLcuWs) unlistenLcuWs()
//   if (unlistenConnectionState) unlistenConnectionState()
// })
</script>

<template>
  <div id="app" class="h-screen flex flex-col overflow-hidden bg-background">
    <Toaster richColors :theme />
    <TitleBar />
    <template v-if="route.path === '/forbidden'">
      <!-- 只渲染403页面，不渲染主布局 -->
      <router-view />
    </template>
    <template v-else>
      <SidebarProvider class="flex-1 flex overflow-hidden">
        <AppSidebar />
        <SidebarInset class="flex-1 flex flex-col overflow-hidden mt-10 bg-background">
          <header
            class="flex h-16 shrink-0 items-center justify-between gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12 border-b border-border/40"
          >
            <div class="flex items-center gap-2 px-4">
              <SidebarTrigger class="-ml-1 text-foreground/70 hover:text-foreground hover:bg-accent" />
              <div class="h-4 w-px bg-border/60 ml-2" />
              <ConnectionStatus />
            </div>
            <RightToolbars />
          </header>

          <div
            class="flex-1 scrollbar-thin scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar-thumb-slate-400/50 dark:scrollbar-thumb-slate-500/50 scrollbar-track-transparent overflow-y-auto scroll-smooth"
          >
            <div class="flex flex-col gap-6 p-6 bg-background">
              <router-view v-slot="{ Component }">
                <transition :name="currentTransition" mode="out-in" @before-leave="handleRouteChange">
                  <component :is="isConnected ? Component : ClientDisconnected" />
                </transition>
              </router-view>
              <BorderBeam
                class="transition-colors"
                :colorFrom="'var(--color-primary)'"
                :size="250"
                :duration="12"
                :delay="9"
                :border-width="2"
              />
            </div>
          </div>
        </SidebarInset>
      </SidebarProvider>
    </template>
  </div>
</template>
·
