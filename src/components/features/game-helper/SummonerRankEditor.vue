<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <Users class="h-5 w-5" />
        段位设置
        <span class="text-muted-foreground">自定义你的段位信息</span>
      </CardTitle>
    </CardHeader>
    <CardContent>
      <div class="flex flex-col md:flex-row items-center gap-4">
        <Select v-model="rankQueue">
          <SelectTrigger class="h-12 px-4 rounded border border-border bg-background/50 min-w-[120px]">
            <span>{{ rankQueues.find((q) => q.value === rankQueue)?.label }}</span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="q in rankQueues" :key="q.value" :value="q.value">
              {{ q.label }}
            </SelectItem>
          </SelectContent>
        </Select>
        <Select v-model="rankTier">
          <SelectTrigger
            class="h-12 px-4 rounded border border-border bg-background/50 min-w-[160px] flex items-center"
          >
            <img
              v-if="getTierIconUrl(rankTier)"
              :src="getTierIconUrl(rankTier)"
              class="w-6 h-6 mr-2 inline-block align-middle"
            />
            <span>{{ tierLabelMap[rankTier] || rankTier }}</span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="t in rankTiers" :key="t" :value="t">
              <div class="flex items-center">
                <img v-if="getTierIconUrl(t)" :src="getTierIconUrl(t)" class="w-6 h-6 mr-2" />
                <span>{{ tierLabelMap[t] || t }}</span>
              </div>
            </SelectItem>
          </SelectContent>
        </Select>
        <Select v-model="rankDivision" :disabled="noDivisionTiers.includes(rankTier)">
          <SelectTrigger class="h-12 px-4 rounded border border-border bg-background/50 min-w-[80px]">
            <span>{{ rankDivision }}</span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="d in rankDivisions" :key="d" :value="d">
              {{ d }}
            </SelectItem>
          </SelectContent>
        </Select>
        <button
          @click="handleSave"
          :disabled="updatingRank"
          class="px-6 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-all duration-200 text-base font-medium shadow-sm hover:shadow-md disabled:opacity-60 disabled:cursor-not-allowed"
        >
          {{ updatingRank ? '保存中...' : '保存段位' }}
        </button>
      </div>
      <div v-if="noDivisionTiers.includes(rankTier)" class="text-xs text-muted-foreground mt-2">
        大师、宗师、王者无小段位，自动设为 I
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Users } from 'lucide-vue-next'
import { useGameHelper } from '@/composables/game-helper'
import { getTierIconUrl } from '@/lib'

const rankQueues = [
  { label: '单双排', value: 'RANKED_SOLO_5x5' },
  { label: '灵活组排', value: 'RANKED_FLEX_SR' }
]
const rankTiers = [
  'IRON',
  'BRONZE',
  'SILVER',
  'GOLD',
  'PLATINUM',
  'EMERALD',
  'DIAMOND',
  'MASTER',
  'GRANDMASTER',
  'CHALLENGER'
]
const rankDivisions = ['I', 'II', 'III', 'IV']
const noDivisionTiers = ['MASTER', 'GRANDMASTER', 'CHALLENGER']
const tierLabelMap: Record<string, string> = {
  IRON: '坚韧黑铁',
  BRONZE: '英勇青铜',
  SILVER: '不屈白银',
  GOLD: '荣耀黄金',
  PLATINUM: '华贵铂金',
  EMERALD: '流光翡翠',
  DIAMOND: '璀璨钻石',
  MASTER: '超凡大师',
  GRANDMASTER: '傲世宗师',
  CHALLENGER: '最强王者'
}

const rankQueue = ref('RANKED_SOLO_5x5')
const rankTier = ref('GOLD')
const rankDivision = ref('IV')
const { setSummonerChatProfile, updatingRank } = useGameHelper()

watch(rankTier, (newTier) => {
  if (noDivisionTiers.includes(newTier)) {
    rankDivision.value = 'I'
  }
})

const handleSave = async () => {
  await setSummonerChatProfile({
    queue: rankQueue.value,
    tier: rankTier.value,
    division: rankDivision.value
  })
}
</script>
