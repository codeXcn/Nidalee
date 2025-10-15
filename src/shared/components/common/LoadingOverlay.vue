<template>
  <div class="fixed inset-0 bg-background/80 backdrop-blur-sm flex items-center justify-center z-50">
    <div class="flex flex-col items-center gap-4">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      <p class="text-sm text-muted-foreground">{{ message || '加载中...' }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  message?: string
}>()
</script>
