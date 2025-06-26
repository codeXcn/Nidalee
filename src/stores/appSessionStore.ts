export const useAppSessionStore = defineStore(
  'appSession',
  () => {
    // 游戏版本
    const gameVersion = ref('15.12.1')

    // 会话信息
    const session = ref<GameSession>({
      startTime: Date.now(),
      duration: 0
    })

    // 更新会话时长
    const updateSessionDuration = () => {
      const currentTime = Date.now()
      const startTime = session.value.startTime || currentTime
      session.value.duration = currentTime - startTime
    }

    // 重置会话
    const resetSession = () => {
      const currentTime = Date.now()
      session.value = {
        startTime: currentTime,
        duration: 0
      }
    }

    // 设置游戏版本
    const setGameVersion = (version: string) => {
      gameVersion.value = version
    }

    // 计算属性
    const sessionDuration = computed(() => {
      const duration = session.value.duration || 0
      const minutes = Math.floor(duration / 60000)
      const hours = Math.floor(minutes / 60)

      if (hours > 0) {
        return `${hours}h ${minutes % 60}m`
      }
      return `${minutes}m`
    })

    const sessionDurationInMinutes = computed(() => {
      return Math.floor((session.value.duration || 0) / 60000)
    })

    const sessionDurationInHours = computed(() => {
      return Math.floor((session.value.duration || 0) / 3600000)
    })

    return {
      // 状态
      gameVersion: readonly(gameVersion),
      session: readonly(session),

      // 计算属性
      sessionDuration,
      sessionDurationInMinutes,
      sessionDurationInHours,

      // 方法
      updateSessionDuration,
      resetSession,
      setGameVersion
    }
  },
  {
    persist: {
      paths: ['gameVersion']
    }
  }
)
