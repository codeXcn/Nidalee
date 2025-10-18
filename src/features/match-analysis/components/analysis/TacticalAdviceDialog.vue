<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="max-w-2xl max-h-[80vh] overflow-y-auto">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Target class="h-5 w-5" v-if="perspective === 'Targeting'" />
          <Users class="h-5 w-5" v-else />
          {{ dialogTitle }}
        </DialogTitle>
        <DialogDescription>
          {{ dialogDescription }}
        </DialogDescription>
      </DialogHeader>

      <!-- 建议列表 -->
      <div v-if="advice && advice.length > 0" class="space-y-3">
        <AdviceCard v-for="(item, index) in advice" :key="index" :advice="item" :perspective="adviceCardPerspective" />
      </div>

      <!-- 无建议 -->
      <div v-else class="flex flex-col items-center justify-center py-12">
        <Sparkles class="h-12 w-12 text-green-500 mb-4" />
        <p class="text-lg font-medium">该玩家表现优秀</p>
        <p class="text-sm text-muted-foreground mt-2">暂无明显弱点可针对</p>
      </div>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Target, Users, Sparkles } from 'lucide-vue-next'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import AdviceCard from '@/features/dashboard/components/AdviceCard.vue'
import type { GameAdvice } from '@/types/generated/GameAdvice'

interface Props {
  open: boolean
  playerName: string
  perspective: 'Targeting' | 'Collaboration'
  advice?: GameAdvice[]
}

const props = defineProps<Props>()

// 转换perspective为AdviceCard需要的格式
const adviceCardPerspective = computed(() => {
  return props.perspective === 'Targeting' ? 'targeting' : 'collaboration'
})

defineEmits<{
  'update:open': [value: boolean]
}>()

const dialogTitle = computed(() => {
  return props.perspective === 'Targeting'
    ? `🎯 针对 ${props.playerName} 的战术建议`
    : `🤝 协作 ${props.playerName} 的建议`
})

const dialogDescription = computed(() => {
  return props.perspective === 'Targeting'
    ? '基于该玩家历史数据分析，识别弱点并制定针对性战术'
    : '了解该队友的特点，优化团队配合策略'
})
</script>
