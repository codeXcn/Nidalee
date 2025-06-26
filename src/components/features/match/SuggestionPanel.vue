<template>
  <Card class="p-6">
    <div class="space-y-6">
      <!-- 对局分析 -->
      <div class="space-y-4">
        <h3 class="text-lg font-semibold flex items-center">
          <BarChart class="h-5 w-5 mr-2 text-blue-500" />
          对局分析
        </h3>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="p-4 rounded-lg bg-muted/30">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium">队伍实力对比</span>
              <span class="text-sm font-bold text-blue-600">{{ teamComparison?.strength || 0 }}%</span>
            </div>
            <div class="h-2 bg-muted rounded-full overflow-hidden">
              <div
                class="h-full bg-blue-500 rounded-full transition-all duration-300"
                :style="{ width: `${teamComparison?.strength || 0}%` }"
              ></div>
            </div>
          </div>
          <div class="p-4 rounded-lg bg-muted/30">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium">英雄池深度</span>
              <span class="text-sm font-bold text-purple-600">{{ teamComparison?.championPool || 0 }}%</span>
            </div>
            <div class="h-2 bg-muted rounded-full overflow-hidden">
              <div
                class="h-full bg-purple-500 rounded-full transition-all duration-300"
                :style="{ width: `${teamComparison?.championPool || 0}%` }"
              ></div>
            </div>
          </div>
          <div class="p-4 rounded-lg bg-muted/30">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium">队伍协同性</span>
              <span class="text-sm font-bold text-green-600">{{ teamComparison?.synergy || 0 }}%</span>
            </div>
            <div class="h-2 bg-muted rounded-full overflow-hidden">
              <div
                class="h-full bg-green-500 rounded-full transition-all duration-300"
                :style="{ width: `${teamComparison?.synergy || 0}%` }"
              ></div>
            </div>
          </div>
        </div>
      </div>

      <!-- 英雄推荐 -->
      <div class="space-y-4">
        <h3 class="text-lg font-semibold flex items-center">
          <Swords class="h-5 w-5 mr-2 text-purple-500" />
          英雄推荐
        </h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- 强势英雄 -->
          <div class="space-y-3">
            <h4 class="text-sm font-medium flex items-center">
              <TrendingUp class="h-4 w-4 mr-2 text-green-500" />
              强势英雄
            </h4>
            <div class="grid grid-cols-1 gap-2">
              <div
                v-for="champ in recommendations?.strong"
                :key="champ.name"
                class="flex items-center p-3 rounded-lg bg-muted/30 hover:bg-muted/50 transition-colors"
              >
                <img :src="champ.icon" :alt="champ.name" class="h-10 w-10 rounded" />
                <div class="ml-3 flex-1">
                  <div class="flex items-center justify-between">
                    <span class="font-medium">{{ champ.name }}</span>
                    <Badge variant="outline" class="text-green-600 border-green-500"> 胜率 {{ champ.winRate }}% </Badge>
                  </div>
                  <div class="text-sm text-muted-foreground mt-1">KDA: {{ champ.kda }}</div>
                </div>
              </div>
            </div>
          </div>

          <!-- 克制英雄 -->
          <div class="space-y-3">
            <h4 class="text-sm font-medium flex items-center">
              <Shield class="h-4 w-4 mr-2 text-red-500" />
              克制英雄
            </h4>
            <div class="grid grid-cols-1 gap-2">
              <div
                v-for="champ in recommendations?.counter"
                :key="champ.name"
                class="flex items-center p-3 rounded-lg bg-muted/30 hover:bg-muted/50 transition-colors"
              >
                <img :src="champ.icon" :alt="champ.name" class="h-10 w-10 rounded" />
                <div class="ml-3 flex-1">
                  <div class="flex items-center justify-between">
                    <span class="font-medium">{{ champ.name }}</span>
                    <Badge variant="outline" class="text-red-600 border-red-500">
                      克制率 {{ champ.counterRate }}%
                    </Badge>
                  </div>
                  <div class="text-sm text-muted-foreground mt-1">对线优势: {{ champ.laneAdvantage }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 战术建议 -->
      <div class="space-y-4">
        <h3 class="text-lg font-semibold flex items-center">
          <Lightbulb class="h-5 w-5 mr-2 text-yellow-500" />
          战术建议
        </h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div v-for="strategy in strategies" :key="strategy.title" class="p-4 rounded-lg bg-muted/30">
            <h4 class="font-medium mb-3">{{ strategy.title }}</h4>
            <ul class="space-y-2">
              <li v-for="(item, index) in strategy.items" :key="index" class="flex items-start space-x-2 text-sm">
                <div class="p-1 rounded-full bg-yellow-500/10 mt-0.5">
                  <Check class="h-3 w-3 text-yellow-500" />
                </div>
                <span>{{ item }}</span>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { BarChart, Swords, TrendingUp, Shield, Lightbulb, Check } from 'lucide-vue-next'

const props = defineProps<{
  teamComparison?: TeamComparison
  recommendations?: {
    strong: ChampionRecommendation[]
    counter: ChampionRecommendation[]
  }
  strategies?: Strategy[]
}>()
</script>
