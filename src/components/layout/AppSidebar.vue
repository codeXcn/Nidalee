<template>
  <Sidebar variant="inset" class="top-8 h-auto">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton size="lg" asChild>
            <router-link to="/">
              <!-- 顶部Logo -->
              <div class="flex items-center gap-3 px-2 py-2">
                <img src="@/assets/logo.png" class="w-9 h-9 rounded-lg shadow" />
                <div>
                  <div class="font-bold text-lg leading-tight">Nidalee</div>
                  <div class="text-xs text-gray-500">LCU Helper</div>
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
                <component :is="item.icon" />
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
                <component :is="item.icon" />
                <span>{{ item.title }}</span>
              </router-link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton asChild :tooltip="'设置'" :is-active="isActiveRoute('/settings')">
            <router-link to="/settings">
              <Settings />
              <span>设置</span>
            </router-link>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>

    <SidebarRail />
  </Sidebar>
</template>

<script setup lang="ts">
import { BarChart3, Gamepad2, Search, Settings, Shield, TrendingUp, Zap, TestTube } from 'lucide-vue-next'

const route = useRoute()

// 检测是否为开发环境
const isDev = import.meta.env.DEV

const menuItems = [
  {
    title: '仪表板',
    url: '/',
    icon: BarChart3
  },
  {
    title: '游戏助手',
    url: '/game-helper',
    icon: Gamepad2
  },
  {
    title: '战绩查询',
    url: '/match-search',
    icon: Search
  },
  {
    title: '对局分析',
    url: '/match-analysis',
    icon: TrendingUp
  },
  {
    title: '自动功能',
    url: '/auto-functions',
    icon: Zap
  },
  {
    title: '安全设置',
    url: '/security',
    icon: Shield
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
