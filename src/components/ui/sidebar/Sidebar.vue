<template>
  <aside 
    :class="[
      'sidebar',
      'h-screen bg-card border-r border-border transition-all duration-300 ease-in-out flex flex-col',
      isCollapsed ? 'w-16' : 'w-64'
    ]"
  >
    <!-- 侧边栏头部 -->
    <div class="p-4 border-b border-border flex items-center justify-between">
      <div v-if="!isCollapsed" class="flex items-center space-x-3">
        <img src="/src/assets/logo-simple.svg" alt="Nidalee Logo" class="size-8" />
        <span class="font-semibold text-foreground text-lg">Nidalee</span>
      </div>
      <div v-else class="flex items-center justify-center">
        <img src="/src/assets/logo-simple.svg" alt="Nidalee Logo" class="size-8" />
      </div>
      <button 
        @click="toggleCollapse"
        class="p-2 rounded-lg hover:bg-accent transition-colors"
      >
        <ChevronLeft
          :class="[
            'size-4 transition-transform duration-300',
            isCollapsed ? 'rotate-180' : ''
          ]"
        />
      </button>
    </div>

    <!-- 导航菜单 -->
    <nav class="flex-1 p-2">
      <ul class="space-y-1">
        <li v-for="item in navigationItems" :key="item.id">
          <button
            @click="$emit('navigate', item.id)"
            :class="[
              'w-full flex items-center px-3 py-2 rounded-lg transition-all duration-200',
              'hover:bg-accent hover:text-accent-foreground',
              activeItem === item.id ? 'bg-primary text-primary-foreground' : 'text-muted-foreground'
            ]"
          >
            <component 
              :is="item.icon" 
              class="w-5 h-5 flex-shrink-0"
            />
            <span 
              v-if="!isCollapsed" 
              class="ml-3 font-medium transition-opacity duration-300"
            >
              {{ item.label }}
            </span>
          </button>
        </li>
      </ul>
    </nav>

    <!-- 侧边栏底部 -->
    <div class="p-2 border-t border-border">
      <button
        @click="$emit('openSettings')"
        :class="[
          'w-full flex items-center px-3 py-2 rounded-lg transition-colors',
          'hover:bg-accent hover:text-accent-foreground text-muted-foreground'
        ]"
      >
        <Settings class="w-5 h-5 flex-shrink-0" />
        <span v-if="!isCollapsed" class="ml-3 font-medium">设置</span>
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { 
  ChevronLeft,
  Settings,
  Gamepad2,
  BarChart3,
  Users,
  Shield,
  Sparkles
} from 'lucide-vue-next'

interface NavigationItem {
  id: string
  label: string
  icon: any
}

defineProps<{
  activeItem?: string
}>()

defineEmits<{
  navigate: [itemId: string]
  openSettings: []
}>()

const isCollapsed = ref(false)

const navigationItems: NavigationItem[] = [
  {
    id: 'dashboard',
    label: '仪表板',
    icon: BarChart3
  },
  {
    id: 'game-helper',
    label: '游戏助手',
    icon: Gamepad2
  },
  {
    id: 'match-analysis',
    label: '对局分析',
    icon: Users
  },
  {
    id: 'auto-functions',
    label: '自动功能',
    icon: Sparkles
  },
  {
    id: 'safety',
    label: '安全设置',
    icon: Shield
  }
]

function toggleCollapse() {
  isCollapsed.value = !isCollapsed.value
}
</script>

<style scoped>
.sidebar {
  user-select: none;
}
</style> 