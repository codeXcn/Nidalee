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
import { Settings } from 'lucide-vue-next'
</script>
