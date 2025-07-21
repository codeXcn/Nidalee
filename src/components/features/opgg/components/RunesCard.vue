<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <Zap class="w-5 h-5" />
        推荐符文
      </CardTitle>
      <CardDescription>选择最适合的符文配置应用到游戏中</CardDescription>
    </CardHeader>
    <CardContent>
      <div v-if="perks && perks.length > 0" class="space-y-4">
        <div v-for="(rune, index) in perks.slice(0, 3)" :key="index" class="p-4 border rounded-lg">
          <div class="flex items-center justify-between mb-4">
            <div class="font-medium">符文配置 #{{ index + 1 }}</div>
            <Button @click="handleApplyRunes(index)" size="sm" variant="outline" class="flex items-center gap-2">
              <Zap class="w-4 h-4" />
              应用此符文
            </Button>
          </div>

          <!-- 符文图标显示 -->
          <div class="flex flex-wrap items-center gap-4">
            <!-- 主系符文 -->
            <div class="flex items-center gap-2">
              <Badge variant="secondary">主系</Badge>
              <div class="flex gap-1">
                <img
                  v-for="runeId in rune.perks.slice(0, 4)"
                  :key="runeId"
                  :src="getRuneIconUrl(runeId)"
                  :alt="`符文 ${runeId}`"
                  class="w-8 h-8 rounded border hover:border-primary transition-colors"
                  @error="onRuneImageError"
                />
              </div>
            </div>

            <!-- 副系符文 -->
            <div class="flex items-center gap-2">
              <Badge variant="outline">副系</Badge>
              <div class="flex gap-1">
                <img
                  v-for="runeId in rune.perks.slice(4, 6)"
                  :key="runeId"
                  :src="getRuneIconUrl(runeId)"
                  :alt="`符文 ${runeId}`"
                  class="w-8 h-8 rounded border hover:border-primary transition-colors"
                  @error="onRuneImageError"
                />
              </div>
            </div>

            <!-- 属性符文 -->
            <div v-if="rune.perks.length > 6" class="flex items-center gap-2">
              <Badge variant="secondary">属性</Badge>
              <div class="flex gap-1">
                <img
                  v-for="runeId in rune.perks.slice(6, 9)"
                  :key="runeId"
                  :src="getRuneIconUrl(runeId)"
                  :alt="`符文 ${runeId}`"
                  class="w-6 h-6 rounded border hover:border-primary transition-colors"
                  @error="onRuneImageError"
                />
              </div>
            </div>
          </div>

          <!-- 符文统计信息 -->
          <div class="mt-3 text-sm text-muted-foreground">
            胜率: {{ formatPercentage(rune.win / rune.play) }} | 选取率: {{ formatPercentage(rune.pickRate) }} |
            {{ rune.play }} 局游戏
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Zap } from 'lucide-vue-next'

// 使用后端生成的类型
interface Props {
  perks: OpggPerk[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  applyRunes: [index: number]
}>()

// 格式化百分比
const formatPercentage = (value: number): string => {
  return (value * 100).toFixed(1) + '%'
}

// 获取符文图标URL
const getRuneIconUrl = (runeId: number): string => {
  return `/src/assets/RuneIconFiles/${runeId}.png`
}

// 符文图标错误处理
const onRuneImageError = (event: Event) => {
  const img = event.target as HTMLImageElement
  img.src = '/src/assets/RuneIconFiles/5001.png'
}

// 处理符文应用
const handleApplyRunes = (index: number) => {
  emit('applyRunes', index)
}
</script>
