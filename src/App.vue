<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { RouterView } from 'vue-router'
import { useThemeStore } from '@/stores/themeStore'
import {
  useGameStore,
  type GamePhase,
  type LobbyInfo,
  type LcuAuthInfo,
  type MatchStatistics
} from './stores/gameStore'
import { useGameMonitor } from '@/composables/useGameMonitor'
import AppSidebar from '@/components/AppSidebar.vue'
import ConnectionStatus from '@/components/ConnectionStatus.vue'
import { SidebarProvider, SidebarInset, SidebarTrigger } from '@/components/ui/sidebar'
import ThemeSettings from '@/components/ThemeSettings.vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import type { SummonerInfo } from './stores/gameStore'
import TitleBar from './components/TitleBar.vue'

const themeStore = useThemeStore()
const gameStore = useGameStore()
const isDark = computed(() => themeStore.isDark)

// 初始化游戏监控
const { startMonitoring } = useGameMonitor()
// 定义动画类型
const transitions = ['fade', 'slide-fade', 'scale', 'slide-up']
const currentTransition = ref(transitions[0])

// 随机切换动画
const randomTransition = () => {
  const index = Math.floor(Math.random() * transitions.length)
  currentTransition.value = transitions[index]
}

// 监听路由变化
const handleRouteChange = () => {
  randomTransition()
}
// 获取对局历史
const fetchMatchHistory = async () => {
  try {
    console.log('[App] 开始获取对局历史...')
    const matchHistory = await invoke<MatchStatistics>('get_match_history')
    console.log('[App] 对局历史数据:', matchHistory)
    gameStore.updateMatchStatistics(matchHistory)
  } catch (error) {
    console.error('[App] 获取对局历史失败:', error)
    gameStore.addActivity('error', `获取对局历史失败: ${error}`)
  }
}

onMounted(async () => {
  try {
    // 启动游戏监控
    startMonitoring()

    // 恢复持久化的数据
    if (gameStore.isConnected) {
      gameStore.addActivity('info', '从缓存恢复连接状态')
    }
    if (gameStore.summonerInfo) {
      gameStore.addActivity('info', `从缓存恢复召唤师信息: ${gameStore.summonerInfo.displayName}`)
      // 如果有召唤师信息，尝试获取对局历史
      await fetchMatchHistory()
    }
    if (gameStore.authInfo) {
      gameStore.addActivity('info', '从缓存恢复认证信息')
    }

    // 监听召唤师信息变化
    await listen('summoner-change', async (event) => {
      console.log('[Event] 召唤师信息变化:', event.payload)
      if (event.payload) {
        gameStore.updateSummonerInfo(event.payload as SummonerInfo)
        // 当召唤师信息更新时，重新获取对局历史
        await fetchMatchHistory()
      } else {
        gameStore.clearSummonerInfo()
      }
    })

    // 监听连接状态变化
    await listen('lcu-status-change', (event) => {
      console.log('[Event] LCU 连接状态变化:', event.payload)
      gameStore.setConnectionStatus(event.payload as boolean)
    })

    // 监听认证信息变化
    await listen('auth-info-change', (event) => {
      console.log('[Event] 认证信息变化:', event.payload)
      if (event.payload) {
        gameStore.setAuthInfo(event.payload as LcuAuthInfo)
      } else {
        gameStore.clearAuthInfo()
      }
    })

    // 监听游戏阶段变化
    await listen('gameflow-phase-change', (event) => {
      gameStore.updateGamePhase(event.payload as GamePhase | null)
    })

    // 监听房间状态变化
    await listen('lobby-change', (event) => {
      gameStore.updateLobbyInfo(event.payload as LobbyInfo | null)
    })

    console.log('[App] 事件监听器已设置')
  } catch (error) {
    console.error('[App] 设置事件监听器失败:', error)
  }
  // 初始化主题
  themeStore.initTheme()
})
</script>

<template>
  <div id="app" :class="{ dark: isDark }">
    <TitleBar />
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset class="top-8">
        <header
          class="flex h-16 shrink-0 items-center justify-between gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12 border-b border-border/40"
        >
          <div class="flex items-center gap-2 px-4">
            <SidebarTrigger class="-ml-1 text-foreground/70 hover:text-foreground hover:bg-accent" />
            <div class="h-4 w-px bg-border/60 ml-2" />
            <ConnectionStatus />
          </div>
          <div class="px-4">
            <ThemeSettings />
          </div>
        </header>
        <div class="flex flex-1 flex-col gap-6 p-6">
          <router-view v-slot="{ Component }">
            <transition :name="currentTransition" mode="out-in" @before-leave="handleRouteChange">
              <component :is="Component" />
            </transition>
          </router-view>
        </div>
      </SidebarInset>
    </SidebarProvider>
  </div>
</template>

<style>
/* 淡入淡出 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* 滑动淡入淡出 */
.slide-fade-enter-active {
  transition: all 0.2s ease;
}
.slide-fade-leave-active {
  transition: all 0.2s cubic-bezier(1, 0.5, 0.8, 1);
}
.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}

/* 缩放 */
.scale-enter-active,
.scale-leave-active {
  transition: all 0.2s ease;
}
.scale-enter-from,
.scale-leave-to {
  transform: scale(0.98);
  opacity: 0;
}

/* 向上滑动 */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.2s ease;
}
.slide-up-enter-from {
  transform: translateY(20px);
  opacity: 0;
}
.slide-up-leave-to {
  transform: translateY(-20px);
  opacity: 0;
}
</style>
