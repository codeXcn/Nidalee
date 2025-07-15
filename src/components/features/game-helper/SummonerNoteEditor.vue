<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <Users class="h-5 w-5" />
        个人签名设置
        <span class="text-muted-foreground">自定义你的个性签名</span>
      </CardTitle>
    </CardHeader>
    <CardContent>
      <div class="flex flex-col md:flex-row items-center gap-4">
        <Input
          v-model="note"
          placeholder="输入新的召唤师签名..."
          maxlength="60"
          class="flex-1 h-12 text-base bg-background/50 border-border/50 focus:border-primary/50 focus:bg-background transition-all duration-200 shadow-sm focus:shadow-md"
        />
        <button
          @click="handleSave"
          :disabled="!note.trim() || updatingNote"
          class="px-6 py-2 bg-primary/80 text-primary-foreground rounded-lg hover:bg-primary/90 transition-all duration-200 text-base font-medium shadow-sm hover:shadow-md disabled:opacity-60 disabled:cursor-not-allowed"
        >
          {{ updatingNote ? '保存中...' : '保存签名' }}
        </button>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Users } from 'lucide-vue-next'
import { useGameHelper } from '@/composables/game-helper'

const note = ref('')
const { setSummonerChatProfile, updatingNote } = useGameHelper()

const handleSave = async () => {
  if (!note.value.trim()) return
  await setSummonerChatProfile({ statusMessage: note.value })
}
</script>
