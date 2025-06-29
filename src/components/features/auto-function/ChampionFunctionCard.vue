<template>
  <Card class="transition-all duration-200 hover:shadow-lg border-border bg-card">
    <CardHeader class="space-y-4">
      <div class="flex items-start justify-between">
        <div class="space-y-2 flex-1">
          <div class="flex items-center gap-2">
            <CardTitle class="text-lg font-semibold text-card-foreground">{{ title }}</CardTitle>
            <Tooltip v-if="showRiskWarning" :delay-duration="100">
              <TooltipTrigger>
                <AlertTriangle class="h-5 w-5 text-amber-500 hover:text-amber-600 transition-colors cursor-help" />
              </TooltipTrigger>
              <TooltipContent side="top" class="max-w-xs">
                <p class="text-sm">{{ riskWarningText }}</p>
              </TooltipContent>
            </Tooltip>
          </div>
          <CardDescription class="text-sm text-muted-foreground leading-relaxed">
            {{ description }}
          </CardDescription>
        </div>
        <Switch :model-value="enabled" @update:model-value="enabled = $event" class="ml-4" />
      </div>
    </CardHeader>

    <CardContent v-if="enabled" class="space-y-6 pt-0">
      <Separator />

      <!-- 英雄选择区域 -->
      <div class="space-y-4">
        <Label class="text-sm font-medium text-foreground">选择英雄</Label>

        <div
          v-if="championInfo"
          class="flex items-center justify-between p-4 rounded-lg border border-border bg-muted/30 hover:bg-muted/50 transition-colors"
        >
          <div class="flex items-center gap-3">
            <Avatar class="h-12 w-12 border-2 border-primary ring-1 ring-primary/20">
              <AvatarImage
                :src="getChampionIconUrlByAlias(championInfo.alias)"
                :alt="championInfo.name"
                class="object-cover"
              />
              <AvatarFallback class="text-xs font-medium bg-primary/10 text-primary">
                {{ championInfo.name.slice(0, 2) }}
              </AvatarFallback>
            </Avatar>
            <div class="space-y-1">
              <p class="font-medium text-foreground">{{ championInfo.name }}</p>
              <p class="text-xs text-muted-foreground bg-muted px-2 py-1 rounded">ID: {{ championInfo.id }}</p>
            </div>
          </div>
          <Button variant="destructive" size="sm" @click="handleClearChampion" class="gap-2 hover:bg-destructive/90">
            <X class="h-4 w-4" />
            清除
          </Button>
        </div>

        <div
          v-else
          class="flex items-center justify-center p-8 rounded-lg border-2 border-dashed border-border bg-muted/20 hover:bg-muted/30 hover:border-muted-foreground/50 transition-all duration-200 group"
        >
          <Button
            variant="outline"
            @click="showChampionSelector = true"
            class="gap-2 group-hover:bg-accent group-hover:text-accent-foreground"
          >
            <Plus class="h-4 w-4" />
            选择英雄
          </Button>
        </div>
      </div>

      <!-- 延迟配置区域 -->
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <Label class="text-sm font-medium text-foreground">执行延迟</Label>
          <div class="flex items-center gap-2 text-xs text-muted-foreground bg-muted px-2 py-1 rounded">
            <span>{{ formatDelayDisplay(debouncedDelay) }}</span>
            <div v-if="isDelayPending" class="h-2 w-2 bg-orange-500 rounded-full animate-pulse" title="设置保存中..." />
            <div v-else class="h-2 w-2 bg-green-500 rounded-full" title="设置已保存" />
          </div>
        </div>

        <div class="space-y-4">
          <Slider
            :model-value="delayModel"
            @update:model-value="(val: number[] | undefined) => (delayModel = val || [0])"
            :max="10000"
            :min="0"
            :step="100"
            class="w-full"
          />

          <div class="flex items-center gap-3 bg-muted/50 p-3 rounded-lg">
            <Input
              :model-value="debouncedDelay"
              @update:model-value="
                (val: string | number) => (debouncedDelay = typeof val === 'string' ? parseInt(val) || 0 : val)
              "
              type="number"
              :min="0"
              :max="10000"
              :step="100"
              class="w-28 text-center text-sm"
            />
            <span class="text-xs text-muted-foreground font-medium">毫秒</span>
          </div>
        </div>
      </div>
    </CardContent>

    <!-- 英雄选择对话框 -->
    <Dialog v-model:open="showChampionSelector">
      <DialogContent class="!max-w-[85vw] w-[85vw] !max-h-[85vh] overflow-hidden">
        <DialogHeader class="pb-4">
          <DialogTitle class="text-xl font-semibold">选择英雄</DialogTitle>
          <DialogDescription class="text-muted-foreground"> 选择要自动操作的英雄 </DialogDescription>
        </DialogHeader>
        <div class="overflow-hidden">
          <ChampionSelector @select="handleChampionSelect" />
        </div>
      </DialogContent>
    </Dialog>
  </Card>
</template>

<script setup lang="ts">
import type { ChampionInfo } from '@/stores/autoFunctionStore'
import { getChampionIconUrlByAlias } from '@/lib'
import { Plus, X, AlertTriangle } from 'lucide-vue-next'

import { useDebouncedNumberModel } from '@/composables/utils/useDebouncedModel'

interface Props {
  title: string
  description: string
  championInfo: ChampionInfo | null
  showRiskWarning?: boolean
  riskWarningText?: string
}

const props = withDefaults(defineProps<Props>(), {
  showRiskWarning: false,
  riskWarningText: '此功能可能存在封号风险，请谨慎使用'
})

// 使用 defineModel 定义双向绑定
const enabled = defineModel<boolean>('enabled', { default: false })
const delay = defineModel<number>('delay', { default: 1000 })

// 使用防抖处理延迟设置
const {
  value: debouncedDelay,
  isPending: isDelayPending,
  flush: flushDelay
} = useDebouncedNumberModel(delay, {
  delay: 500, // 500ms 防抖
  min: 0,
  max: 10000,
  step: 100
})

// 定义其他事件
const emit = defineEmits<{
  'champion-select': [champion: ChampionInfo]
  'champion-clear': []
}>()

// 内部状态
const showChampionSelector = ref(false)

// Slider 需要数组格式，创建计算属性处理转换
const delayModel = computed({
  get: () => {
    return [debouncedDelay.value]
  },
  set: (value: number[] | undefined) => {
    const newDelay = value?.[0] || 0
    debouncedDelay.value = newDelay
  }
})

// 事件处理
const handleChampionSelect = (champion: ChampionInfo) => {
  console.log(`[ChampionFunctionCard] ${props.title} - champion select:`, champion)
  emit('champion-select', champion)
  showChampionSelector.value = false
}

const handleClearChampion = () => {
  console.log(`[ChampionFunctionCard] ${props.title} - champion clear`)
  emit('champion-clear')
}

// 格式化延迟显示
const formatDelayDisplay = (value: number) => {
  if (value < 1000) {
    return `${value}ms`
  }
  return `${(value / 1000).toFixed(1)}s`
}

// 组件卸载时确保保存最后的更改
onBeforeUnmount(() => {
  flushDelay()
})
</script>
