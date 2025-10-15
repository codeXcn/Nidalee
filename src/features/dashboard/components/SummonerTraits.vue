<template>
  <div class="space-y-4">
    <h4 class="font-semibold flex items-center">
      <UserCheck class="h-4 w-4 mr-2 text-purple-500" />
      召唤师特征
    </h4>

    <div class="flex flex-wrap gap-1">
      <template v-for="trait in traits" :key="trait.name">
        <button
          class="flex items-center gap-1 px-2 h-8 rounded-full border text-xs font-medium transition-all duration-150 focus:outline-none cursor-pointer select-none min-w-[96px] max-w-[30%] mb-[2px] whitespace-nowrap"
          :class="[
            selectedTrait?.name === trait.name
              ? trait.type === 'bad'
                ? 'border-red-500 bg-red-50 text-red-600 ring-1 ring-red-400'
                : 'border-primary bg-primary/10 text-primary ring-1 ring-primary'
              : trait.type === 'bad'
                ? 'border-red-200 bg-red-50 text-red-500 hover:border-red-400 hover:text-red-600'
                : 'border-border bg-muted text-muted-foreground hover:border-primary/40 hover:text-primary'
          ]"
          @click="selectTrait(trait)"
        >
          <component v-if="trait.icon" :is="trait.icon" class="h-4 w-4" />
          <span v-else class="h-4 w-4 inline-block"></span>
          <span>{{ trait.name }}</span>
          <span
            class="ml-1 flex items-center justify-center w-6 h-6 rounded-full text-[10px] font-bold transition-colors"
            :class="
              selectedTrait?.name === trait.name
                ? trait.type === 'bad'
                  ? 'bg-red-500/95 text-white/95 dark:bg-red-500/85 dark:text-white/85'
                  : 'bg-primary/90 text-primary-foreground/95 dark:bg-primary/70 dark:text-primary-foreground/85'
                : trait.type === 'bad'
                  ? 'bg-red-100 text-red-600 dark:bg-red-950 dark:text-red-300'
                  : 'bg-muted text-muted-foreground/90 dark:bg-muted dark:text-muted-foreground/80'
            "
          >
            {{ trait.score }}
          </span>
        </button>
      </template>
    </div>

    <div v-if="selectedTrait" class="bg-card border rounded-lg p-4 transition-all duration-200 hover:shadow-sm">
      <div class="flex items-start gap-3">
        <div class="p-2 bg-muted rounded-lg">
          <component :is="selectedTrait.icon" class="h-5 w-5" />
        </div>
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-1">
            <h5 class="font-semibold">
              {{ selectedTrait.name }}
            </h5>
            <Badge
              :variant="selectedTrait.name === primaryTrait?.name ? 'default' : 'outline'"
              class="text-xs px-2 py-0.5 font-medium border"
              :class="
                selectedTrait.name === primaryTrait?.name
                  ? 'bg-primary/85 dark:bg-primary/70 text-primary-foreground/95 dark:text-primary-foreground/85 border-primary/30 dark:border-primary/25'
                  : 'bg-primary/10 dark:bg-primary/15 text-primary/90 dark:text-primary/85 border-border dark:border-border/60'
              "
            >
              {{ selectedTrait.name === primaryTrait?.name ? '主要特征' : '特征详情' }}
            </Badge>
          </div>
          <p class="text-sm text-muted-foreground mb-2">
            {{ selectedTrait.description }}
          </p>
          <p class="text-xs text-muted-foreground">
            {{ selectedTrait.detail }}
          </p>
        </div>
      </div>
    </div>

    <div v-if="traits.length > 1" class="text-xs text-muted-foreground text-center">
      共识别出 {{ traits.length }} 个特征，点击标签查看详情
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  AlertTriangle,
  Award,
  Clock,
  Coffee,
  Crown,
  Eye,
  Flame,
  Gamepad2,
  Heart,
  Meh,
  Star,
  Swords,
  TrendingUp,
  Trophy,
  UserCheck,
  Users,
  Zap
} from 'lucide-vue-next'

interface Trait {
  name: string
  description: string
  detail: string
  score: number
  variant: 'default' | 'secondary' | 'destructive' | 'outline'
  icon: any
  type: 'good' | 'bad' // Added type field
}

const props = defineProps<{
  matchStatistics: MatchStatistics | null
}>()

// 选中的特征
const selectedTrait = ref<Trait | null>(null)

// 选择特征
const selectTrait = (trait: Trait) => {
  selectedTrait.value = trait
}

// 分析召唤师特征
const analyzeTraits = (): Trait[] => {
  const traits: Trait[] = []
  const stats = props.matchStatistics

  // 防御性检查：如果统计数据或最近表现不存在，则不进行分析
  if (!stats || !stats.recentPerformance) {
    return []
  }

  // 1. 胜率特征
  if (stats.winRate >= 65) {
    traits.push({
      name: '大神',
      description: '胜率超高的实力玩家',
      detail: `胜率${stats.winRate.toFixed(1)}%，展现了出色的游戏水平`,
      score: Math.round(stats.winRate),
      variant: 'default',
      icon: Crown,
      type: 'good' // Added type
    })
  } else if (stats.winRate >= 55) {
    traits.push({
      name: '稳定',
      description: '胜率稳定的可靠队友',
      detail: `胜率${stats.winRate.toFixed(1)}%，表现稳定可靠`,
      score: Math.round(stats.winRate),
      variant: 'secondary',
      icon: Award, // 更换为奖章，突出稳定荣誉
      type: 'good' // Added type
    })
  } else if (stats.winRate <= 40) {
    traits.push({
      name: '坑货',
      description: '胜率偏低的玩家',
      detail: `胜率${stats.winRate.toFixed(1)}%，需要提升游戏水平`,
      score: Math.round(stats.winRate),
      variant: 'destructive',
      icon: Meh, // 保持无语表情
      type: 'bad' // Added type
    })
  }

  // 2. KDA特征
  if (stats.avgKda >= 4.0) {
    traits.push({
      name: '大爹',
      description: 'KDA超高的carry玩家',
      detail: `平均KDA ${stats.avgKda.toFixed(2)}，经常carry全场`,
      score: Math.round(stats.avgKda * 10),
      variant: 'default',
      icon: Flame, // 用火焰突出carry
      type: 'good' // Added type
    })
  } else if (stats.avgKda >= 2.5) {
    traits.push({
      name: '输出',
      description: '输出能力强的玩家',
      detail: `平均KDA ${stats.avgKda.toFixed(2)}，输出表现不错`,
      score: Math.round(stats.avgKda * 10),
      variant: 'secondary',
      icon: Zap, // 用闪电突出输出
      type: 'good' // Added type
    })
  } else if (stats.avgDeaths > stats.avgKills * 2) {
    traits.push({
      name: '送分',
      description: '死亡次数过多的玩家',
      detail: `平均死亡${stats.avgDeaths.toFixed(1)}次，需要减少失误`,
      score: Math.round(stats.avgDeaths),
      variant: 'destructive',
      icon: AlertTriangle,
      type: 'bad' // Added type
    })
  }

  // 3. 击杀特征
  if (stats.avgKills >= 8) {
    traits.push({
      name: '人头狗',
      description: '击杀能力超强的玩家',
      detail: `平均击杀${stats.avgKills.toFixed(1)}个，收割能力极强`,
      score: Math.round(stats.avgKills),
      variant: 'default',
      icon: Swords,
      type: 'good' // Added type
    })
  } else if (stats.avgKills >= 5) {
    traits.push({
      name: '输出能力',
      description: '输出能力不错的玩家',
      detail: `平均击杀${stats.avgKills.toFixed(1)}个，输出表现良好`,
      score: Math.round(stats.avgKills),
      variant: 'secondary',
      icon: Zap, // 统一输出类用闪电
      type: 'good' // Added type
    })
  }

  // 4. 助攻特征
  if (stats.avgAssists >= 10) {
    traits.push({
      name: '辅助王',
      description: '助攻能力超强的团队玩家',
      detail: `平均助攻${stats.avgAssists.toFixed(1)}次，团队贡献突出`,
      score: Math.round(stats.avgAssists),
      variant: 'secondary',
      icon: Heart,
      type: 'good' // Added type
    })
  } else if (stats.avgAssists >= 7) {
    traits.push({
      name: '团队',
      description: '团队协作能力强的玩家',
      detail: `平均助攻${stats.avgAssists.toFixed(1)}次，团队意识不错`,
      score: Math.round(stats.avgAssists),
      variant: 'outline',
      icon: Users,
      type: 'good' // Added type
    })
  }

  // 5. 游戏模式特征
  const totalRecentGames = stats.recentPerformance.length
  if (totalRecentGames < 5) {
    traits.push({
      name: '数据不足',
      description: '最近对局场次过少',
      detail: '最近对局不足5场，特征仅供参考',
      score: 0,
      variant: 'outline',
      icon: AlertTriangle,
      type: 'bad' // Added type
    })
  } else {
    const soloGames = stats.recentPerformance.filter((g) => g.queueId === 420).length
    const flexGames = stats.recentPerformance.filter((g) => g.queueId === 440).length
    const aramGames = stats.recentPerformance.filter((g) => g.queueId === 450).length
    const cherryGames = stats.recentPerformance.filter((g) => g.queueId === 1700).length
    const urfGames = stats.recentPerformance.filter((g) => g.queueId === 900).length
    const tftGames = stats.recentPerformance.filter((g) =>
      [1090, 1100, 1130, 1150, 1160, 1170, 1180, 1190].includes(g.queueId)
    ).length
    const cloneGames = stats.recentPerformance.filter((g) => g.queueId === 700).length

    const modeArr = [
      {
        name: '单双选手',
        count: soloGames,
        min: 10,
        icon: Swords as any,
        desc: '偏爱单双排的玩家',
        detail: `最近${soloGames}场单双排，专注于竞技排位`,
        type: 'good'
      },
      {
        name: '灵活选手',
        count: flexGames,
        min: 10,
        icon: Users as any,
        desc: '偏爱灵活组排的玩家',
        detail: `最近${flexGames}场灵活组排，团队协作能力强`,
        type: 'good'
      },
      {
        name: '大乱斗达人',
        count: aramGames,
        min: 7,
        icon: Gamepad2 as any,
        desc: '热爱大乱斗的玩家',
        detail: `最近${aramGames}场大乱斗`,
        type: 'good'
      },
      {
        name: '斗魂选手',
        count: cherryGames,
        min: 7,
        icon: Flame as any,
        desc: '热爱斗魂竞技场的玩家',
        detail: `最近${cherryGames}场斗魂竞技场`,
        type: 'good'
      },
      {
        name: '火力达人',
        count: urfGames,
        min: 5,
        icon: Zap as any,
        desc: '热爱无限火力的玩家',
        detail: `最近${urfGames}场无限火力`,
        type: 'good'
      },
      {
        name: '云顶玩家',
        count: tftGames,
        min: 5,
        icon: Trophy as any,
        desc: '热爱云顶之弈的玩家',
        detail: `最近${tftGames}场云顶之弈`,
        type: 'good'
      },
      {
        name: '克隆达人',
        count: cloneGames,
        min: 3,
        icon: Star as any,
        desc: '喜欢克隆模式的玩家',
        detail: `最近${cloneGames}场克隆模式`,
        type: 'good'
      }
    ]
    const mainMode = modeArr.filter((m) => m.count >= m.min).sort((a, b) => b.count - a.count)[0]
    if (mainMode) {
      traits.push({
        name: mainMode.name,
        description: mainMode.desc,
        detail: mainMode.detail,
        score: mainMode.count,
        variant: 'outline',
        icon: mainMode.icon,
        type: mainMode.type as 'good' | 'bad'
      })
    }
  }

  // 6. 游戏时长特征
  const avgGameDuration =
    stats.recentPerformance.reduce((sum, game) => sum + game.gameDuration, 0) / stats.recentPerformance.length
  if (avgGameDuration >= 1800) {
    traits.push({
      name: '持久战',
      description: '喜欢长时间对局的玩家',
      detail: `平均游戏时长${Math.round(avgGameDuration / 60)}分钟，擅长后期运营`,
      score: Math.round(avgGameDuration / 60),
      variant: 'outline',
      icon: Clock,
      type: 'good' // Added type
    })
  } else if (avgGameDuration <= 1200) {
    traits.push({
      name: '快节奏',
      description: '喜欢快速结束游戏的玩家',
      detail: `平均游戏时长${Math.round(avgGameDuration / 60)}分钟，追求快速胜利`,
      score: Math.round(avgGameDuration / 60),
      variant: 'outline',
      icon: Zap,
      type: 'bad' // Added type
    })
  }

  // 7. 稳定性特征
  const winStreak = calculateWinStreak(stats.recentPerformance)
  if (winStreak >= 5) {
    traits.push({
      name: '连胜王',
      description: '近期状态火热的玩家',
      detail: `最近${winStreak}连胜，状态极佳`,
      score: winStreak,
      variant: 'default',
      icon: TrendingUp, // 用趋势上升图标
      type: 'good' // Added type
    })
  } else if (winStreak <= -3) {
    traits.push({
      name: '连败',
      description: '近期状态不佳的玩家',
      detail: `最近${Math.abs(winStreak)}连败，需要调整状态`,
      score: Math.abs(winStreak),
      variant: 'destructive',
      icon: Coffee, // 咖啡表示需要休息调整
      type: 'bad' // Added type
    })
  }

  // 9. 视野控制特征
  if (stats.avgAssists >= 8 && stats.avgKills <= 3) {
    traits.push({
      name: '视野王',
      description: '视野控制能力强的玩家',
      detail: `助攻多击杀少，专注于视野控制和团队支援`,
      score: Math.round(stats.avgAssists),
      variant: 'secondary',
      icon: Eye,
      type: 'good' // Added type
    })
  }

  // 10. 综合评分
  const overallScore = calculateOverallScore(stats)
  if (overallScore >= 80) {
    traits.push({
      name: '全能王',
      description: '综合能力极强的玩家',
      detail: `综合评分${overallScore}分，各项能力都很出色`,
      score: overallScore,
      variant: 'default',
      icon: Crown, // 用皇冠突出全能
      type: 'good' // Added type
    })
  }

  return traits.sort((a, b) => b.score - a.score)
}

// 计算连胜/连败
const calculateWinStreak = (recentGames: Array<{ win: boolean }>): number => {
  let streak = 0
  for (const game of recentGames) {
    if (game.win) {
      if (streak >= 0) streak++
      else break
    } else {
      if (streak <= 0) streak--
      else break
    }
  }
  return streak
}

// 计算综合评分
const calculateOverallScore = (stats: any): number => {
  const winRateScore = Math.min(stats.winRate * 0.6, 60) // 胜率权重60%
  const kdaScore = Math.min(stats.avgKda * 5, 20) // KDA权重20%
  const gamesScore = Math.min(stats.totalGames * 0.2, 20) // 游戏场次权重20%

  return Math.round(winRateScore + kdaScore + gamesScore)
}

// 计算特征
const traits = computed(() => analyzeTraits())

// 主要特征（得分最高的）
const primaryTrait = computed(() => traits.value[0])

// 初始化选中主要特征
watch(
  traits,
  (newTraits) => {
    if (newTraits.length > 0 && !selectedTrait.value) {
      selectedTrait.value = newTraits[0]
    }
  },
  { immediate: true }
)
</script>
