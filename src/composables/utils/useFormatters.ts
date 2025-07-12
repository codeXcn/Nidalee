export function useFormatters() {
  const formatRelativeTime = (timestamp: number) => {
    const date = new Date(timestamp)
    const now = new Date()
    const diff = now.getTime() - date.getTime()
    const hours = Math.floor(diff / (1000 * 60 * 60))

    if (hours < 1) {
      return '刚刚'
    } else if (hours < 24) {
      return `${hours}小时前`
    } else {
      const days = Math.floor(hours / 24)
      return `${days}天前`
    }
  }

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit'
    })
  }

  const formatGameTime = (seconds: number): string => {
    const minutes = Math.floor(seconds / 60)
    const remainingSeconds = seconds % 60
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`
  }

  const formatGameMode = (mode: string): string => {
    const modeMap: Record<string, string> = {
      CLASSIC: '经典模式',
      ARAM: '大乱斗',
      URF: '无限火力',
      TUTORIAL: '教程',
      ONEFORALL: '克隆大作战',
      ARSR: '极地大乱斗',
      PRACTICETOOL: '训练工具',
      NEXUSBLITZ: '极地突围',
      ODIN: '统治战场',
      FIRSTBLOOD: '血月杀',
      ASCENSION: '飞升模式',
      HEXAKILL: '六杀争夺战',
      KINGPORO: '魄罗王',
      SIEGE: '守卫雅典娜',
      ASSASSINATE: '暗杀模式',
      DARKSTAR: '暗星乱斗',
      STARGUARDIAN: '星之守护者',
      PROJECT: '源计划：超越',
      GAMEMODEX: '极限闪击',
      CHERRY: '斗魂竞技场',
      ULTBOOK: '终极魔典',
      ARENA: '斗魂竞技场',
      BLITZ: '极地突围',
      // 其他常见自定义/活动模式
      SNOWURF: '雪地无限火力',
      ALLRANDOM: '全随机',
      // 兼容部分服务端返回
      CLASSIC_5V5: '召唤师峡谷',
      ARAM_5V5: '嚎哭深渊',
      TUTORIAL_MODULE_1: '教程1',
      TUTORIAL_MODULE_2: '教程2',
      TUTORIAL_MODULE_3: '教程3'
    }
    return modeMap[mode] || mode
  }

  const formatDuration = (seconds: number) => {
    const minutes = Math.floor(seconds / 60)
    const remainingSeconds = seconds % 60
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`
  }

  const formatNumber = (num: number): string => {
    return num?.toLocaleString() || '0'
  }

  const formatChallengePoints = (points: string): string => {
    const num = parseInt(points)
    if (num >= 1000) {
      return `${(num / 1000).toFixed(1)}k`
    }
    return points
  }

  return {
    formatRelativeTime,
    formatTime,
    formatGameTime,
    formatGameMode,
    formatDuration,
    formatNumber,
    formatChallengePoints
  }
}
