<template>
  <Card class="w-full max-w-md mx-auto p-6">
    <form @submit.prevent="onSearch" class="flex items-center gap-2">
      <Input
        v-model="summonerName"
        placeholder="输入召唤师名称"
        class="flex-1"
        @keyup.enter="onSearch"
      />
      <Button type="submit" :disabled="loading || !summonerName.trim()" class="shrink-0">
        <Search class="w-4 h-4 mr-1" />
        查询
      </Button>
    </form>
    <div v-if="error" class="mt-4 text-destructive text-sm">{{ error }}</div>
    <div v-if="loading" class="mt-4 text-muted-foreground text-sm">正在查询...</div>
    <div v-if="result" class="mt-6">
      <h4 class="font-bold text-primary mb-2">召唤师信息</h4>
      <div class="flex items-center gap-3">
        <img :src="result.avatar" class="w-12 h-12 rounded-full border" alt="avatar" />
        <div>
          <div class="font-semibold">{{ result.name }}</div>
          <div class="text-xs text-muted-foreground">ID: {{ result.id }}</div>
        </div>
      </div>
      <div class="mt-4">
        <h5 class="font-semibold mb-1">最近战绩</h5>
        <ul class="space-y-1 text-sm">
          <li v-for="(match, idx) in result.matches" :key="idx" class="flex items-center gap-2">
            <span class="font-medium">{{ match.champion }}</span>
            <span class="text-xs text-muted-foreground">KDA: {{ match.kda }}</span>
            <span
              :class="match.win ? 'text-success' : 'text-destructive'"
              class="text-xs"
            >{{ match.win ? '胜利' : '失败' }}</span>
          </li>
        </ul>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Card } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Search } from 'lucide-vue-next'

// 组件内部状态
const summonerName = ref('')
const loading = ref(false)
const error = ref('')
const result = ref<null | {
  name: string
  id: string
  avatar: string
  matches: Array<{ champion: string; kda: string; win: boolean }>
}>(null)

// 查询逻辑（请替换为实际API调用）
async function fetchSummonerInfo(name: string) {
  // 这里用模拟数据，实际请替换为后端接口
  await new Promise(r => setTimeout(r, 1000))
  if (name === 'notfound') throw new Error('未找到该召唤师')
  return {
    name,
    id: '123456789',
    avatar: 'https://game.gtimg.cn/images/lol/act/img/champion/Garen.png',
    matches: [
      { champion: '盖伦', kda: '10/2/5', win: true },
      { champion: '拉克丝', kda: '2/8/7', win: false },
      { champion: '伊泽瑞尔', kda: '7/1/9', win: true }
    ]
  }
}

const onSearch = async () => {
  error.value = ''
  result.value = null
  if (!summonerName.value.trim()) return
  loading.value = true
  try {
    result.value = await fetchSummonerInfo(summonerName.value.trim())
  } catch (e: any) {
    error.value = e.message || '查询失败'
  } finally {
    loading.value = false
  }
}
</script>
