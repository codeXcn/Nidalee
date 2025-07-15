<script setup lang="ts">
import { Toaster } from 'vue-sonner'
import 'vue-sonner/style.css'
import { appContextKey } from './types'
import { usePlayerListQuery } from './composables/useLolApiQuery'
const { isDark, checkConnection, isConnected, fetchMatchHistory } = useApp()
const theme = computed(() => (isDark.value ? 'dark' : 'light'))
// 提供方法给子组件使用
provide(appContextKey, {
  checkConnection,
  fetchMatchHistory,
  isConnected,
  isDark
})
usePlayerListQuery(true)
const transitions = ['fade', 'slide-fade', 'scale', 'slide-up']
const currentTransition = ref(transitions[0])
const randomTransition = () => {
  const index = Math.floor(Math.random() * transitions.length)
  currentTransition.value = transitions[index]
}
const handleRouteChange = () => {
  randomTransition()
}
</script>

<template>
  <div id="app" :class="{ dark: isDark }" class="h-screen flex flex-col overflow-hidden">
    <Toaster richColors :theme />
    <TitleBar />
    <SidebarProvider class="flex-1 flex overflow-hidden">
      <AppSidebar />
      <SidebarInset class="flex-1 flex flex-col overflow-hidden mt-10">
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
          <div class="flex flex-col gap-6 p-6">
            <router-view v-slot="{ Component }">
              <transition :name="currentTransition" mode="out-in" @before-leave="handleRouteChange">
                <component :is="Component" />
              </transition>
            </router-view>
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  </div>
</template>
