<script setup lang="ts">
import { Toaster } from 'vue-sonner'
import { appContextKey } from '@/types'
import { usePlayerListQuery } from '@/composables/useLolApiQuery'

const { isDark, checkConnection, isConnected, fetchMatchHistory } = useApp()
provide(appContextKey, { checkConnection, fetchMatchHistory, isConnected, isDark })

usePlayerListQuery(true)

const transitions = ['fade', 'slide-fade', 'scale', 'slide-up']
const currentTransition = ref(transitions[0])
</script>

<template>
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

      <ClientDisconnected v-if="!isConnected" />
      <div
        v-else
        class="flex-1 scrollbar-thin scrollbar-thumb-rounded-full scrollbar-track-rounded-full scrollbar-thumb-slate-400/50 dark:scrollbar-thumb-slate-500/50 scrollbar-track-transparent overflow-y-auto scroll-smooth"
      >
        <div class="flex flex-col gap-6 p-6 bg-background">
          <router-view v-slot="{ Component }">
            <transition :name="currentTransition" mode="out-in">
              <component :is="Component" />
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
