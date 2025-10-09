<template>
  <Sidebar variant="inset" class="top-8 h-auto">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton size="lg" asChild>
            <router-link to="/">
              <div class="flex items-center gap-3 py-2 select-none">
                <div
                  class="relative isolate overflow-hidden rounded-xl p-[1px] bg-gradient-to-br from-white/70 to-black/10"
                >
                  <img
                    src="@/assets/logo.png"
                    class="w-10 h-10 rounded-[10px] bg-white shadow-[inset_0_1px_2px_rgba(0,0,0,0.06)]"
                  />
                  <div
                    class="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_top_left,rgba(255,255,255,0.6),transparent_55%)]"
                  />
                </div>
                <div class="flex flex-col justify-center min-w-0">
                  <div
                    class="font-extrabold text-xl leading-tight tracking-wide truncate bg-gradient-to-r bg-clip-text text-transparent from-primary to-purple-600"
                  >
                    <RadiantText class="transition ease-out" :duration="5">
                      <span>Nidalee~</span>
                    </RadiantText>
                  </div>
                </div>
              </div>
            </router-link>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>

    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>应用功能</SidebarGroupLabel>
        <SidebarMenu>
          <SidebarMenuItem v-for="item in menuItems" :key="item.title">
            <SidebarMenuButton class="text-md" asChild :tooltip="item.title" :is-active="isActiveRoute(item.url)">
              <router-link :to="item.url">
                <component
                  :is="item.icon"
                  :size="18"
                  :stroke-width="2"
                  class="shrink-0"
                  :class="{ 'text-primary': isActiveRoute(item.url) }"
                />
                <span>{{ item.title }}</span>
              </router-link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>

      <!-- 开发者工具分组 - 仅在开发环境显示 -->
      <SidebarGroup v-if="isDev">
        <SidebarGroupLabel>开发者工具</SidebarGroupLabel>
        <SidebarMenu>
          <SidebarMenuItem v-for="item in devItems" :key="item.title">
            <SidebarMenuButton class="text-md" asChild :tooltip="item.title" :is-active="isActiveRoute(item.url)">
              <router-link :to="item.url">
                <component
                  :is="item.icon"
                  :size="18"
                  :stroke-width="2"
                  class="shrink-0"
                  :class="{ 'text-primary': isActiveRoute(item.url) }"
                />
                <span>{{ item.title }}</span>
              </router-link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter>
      <div class="px-2">
        <GitHubStarButtonBeautiful />
      </div>

      <div class="px-2 text-xs text-muted-foreground select-none">软件版本 {{ `v${appVersion}` || '-' }}</div>

      <div class="px-2 text-xs text-muted-foreground select-none">游戏版本 {{ `v${lolGameVersion}` || '-' }}</div>

      <div v-if="updateAvailable" class="px-2 py-1">
        <div
          v-if="!isUpdating"
          class="text-xs flex items-center gap-2 rounded bg-primary/10 text-primary px-2 py-1 cursor-pointer hover:bg-primary/15"
          @click="startUpdateNow()"
        >
          <span class="inline-flex h-2 w-2 rounded-full bg-primary animate-pulse" />
          有可用更新 → {{ updateVersion ? `v${updateVersion}` : '' }}（点击下载并安装）
        </div>
        <div v-else class="rounded bg-primary/10 text-primary px-2 py-1">
          <div class="text-xs flex items-center gap-2">
            <span class="inline-flex h-2 w-2 rounded-full bg-primary animate-pulse" />
            {{ updateStatusText }}
          </div>
          <Progress :model-value="updateProgress" class="mt-1 h-1.5" />
          <span
            class="sr-only"
            role="progressbar"
            :aria-valuemin="0"
            :aria-valuemax="100"
            :aria-valuenow="updateProgress"
          >
            更新下载进度
          </span>
        </div>
      </div>

      <!-- <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton asChild :tooltip="'设置'" :is-active="isActiveRoute('/settings')">
            <router-link to="/settings">
              <Settings />
              <span>设置</span>
            </router-link>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu> -->
    </SidebarFooter>

    <SidebarRail />
  </Sidebar>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
import { Radar, BarChart3, Settings, Sparkles, TestTube, Swords, Trophy } from 'lucide-vue-next'
import { useDataStore } from '@/stores/core/dataStore'
import { useAppUpdater } from '@/composables/app/useAppUpdater'
import { Progress } from '@/components/ui/progress'
const route = useRoute()
// 组合式方式获取更新状态与方法（内部是单例，不会重复实例化）
const { updateAvailable, updateVersion, isUpdating, updateProgress, startUpdateNow } = useAppUpdater()

// 动态状态文案：0% -> 正在连接…；1-99% -> 正在下载…N%；100% -> 正在安装/准备重启
const updateStatusText = computed(() => {
  const p = updateProgress.value
  if (p <= 0) return '正在连接更新服务器…'
  if (p >= 100) return '正在安装/准备重启…'
  return `正在下载更新 ${p}%`
})

const isDev = import.meta.env.DEV

const appVersion = ref<string>('')
onMounted(async () => {
  try {
    appVersion.value = await getVersion()
  } catch {
    appVersion.value = ''
  }
})

// 游戏版本：从 dataStore 读取（由初始化逻辑 setGameVersion）
const dataStore = useDataStore()
const lolGameVersion = computed(() => dataStore.gameVersion)

const menuItems = [
  {
    title: '个人仪表盘',
    url: '/',
    icon: Trophy
  },
  {
    title: '游戏小助手',
    url: '/game-helper',
    icon: Sparkles
  },
  {
    title: '战绩查询器',
    url: '/match-search',
    icon: Radar
  },
  {
    title: '对局分析报',
    url: '/match-analysis',
    icon: Swords
  },
  {
    title: 'OP.GG查询',
    url: '/opgg',
    icon: BarChart3
  },
  {
    title: '客户端设置',
    url: '/settings',
    icon: Settings
  }
]

const devItems = [
  {
    title: 'API测试',
    url: '/test-api',
    icon: TestTube
  },

  {
    title: '选人测试',
    url: '/champ-select-test',
    icon: TestTube
  }
]

const isActiveRoute = (url: string) => {
  if (url === '/') {
    return route.path === '/'
  }
  return route.path.startsWith(url)
}
</script>
