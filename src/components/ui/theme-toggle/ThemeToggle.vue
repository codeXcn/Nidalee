<template>
  <div class="theme-toggle">
    <DropdownMenu>
      <DropdownMenuTrigger as-child>
        <Button variant="outline" size="icon" class="h-9 w-9">
          <Sun class="size-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
          <Moon
            class="absolute size-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          />
          <span class="sr-only">切换主题</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        align="end"
        class="w-48 backdrop-blur-xl bg-popover/95 dark:bg-popover/90"
      >
        <DropdownMenuItem class="cursor-pointer group" @click="themeStore.setTheme('light')">
          <Sun class="mr-2 size-4 group-hover:text-yellow-500 transition-colors" />
          <span>浅色模式</span>
          <Check v-if="themeStore.theme === 'light'" class="ml-auto size-4 text-primary" />
        </DropdownMenuItem>
        <DropdownMenuItem class="cursor-pointer group" @click="themeStore.setTheme('dark')">
          <Moon class="mr-2 size-4 group-hover:text-blue-400 transition-colors" />
          <span>深色模式</span>
          <Check v-if="themeStore.theme === 'dark'" class="ml-auto size-4 text-primary" />
        </DropdownMenuItem>
        <DropdownMenuItem class="cursor-pointer group" @click="themeStore.setTheme('auto')">
          <Monitor class="mr-2 size-4 group-hover:text-green-500 transition-colors" />
          <span>跟随系统</span>
          <Check v-if="themeStore.theme === 'auto'" class="ml-auto size-4 text-primary" />
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem class="cursor-pointer group" @click="openThemeSelector">
          <Palette class="mr-2 size-4 group-hover:text-purple-500 transition-colors" />
          <span>主题颜色</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>

    <!-- 主题选择对话框 -->
    <Dialog v-model:open="showThemeSelector">
      <DialogContent class="max-w-md">
        <DialogHeader>
          <DialogTitle>选择主题</DialogTitle>
          <DialogDescription> 选择一个预设的主题配色方案 </DialogDescription>
        </DialogHeader>

        <div class="space-y-4">
          <div class="grid grid-cols-1 gap-3">
            <button
              v-for="(preset, key) in themeStore.themePresets"
              :key="key"
              :class="[
                'p-4 rounded-lg border text-left transition-all duration-200 hover:shadow-lg group',
                themeStore.selectedPreset === key
                  ? 'border-primary bg-primary/10 shadow-md ring-2 ring-primary/20'
                  : 'border-border hover:border-primary/50 hover:bg-accent/20 hover:scale-[1.02]'
              ]"
              @click="selectPreset(key)"
            >
              <div class="flex items-center justify-between mb-3">
                <span class="font-medium group-hover:text-primary transition-colors">{{
                  preset.name
                }}</span>
                <Check
                  v-if="themeStore.selectedPreset === key"
                  class="size-4 text-primary animate-in zoom-in-50"
                />
              </div>
              <div class="flex space-x-1.5">
                <div
                  class="size-4 rounded-full border border-border/50 group-hover:scale-110 transition-transform duration-200 shadow-sm"
                  :style="{ backgroundColor: preset.light.primary }"
                />
                <div
                  class="size-4 rounded-full border border-border/50 group-hover:scale-110 transition-transform duration-200 shadow-sm"
                  :style="{ backgroundColor: preset.light.secondary }"
                />
                <div
                  class="size-4 rounded-full border border-border/50 group-hover:scale-110 transition-transform duration-200 shadow-sm"
                  :style="{ backgroundColor: preset.light.accent }"
                />
              </div>
            </button>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" @click="resetTheme"> 重置默认 </Button>
          <Button @click="closeThemeSelector"> 确定 </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator
} from '@/components/ui/dropdown-menu'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Sun, Moon, Monitor, Check, Palette } from 'lucide-vue-next'
import { useThemeStore } from '@/stores/useThemeStore'

const themeStore = useThemeStore()
const showThemeSelector = ref(false)

function openThemeSelector() {
  showThemeSelector.value = true
}

function closeThemeSelector() {
  showThemeSelector.value = false
}

function selectPreset(presetKey: string) {
  themeStore.setPreset(presetKey as any)
}

function resetTheme() {
  themeStore.resetToDefault()
  showThemeSelector.value = false
}
</script>
