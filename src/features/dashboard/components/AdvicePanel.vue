<template>
  <Card class="p-6">
    <div class="space-y-6">
      <!-- 标题 -->
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold flex items-center">
            <Lightbulb class="h-5 w-5 mr-2 text-yellow-500" />
            {{ title }}
          </h3>
          <p v-if="subtitle" class="text-sm text-muted-foreground">
            {{ subtitle }}
          </p>
        </div>

        <!-- 视角切换按钮组 -->
        <div class="flex items-center gap-2">
          <div class="flex rounded-lg border bg-muted/30 p-1">
            <button
              v-for="option in perspectiveOptions"
              :key="option.value"
              @click="$emit('perspective-change', option.value)"
              class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium transition-all"
              :class="[
                perspective === option.value
                  ? 'bg-background shadow-sm text-foreground'
                  : 'text-muted-foreground hover:text-foreground'
              ]"
            >
              <component :is="option.icon" class="h-3.5 w-3.5" />
              <span>{{ option.label }}</span>
            </button>
          </div>
        </div>
      </div>

      <!-- 无建议状态 -->
      <div v-if="!advice || advice.length === 0" class="flex items-center justify-center py-12">
        <div class="text-center">
          <Sparkles class="h-12 w-12 text-green-500 mx-auto mb-4" />
          <p class="text-lg font-medium text-foreground">
            {{ emptyMessage }}
          </p>
          <p class="text-sm text-muted-foreground mt-2">
            {{ emptySubMessage }}
          </p>
        </div>
      </div>

      <!-- 建议列表 -->
      <div v-else class="space-y-4">
        <AdviceCard v-for="(item, index) in advice" :key="index" :advice="item" :perspective="perspective" />

        <!-- 底部提示 -->
        <div class="text-xs text-muted-foreground text-center pt-2 border-t">
          共 {{ advice.length }} 条建议，按优先级排序
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Lightbulb, Sparkles, User, Target, Users } from 'lucide-vue-next'
import { Card } from '@/components/ui/card'
import AdviceCard from './AdviceCard.vue'
import type { GameAdvice } from '@/types/generated/GameAdvice'

interface Props {
  advice?: GameAdvice[]
  perspective?: 'self-improvement' | 'targeting' | 'collaboration'
  title?: string
  subtitle?: string
}

const props = withDefaults(defineProps<Props>(), {
  perspective: 'self-improvement',
  title: '💡 智能建议',
  subtitle: '基于你的近期数据分析'
})

defineEmits<{
  'perspective-change': [value: 'self-improvement' | 'targeting' | 'collaboration']
}>()

// 视角选项配置
const perspectiveOptions = [
  {
    value: 'self-improvement',
    label: '自我提升',
    icon: User
  },
  {
    value: 'targeting',
    label: '针对敌人',
    icon: Target
  },
  {
    value: 'collaboration',
    label: '团队协作',
    icon: Users
  }
] as const

// 空状态消息
const emptyMessage = computed(() => {
  switch (props.perspective) {
    case 'self-improvement':
      return '表现优秀！'
    case 'targeting':
      return '对手表现均衡'
    case 'collaboration':
      return '队友表现稳定'
    default:
      return '暂无建议'
  }
})

const emptySubMessage = computed(() => {
  switch (props.perspective) {
    case 'self-improvement':
      return '暂时没有需要改进的地方，继续保持！'
    case 'targeting':
      return '暂无明显弱点可以针对'
    case 'collaboration':
      return '队友发挥稳定，正常配合即可'
    default:
      return '继续保持'
  }
})
</script>
