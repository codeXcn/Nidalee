import { ref, onMounted, onUnmounted } from 'vue'
import { ChampSelectSession } from '@/components/analysis/types/champSelect'

export function useChampSelectSession() {
  const session = ref<ChampSelectSession | null>(null)

  function handleSessionChange(e: any) {
    session.value = e.detail as ChampSelectSession
  }

  onMounted(() => {
    window.addEventListener('champ-select-session-changed', handleSessionChange as any)
  })
  onUnmounted(() => {
    window.removeEventListener('champ-select-session-changed', handleSessionChange as any)
  })

  return { session }
}
