<template>
  <a
    href="https://github.com/codexlin/Nidalee"
    target="_blank"
    rel="noopener noreferrer"
    title="给项目点个 Star 支持开源！⭐"
    class="group block w-full relative overflow-hidden"
    @click="handleClick"
  >
    <!-- 背景渐变层 -->
    <div
      class="absolute inset-0 transition-all duration-500 rounded-xl"
      :class="[
        isDark
          ? 'bg-gradient-to-br from-slate-800/80 via-slate-900/90 to-gray-900/95 group-hover:from-slate-700/90 group-hover:via-slate-800/95 group-hover:to-gray-800/95'
          : 'bg-gradient-to-br from-white via-slate-50/90 to-gray-100/80 group-hover:from-yellow-50/60 group-hover:via-orange-50/40 group-hover:to-amber-50/60'
      ]"
    />

    <!-- 光晕效果层 -->
    <div
      class="absolute inset-0 rounded-xl opacity-0 group-hover:opacity-100 transition-all duration-700"
      :class="[
        isDark
          ? 'bg-gradient-to-r from-yellow-500/10 via-amber-400/5 to-orange-500/10'
          : 'bg-gradient-to-r from-yellow-400/15 via-amber-300/10 to-orange-400/15'
      ]"
    />

    <!-- 主要内容 -->
    <div
      class="relative flex items-center gap-2.5 px-3 py-2.5 rounded-xl border backdrop-blur-sm transition-all duration-300 hover:shadow-lg"
      :class="[
        isDark
          ? 'border-slate-700/60 hover:border-yellow-500/40 hover:shadow-yellow-500/20'
          : 'border-slate-200/80 hover:border-yellow-400/50 hover:shadow-yellow-400/25'
      ]"
    >
      <!-- GitHub 图标区域 -->
      <div class="relative flex-shrink-0">
        <!-- 主图标容器 -->
        <div
          class="w-8 h-8 rounded-full flex items-center justify-center transition-all duration-300 relative overflow-hidden"
          :class="[
            isDark
              ? 'bg-slate-700/50 group-hover:bg-gradient-to-br group-hover:from-yellow-500/20 group-hover:to-amber-500/20'
              : 'bg-slate-100/80 group-hover:bg-gradient-to-br group-hover:from-yellow-400/25 group-hover:to-amber-400/25'
          ]"
        >
          <!-- 图标光晕 -->
          <div
            class="absolute inset-0 rounded-full opacity-0 group-hover:opacity-100 transition-all duration-500"
            :class="[isDark ? 'bg-yellow-400/10' : 'bg-yellow-300/20']"
          />

          <!-- GitHub SVG 图标 -->
          <svg
            class="w-4 h-4 z-10 relative transition-all duration-300"
            :class="[
              isDark ? 'text-slate-300 group-hover:text-yellow-400' : 'text-slate-600 group-hover:text-yellow-600'
            ]"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
            />
          </svg>
        </div>

        <!-- 装饰星星们 -->
        <div class="absolute -top-1 -right-1 opacity-0 group-hover:opacity-100 transition-all duration-700 delay-300">
          <div class="w-2.5 h-2.5 rounded-full animate-ping" :class="[isDark ? 'bg-yellow-400' : 'bg-yellow-500']" />
        </div>
        <div
          class="absolute -bottom-0.5 -left-0.5 opacity-0 group-hover:opacity-100 transition-all duration-700 delay-500"
        >
          <div class="w-1.5 h-1.5 rounded-full animate-pulse" :class="[isDark ? 'bg-amber-400' : 'bg-amber-500']" />
        </div>
      </div>

      <!-- 文本内容区域 -->
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-1.5">
          <span
            class="font-semibold text-sm leading-tight transition-all duration-300 whitespace-nowrap"
            :class="[isDark ? 'text-slate-100 group-hover:text-white' : 'text-slate-800 group-hover:text-slate-900']"
          >
            鼓励作者⭐
          </span>
          <ExternalLink
            class="w-3.5 h-3.5 transition-all duration-300 group-hover:translate-x-0.5 group-hover:-translate-y-0.5 flex-shrink-0"
            :class="[
              isDark ? 'text-slate-400 group-hover:text-slate-300' : 'text-slate-500 group-hover:text-slate-600'
            ]"
          />
        </div>
      </div>
    </div>

    <!-- 底部发光条 -->
    <div
      class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-0 h-0.5 group-hover:w-3/4 transition-all duration-700 rounded-full"
      :class="[
        isDark
          ? 'bg-gradient-to-r from-transparent via-yellow-400 to-transparent'
          : 'bg-gradient-to-r from-transparent via-yellow-500 to-transparent'
      ]"
    />
  </a>
</template>

<script setup lang="ts">
import { ExternalLink } from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { isDark } = useApp()

const handleClick = () => {
  setTimeout(() => {
    toast.success('谢谢你的⭐！', {
      description: '开发者收到了你的鼓励，超级开心 🥳',
      duration: 3000
    })
  }, 200)
}
</script>

<style scoped>
/* 额外的动画效果 */
@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.group:hover::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  animation: shimmer 2s ease-out;
  z-index: 1;
}
</style>
