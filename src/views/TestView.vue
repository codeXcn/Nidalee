<template>
  <div class="min-h-screen bg-background p-6">
    <div class="max-w-6xl mx-auto space-y-6">
      <!-- 标题 -->
      <div class="text-center">
        <h1 class="text-3xl font-bold text-foreground mb-2">🛡️ 英雄出装测试</h1>
        <p class="text-muted-foreground">测试英雄出装数据获取功能</p>
      </div>

      <!-- 测试区域 -->
      <Card>
        <CardHeader>
          <CardTitle class="text-foreground">出装API测试控制台</CardTitle>
        </CardHeader>
        <CardContent>
          <!-- 测试参数输入 -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            <div>
              <label class="block text-sm font-medium mb-2 text-foreground">数据源</label>
              <select
                v-model="testSource"
                class="w-full p-3 border border-border rounded-lg bg-background text-foreground focus:border-primary focus:ring-1 focus:ring-primary transition-colors"
              >
                <option value="op.gg">OP.GG</option>
                <option value="u.gg">U.GG</option>
                <option value="lolalytics">Lolalytics</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium mb-2 text-foreground">英雄别名</label>
              <Input v-model="testChampionAlias" type="text" placeholder="例如: Annie, Yasuo, Jinx" class="w-full" />
            </div>
          </div>

          <!-- 测试按钮区域 -->
          <div class="flex gap-4 flex-wrap mb-6">
            <Button
              @click="testGetChampionBuilds"
              :disabled="testingBuilds"
              class="flex items-center gap-2"
              variant="default"
            >
              <div
                v-if="testingBuilds"
                class="w-4 h-4 border-2 border-primary-foreground border-t-transparent rounded-full animate-spin"
              ></div>
              🛡️ {{ testingBuilds ? '测试中...' : '测试获取出装' }}
            </Button>

            <Button @click="clearResults" variant="ghost" class="flex items-center gap-2"> 🗑️ 清空结果 </Button>
          </div>

          <!-- 测试状态显示 -->
          <div v-if="lastTestInfo" class="mb-4 p-3 bg-primary/10 border border-primary/20 rounded-lg">
            <div class="flex items-center gap-2 text-primary">
              <div class="w-2 h-2 bg-primary rounded-full animate-pulse"></div>
              <span class="font-medium">{{ lastTestInfo }}</span>
            </div>
          </div>

          <!-- 错误信息显示 -->
          <div v-if="testError" class="mb-4 p-4 bg-destructive/10 border border-destructive/20 rounded-lg">
            <div class="flex items-start gap-3">
              <div class="text-destructive text-lg">❌</div>
              <div>
                <h3 class="font-semibold text-destructive mb-1">测试失败</h3>
                <p class="text-destructive/80 text-sm">{{ testError }}</p>
              </div>
            </div>
          </div>

          <!-- 成功信息显示 -->
          <div v-if="testSuccess" class="mb-4 p-4 bg-green-500/10 border border-green-500/20 rounded-lg">
            <div class="flex items-center gap-3">
              <div class="text-green-600 dark:text-green-400 text-lg">✅</div>
              <div>
                <h3 class="font-semibold text-green-700 dark:text-green-300">测试成功</h3>
                <p class="text-green-600 dark:text-green-400 text-sm">{{ testSuccess }}</p>
              </div>
            </div>
          </div>

          <!-- LCU符文详情状态显示 -->
          <div
            v-if="lcuPerks && lcuPerks.length > 0"
            class="mb-4 p-3 bg-blue-500/10 border border-blue-500/20 rounded-lg"
          >
            <div class="flex items-center gap-2 text-blue-600 dark:text-blue-400">
              <div class="w-2 h-2 bg-blue-500 rounded-full"></div>
              <span class="font-medium text-sm">LCU符文详情已加载 ({{ lcuPerks.length }} 个符文)</span>
              <Badge variant="outline" class="text-xs">符文图标可用</Badge>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 出装测试结果 -->
      <div v-if="championData" class="space-y-6">
        <!-- 英雄信息 -->
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-3">
              <img
                :src="`https://ddragon.leagueoflegends.com/cdn/${dataStore.gameVersion}/img/champion/${getChampionName()}.png`"
                :alt="getChampionName()"
                class="w-12 h-12 rounded-lg border-2 border-primary/20"
              />
              <div>
                <h2 class="text-xl font-bold text-foreground">{{ getChampionName() }}</h2>
                <p class="text-sm text-muted-foreground">{{ testSource.toUpperCase() }} 推荐出装</p>
              </div>
            </CardTitle>
          </CardHeader>
        </Card>

        <!-- 出装配置 -->
        <div v-for="(build, buildIndex) in getBuilds()" :key="`build-${buildIndex}`" class="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle class="flex items-center justify-between">
                <span class="text-foreground">出装配置 {{ buildIndex + 1 }}</span>
                <div class="flex gap-2 text-sm">
                  <Badge variant="outline">胜率: {{ build.win_rate }}%</Badge>
                  <Badge variant="outline">场次: {{ build.games }}</Badge>
                </div>
              </CardTitle>
            </CardHeader>
            <CardContent class="space-y-6">
              <!-- 初始出装 -->
              <div>
                <h3 class="font-semibold text-foreground mb-2 flex items-center gap-2">🏃 初始出装</h3>
                <div class="flex gap-1">
                  <div v-for="(item, itemIndex) in getStarterItems()" :key="`starter-${buildIndex}-${itemIndex}`">
                    <div class="relative group">
                      <img
                        :src="`https://ddragon.leagueoflegends.com/cdn/${dataStore.gameVersion}/img/item/${item}.png`"
                        :alt="`初始装备 ${item}`"
                        class="w-10 h-10 rounded border border-muted hover:border-primary transition-colors"
                      />
                    </div>
                  </div>
                </div>
              </div>

              <!-- 核心出装 -->
              <div>
                <h3 class="font-semibold text-foreground mb-2 flex items-center gap-2">🛡️ 核心出装</h3>
                <div class="flex gap-1">
                  <div v-for="(item, itemIndex) in build.items" :key="`item-${buildIndex}-${itemIndex}`">
                    <div class="relative group">
                      <img
                        :src="`https://ddragon.leagueoflegends.com/cdn/${dataStore.gameVersion}/img/item/${item}.png`"
                        :alt="`物品 ${item}`"
                        class="w-10 h-10 rounded border border-muted hover:border-primary transition-colors"
                      />
                    </div>
                  </div>
                </div>
              </div>

              <!-- 符文配置 -->
              <div v-if="build.runes && build.runes.length > 0">
                <div class="flex items-center justify-between mb-2">
                  <h3 class="font-semibold text-foreground flex items-center gap-2">⚡ 符文配置</h3>
                  <div class="flex items-center gap-3">
                    <!-- 技能加点顺序 -->
                    <div class="flex items-center gap-2">
                      <span class="text-xs text-muted-foreground font-medium">加点:</span>
                      <div class="flex gap-1">
                        <div
                          v-for="(skill, skillIndex) in getSkillOrder()"
                          :key="`skill-${buildIndex}-${skillIndex}`"
                          class="flex items-center justify-center w-6 h-6 rounded bg-primary/10 border border-primary/30 text-xs font-bold text-primary"
                        >
                          {{ skill }}
                        </div>
                      </div>
                    </div>

                    <!-- 应用天赋按钮 -->
                    <Button @click="applyRunes(build)" size="sm" variant="outline" class="text-xs h-6 px-2">
                      应用天赋
                    </Button>
                  </div>
                </div>

                <!-- 所有符文一排显示 -->
                <div class="flex flex-wrap items-center gap-2">
                  <!-- 主系符文 -->
                  <div class="flex items-center gap-1">
                    <span class="text-xs text-muted-foreground font-medium">主系:</span>
                    <div class="flex gap-1">
                      <div
                        v-for="rune in getPrimaryRunes(build.runes)"
                        :key="`primary-rune-${buildIndex}-${rune}`"
                        class="relative group"
                      >
                        <img
                          :src="getRuneIconUrl(rune)"
                          :alt="`符文 ${rune}`"
                          class="w-8 h-8 rounded border border-primary/30 hover:border-primary transition-colors"
                          @error="onRuneImageError"
                          :title="getRuneName(rune)"
                        />
                      </div>
                    </div>
                  </div>

                  <!-- 分隔符 -->
                  <div class="w-px h-6 bg-border mx-1"></div>

                  <!-- 副系符文 -->
                  <div class="flex items-center gap-1">
                    <span class="text-xs text-muted-foreground font-medium">副系:</span>
                    <div class="flex gap-1">
                      <div
                        v-for="rune in getSecondaryRunes(build.runes)"
                        :key="`secondary-rune-${buildIndex}-${rune}`"
                        class="relative group"
                      >
                        <img
                          :src="getRuneIconUrl(rune)"
                          :alt="`符文 ${rune}`"
                          class="w-8 h-8 rounded border border-secondary/30 hover:border-secondary transition-colors"
                          @error="onRuneImageError"
                          :title="getRuneName(rune)"
                        />
                      </div>
                    </div>
                  </div>

                  <!-- 分隔符 -->
                  <div class="w-px h-6 bg-border mx-1"></div>

                  <!-- 小天赋/属性符文 -->
                  <div v-if="getStatRunes(build.runes).length > 0" class="flex items-center gap-1">
                    <span class="text-xs text-muted-foreground font-medium">属性:</span>
                    <div class="flex gap-1">
                      <div
                        v-for="(rune, runeIndex) in getStatRunes(build.runes)"
                        :key="`stat-rune-${buildIndex}-${rune}-${runeIndex}`"
                        class="relative group"
                      >
                        <img
                          :src="getRuneIconUrl(rune)"
                          :alt="`属性符文 ${rune}`"
                          class="w-6 h-6 rounded border border-accent/30 hover:border-accent transition-colors"
                          @error="onRuneImageError"
                          :title="getRuneName(rune)"
                        />
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        <!-- 调试信息 -->
        <Card class="bg-muted/20 border-muted">
          <CardHeader>
            <CardTitle class="text-sm">🔍 调试信息</CardTitle>
          </CardHeader>
          <CardContent>
            <div class="text-xs text-muted-foreground space-y-1">
              <div>数据源: {{ testSource }}</div>
              <div>英雄: {{ testChampionAlias }}</div>
              <div>数据结构字段: {{ Object.keys(championData || {}).join(', ') }}</div>
              <div>出装数量: {{ getBuilds().length }}</div>
              <div>LCU符文数量: {{ lcuPerks.length }}</div>
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- 使用说明 -->
      <Card class="bg-orange-500/10 border-orange-500/20">
        <CardContent class="p-4">
          <h3 class="font-semibold text-orange-800 dark:text-orange-300 mb-2 flex items-center gap-2">💡 使用说明</h3>
          <ul class="text-orange-700 dark:text-orange-400 text-sm space-y-1">
            <li>• 选择数据源（op.gg、u.gg、lolalytics）</li>
            <li>• 输入英雄别名（如 Annie, Yasuo, Jinx）</li>
            <li>• 🛡️ 点击 "测试获取出装" - 自动获取出装和符文数据</li>
            <li>• ⚡ 符文图标自动显示：出装中的符文会显示客户端真实图标</li>
            <li>• 📊 查看完整数据：物品图标、符文配置、胜率统计</li>
            <li>• 查看控制台获取详细日志</li>
          </ul>

          <!-- 符文图标流程说明 -->
          <div class="mt-4 pt-3 border-t border-orange-500/20">
            <div class="mb-2">
              <span class="text-sm font-medium text-orange-800 dark:text-orange-300">🔧 符文图标显示流程：</span>
            </div>
            <div class="text-xs text-orange-600 dark:text-orange-400 space-y-1">
              <div>1. 获取出装数据（包含符文ID）</div>
              <div>2. 自动调用 get_lcu_perks 获取符文详情</div>
              <div>3. 根据符文ID匹配获取 iconPath</div>
              <div>4. 调用 get_lcu_perk_icon 显示真实图标</div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { useDataStore } from '@/stores'

// 获取数据存储
const dataStore = useDataStore()

// 测试状态
const testingBuilds = ref(false)
const testError = ref('')
const testSuccess = ref('')
const lastTestInfo = ref('')

// 测试参数
const testSource = ref('op.gg')
const testChampionAlias = ref('Annie')

// 结果数据
const championData = ref<any>(null)
const lcuPerks = ref<any[]>([])

// 符文图标缓存
const runeIconCache = ref<Map<string, string>>(new Map())

// 测试获取英雄出装
const testGetChampionBuilds = async () => {
  if (!testChampionAlias.value.trim()) {
    testError.value = '请输入英雄别名'
    return
  }

  testingBuilds.value = true
  testError.value = ''
  testSuccess.value = ''
  championData.value = null
  lastTestInfo.value = `正在获取 ${testChampionAlias.value} 在 ${testSource.value} 的出装数据...`

  try {
    console.log(`🛡️ 开始测试获取出装: ${testSource.value} - ${testChampionAlias.value}`)

    // 先获取LCU符文详情数据（如果还没有）
    if (!lcuPerks.value || lcuPerks.value.length === 0) {
      lastTestInfo.value = '正在获取 LCU 符文详情数据...'
      try {
        console.log('🔧 自动获取 LCU 符文详情数据')
        const runePerksResult = await invoke('get_lcu_perks')
        lcuPerks.value = runePerksResult as any[]
        console.log('🔧 LCU 符文详情获取成功:', runePerksResult)
      } catch (runeError) {
        console.warn('⚠️ 获取 LCU 符文详情失败，将使用回退方案:', runeError)
        // 继续执行，使用回退的图标方案
      }
    }

    // 获取出装数据
    lastTestInfo.value = `正在获取 ${testChampionAlias.value} 在 ${testSource.value} 的出装数据...`
    const result = await invoke('get_champion_builds', {
      source: testSource.value,
      championAlias: testChampionAlias.value
    })

    championData.value = result
    testSuccess.value = `成功获取 ${testChampionAlias.value} 的出装数据`
    lastTestInfo.value = ''

    console.log('🛡️ 出装数据获取成功:', result)
    console.log('🔍 championData.value 结构:', championData.value)
    console.log('🔍 championData.builds 是否存在:', championData.value?.builds)
    console.log('🔍 championData 所有字段:', Object.keys(championData.value || {}))

    // 如果有符文数据，输出日志便于调试
    const runeData = getRuneData()
    if (runeData.length > 0) {
      console.log('⚡ 符文配置数据:', runeData)
      console.log('🔧 LCU 符文详情数据:', lcuPerks.value)
    }
  } catch (error) {
    testError.value = `获取出装失败: ${error}`
    lastTestInfo.value = ''
    console.error('🛡️ 获取出装失败:', error)
  } finally {
    testingBuilds.value = false
  }
}

// 清空结果
const clearResults = () => {
  championData.value = null
  testError.value = ''
  testSuccess.value = ''
  lastTestInfo.value = ''
  runeIconCache.value.clear()
  console.log('🗑️ 清空所有测试结果')
}

// 获取符文数据
const getRuneData = () => {
  if (!championData.value) return []
  const builds = getBuilds()
  return builds.flatMap((build: any) => build.runes)
}

// 获取主系符文
const getPrimaryRunes = (runes: number[]) => {
  return runes.slice(0, 4) // 前4个是主系符文
}

// 获取副系符文
const getSecondaryRunes = (runes: number[]) => {
  return runes.slice(4, 6) // 5-6个是副系符文
}

// 获取小天赋/属性符文
const getStatRunes = (runes: number[]) => {
  return runes.slice(6, 9) // 7-9个是属性符文
}

// 查找符文详情
const findRuneById = (runeId: number) => {
  if (!lcuPerks.value || lcuPerks.value.length === 0) return null
  return lcuPerks.value.find((rune: any) => rune.id === runeId)
}

// 获取符文名称
const getRuneName = (runeId: number) => {
  const rune = findRuneById(runeId)
  return rune?.name || `符文 ${runeId}`
}

// 同步获取LCU符文图标URL（使用缓存）
const getLcuPerkIconUrlSync = (iconPath: string) => {
  if (!iconPath) return '/src/assets/RuneIconFiles/5001.png' // 默认图标

  // 检查缓存
  if (runeIconCache.value.has(iconPath)) {
    return runeIconCache.value.get(iconPath)!
  }

  // 异步获取并缓存
  const getLcuPerkIconAsync = async () => {
    try {
      const arrayBuffer = await invoke('get_lcu_perk_icon', { iconPath })
      const blob = new Blob([new Uint8Array(arrayBuffer as ArrayBuffer)], { type: 'image/png' })
      const url = URL.createObjectURL(blob)
      runeIconCache.value.set(iconPath, url)
      return url
    } catch (error) {
      console.warn(`⚠️ 获取LCU符文图标失败 ${iconPath}:`, error)
      return '/src/assets/RuneIconFiles/5001.png'
    }
  }

  // 立即调用异步函数
  getLcuPerkIconAsync()

  // 先返回默认图标，异步获取后会自动更新
  return '/src/assets/RuneIconFiles/5001.png'
}

// 获取符文图标URL
const getRuneIconUrl = (runeId: number) => {
  const rune = findRuneById(runeId)

  if (rune?.iconPath) {
    // 使用LCU真实图标
    return getLcuPerkIconUrlSync(rune.iconPath)
  }

  // 回退到本地图标
  return `/src/assets/RuneIconFiles/${runeId}.png`
}

// 符文图标加载错误处理
const onRuneImageError = (event: Event) => {
  const img = event.target as HTMLImageElement
  if (img.src.includes('RuneIconFiles')) {
    img.src = '/src/assets/RuneIconFiles/5001.png' // 默认符文图标
  }
}

// 获取英雄名称
const getChampionName = () => {
  if (!championData.value) return 'Unknown Champion'
  return championData.value.champion_alias || championData.value.content?.[0]?.name || 'Unknown Champion'
}

// 获取出装数据 - 简化版本，保持原有UI
const getBuilds = () => {
  if (!championData.value?.content?.[0]?.runes) return []

  // 直接使用符文配置作为出装方案，每个符文配置对应一套出装
  const runes = championData.value.content[0].runes
  const itemBuilds = championData.value.content[0].itemBuilds?.[0]?.blocks || []

  return runes.map((rune: any, index: number) => ({
    items: getItemsForBuild(itemBuilds, index),
    runes: rune.selectedPerkIds || [],
    win_rate: parseFloat(rune.winRate?.replace('%', '')) || 0,
    games: rune.pickCount || 0
  }))
}

// 为每个出装方案获取物品（简化逻辑）
const getItemsForBuild = (blocks: any[], buildIndex: number) => {
  // 获取核心装备
  const coreBlocks = blocks.filter((block: any) => block.type.includes('Core'))
  const bootBlocks = blocks.filter((block: any) => block.type.includes('Boots'))

  const items: string[] = []

  // 添加核心装备（如果有对应索引的话）
  if (coreBlocks[buildIndex]) {
    items.push(...coreBlocks[buildIndex].items.map((item: any) => item.id))
  } else if (coreBlocks[0]) {
    items.push(...coreBlocks[0].items.map((item: any) => item.id))
  }

  // 添加鞋子（取第一个）
  if (bootBlocks[0]) {
    items.push(bootBlocks[0].items[0].id)
  }

  return items
}

// 获取出装胜率
const getBuildWinRate = (build: any) => {
  if (!build || build.win_rate === undefined) return 'N/A'
  return build.win_rate
}

// 获取出装场次
const getBuildGames = (build: any) => {
  if (!build || build.games === undefined) return 'N/A'
  return build.games
}

// 获取出装符文
const getBuildRunes = (build: any) => {
  if (!build || !build.runes) return []
  return build.runes
}

// 获取初始出装物品
const getStarterItems = () => {
  if (!championData.value?.content?.[0]?.itemBuilds?.[0]?.blocks) return []
  const starterBlock = championData.value.content[0].itemBuilds[0].blocks.find((block: any) =>
    block.type.includes('Starter')
  )
  if (!starterBlock) return []
  return starterBlock.items.map((item: any) => item.id)
}

// 应用天赋
const applyRunes = async (build: any) => {
  if (!championData.value) return

  testingBuilds.value = true
  testError.value = ''
  testSuccess.value = ''
  lastTestInfo.value = `正在应用 ${getChampionName()} 的符文配置...`

  try {
    const result = await invoke('apply_champion_build', {
      championAlias: getChampionName(),
      buildIndex: getBuilds().findIndex((b: any) => b.runes.toString() === build.runes.toString())
    })

    testSuccess.value = `成功应用符文配置！`
    lastTestInfo.value = ''
    console.log('🛡️ 符文应用成功:', result)
  } catch (error) {
    testError.value = `应用符文失败: ${error}`
    lastTestInfo.value = ''
    console.error('🛡️ 符文应用失败:', error)
  } finally {
    testingBuilds.value = false
  }
}

// 获取技能加点顺序
const getSkillOrder = () => {
  if (!championData.value?.content?.[0]?.skills) return ['W', 'Q', 'E'] // 默认加点顺序
  return championData.value.content[0].skills
}
</script>

<style scoped>
.animate-spin {
  animation: spin 1s linear infinite;
}

.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
