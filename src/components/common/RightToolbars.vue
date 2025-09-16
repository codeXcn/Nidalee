<template>
  <div class="px-4">
    <div class="flex items-center space-x-2">
      <NotificationHoverCard title="系统活动" side="bottom" align="end" />

      <button
        class="cursor-pointer p-3 rounded-xl bg-gradient-to-br from-background/80 to-muted/60 backdrop-blur-sm border border-border/50 hover:border-border transition-all duration-200 focus:outline-none shadow-lg hover:shadow-xl group"
        @click="refreshData"
        title="刷新数据"
      >
        <RefreshCw :size="17" class="text-muted-foreground group-hover:text-foreground transition-colors" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSummonerAndMatchUpdater } from '@/composables/game/useSummonerAndMatchUpdater'
import { useActivityLogger } from '@/composables/utils/useActivityLogger'
import { useConnectionStore } from '@/stores/core/connectionStore'
import { RefreshCw } from 'lucide-vue-next'

const activityLogger = useActivityLogger()
const connectionStore = useConnectionStore()

const { isConnected } = storeToRefs(connectionStore)
const { updateSummonerAndMatches } = useSummonerAndMatchUpdater()

const refreshData = async () => {
  console.log('刷新数据')
  try {
    if (isConnected.value) {
      updateSummonerAndMatches()
    }
  } catch (error) {
    console.error('刷新数据失败:', error)
    activityLogger.logError.apiError('数据刷新失败')
  }
}
</script>

<style></style>
