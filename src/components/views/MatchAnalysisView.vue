<template>
  <div class="match-analysis-view space-y-6">
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold">对局分析</h2>
        <p class="text-muted-foreground">实时对局数据分析和战术建议</p>
      </div>
      <Button :disabled="!gameStatus.inMatch" @click="analyzeCurrentMatch">
        <Search class="w-4 h-4 mr-2" />
        分析当前对局
      </Button>
    </div>

    <!-- 对局状态 -->
    <Card>
      <CardHeader>
        <CardTitle>对局状态</CardTitle>
      </CardHeader>
      <CardContent>
        <div v-if="!gameStatus.inMatch" class="text-center py-8">
          <Users class="w-12 h-12 mx-auto mb-4 text-muted-foreground" />
          <p class="text-muted-foreground">当前不在对局中</p>
          <p class="text-sm text-muted-foreground mt-1">进入英雄选择阶段后开始分析</p>
        </div>
        <div v-else class="space-y-4">
          <div class="grid grid-cols-3 gap-4 text-center">
            <div>
              <p class="text-2xl font-bold text-blue-600">{{ matchData.teamScore }}</p>
              <p class="text-sm text-muted-foreground">我方评分</p>
            </div>
            <div>
              <p class="text-2xl font-bold">VS</p>
              <p class="text-sm text-muted-foreground">对战</p>
            </div>
            <div>
              <p class="text-2xl font-bold text-red-600">{{ matchData.enemyScore }}</p>
              <p class="text-sm text-muted-foreground">敌方评分</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 队伍分析 -->
    <div v-if="gameStatus.inMatch" class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 我方队伍 -->
      <Card>
        <CardHeader>
          <CardTitle class="text-blue-600">我方队伍</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            <div
              v-for="player in matchData.myTeam"
              :key="player.summonerId"
              class="flex items-center space-x-3 p-3 border rounded-lg"
            >
              <div
                class="w-10 h-10 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center"
              >
                <span class="text-blue-600 dark:text-blue-400 font-bold text-sm">
                  {{ player.championName.charAt(0) }}
                </span>
              </div>
              <div class="flex-1">
                <p class="font-medium">{{ player.summonerName }}</p>
                <p class="text-sm text-muted-foreground">
                  {{ player.championName }} - {{ player.position }}
                </p>
              </div>
              <div class="text-right">
                <p class="text-sm font-medium">{{ player.rank }}</p>
                <p class="text-xs text-muted-foreground">胜率: {{ player.winRate }}%</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 敌方队伍 -->
      <Card>
        <CardHeader>
          <CardTitle class="text-red-600">敌方队伍</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            <div
              v-for="player in matchData.enemyTeam"
              :key="player.summonerId"
              class="flex items-center space-x-3 p-3 border rounded-lg"
            >
              <div
                class="w-10 h-10 bg-red-100 dark:bg-red-900 rounded-lg flex items-center justify-center"
              >
                <span class="text-red-600 dark:text-red-400 font-bold text-sm">
                  {{ player.championName.charAt(0) }}
                </span>
              </div>
              <div class="flex-1">
                <p class="font-medium">{{ player.summonerName }}</p>
                <p class="text-sm text-muted-foreground">
                  {{ player.championName }} - {{ player.position }}
                </p>
              </div>
              <div class="text-right">
                <p class="text-sm font-medium">{{ player.rank }}</p>
                <p class="text-xs text-muted-foreground">胜率: {{ player.winRate }}%</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 战术建议 -->
    <Card v-if="gameStatus.inMatch">
      <CardHeader>
        <CardTitle class="flex items-center">
          <Lightbulb class="w-5 h-5 mr-2" />
          战术建议
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="space-y-3">
            <h4 class="font-medium text-green-600">优势项</h4>
            <ul class="space-y-2">
              <li
                v-for="advantage in matchData.advantages"
                :key="advantage"
                class="flex items-center text-sm"
              >
                <CheckCircle class="w-4 h-4 mr-2 text-green-500" />
                {{ advantage }}
              </li>
            </ul>
          </div>
          <div class="space-y-3">
            <h4 class="font-medium text-red-600">注意事项</h4>
            <ul class="space-y-2">
              <li
                v-for="warning in matchData.warnings"
                :key="warning"
                class="flex items-center text-sm"
              >
                <AlertTriangle class="w-4 h-4 mr-2 text-red-500" />
                {{ warning }}
              </li>
            </ul>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- 历史分析 -->
    <Card>
      <CardHeader>
        <CardTitle>最近对局</CardTitle>
        <CardDescription> 最近的对局分析结果 </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="space-y-3">
          <div
            v-for="match in recentMatches"
            :key="match.id"
            class="flex items-center justify-between p-3 border rounded-lg"
          >
            <div class="flex items-center space-x-3">
              <div
                :class="[
                  'w-3 h-3 rounded-full',
                  match.result === 'win' ? 'bg-green-500' : 'bg-red-500'
                ]"
              />
              <div>
                <p class="font-medium">{{ match.champion }}</p>
                <p class="text-sm text-muted-foreground">{{ match.mode }} - {{ match.duration }}</p>
              </div>
            </div>
            <div class="text-right">
              <p class="text-sm font-medium">{{ match.kda }}</p>
              <p class="text-xs text-muted-foreground">{{ match.time }}</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Search, Users, Lightbulb, CheckCircle, AlertTriangle } from 'lucide-vue-next'

const gameStatus = ref({
  inMatch: true // 模拟在对局中
})

const matchData = ref({
  teamScore: 78,
  enemyScore: 65,
  myTeam: [
    {
      summonerId: 1,
      summonerName: 'Player1',
      championName: '尼德丽',
      position: '打野',
      rank: '钻石 III',
      winRate: 68
    },
    {
      summonerId: 2,
      summonerName: 'Player2',
      championName: '阿狸',
      position: '中单',
      rank: '钻石 II',
      winRate: 72
    },
    {
      summonerId: 3,
      summonerName: 'Player3',
      championName: '金克丝',
      position: 'ADC',
      rank: '钻石 IV',
      winRate: 65
    },
    {
      summonerId: 4,
      summonerName: 'Player4',
      championName: '锤石',
      position: '辅助',
      rank: '钻石 III',
      winRate: 70
    },
    {
      summonerId: 5,
      summonerName: 'Player5',
      championName: '奥恩',
      position: '上单',
      rank: '钻石 II',
      winRate: 58
    }
  ],
  enemyTeam: [
    {
      summonerId: 6,
      summonerName: 'Enemy1',
      championName: '盲僧',
      position: '打野',
      rank: '钻石 IV',
      winRate: 62
    },
    {
      summonerId: 7,
      summonerName: 'Enemy2',
      championName: '亚索',
      position: '中单',
      rank: '钻石 III',
      winRate: 55
    },
    {
      summonerId: 8,
      summonerName: 'Enemy3',
      championName: '卡莎',
      position: 'ADC',
      rank: '钻石 II',
      winRate: 74
    },
    {
      summonerId: 9,
      summonerName: 'Enemy4',
      championName: '娜美',
      position: '辅助',
      rank: '钻石 IV',
      winRate: 67
    },
    {
      summonerId: 10,
      summonerName: 'Enemy5',
      championName: '剑姬',
      position: '上单',
      rank: '钻石 I',
      winRate: 78
    }
  ],
  advantages: ['我方团战能力更强', '中期推塔节奏优势', '打野前期gank能力突出', '下路组合配合良好'],
  warnings: [
    '敌方剑姬单带威胁较大',
    '注意敌方卡莎后期输出',
    '小心盲僧前期入侵',
    '控制视野防止被开团'
  ]
})

const recentMatches = ref([
  {
    id: 1,
    champion: '尼德丽',
    result: 'win',
    mode: '排位赛',
    duration: '28分32秒',
    kda: '12/3/8',
    time: '2小时前'
  },
  {
    id: 2,
    champion: '阿狸',
    result: 'loss',
    mode: '排位赛',
    duration: '35分15秒',
    kda: '8/7/12',
    time: '4小时前'
  },
  {
    id: 3,
    champion: '盲僧',
    result: 'win',
    mode: '排位赛',
    duration: '22分48秒',
    kda: '6/2/15',
    time: '昨天'
  }
])

function analyzeCurrentMatch() {
  // TODO: 实现分析当前对局的逻辑
  console.log('分析当前对局')
}
</script>
