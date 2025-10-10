<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-2xl max-h-[80vh] overflow-y-auto">
      <DialogHeader>
        <DialogTitle>召唤师详情</DialogTitle>
        <DialogDescription v-if="summoner">
          {{ summoner.displayName }} 的详细信息
        </DialogDescription>
      </DialogHeader>

      <div v-if="summoner" class="space-y-6">
        <!-- 基本信息 -->
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <Label>召唤师名称</Label>
            <p class="text-sm font-medium">{{ summoner.displayName }}</p>
          </div>
          <div class="space-y-2" v-if="summoner.championName">
            <Label>当前英雄</Label>
            <p class="text-sm font-medium">{{ summoner.championName }}</p>
          </div>
          <div class="space-y-2" v-if="summoner.assignedPosition">
            <Label>位置</Label>
            <p class="text-sm font-medium">{{ summoner.assignedPosition }}</p>
          </div>
          <div class="space-y-2" v-if="summoner.tier">
            <Label>段位</Label>
            <p class="text-sm font-medium">{{ summoner.tier }}</p>
          </div>
        </div>

        <!-- 战绩信息 -->
        <div v-if="matchHistory && matchHistory.length > 0" class="space-y-4">
          <h3 class="text-lg font-semibold">最近战绩</h3>
          <div class="space-y-2 max-h-60 overflow-y-auto">
            <div
              v-for="(match, index) in matchHistory.slice(0, 10)"
              :key="index"
              class="flex items-center justify-between p-3 border rounded-lg"
            >
              <div class="flex items-center gap-3">
                <div
                  class="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold"
                  :class="match.win ? 'bg-green-500 text-white' : 'bg-red-500 text-white'"
                >
                  {{ match.win ? 'W' : 'L' }}
                </div>
                <div>
                  <p class="text-sm font-medium">{{ match.championName }}</p>
                  <p class="text-xs text-muted-foreground">{{ formatDate(match.gameCreation) }}</p>
                </div>
              </div>
              <div class="text-right">
                <p class="text-sm font-medium">{{ match.kills }}/{{ match.deaths }}/{{ match.assists }}</p>
                <p class="text-xs text-muted-foreground">KDA</p>
              </div>
            </div>
          </div>
        </div>

        <div v-else class="text-center py-8 text-muted-foreground">
          <p>暂无战绩数据</p>
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="open = false">关闭</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'

interface Props {
  open: boolean
  summoner?: any
  matchHistory?: any[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:open': [value: boolean]
  'close': []
}>()

const open = computed({
  get: () => props.open,
  set: (value) => {
    emit('update:open', value)
    if (!value) {
      emit('close')
    }
  }
})

const formatDate = (timestamp: number) => {
  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffDays === 0) {
    return '今天'
  } else if (diffDays === 1) {
    return '昨天'
  } else if (diffDays < 7) {
    return `${diffDays}天前`
  } else {
    return date.toLocaleDateString()
  }
}
</script>

