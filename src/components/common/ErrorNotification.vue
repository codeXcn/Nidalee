<template>
  <div v-if="hasErrors" class="fixed bottom-4 right-4 z-50">
    <div class="bg-destructive text-destructive-foreground rounded-lg p-4 max-w-sm">
      <div class="flex items-center gap-2">
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
            clip-rule="evenodd"
          />
        </svg>
        <span class="font-medium">发生错误</span>
      </div>
      <p class="text-sm mt-1">{{ errorMessage }}</p>
      <button @click="$emit('clear')" class="text-xs underline mt-2">清除错误</button>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  hasErrors: boolean
  errorMessage?: string
}>()

defineEmits<{
  clear: []
}>()
</script>
