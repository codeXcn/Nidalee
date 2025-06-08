<template>
  <HoverCard>
    <HoverCardTrigger asChild>
      <div class="p-2 hover:bg-accent rounded-md cursor-pointer">
        <Settings class="h-4 w-4 text-foreground" />
      </div>
    </HoverCardTrigger>
    <HoverCardContent class="w-auto">
      <ThemeCustomizer />
    </HoverCardContent>
  </HoverCard>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Settings } from 'lucide-vue-next'
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@/components/ui/hover-card'
import ThemeCustomizer from '@/components/ThemeCustomizer.vue'

const theme = ref('system')

const setTheme = (newTheme: 'light' | 'dark' | 'system') => {
  theme.value = newTheme
  // 这里可以添加切换主题的逻辑
}
</script>
