<template>
  <Sidebar variant="inset">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton size="lg" asChild>
            <router-link to="/">
              <div
                class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="size-4"
                >
                  <circle cx="12" cy="12" r="10" />
                  <path d="m9 12 2 2 4-4" />
                </svg>
              </div>
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">Nidalee</span>
                <span class="truncate text-xs">LCU Helper</span>
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
            <SidebarMenuButton asChild :tooltip="item.title" :is-active="isActiveRoute(item.url)">
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
import { useRoute } from 'vue-router'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail
} from '@/components/ui/sidebar'
import { BarChart3, Gamepad2, TrendingUp, Zap, Shield, Settings } from 'lucide-vue-next'

const route = useRoute()

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

const isActiveRoute = (url: string) => {
  if (url === '/') {
    return route.path === '/'
  }
  return route.path.startsWith(url)
}
</script>
