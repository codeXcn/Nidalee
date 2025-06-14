<template>
  <Card class="w-80 p-6">
    <div class="space-y-6">
      <!-- Header -->
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold">主题定制器</h3>
          <p class="text-sm text-muted-foreground">自定义您的组件颜色</p>
        </div>
        <Button size="sm" variant="outline" @click="themeStore.resetTheme">
          <RotateCcw class="h-4 w-4" />
        </Button>
      </div>

      <!-- Color Selection -->
      <div class="space-y-3">
        <Label class="text-sm font-medium">颜色</Label>
        <div class="grid grid-cols-3 gap-2">
          <button
            v-for="color in themeStore.colors"
            :key="color.name"
            :class="cn(
              'flex h-9 w-full items-center justify-center rounded-md border-2 text-xs transition-colors',
              themeStore.selectedColor === color.name
                ? 'border-primary ring-2 ring-primary'
                : 'border-muted hover:border-muted-foreground/25'
            )"
            @click="themeStore.setColor(color.name)"
            type="button"
          >
            <div :class="cn('h-4 w-4 rounded-full', color.bgClass)" />
            <span class="ml-2 text-foreground">{{ color.label }}</span>
          </button>
        </div>
      </div>

      <!-- Radius Selection -->
      <div class="space-y-3">
        <Label class="text-sm font-medium">圆角</Label>
        <div class="grid grid-cols-5 gap-2">
          <button
            v-for="radius in themeStore.radiusOptions"
            :key="radius.value"
            :class="cn(
              'flex h-9 w-9 items-center justify-center rounded-md border text-xs transition-colors',
              themeStore.selectedRadius === radius.value
                ? 'border-primary bg-accent'
                : 'border-muted hover:border-muted-foreground/25'
            )"
            @click="themeStore.setRadius(radius.value)"
            type="button"
          >
            {{ radius.label }}
          </button>
        </div>
      </div>

      <!-- Theme Mode -->
      <div class="space-y-3">
        <Label class="text-sm font-medium">主题模式</Label>
        <div class="flex items-center space-x-2">
          <Switch
            :model-value="themeStore.isDark"
            @update:model-value="themeStore.toggleTheme"
            id="dark-mode"
          />
          <Label for="dark-mode" class="text-sm">
            {{ themeStore.isDark ? '深色模式' : '浅色模式' }}
          </Label>
        </div>
        <div class="text-xs text-muted-foreground">
          当前状态: {{ themeStore.isDark ? '深色' : '浅色' }} |
          HTML类: {{ htmlHasDarkClass ? '已添加' : '未添加' }}
        </div>
      </div>

      <!-- Style Selection -->
      <div class="space-y-3">
        <Label class="text-sm font-medium">风格</Label>
        <div class="grid grid-cols-2 gap-2">
          <button
            v-for="style in themeStore.styles"
            :key="style.name"
            :class="cn(
              'flex items-center justify-center rounded-md border text-xs h-9 transition-colors',
              themeStore.selectedStyle === style.name
                ? 'border-primary bg-accent'
                : 'border-muted hover:border-muted-foreground/25'
            )"
            @click="themeStore.setStyle(style.name)"
            type="button"
          >
            {{ style.label }}
          </button>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { RotateCcw } from 'lucide-vue-next'
import { cn } from '@/lib/utils'
import { useThemeStore } from '@/stores/themeStore'

const themeStore = useThemeStore()

// 检查 HTML 是否有 dark 类
const htmlHasDarkClass = ref(false)
const updateHtmlClassStatus = () => {
  htmlHasDarkClass.value = document.documentElement.classList.contains('dark')
}

onMounted(() => {
  themeStore.initTheme()
  updateHtmlClassStatus()
  // 监听主题变化，实时更新状态
  const observer = new MutationObserver(updateHtmlClassStatus)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})
</script>
