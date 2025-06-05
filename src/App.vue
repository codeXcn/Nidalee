<template>
  <div class="app h-screen bg-background text-foreground flex">
    <!-- 侧边栏 -->
    <Sidebar :active-item="activeView" @navigate="setActiveView" @open-settings="openSettings" />

    <!-- 主内容区域 -->
    <main class="flex-1 flex flex-col overflow-hidden">
      <!-- 顶部导航栏 -->
      <header class="h-16 border-b border-border bg-card px-6 flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <h1 class="text-xl font-semibold">{{ pageTitle }}</h1>
          <div v-if="gameStore.gameStatus" class="flex items-center space-x-2">
                        <div
              :class="[
                'w-2 h-2 rounded-full',
                gameStore.isConnected ? 'bg-green-500' : 'bg-orange-500 animate-pulse'
              ]"
            />
            <span class="text-sm text-muted-foreground">
              {{ gameStore.isConnected ? '已连接' : '等待游戏' }}
            </span>
            <span v-if="gameStore.gameStatus.lockfileInfo" class="text-xs text-muted-foreground">
              (端口: {{ gameStore.gameStatus.lockfileInfo.port }})
            </span>
          </div>
        </div>

        <div class="flex items-center space-x-3">
          <!-- 通知按钮 -->
          <Button variant="outline" size="icon" class="h-9 w-9 relative">
            <Bell class="h-4 w-4" />
            <span
              v-if="gameStore.unreadNotifications > 0"
              class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center"
            >
              {{ gameStore.unreadNotifications }}
            </span>
          </Button>

          <!-- 主题切换 -->
          <ThemeToggle />

          <!-- 设置按钮 -->
          <Button variant="outline" size="icon" class="h-9 w-9" @click="openSettings">
            <Settings class="h-4 w-4" />
          </Button>
        </div>
      </header>

      <!-- 页面内容 -->
      <div class="flex-1 overflow-auto p-6">
        <Transition name="page" mode="out-in">
          <component :is="currentComponent" :key="activeView" />
        </Transition>
      </div>
    </main>

    <!-- 设置对话框 -->
    <Dialog v-model:open="showSettings">
      <DialogContent class="max-w-2xl">
        <DialogHeader>
          <DialogTitle>设置</DialogTitle>
          <DialogDescription> 配置 Nidalee 的各项设置 </DialogDescription>
        </DialogHeader>
        <SettingsPanel />
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription
} from '@/components/ui/dialog'
import { Bell, Settings } from 'lucide-vue-next'

import Sidebar from '@/components/ui/sidebar/Sidebar.vue'
import ThemeToggle from '@/components/ui/theme-toggle/ThemeToggle.vue'
import DashboardView from '@/components/views/DashboardView.vue'
import GameHelperView from '@/components/views/GameHelperView.vue'
import MatchAnalysisView from '@/components/views/MatchAnalysisView.vue'
import AutoFunctionsView from '@/components/views/AutoFunctionsView.vue'
import SafetyView from '@/components/views/SafetyView.vue'
import SettingsPanel from '@/components/settings/SettingsPanel.vue'

// 导入 stores
import { useThemeStore } from '@/stores/useThemeStore'
import { useGameStore } from '@/stores/useGameStore'
import { useUserConfigStore } from '@/stores/useUserConfigStore'

// 使用 stores
const themeStore = useThemeStore()
const gameStore = useGameStore()
const configStore = useUserConfigStore()

const activeView = ref('dashboard')
const showSettings = ref(false)

const viewComponents = {
  dashboard: DashboardView,
  'game-helper': GameHelperView,
  'match-analysis': MatchAnalysisView,
  'auto-functions': AutoFunctionsView,
  safety: SafetyView
}

const pageTitles = {
  dashboard: '仪表板',
  'game-helper': '游戏助手',
  'match-analysis': '对局分析',
  'auto-functions': '自动功能',
  safety: '安全设置'
}

const currentComponent = computed(
  () => viewComponents[activeView.value as keyof typeof viewComponents]
)
const pageTitle = computed(() => pageTitles[activeView.value as keyof typeof pageTitles])

function setActiveView(view: string) {
  activeView.value = view
}

function openSettings() {
  showSettings.value = true
}

// 初始化应用
onMounted(() => {
  // 初始化主题
  themeStore.initializeTheme()

  // 添加一些示例通知
  gameStore.addNotification({
    message: '欢迎使用 Nidalee 游戏助手',
    type: 'info'
  })

  // 启动自动连接检测
  gameStore.startAutoConnect()

  // 如果已经有连接状态，启动进程监控
  if (gameStore.gameStatus.connected) {
    gameStore.startProcessMonitoring()
  }
})

// 组件卸载时清理资源
onUnmounted(() => {
  gameStore.stopProcessMonitoring()
  gameStore.stopAutoConnect()
})
</script>

<style>
/* 页面切换动画 */
.page-enter-active,
.page-leave-active {
  transition: all 0.3s ease;
}

.page-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.page-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

/* 全局样式重置 */
html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  overflow: hidden;
}

.app {
  font-family:
    'Inter',
    'SF Pro Display',
    system-ui,
    -apple-system,
    sans-serif;
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: hsl(var(--border));
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground));
}

/* 响应式设计 */
@media (max-width: 768px) {
  .app {
    flex-direction: column;
  }
}
</style>
