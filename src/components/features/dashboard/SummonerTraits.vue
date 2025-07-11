<template>
  <div class="space-y-4">
    <h4 class="font-semibold flex items-center">
      <UserCheck class="h-4 w-4 mr-2 text-purple-500" />
      召唤师特征
    </h4>

    <!-- 特征标签 -->
    <div class="flex flex-wrap gap-2">
      <template v-for="trait in traits" :key="trait.name">
        <Badge
          :variant="trait.variant"
          class="flex items-center gap-1 px-3 py-1 cursor-pointer transition-colors duration-200 border border-transparent group-hover:bg-primary/10 group-hover:text-primary"
          :class="{ 'ring-2 ring-primary ring-offset-2 border-primary/40': selectedTrait?.name === trait.name }"
          @click="selectTrait(trait)"
        >
          <component :is="trait.icon" class="h-3 w-3" />
          {{ trait.name }}
          <span class="text-xs opacity-75">({{ trait.score }})</span>
        </Badge>
      </template>
    </div>

    <!-- 特征详情 -->
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
              class="text-xs px-2 py-0.5 font-bold border-2"
              :class="selectedTrait.name === primaryTrait?.name ? 'border-primary/400' : 'border-gray-300'"
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

    <!-- 特征统计 -->
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
  Target,
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
}

interface Props {
  matchStatistics: {
    total_games: number
    wins: number
    losses: number
    win_rate: number
    avg_kills: number
    avg_deaths: number
    avg_assists: number
    avg_kda: number
    recent_performance: Array<{
      game_mode: string
      queue_id: number
      win: boolean
      kills: number
      deaths: number
      assists: number
      game_duration: number
    }>
  }
}

const props = defineProps<Props>()

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

  // 1. 胜率特征
  if (stats.win_rate >= 65) {
    traits.push({
      name: '大神',
      description: '胜率超高的实力玩家',
      detail: `胜率${stats.win_rate.toFixed(1)}%，展现了出色的游戏水平`,
      score: Math.round(stats.win_rate),
      variant: 'default',
      icon: Crown
    })
  } else if (stats.win_rate >= 55) {
    traits.push({
      name: '稳定',
      description: '胜率稳定的可靠队友',
      detail: `胜率${stats.win_rate.toFixed(1)}%，表现稳定可靠`,
      score: Math.round(stats.win_rate),
      variant: 'secondary',
      icon: Award // 更换为奖章，突出稳定荣誉
    })
  } else if (stats.win_rate <= 40) {
    traits.push({
      name: '坑货',
      description: '胜率偏低的玩家',
      detail: `胜率${stats.win_rate.toFixed(1)}%，需要提升游戏水平`,
      score: Math.round(stats.win_rate),
      variant: 'destructive',
      icon: Meh // 保持无语表情
    })
  }

  // 2. KDA特征
  if (stats.avg_kda >= 4.0) {
    traits.push({
      name: '大爹',
      description: 'KDA超高的carry玩家',
      detail: `平均KDA ${stats.avg_kda.toFixed(2)}，经常carry全场`,
      score: Math.round(stats.avg_kda * 10),
      variant: 'default',
      icon: Flame // 用火焰突出carry
    })
  } else if (stats.avg_kda >= 2.5) {
    traits.push({
      name: '输出',
      description: '输出能力强的玩家',
      detail: `平均KDA ${stats.avg_kda.toFixed(2)}，输出表现不错`,
      score: Math.round(stats.avg_kda * 10),
      variant: 'secondary',
      icon: Zap // 用闪电突出输出
    })
  } else if (stats.avg_deaths > stats.avg_kills * 2) {
    traits.push({
      name: '送分',
      description: '死亡次数过多的玩家',
      detail: `平均死亡${stats.avg_deaths.toFixed(1)}次，需要减少失误`,
      score: Math.round(stats.avg_deaths),
      variant: 'destructive',
      icon: AlertTriangle
    })
  }

  // 3. 击杀特征
  if (stats.avg_kills >= 8) {
    traits.push({
      name: '人头狗',
      description: '击杀能力超强的玩家',
      detail: `平均击杀${stats.avg_kills.toFixed(1)}个，收割能力极强`,
      score: Math.round(stats.avg_kills),
      variant: 'default',
      icon: Swords
    })
  } else if (stats.avg_kills >= 5) {
    traits.push({
      name: '输出',
      description: '输出能力不错的玩家',
      detail: `平均击杀${stats.avg_kills.toFixed(1)}个，输出表现良好`,
      score: Math.round(stats.avg_kills),
      variant: 'secondary',
      icon: Zap // 统一输出类用闪电
    })
  }

  // 4. 助攻特征
  if (stats.avg_assists >= 10) {
    traits.push({
      name: '辅助王',
      description: '助攻能力超强的团队玩家',
      detail: `平均助攻${stats.avg_assists.toFixed(1)}次，团队贡献突出`,
      score: Math.round(stats.avg_assists),
      variant: 'secondary',
      icon: Heart
    })
  } else if (stats.avg_assists >= 7) {
    traits.push({
      name: '团队',
      description: '团队协作能力强的玩家',
      detail: `平均助攻${stats.avg_assists.toFixed(1)}次，团队意识不错`,
      score: Math.round(stats.avg_assists),
      variant: 'outline',
      icon: Users
    })
  }

  // 5. 游戏模式特征
  const totalRecentGames = stats.recent_performance.length
  if (totalRecentGames < 5) {
    traits.push({
      name: '数据不足',
      description: '最近对局场次过少',
      detail: '最近对局不足5场，特征仅供参考',
      score: 0,
      variant: 'outline',
      icon: AlertTriangle
    })
  } else {
    const soloGames = stats.recent_performance.filter((g) => g.queue_id === 420).length
    const flexGames = stats.recent_performance.filter((g) => g.queue_id === 440).length
    const aramGames = stats.recent_performance.filter((g) => g.queue_id === 450).length
    const cherryGames = stats.recent_performance.filter((g) => g.queue_id === 1700).length
    const urfGames = stats.recent_performance.filter((g) => g.queue_id === 900).length
    const tftGames = stats.recent_performance.filter((g) =>
      [1090, 1100, 1130, 1150, 1160, 1170, 1180, 1190].includes(g.queue_id)
    ).length
    const cloneGames = stats.recent_performance.filter((g) => g.queue_id === 700).length

    const modeArr = [
      {
        name: '单双选手',
        count: soloGames,
        min: 10,
        icon: Swords,
        desc: '偏爱单双排的玩家',
        detail: `最近${soloGames}场单双排，专注于竞技排位`
      },
      {
        name: '灵活选手',
        count: flexGames,
        min: 10,
        icon: Users,
        desc: '偏爱灵活组排的玩家',
        detail: `最近${flexGames}场灵活组排，团队协作能力强`
      },
      {
        name: '大乱斗达人',
        count: aramGames,
        min: 7,
        icon: Gamepad2,
        desc: '热爱大乱斗的玩家',
        detail: `最近${aramGames}场大乱斗`
      },
      {
        name: '斗魂选手',
        count: cherryGames,
        min: 7,
        icon: Flame,
        desc: '热爱斗魂竞技场的玩家',
        detail: `最近${cherryGames}场斗魂竞技场`
      },
      {
        name: '火力达人',
        count: urfGames,
        min: 5,
        icon: Zap,
        desc: '热爱无限火力的玩家',
        detail: `最近${urfGames}场无限火力`
      },
      {
        name: '云顶玩家',
        count: tftGames,
        min: 5,
        icon: Trophy,
        desc: '热爱云顶之弈的玩家',
        detail: `最近${tftGames}场云顶之弈`
      },
      {
        name: '克隆达人',
        count: cloneGames,
        min: 3,
        icon: Star,
        desc: '喜欢克隆模式的玩家',
        detail: `最近${cloneGames}场克隆模式`
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
        icon: mainMode.icon
      })
    }
  }

  // 6. 游戏时长特征
  const avgGameDuration =
    stats.recent_performance.reduce((sum, game) => sum + game.game_duration, 0) / stats.recent_performance.length
  if (avgGameDuration >= 1800) {
    traits.push({
      name: '持久战',
      description: '喜欢长时间对局的玩家',
      detail: `平均游戏时长${Math.round(avgGameDuration / 60)}分钟，擅长后期运营`,
      score: Math.round(avgGameDuration / 60),
      variant: 'outline',
      icon: Clock
    })
  } else if (avgGameDuration <= 1200) {
    traits.push({
      name: '快节奏',
      description: '喜欢快速结束游戏的玩家',
      detail: `平均游戏时长${Math.round(avgGameDuration / 60)}分钟，追求快速胜利`,
      score: Math.round(avgGameDuration / 60),
      variant: 'outline',
      icon: Zap
    })
  }

  // 7. 稳定性特征
  const winStreak = calculateWinStreak(stats.recent_performance)
  if (winStreak >= 5) {
    traits.push({
      name: '连胜王',
      description: '近期状态火热的玩家',
      detail: `最近${winStreak}连胜，状态极佳`,
      score: winStreak,
      variant: 'default',
      icon: TrendingUp // 用趋势上升图标
    })
  } else if (winStreak <= -3) {
    traits.push({
      name: '连败',
      description: '近期状态不佳的玩家',
      detail: `最近${Math.abs(winStreak)}连败，需要调整状态`,
      score: Math.abs(winStreak),
      variant: 'destructive',
      icon: Coffee // 咖啡表示需要休息调整
    })
  }

  // 9. 视野控制特征
  if (stats.avg_assists >= 8 && stats.avg_kills <= 3) {
    traits.push({
      name: '视野王',
      description: '视野控制能力强的玩家',
      detail: `助攻多击杀少，专注于视野控制和团队支援`,
      score: Math.round(stats.avg_assists),
      variant: 'secondary',
      icon: Eye
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
      icon: Crown // 用皇冠突出全能
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
  const winRateScore = Math.min(stats.win_rate * 0.6, 60) // 胜率权重60%
  const kdaScore = Math.min(stats.avg_kda * 5, 20) // KDA权重20%
  const gamesScore = Math.min(stats.total_games * 0.2, 20) // 游戏场次权重20%

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
