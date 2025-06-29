<template>
  <div class="space-y-8">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h3 class="text-lg font-bold text-foreground">主题定制器</h3>
        <p class="text-xs text-muted-foreground">自定义您的组件颜色</p>
      </div>
      <Button
        size="sm"
        variant="outline"
        @click="themeStore.resetTheme"
        class="hover:bg-destructive/10 hover:text-destructive transition"
      >
        <RotateCcw class="h-4 w-4" />
      </Button>
    </div>

    <!-- Color Selection -->
    <div>
      <Label class="text-sm font-semibold mb-2 block">主题主色</Label>
      <div class="grid grid-cols-3 gap-3">
        <button
          v-for="color in themeStore.colors"
          :key="color.name"
          :class="
            cn(
              'flex h-10 w-full items-center justify-center rounded-lg border-2 text-xs font-medium transition-all duration-150',
              themeStore.selectedColor === color.name
                ? 'border-primary ring-2 ring-primary/60 shadow-lg scale-105'
                : 'border-muted hover:border-primary/40 hover:scale-105'
            )
          "
          @click="themeStore.setColor(color.name)"
          type="button"
        >
          <div
            :class="
              cn(
                'h-5 w-5 rounded-full border',
                color.bgClass,
                themeStore.selectedColor === color.name ? 'border-primary' : 'border-muted'
              )
            "
          />
          <span class="ml-2 text-foreground">{{ color.label }}</span>
        </button>
      </div>
    </div>

    <!-- Radius Selection -->
    <div>
      <Label class="text-sm font-semibold mb-2 block">圆角风格</Label>
      <div class="flex gap-2">
        <button
          v-for="radius in themeStore.radiusOptions"
          :key="radius.value"
          :class="
            cn(
              'flex h-9 w-12 items-center justify-center rounded-lg border text-xs font-medium transition-all duration-150',
              themeStore.selectedRadius === radius.value
                ? 'border-primary bg-accent shadow scale-105'
                : 'border-muted hover:border-primary/40 hover:scale-105'
            )
          "
          @click="themeStore.setRadius(radius.value)"
          type="button"
        >
          {{ radius.label }}
        </button>
      </div>
    </div>

    <!-- Theme Mode -->
    <div>
      <Label class="text-sm font-semibold mb-2 block">主题模式</Label>
      <div class="flex items-center gap-3">
        <Switch :model-value="themeStore.isDark" @update:model-value="themeStore.toggleTheme" id="dark-mode" />
        <Label for="dark-mode" class="text-sm font-medium">
          <transition name="fade">
            <span :key="themeStore.isDark">{{ themeStore.isDark ? '深色模式' : '浅色模式' }}</span>
          </transition>
        </Label>
      </div>
      <div class="text-xs text-muted-foreground mt-1">
        当前状态: <span class="font-semibold">{{ themeStore.isDark ? '深色' : '浅色' }}</span>
        <span class="mx-2">|</span>
        HTML类:
        <span :class="htmlHasDarkClass ? 'text-green-500' : 'text-red-400'">{{
          htmlHasDarkClass ? '已添加' : '未添加'
        }}</span>
      </div>
    </div>

    <!-- Style Selection -->
    <div>
      <Label class="text-sm font-semibold mb-2 block">风格</Label>
      <div class="grid grid-cols-2 gap-3">
        <button
          v-for="style in themeStore.styles"
          :key="style.name"
          :class="
            cn(
              'flex items-center justify-center rounded-lg border text-xs h-10 font-medium transition-all duration-150',
              themeStore.selectedStyle === style.name
                ? 'border-primary bg-accent shadow scale-105'
                : 'border-muted hover:border-primary/40 hover:scale-105'
            )
          "
          @click="themeStore.setStyle(style.name)"
          type="button"
        >
          {{ style.label }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { cn } from '@/lib/utils'
import { useThemeStore } from '@/stores'
import { RotateCcw } from 'lucide-vue-next'

const themeStore = useThemeStore()

const htmlHasDarkClass = ref(false)
const updateHtmlClassStatus = () => {
  htmlHasDarkClass.value = document.documentElement.classList.contains('dark')
}

onMounted(() => {
  themeStore.initTheme()
  updateHtmlClassStatus()
  const observer = new MutationObserver(updateHtmlClassStatus)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})
</script>
