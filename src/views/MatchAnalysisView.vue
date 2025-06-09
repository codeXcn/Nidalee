<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-foreground">对局分析</h1>
        <p class="text-muted-foreground">实时对局数据分析和战术建议</p>
      </div>
      <Button size="sm">
        <Search class="h-4 w-4 mr-2" />
        分析当前对局
      </Button>
    </div>

    <!-- 对局状态 -->
    <Card class="p-8">
      <div class="text-center space-y-6">
        <h2 class="text-xl font-semibold">对局状态</h2>

        <div class="flex items-center justify-center space-x-8">
          <!-- 蓝方分数 -->
          <div class="text-center">
            <div class="text-4xl font-bold text-blue-500">78</div>
            <p class="text-sm text-muted-foreground mt-2">我方评分</p>
          </div>

          <!-- VS -->
          <div class="text-center">
            <div class="text-2xl font-bold text-muted-foreground">VS</div>
            <p class="text-sm text-muted-foreground mt-2">对战</p>
          </div>

          <!-- 红方分数 -->
          <div class="text-center">
            <div class="text-4xl font-bold text-red-500">65</div>
            <p class="text-sm text-muted-foreground mt-2">敌方评分</p>
          </div>
        </div>
      </div>
    </Card>

    <!-- 队伍信息 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 蓝方队伍 -->
      <Card class="p-6">
        <div class="space-y-4">
          <h3 class="text-lg font-semibold text-blue-500">蓝方队伍</h3>

          <div class="space-y-3">
            <div
              v-for="player in blueTeam"
              :key="player.id"
              class="flex items-center justify-between p-3 rounded-lg bg-blue-500/5 border border-blue-500/20"
            >
              <div class="flex items-center space-x-3">
                <div
                  class="w-8 h-8 rounded bg-blue-500 text-white flex items-center justify-center text-sm font-medium"
                >
                  {{ player.champion.charAt(0) }}
                </div>
                <div>
                  <p class="font-medium">{{ player.name }}</p>
                  <p class="text-sm text-muted-foreground">{{ player.champion }} - {{ player.position }}</p>
                </div>
              </div>
              <div class="text-right">
                <p class="text-sm font-medium">{{ player.rank }}</p>
                <p class="text-xs text-muted-foreground">胜率: {{ player.winRate }}%</p>
              </div>
            </div>
          </div>
        </div>
      </Card>

      <!-- 红方队伍 -->
      <Card class="p-6">
        <div class="space-y-4">
          <h3 class="text-lg font-semibold text-red-500">红方队伍</h3>

          <div class="space-y-3">
            <div
              v-for="player in redTeam"
              :key="player.id"
              class="flex items-center justify-between p-3 rounded-lg bg-red-500/5 border border-red-500/20"
            >
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 rounded bg-red-500 text-white flex items-center justify-center text-sm font-medium">
                  {{ player.champion.charAt(0) }}
                </div>
                <div>
                  <p class="font-medium">{{ player.name }}</p>
                  <p class="text-sm text-muted-foreground">{{ player.champion }} - {{ player.position }}</p>
                </div>
              </div>
              <div class="text-right">
                <p class="text-sm font-medium">{{ player.rank }}</p>
                <p class="text-xs text-muted-foreground">胜率: {{ player.winRate }}%</p>
              </div>
            </div>
          </div>
        </div>
      </Card>
    </div>

    <!-- 战术建议 -->
    <Card class="p-6">
      <div class="space-y-4">
        <div class="flex items-center space-x-2">
          <Lightbulb class="h-5 w-5 text-yellow-500" />
          <h3 class="text-lg font-semibold">战术建议</h3>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <!-- 优势项 -->
          <div class="space-y-3">
            <h4 class="font-medium text-green-500">优势项</h4>
            <div class="space-y-2">
              <div v-for="advantage in advantages" :key="advantage" class="flex items-center space-x-2">
                <CheckCircle class="h-4 w-4 text-green-500" />
                <span class="text-sm">{{ advantage }}</span>
              </div>
            </div>
          </div>

          <!-- 注意事项 -->
          <div class="space-y-3">
            <h4 class="font-medium text-red-500">注意事项</h4>
            <div class="space-y-2">
              <div v-for="warning in warnings" :key="warning" class="flex items-center space-x-2">
                <AlertTriangle class="h-4 w-4 text-red-500" />
                <span class="text-sm">{{ warning }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Card>

    <!-- 最近对局 -->
    <Card class="p-6">
      <div class="space-y-4">
        <div>
          <h3 class="text-lg font-semibold">最近对局</h3>
          <p class="text-sm text-muted-foreground">最近的对局分析结果</p>
        </div>

        <div class="space-y-3">
          <div
            v-for="match in recentMatches"
            :key="match.id"
            class="flex items-center justify-between p-4 rounded-lg border"
          >
            <div class="flex items-center space-x-3">
              <div :class="['h-3 w-3 rounded-full', match.result === 'win' ? 'bg-green-500' : 'bg-red-500']"></div>
              <div>
                <p class="font-medium">{{ match.champion }}</p>
                <p class="text-sm text-muted-foreground">{{ match.position }} - {{ match.duration }}</p>
              </div>
            </div>
            <div class="text-right">
              <p class="text-sm font-medium">{{ match.score }}</p>
              <p class="text-xs text-muted-foreground">{{ match.date }}</p>
            </div>
          </div>
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Search, Lightbulb, CheckCircle, AlertTriangle } from 'lucide-vue-next'

// 静态数据
const blueTeam = ref([
  { id: 1, name: 'Player1', champion: '尼德丽', position: '打野', rank: '钻石 III', winRate: 68 },
  { id: 2, name: 'Player2', champion: '阿狸', position: '中单', rank: '钻石 II', winRate: 72 },
  { id: 3, name: 'Player3', champion: '金克丝', position: 'ADC', rank: '钻石 IV', winRate: 65 },
  { id: 4, name: 'Player4', champion: '璐璐', position: '辅助', rank: '钻石 III', winRate: 70 },
  { id: 5, name: 'Player5', champion: '奥恩', position: '上单', rank: '钻石 II', winRate: 58 }
])

const redTeam = ref([
  { id: 6, name: 'Enemy1', champion: '盲僧', position: '打野', rank: '钻石 IV', winRate: 62 },
  { id: 7, name: 'Enemy2', champion: '亚索', position: '中单', rank: '钻石 III', winRate: 55 },
  { id: 8, name: 'Enemy3', champion: '卡莎', position: 'ADC', rank: '钻石 II', winRate: 74 },
  { id: 9, name: 'Enemy4', champion: '锤石', position: '辅助', rank: '钻石 IV', winRate: 67 },
  { id: 10, name: 'Enemy5', champion: '剑姬', position: '上单', rank: '钻石 I', winRate: 78 }
])

const advantages = ref(['我方团战能力更强', '中期推进实力优势', '打野前期gank能力出色', '下路组合配合默契'])

const warnings = ref(['敌方剑姬单带能力强大', '注意敌方卡莎的后期能力', '小心盲僧前期入侵', '控制视野防止被开团'])

const recentMatches = ref([
  {
    id: 1,
    champion: '尼德丽',
    position: '打野',
    duration: '28分32秒',
    score: '12/3/8',
    date: '2小时前',
    result: 'win'
  },
  {
    id: 2,
    champion: '阿理',
    position: '中路',
    duration: '35分15秒',
    score: '8/7/12',
    date: '4小时前',
    result: 'lose'
  },
  {
    id: 3,
    champion: '盲僧',
    position: '打野',
    duration: '22分48秒',
    score: '6/2/15',
    date: '6小时前',
    result: 'win'
  }
])
</script>
