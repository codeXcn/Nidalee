<template>
  <div class="flex flex-col items-center justify-center py-16 text-center select-none">
    <!-- 动画图标 -->
    <div class="relative mb-6">
      <div class="absolute inset-0 flex items-center justify-center">
        <span class="inline-block w-16 h-16 rounded-full bg-destructive/10 animate-pulse"></span>
      </div>
      <Plug class="relative z-10 w-12 h-12 text-destructive animate-bounce-slow" />
    </div>
    <h2 class="text-xl font-bold text-destructive mb-2">客户端未连接</h2>
    <p class="text-muted-foreground mb-6">请确保客户端已启动并联网，或稍后重试。</p>
    <button
      v-if="onRetry"
      @click="onRetry"
      class="px-6 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-all duration-200 font-medium shadow-sm hover:shadow-md"
    >
      重试连接
    </button>
  </div>
</template>

<script setup lang="ts">
import { Plug } from 'lucide-vue-next'

defineProps<{
  onRetry?: () => void
}>()
</script>

<style scoped>
@keyframes bounce-slow {
  0%,
  100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-12px);
  }
}
.animate-bounce-slow {
  animation: bounce-slow 1.6s infinite cubic-bezier(0.68, -0.55, 0.27, 1.55);
}
</style>
