<template>
  <div class="space-y-6">
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <Users class="h-5 w-5" />
          生涯背景设置
          <span class="text-muted-foreground">选择英雄并设置皮肤为生涯背景</span>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div v-if="!selectedChampion">
          <!-- 搜索栏 -->
          <div class="relative mb-4">
            <Search class="absolute left-4 top-1/2 transform -translate-y-1/2 h-5 w-5 text-muted-foreground" />
            <Input
              v-model="searchText"
              placeholder="搜索英雄名称或别名..."
              class="pl-12 h-12 text-base bg-background/50 border-border/50 focus:border-primary/50 focus:bg-background transition-all duration-200 shadow-sm focus:shadow-md"
            />
            <div v-if="searchText" class="absolute right-3 top-1/2 transform -translate-y-1/2">
              <button
                @click="searchText = ''"
                class="h-7 w-7 rounded-full bg-secondary hover:bg-destructive/10 hover:text-destructive flex items-center justify-center transition-all duration-200 border border-border hover:border-destructive/30 group"
                title="清除搜索"
              >
                <X class="h-4 w-4 text-muted-foreground group-hover:text-destructive transition-colors duration-200" />
              </button>
            </div>
          </div>
          <!-- 英雄网格 -->
          <ScrollArea class="h-[min(600px,calc(85vh-200px))] w-full rounded-lg border border-border bg-muted/20">
            <div class="p-6">
              <div v-if="loadingChampions" class="flex flex-col items-center justify-center py-16 text-center">
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                  <Skeleton
                    v-for="i in 12"
                    :key="i"
                    class="h-24 rounded-xl bg-gradient-to-br from-muted/60 to-muted/30 animate-pulse shadow-md"
                  />
                </div>
                <p class="text-muted-foreground font-medium text-lg mt-8">正在加载英雄数据...</p>
              </div>
              <div v-else-if="championsError" class="flex flex-col items-center justify-center py-16 text-center">
                <div
                  class="h-16 w-16 rounded-full bg-destructive/10 flex items-center justify-center mb-6 border border-destructive/20"
                >
                  <Search class="h-8 w-8 text-destructive" />
                </div>
                <p class="text-destructive font-semibold text-lg mb-2">加载失败</p>
                <p class="text-sm text-muted-foreground mb-6 max-w-md">{{ championsError }}</p>
                <button
                  @click="loadChampions"
                  class="px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-all duration-200 text-sm font-medium shadow-sm hover:shadow-md"
                >
                  重新加载
                </button>
              </div>
              <div v-else>
                <div class="grid grid-cols-3 md:grid-cols-5 lg:grid-cols-7 gap-2">
                  <div
                    v-for="champion in filteredChampions"
                    :key="champion.id"
                    class="flex flex-col items-center p-1 rounded-lg cursor-pointer transition-all duration-200 bg-gradient-to-br from-background/80 to-background/40 hover:from-primary/10 hover:to-primary/20 hover:scale-105 hover:shadow group"
                    @click="handleChampionSelect(champion)"
                    style="will-change: transform, opacity; transform: translateZ(0); backface-visibility: hidden"
                  >
                    <div class="relative">
                      <Avatar
                        class="h-12 w-12 border-2 border-transparent group-hover:border-primary/60 transition-all duration-200 shadow group-hover:shadow-md bg-gradient-to-br from-primary/10 to-primary/30"
                      >
                        <AvatarImage
                          :src="getChampionIconUrlByAlias(champion.alias)"
                          :alt="champion.name"
                          class="object-cover"
                        />
                        <AvatarFallback
                          class="text-xs font-semibold bg-gradient-to-br from-primary/10 to-primary/20 text-primary border border-primary/30"
                        >
                          {{ champion.name.slice(0, 2).toUpperCase() }}
                        </AvatarFallback>
                      </Avatar>

                      <!-- 悬停发光环 -->
                      <div
                        class="absolute inset-0 rounded-full bg-primary/20 opacity-0 group-hover:opacity-100 transition-opacity duration-200 -z-10 blur-sm pointer-events-none"
                        style="will-change: opacity"
                      ></div>
                    </div>
                    <div class="w-full text-center mt-1">
                      <span
                        class="text-[11px] text-foreground font-medium leading-tight inline-block w-full overflow-hidden text-ellipsis whitespace-nowrap group-hover:text-primary transition-colors duration-200"
                        :title="champion.name"
                      >
                        {{ champion.name }}
                      </span>
                    </div>
                  </div>
                </div>
                <div
                  v-if="!loadingChampions && !championsError && filteredChampions.length === 0"
                  class="flex flex-col items-center justify-center py-20 text-center"
                >
                  <div
                    class="h-20 w-20 rounded-full bg-muted flex items-center justify-center mb-6 border-2 border-dashed border-muted-foreground/30"
                  >
                    <Search class="h-10 w-10 text-muted-foreground/50" />
                  </div>
                  <p class="text-muted-foreground font-semibold text-lg mb-2">没有找到匹配的英雄</p>
                  <p class="text-sm text-muted-foreground">尝试使用不同的关键词搜索，如英雄名称或别名</p>
                </div>
              </div>
            </div>
          </ScrollArea>
        </div>
        <div v-else>
          <div class="flex items-center mb-4">
            <button @click="clearChampion" class="cursor-pointer flex items-center gap-1 text-primary hover:underline">
              <ArrowLeft class="h-4 w-4 "/> 返回英雄列表
            </button>
            <span class="ml-4 font-semibold text-2xl tracking-wide">{{ selectedChampion.name }}</span>
          </div>
          <div>
            <div v-if="loadingSkins" class="flex flex-col items-center justify-center py-16 text-center">
              <div class="columns-1 sm:columns-1 md:columns-2 lg:columns-2 gap-10 w-full">
                <Skeleton
                  v-for="i in 6"
                  :key="i"
                  class="aspect-[1.7] rounded-3xl mb-10 w-full bg-gradient-to-br from-muted/60 to-muted/30 animate-pulse shadow-xl"
                />
              </div>
              <p class="text-muted-foreground font-medium text-lg mt-8">正在加载皮肤数据...</p>
            </div>
            <div v-else-if="skinsError" class="flex flex-col items-center justify-center py-16 text-center">
              <div
                class="h-16 w-16 rounded-full bg-destructive/10 flex items-center justify-center mb-6 border border-destructive/20"
              >
                <X class="h-8 w-8 text-destructive" />
              </div>
              <p class="text-destructive font-semibold text-lg mb-2">加载失败</p>
              <p class="text-sm text-muted-foreground mb-6 max-w-md">{{ skinsError }}</p>
              <button
                @click="loadChampionSkins(selectedChampion)"
                class="px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-all duration-200 text-sm font-medium shadow-sm hover:shadow-md"
              >
                重新加载
              </button>
            </div>
            <div v-else>
              <!-- Masonry 瀑布流皮肤卡片区，每排更少，卡片更大 -->
              <div class="columns-1 sm:columns-1 md:columns-2 lg:columns-2 gap-10">
                <div
                  v-for="(skin, idx) in championSkins"
                  :key="skin.id"
                  :style="getCard3DStyle(idx)"
                  class="mb-10 break-inside-avoid relative rounded-3xl overflow-hidden border-2 border-transparent cursor-pointer group shadow-2xl transition-all duration-300 bg-gradient-to-br from-background/80 to-background/60 hover:scale-105 hover:shadow-2xl hover:border-primary/80 hover:z-20 transform-gpu group-hover:[transform:perspective(800px)_rotateX(8deg)_rotateY(8deg)_scale3d(1.04,1.04,1.04)] hover:tw-animate-tilt hover:tw-animate-glow"
                  :class="[{ 'pointer-events-none opacity-70': applyingSkinId === skin.id }, shakeSkinId === skin.id ? 'animate-shake' : '']"
                  @click="applySkinBackground(skin)"
                  :id="'skin-card-' + skin.id"
                >
                  <img
                    :src="getSkinImageUrl(skin)"
                    :alt="skin.name"
                    class="w-full aspect-[1.7] object-cover group-hover:scale-110 transition-transform duration-500"
                  />
                  <div
                    class="absolute inset-0 bg-black/40 group-hover:bg-black/20 transition-colors duration-300 z-10"
                  ></div>

                  <!-- 应用中的加载覆盖层 -->
                  <div
                    v-if="applyingSkinId === skin.id"
                    class="absolute inset-0 bg-primary/80 flex items-center justify-center z-30 rounded-3xl"
                  >
                    <LoadingSpinner />
                    <span class="text-sm font-medium text-white ml-3">正在应用...</span>
                  </div>

                  <div class="absolute bottom-0 left-0 right-0 px-4 py-2 flex flex-col items-start z-20">
                    <span class="text-base font-bold text-white drop-shadow-lg">{{ skin.name }}</span>
                    <span
                      v-if="skin.isBase"
                      class="mt-1 px-3 py-0.5 rounded-full bg-primary/80 text-xs text-white font-semibold shadow"
                      >经典</span
                    >
                  </div>
                  <div
                    class="absolute inset-0 pointer-events-none group-hover:ring-4 group-hover:ring-primary/40 rounded-3xl transition-all duration-300"
                  ></div>
                </div>
              </div>
              <div
                v-if="!loadingSkins && !skinsError && championSkins.length === 0"
                class="flex flex-col items-center justify-center py-20 text-center"
              >
                <div
                  class="h-20 w-20 rounded-full bg-muted flex items-center justify-center mb-6 border-2 border-dashed border-muted-foreground/30"
                >
                  <Users class="h-10 w-10 text-muted-foreground/50" />
                </div>
                <p class="text-muted-foreground font-semibold text-lg mb-2">暂无皮肤数据</p>
                <p class="text-sm text-muted-foreground">该英雄暂时没有可用的皮肤数据</p>
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<script setup lang="tsx">
import { ref, computed, onMounted, watch } from 'vue'
import { debounce } from 'radash'
import { Users, X, Search, ArrowLeft } from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import type { ChampionInfo } from '@/stores/autoFunctionStore'
import { getCommunityDragonUrl, getChampionIconUrlByAlias } from '@/lib'
import { fetchChampionDetails, fetchChampionSummary } from '@/lib/dataApi'
import { useGameHelper } from '@/composables/game-helper'
const { setSummonerBackgroundSkin } = useGameHelper()
const searchText = ref('')
const debouncedSearchText = ref('')
const champions = ref<ChampionInfo[]>([])
const loadingChampions = ref(true)
const championsError = ref<string | null>(null)
const selectedChampion = ref<ChampionInfo | null>(null)
const championSkins = ref<any[]>([])
const loadingSkins = ref(false)
const skinsError = ref<string | null>(null)
const applyingSkinId = ref<number | null>(null) // 跟踪正在应用的皮肤ID
const shakeSkinId = ref<number | null>(null)


// 使用 Radash 的防抖函数，更简洁可靠
const debouncedUpdateSearch = debounce({ delay: 300 }, (value: string) => {
  debouncedSearchText.value = value
})

watch(searchText, (newValue) => {
  debouncedUpdateSearch(newValue)
})

const filteredChampions = computed(() => {
  if (!debouncedSearchText.value.trim()) return champions.value
  const search = debouncedSearchText.value.toLowerCase()
  return champions.value.filter(
    (c) =>
      c.name.toLowerCase().includes(search) ||
      c.alias.toLowerCase().includes(search) ||
      (c.description && c.description.toLowerCase().includes(search))
  )
})

const handleChampionSelect = (champion: ChampionInfo) => {
  selectedChampion.value = champion
  loadChampionSkins(champion)
}

const clearChampion = () => {
  selectedChampion.value = null
  championSkins.value = []
  skinsError.value = null
}

const loadChampions = async () => {
  try {
    loadingChampions.value = true
    championsError.value = null
    const response = await fetchChampionSummary()
    if (response.success && response.data) {
      const championList: ChampionInfo[] = response.data.filter((champion) => champion.id > 0)
      championList.sort((a, b) => a.name.localeCompare(b.name))
      champions.value = championList
      // 直接移除 imageLoadingStates 相关逻辑
    } else {
      throw new Error(response.error || '获取英雄数据失败')
    }
  } catch (err) {
    championsError.value = err instanceof Error ? err.message : '未知错误'
  } finally {
    loadingChampions.value = false
  }
}

const loadChampionSkins = async (champion: ChampionInfo) => {
  loadingSkins.value = true
  skinsError.value = null
  try {
    const championDetailsResult = await fetchChampionDetails(champion.id)
    if (championDetailsResult.success && championDetailsResult.data?.skins) {
      championSkins.value = championDetailsResult.data.skins
    } else {
      throw new Error('无法获取皮肤数据')
    }
  } catch (error) {
    skinsError.value = error instanceof Error ? error.message : '加载皮肤失败'
    championSkins.value = []
  } finally {
    loadingSkins.value = false
  }
}

const getSkinImageUrl = (skin: any): string => {
  // 皮肤ID一般为 英雄ID*1000+皮肤编号
  if (!selectedChampion.value?.alias || typeof skin.id !== 'number') return ''
  const skinNum = skin.id % 1000
  return `https://ddragon.leagueoflegends.com/cdn/img/champion/splash/${selectedChampion.value.alias}_${skinNum}.jpg`
}

const applySkinBackground = async (skin: any) => {
  try {
    applyingSkinId.value = skin.id
    shakeSkinId.value = skin.id // 触发抖动
    // 调用后端API设置生涯背景皮肤
    await setSummonerBackgroundSkin(skin.id)

    // 使用 Sonner 显示成功提示
    toast.success(`皮肤"${skin.name}"已设置为生涯背景`, {
      description: '生涯背景设置成功完成',
      duration: 3000
    })
  } catch (error) {
    // 使用 Sonner 显示错误提示
    console.error('设置生涯背景失败:', error)
    toast.error('设置生涯背景失败', {
      description: String(error),
      duration: 5000
    })
  } finally {
    setTimeout(() => {
      shakeSkinId.value = null // 抖动动画结束后移除
      applyingSkinId.value = null
    }, 600)
  }
}

// 生成每张卡片不同的初始3D角度
const getCard3DStyle = (idx: number) => {
  // 让角度有规律但不重复，视觉更丰富
  const rotateX = ((idx % 4) - 1.5) * 6 // -9, -3, 3, 9
  const rotateY = ((idx % 6) - 2.5) * 5 // -12.5, -7.5, -2.5, 2.5, 7.5, 12.5
  return {
    willChange: 'transform, opacity',
    transform: `perspective(800px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale3d(1,1,1)`
  }
}

// Loading 动画组件
const LoadingSpinner = defineComponent({
  name: 'LoadingSpinner',
  setup() {
    return () => (
      <div className="flex items-center justify-center">
        <span className="inline-block w-8 h-8 border-4 border-primary border-t-transparent rounded-full animate-spin animate__animated animate__fadeIn" />
      </div>
    )
  }
})

onMounted(() => {
  loadChampions()
})
</script>
