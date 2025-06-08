<template>
  <Card class="p-6">
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold">快捷功能</h3>
          <p class="text-sm text-muted-foreground">常用功能的快速访问</p>
        </div>
        <div class="flex space-x-2">
          <Button variant="outline" size="sm" class="text-xs" @click="$emit('debug')">
            <Settings class="h-3 w-3 mr-1" />
            调试
          </Button>
          <Button variant="outline" size="sm" class="text-xs" @click="$emit('simulate')">
            <Play class="h-3 w-3 mr-1" />
            测试对局
          </Button>
        </div>
      </div>

      <div class="space-y-4">
        <!-- 自动接受对局 -->
        <div
          class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
        >
          <div class="flex items-center space-x-3">
            <div class="p-2 rounded-lg bg-blue-500/10">
              <Zap class="h-5 w-5 text-blue-500" />
            </div>
            <div>
              <p class="font-medium">自动接受对局</p>
              <p class="text-sm text-muted-foreground">自动接受匹配成功后的对局</p>
            </div>
          </div>
          <Switch
            :checked="autoFunctions.acceptMatch"
            @update:checked="() => toggleAutoFunction('acceptMatch')"
          />
        </div>

        <!-- 自动选择英雄 -->
        <div
          class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
        >
          <div class="flex items-center space-x-3">
            <div class="p-2 rounded-lg bg-green-500/10">
              <User class="h-5 w-5 text-green-500" />
            </div>
            <div>
              <p class="font-medium">自动选择英雄</p>
              <p class="text-sm text-muted-foreground">预设英雄自动选择</p>
            </div>
          </div>
          <Switch
            :checked="autoFunctions.selectChampion"
            @update:checked="() => toggleAutoFunction('selectChampion')"
          />
        </div>

        <!-- 自动符文配置 -->
        <div
          class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
        >
          <div class="flex items-center space-x-3">
            <div class="p-2 rounded-lg bg-purple-500/10">
              <Bookmark class="h-5 w-5 text-purple-500" />
            </div>
            <div>
              <p class="font-medium">自动符文配置</p>
              <p class="text-sm text-muted-foreground">根据英雄自动配置最优符文</p>
            </div>
          </div>
          <Switch
            :checked="autoFunctions.runeConfig"
            @update:checked="() => toggleAutoFunction('runeConfig')"
          />
        </div>

        <!-- 自动禁用英雄 -->
        <div
          class="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
        >
          <div class="flex items-center space-x-3">
            <div class="p-2 rounded-lg bg-red-500/10">
              <Shield class="h-5 w-5 text-red-500" />
            </div>
            <div>
              <p class="font-medium">自动禁用英雄</p>
              <p class="text-sm text-muted-foreground">智能禁用敌方强势英雄</p>
            </div>
          </div>
          <Switch
            :checked="autoFunctions.banChampion"
            @update:checked="() => toggleAutoFunction('banChampion')"
          />
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { User, Settings, Bookmark, Shield, Zap, Play } from 'lucide-vue-next'

const props = defineProps<{
  autoFunctions: {
    acceptMatch: boolean
    selectChampion: boolean
    runeConfig: boolean
    banChampion: boolean
  }
}>()

const emit = defineEmits<{
  (e: 'toggle-auto-function', functionName: 'acceptMatch' | 'selectChampion' | 'runeConfig' | 'banChampion'): void
  (e: 'debug'): void
  (e: 'simulate'): void
}>()

const toggleAutoFunction = (functionName: 'acceptMatch' | 'selectChampion' | 'runeConfig' | 'banChampion') => {
  emit('toggle-auto-function', functionName)
}
</script>
