<template>
  <div class="relative flex min-h-[calc(100vh-180px)] w-full flex-col items-center justify-center bg-background">
    <span
      class="pointer-events-none whitespace-pre-wrap bg-gradient-to-b from-black to-gray-300/80 bg-clip-text text-center text-8xl font-semibold leading-none text-transparent dark:from-white dark:to-slate-900/10"
    >
      Nidalee
    </span>
    <p class="text-muted-foreground my-2">请确保客户端已启动并联网，或稍后重试。</p>
    <ParticlesBg
      class="absolute inset-0"
      :quantity="100"
      :ease="100"
      :color="useSettingsStore().isDark ? '#FFF' : '#000'"
      :staticity="10"
      refresh
    />
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from '@/stores/ui/settingsStore'
</script>
