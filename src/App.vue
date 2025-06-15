<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue'
import ConnectionStatus from '@/components/ConnectionStatus.vue'
import { SidebarProvider, SidebarInset, SidebarTrigger } from '@/components/ui/sidebar'
import ThemeSettings from '@/components/ThemeSettings.vue'
import TitleBar from './components/TitleBar.vue'
import { useApp } from '@/hooks/useApp'
const { isDark } = useApp()

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
  <div id="app" :class="{ dark: isDark }">
    <TitleBar />
    <SidebarProvider class="mt-8">
      <AppSidebar/>
      <SidebarInset>
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
