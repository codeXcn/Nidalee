import { computed } from 'vue'

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
      NEXUSBLITZ: '极地大乱斗'
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
