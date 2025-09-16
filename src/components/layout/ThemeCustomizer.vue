<template>
  <div class="space-y-8">
    <div class="flex items-center justify-between">
      <div>
        <h3 class="text-lg font-bold text-foreground">主题定制器</h3>
        <p class="text-xs text-muted-foreground">自定义您的组件颜色</p>
      </div>
      <Button
        size="sm"
        variant="outline"
        @click="settingsStore.resetTheme"
        class="hover:bg-destructive/10 hover:text-destructive transition"
      >
        <RotateCcw class="h-4 w-4" />
      </Button>
    </div>

    <div>
      <Label class="text-sm font-semibold mb-2 block">主题主色</Label>
      <div class="grid grid-cols-3 gap-3">
        <button
          v-for="color in settingsStore.colors"
          :key="color.name"
          :class="
            cn(
              'flex h-10 w-full items-center justify-center rounded-lg border-2 text-xs font-medium transition-all duration-150',
              settingsStore.selectedColor === color.name
                ? 'border-primary ring-2 ring-primary/60 shadow-lg scale-105'
                : 'border-muted hover:border-primary/40 hover:scale-105'
            )
          "
          @click="() => settingsStore.setColor(color.name)"
          type="button"
        >
          <div
            :class="
              cn(
                'h-5 w-5 rounded-full border',
                color.bgClass,
                settingsStore.selectedColor === color.name ? 'border-primary' : 'border-muted'
              )
            "
          />
          <span class="ml-2 text-foreground">{{ color.label }}</span>
        </button>
      </div>
    </div>

    <div>
      <Label class="text-sm font-semibold mb-2 block">圆角风格</Label>
      <div class="flex gap-2">
        <button
          v-for="radius in settingsStore.radiusOptions"
          :key="radius.value"
          :class="
            cn(
              'flex h-9 w-12 items-center justify-center rounded-lg border text-xs font-medium transition-all duration-150',
              settingsStore.selectedRadius === radius.value
                ? 'border-primary bg-accent shadow '
                : 'border-muted hover:border-primary/40 '
            )
          "
          @click="settingsStore.setRadius(radius.value)"
          type="button"
        >
          {{ radius.label }}
        </button>
      </div>
    </div>

    <div>
      <Label class="text-sm font-semibold mb-2 block">主题模式</Label>
      <div class="flex items-center gap-3">
        <button
          :class="
            cn(
              'flex items-center rounded-lg gap-1 border text-sm font-medium transition-co duration-150  border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground px-4 py-2 h-8',
              !settingsStore.isDark ? 'border-primary shadow' : 'border-muted hover:border-primary/40  '
            )
          "
          @click="() => settingsStore.toggleTheme(false)"
          type="button"
        >
          <Sun class="w-4 h-4" /> Light
        </button>
        <button
          :class="
            cn(
              'flex items-center rounded-lg gap-1  border text-sm font-medium transition-co duration-150  border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground px-4 py-2 h-8',
              settingsStore.isDark ? 'border-primary  shadow  ' : 'border-muted hover:border-primary/40  '
            )
          "
          @click="() => settingsStore.toggleTheme(true)"
          type="button"
        >
          <Moon class="w-4 h-4" /> Dark
        </button>
      </div>
      <div class="text-xs text-muted-foreground mt-1">
        当前状态: <span class="font-semibold">{{ settingsStore.isDark ? '深色' : '浅色' }}</span>
        <span class="mx-2">|</span>
        HTML类:
        <span :class="htmlHasDarkClass ? 'text-green-500' : 'text-red-400'">{{
          htmlHasDarkClass ? '已添加' : '未添加'
        }}</span>
      </div>
    </div>

    <div>
      <Label class="text-sm font-semibold mb-2 block">风格</Label>
      <div class="grid grid-cols-2 gap-3">
        <button
          v-for="style in settingsStore.styles"
          :key="style.name"
          :class="
            cn(
              'flex items-center justify-center rounded-lg border text-xs h-10 font-medium transition-all duration-150',
              settingsStore.selectedStyle === style.name
                ? 'border-primary bg-accent shadow '
                : 'border-muted hover:border-primary/40 '
            )
          "
          @click="settingsStore.setStyle(style.name)"
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
import { useSettingsStore } from '@/stores/ui/settingsStore'
import { RotateCcw, Sun, Moon } from 'lucide-vue-next'
import { ref, onMounted } from 'vue'

const settingsStore = useSettingsStore()

// 动态切换 <html> 的 class
function setThemeClass(theme: string, isDark: boolean) {
  const html = document.documentElement
  // 先收集所有 theme-xxx 和 dark
  const removeList: string[] = []
  html.classList.forEach((cls) => {
    if (cls.startsWith('theme-') || cls === 'dark') removeList.push(cls)
  })
  removeList.forEach((cls) => html.classList.remove(cls))
  html.classList.add(`theme-${theme}`)
  if (isDark) html.classList.add('dark')
}

const htmlHasDarkClass = ref(false)
const updateHtmlClassStatus = () => {
  htmlHasDarkClass.value = document.documentElement.classList.contains('dark')
}

onMounted(() => {
  settingsStore.initTheme()
  setThemeClass(settingsStore.selectedColor, settingsStore.isDark)
  updateHtmlClassStatus()
  const observer = new MutationObserver(updateHtmlClassStatus)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})
</script>
