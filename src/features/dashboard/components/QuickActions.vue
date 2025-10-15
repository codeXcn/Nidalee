<template>
  <Card class="p-6">
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold">快捷功能</h3>
          <p class="text-sm text-muted-foreground">常用功能的快速访问</p>
        </div>
      </div>
      <!-- <MatchmakingPanel /> -->

      <div class="space-y-4">
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
          <Switch v-model="autoFunctions.acceptMatch.enabled" />
        </div>

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
          <Switch v-model="autoFunctions.selectChampion.enabled" />
        </div>

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
          <Switch v-model="autoFunctions.runeConfig.enabled" />
        </div>

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
          <Switch v-model="autoFunctions.banChampion.enabled" />
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { Bookmark, Shield, User, Zap } from 'lucide-vue-next'

const { autoFunctions } = storeToRefs(useAutoFunctionStore())
</script>
