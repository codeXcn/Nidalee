import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useSessionStore = defineStore(
  'session',
  () => {
    // 总累计
    const totalActiveSeconds = ref(0)
    // 计时状态
    let _timer: ReturnType<typeof setInterval> | null = null

    function startSession() {
      if (_timer) return
      _timer = setInterval(() => {
        addActiveSeconds(1)
      }, 1000)
    }
    function stopSession() {
      if (_timer) {
        clearInterval(_timer)
        _timer = null
      }
    }
    function addActiveSeconds(sec: number) {
      totalActiveSeconds.value += sec
    }
    function formatSeconds(sec: number) {
      if (sec < 60) return `${sec}秒`
      if (sec < 3600) return `${Math.floor(sec / 60)}分${sec % 60}秒`
      return `${Math.floor(sec / 3600)}小时${Math.floor((sec % 3600) / 60)}分`
    }
    const formattedTotal = computed(() => formatSeconds(totalActiveSeconds.value))

    return {
      totalActiveSeconds,
      formattedTotal,
      startSession,
      stopSession
    }
  },
  { persist: true }
)
