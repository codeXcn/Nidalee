<template>
  <div class="space-y-4">
    <h4 class="font-semibold flex items-center">
      <UserCheck class="h-4 w-4 mr-2 text-purple-500" />
      召唤师特征
    </h4>

    <!-- 特征标签 -->
    <div class="flex flex-wrap gap-2">
      <Tooltip v-for="trait in traits" :key="trait.name">
        <TooltipTrigger as-child>
          <Badge
            :variant="trait.variant"
            class="flex items-center gap-1 px-3 py-1 cursor-help transition-all duration-200 hover:scale-105 group"
          >
            <component :is="trait.icon" class="h-3 w-3" />
            {{ trait.name }}
            <span class="text-xs opacity-75">({{ trait.score }})</span>
          </Badge>
        </TooltipTrigger>
        <TooltipContent side="top" class="max-w-sm">
          <div class="space-y-2">
            <div class="flex items-center gap-2">
              <component :is="trait.icon" class="h-4 w-4" />
              <span class="font-medium">{{ trait.name }}</span>
            </div>
            <p class="text-sm">{{ trait.description }}</p>
            <p class="text-xs text-muted-foreground">{{ trait.detail }}</p>
          </div>
        </TooltipContent>
      </Tooltip>
    </div>

    <!-- 特征详情 -->
    <div v-if="primaryTrait" class="bg-card border rounded-lg p-4 transition-all duration-200 hover:shadow-sm">
      <div class="flex items-start gap-3">
        <div class="p-2 bg-muted rounded-lg">
          <component :is="primaryTrait.icon" class="h-5 w-5" />
        </div>
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-1">
            <h5 class="font-semibold">
              {{ primaryTrait.name }}
            </h5>
            <Badge variant="outline" class="text-xs"> 主要特征 </Badge>
          </div>
          <p class="text-sm text-muted-foreground mb-2">
            {{ primaryTrait.description }}
          </p>
          <p class="text-xs text-muted-foreground">
            {{ primaryTrait.detail }}
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
import { Badge } from '@/components/ui/badge'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'
import {
  AlertTriangle,
  Award,
  Brain,
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
      win: boolean
      kills: number
      deaths: number
      assists: number
      game_duration: number
    }>
  }
}

const props = defineProps<Props>()

// 点击状态管理
const clickedTrait = ref<string | null>(null)

// 处理标签点击
const handleTraitClick = (traitName: string) => {
  clickedTrait.value = traitName
  // 重置点击状态
  setTimeout(() => {
    clickedTrait.value = null
  }, 300)
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
      icon: TrendingUp
    })
  } else if (stats.win_rate <= 40) {
    traits.push({
      name: '坑货',
      description: '胜率偏低的玩家',
      detail: `胜率${stats.win_rate.toFixed(1)}%，需要提升游戏水平`,
      score: Math.round(stats.win_rate),
      variant: 'destructive',
      icon: Meh
    })
  }

  // 2. KDA特征
  if (stats.avg_kda >= 4.0) {
    traits.push({
      name: '野爹',
      description: 'KDA超高的carry玩家',
      detail: `平均KDA ${stats.avg_kda.toFixed(2)}，经常carry全场`,
      score: Math.round(stats.avg_kda * 10),
      variant: 'default',
      icon: Trophy
    })
  } else if (stats.avg_kda >= 2.5) {
    traits.push({
      name: '输出',
      description: '输出能力强的玩家',
      detail: `平均KDA ${stats.avg_kda.toFixed(2)}，输出表现不错`,
      score: Math.round(stats.avg_kda * 10),
      variant: 'secondary',
      icon: Target
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
      icon: Target
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
  const aramGames = stats.recent_performance.filter(
    (game) => game.game_mode.includes('ARAM') || game.game_mode.includes('大乱斗')
  ).length
  const totalRecentGames = stats.recent_performance.length

  if (totalRecentGames > 0 && aramGames / totalRecentGames >= 0.7) {
    traits.push({
      name: '乱斗选手',
      description: '热爱大乱斗模式的玩家',
      detail: `最近${totalRecentGames}场中有${aramGames}场大乱斗，占比${((aramGames / totalRecentGames) * 100).toFixed(0)}%`,
      score: Math.round((aramGames / totalRecentGames) * 100),
      variant: 'outline',
      icon: Gamepad2
    })
  }

  // 6. 游戏时长特征
  const avgGameDuration =
    stats.recent_performance.reduce((sum, game) => sum + game.game_duration, 0) / stats.recent_performance.length
  if (avgGameDuration >= 1800) {
    // 30分钟以上
    traits.push({
      name: '持久战',
      description: '喜欢长时间对局的玩家',
      detail: `平均游戏时长${Math.round(avgGameDuration / 60)}分钟，擅长后期运营`,
      score: Math.round(avgGameDuration / 60),
      variant: 'outline',
      icon: Clock
    })
  } else if (avgGameDuration <= 1200) {
    // 20分钟以下
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
      icon: Flame
    })
  } else if (winStreak <= -3) {
    traits.push({
      name: '连败',
      description: '近期状态不佳的玩家',
      detail: `最近${Math.abs(winStreak)}连败，需要调整状态`,
      score: Math.abs(winStreak),
      variant: 'destructive',
      icon: Coffee
    })
  }

  // 8. 游戏场次特征
  if (stats.total_games >= 100) {
    traits.push({
      name: '老玩家',
      description: '游戏经验丰富的玩家',
      detail: `总共${stats.total_games}场游戏，经验丰富`,
      score: Math.min(stats.total_games, 200),
      variant: 'outline',
      icon: Star
    })
  } else if (stats.total_games <= 20) {
    traits.push({
      name: '新手',
      description: '游戏场次较少的玩家',
      detail: `总共${stats.total_games}场游戏，还在学习阶段`,
      score: stats.total_games,
      variant: 'outline',
      icon: Brain
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
      icon: Award
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
</script>
