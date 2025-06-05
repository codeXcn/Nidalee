import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import { useColorMode, usePreferredDark } from '@vueuse/core'

export type Theme = 'light' | 'dark' | 'auto'

// shadcn-vue 官方标准主题预设 - Tailwind v4 格式
const THEME_PRESETS = {
  zinc: {
    name: '锌灰色 (默认)',
    light: {
      primary: 'hsl(240 5.9% 10%)',
      primaryForeground: 'hsl(0 0% 98%)',
      secondary: 'hsl(240 4.8% 95.9%)',
      accent: 'hsl(240 4.8% 95.9%)',
      muted: 'hsl(240 4.8% 95.9%)',
      mutedForeground: 'hsl(240 3.8% 46.1%)',
      border: 'hsl(240 5.9% 90%)',
      input: 'hsl(240 5.9% 90%)',
      ring: 'hsl(240 5.9% 10%)'
    },
    dark: {
      primary: 'hsl(0 0% 98%)',
      primaryForeground: 'hsl(240 5.9% 10%)',
      secondary: 'hsl(240 3.7% 15.9%)',
      accent: 'hsl(240 3.7% 15.9%)',
      muted: 'hsl(240 3.7% 15.9%)',
      mutedForeground: 'hsl(240 5% 64.9%)',
      border: 'hsl(240 3.7% 15.9%)',
      input: 'hsl(240 3.7% 15.9%)',
      ring: 'hsl(240 4.9% 83.9%)'
    }
  },
  rose: {
    name: '玫瑰色',
    light: {
      primary: 'hsl(346.8 77.2% 49.8%)',
      primaryForeground: 'hsl(355.7 100% 97.3%)',
      secondary: 'hsl(240 4.8% 95.9%)',
      accent: 'hsl(240 4.8% 95.9%)',
      muted: 'hsl(240 4.8% 95.9%)',
      mutedForeground: 'hsl(240 3.8% 46.1%)',
      border: 'hsl(240 5.9% 90%)',
      input: 'hsl(240 5.9% 90%)',
      ring: 'hsl(346.8 77.2% 49.8%)'
    },
    dark: {
      primary: 'hsl(346.8 77.2% 49.8%)',
      primaryForeground: 'hsl(355.7 100% 97.3%)',
      secondary: 'hsl(240 3.7% 15.9%)',
      accent: 'hsl(12 6.5% 15.1%)',
      muted: 'hsl(0 0% 15%)',
      mutedForeground: 'hsl(240 5% 64.9%)',
      border: 'hsl(240 3.7% 15.9%)',
      input: 'hsl(240 3.7% 15.9%)',
      ring: 'hsl(346.8 77.2% 49.8%)'
    }
  },
  green: {
    name: '绿色',
    light: {
      primary: 'hsl(142.1 76.2% 36.3%)',
      primaryForeground: 'hsl(355.7 100% 97.3%)',
      secondary: 'hsl(240 4.8% 95.9%)',
      accent: 'hsl(240 4.8% 95.9%)',
      muted: 'hsl(240 4.8% 95.9%)',
      mutedForeground: 'hsl(240 3.8% 46.1%)',
      border: 'hsl(240 5.9% 90%)',
      input: 'hsl(240 5.9% 90%)',
      ring: 'hsl(142.1 76.2% 36.3%)'
    },
    dark: {
      primary: 'hsl(142.1 70.6% 45.3%)',
      primaryForeground: 'hsl(144.9 80.4% 10%)',
      secondary: 'hsl(240 3.7% 15.9%)',
      accent: 'hsl(12 6.5% 15.1%)',
      muted: 'hsl(0 0% 15%)',
      mutedForeground: 'hsl(240 5% 64.9%)',
      border: 'hsl(240 3.7% 15.9%)',
      input: 'hsl(240 3.7% 15.9%)',
      ring: 'hsl(142.4 71.8% 29.2%)'
    }
  },
  blue: {
    name: '蓝色',
    light: {
      primary: 'hsl(221.2 83.2% 53.3%)',
      primaryForeground: 'hsl(210 40% 98%)',
      secondary: 'hsl(210 40% 96.1%)',
      accent: 'hsl(210 40% 96.1%)',
      muted: 'hsl(210 40% 96.1%)',
      mutedForeground: 'hsl(215.4 16.3% 46.9%)',
      border: 'hsl(214.3 31.8% 91.4%)',
      input: 'hsl(214.3 31.8% 91.4%)',
      ring: 'hsl(221.2 83.2% 53.3%)'
    },
    dark: {
      primary: 'hsl(217.2 91.2% 59.8%)',
      primaryForeground: 'hsl(222.2 47.4% 11.2%)',
      secondary: 'hsl(217.2 32.6% 17.5%)',
      accent: 'hsl(217.2 32.6% 17.5%)',
      muted: 'hsl(217.2 32.6% 17.5%)',
      mutedForeground: 'hsl(215 20.2% 65.1%)',
      border: 'hsl(217.2 32.6% 17.5%)',
      input: 'hsl(217.2 32.6% 17.5%)',
      ring: 'hsl(224.3 76.3% 48%)'
    }
  },
  yellow: {
    name: '黄色',
    light: {
      primary: 'hsl(47.9 95.8% 53.1%)',
      primaryForeground: 'hsl(26 83.3% 14.1%)',
      secondary: 'hsl(60 4.8% 95.9%)',
      accent: 'hsl(60 4.8% 95.9%)',
      muted: 'hsl(60 4.8% 95.9%)',
      mutedForeground: 'hsl(25 5.3% 44.7%)',
      border: 'hsl(20 5.9% 90%)',
      input: 'hsl(20 5.9% 90%)',
      ring: 'hsl(20 14.3% 4.1%)'
    },
    dark: {
      primary: 'hsl(47.9 95.8% 53.1%)',
      primaryForeground: 'hsl(26 83.3% 14.1%)',
      secondary: 'hsl(12 6.5% 15.1%)',
      accent: 'hsl(12 6.5% 15.1%)',
      muted: 'hsl(12 6.5% 15.1%)',
      mutedForeground: 'hsl(24 5.4% 63.9%)',
      border: 'hsl(12 6.5% 15.1%)',
      input: 'hsl(12 6.5% 15.1%)',
      ring: 'hsl(35.5 91.7% 32.9%)'
    }
  }
}

export const useThemeStore = defineStore(
  'theme',
  () => {
    // 使用 VueUse 的 colorMode
    const colorMode = useColorMode({
      modes: {
        light: 'light',
        dark: 'dark',
        auto: 'auto'
      }
    })

    const prefersDark = usePreferredDark()

    // 主题相关状态
    const theme = ref<Theme>('auto')
    const selectedPreset = ref<keyof typeof THEME_PRESETS>('zinc')

    // 计算属性
    const isDark = computed(() => {
      if (theme.value === 'dark') return true
      if (theme.value === 'light') return false
      return prefersDark.value
    })

    const currentTheme = computed(() => {
      return theme.value
    })

    const themePresets = computed(() => THEME_PRESETS)

    // 方法
    function setTheme(newTheme: Theme) {
      theme.value = newTheme
      colorMode.value = newTheme === 'auto' ? 'auto' : newTheme
      updateDOMTheme()
    }

    function setPreset(preset: keyof typeof THEME_PRESETS) {
      selectedPreset.value = preset
      applyPreset()
    }

    function applyPreset() {
      const preset = THEME_PRESETS[selectedPreset.value]
      if (!preset) return

      const root = document.documentElement

      // 应用浅色主题变量
      Object.entries(preset.light).forEach(([key, value]) => {
        const cssVar = key.replace(/([A-Z])/g, '-$1').toLowerCase()
        root.style.setProperty(`--${cssVar}`, value)
      })

      // 创建或更新 .dark 样式
      let darkStyleEl = document.getElementById('dynamic-dark-theme')
      if (!darkStyleEl) {
        darkStyleEl = document.createElement('style')
        darkStyleEl.id = 'dynamic-dark-theme'
        document.head.appendChild(darkStyleEl)
      }

      const darkVars = Object.entries(preset.dark)
        .map(([key, value]) => {
          const cssVar = key.replace(/([A-Z])/g, '-$1').toLowerCase()
          return `--${cssVar}: ${value};`
        })
        .join('\n    ')

      darkStyleEl.textContent = `.dark {\n    ${darkVars}\n  }`
    }

    function updateDOMTheme() {
      const root = document.documentElement

      // 移除所有主题类
      root.classList.remove('light', 'dark')

      // 添加当前主题类
      if (isDark.value) {
        root.classList.add('dark')
      } else {
        root.classList.add('light')
      }
    }

    function resetToDefault() {
      theme.value = 'auto'
      selectedPreset.value = 'zinc'

      // 移除动态样式
      const darkStyleEl = document.getElementById('dynamic-dark-theme')
      if (darkStyleEl) {
        darkStyleEl.remove()
      }

      // 重置为默认 CSS 变量
      const root = document.documentElement
      const defaultVars = [
        '--primary',
        '--primary-foreground',
        '--secondary',
        '--secondary-foreground',
        '--accent',
        '--accent-foreground',
        '--muted',
        '--muted-foreground',
        '--border',
        '--input',
        '--ring'
      ]

      defaultVars.forEach(varName => {
        root.style.removeProperty(varName)
      })

      updateDOMTheme()
    }

    // 初始化
    function initializeTheme() {
      // 监听主题变化
      watch(
        [theme, isDark],
        () => {
          updateDOMTheme()
        },
        { immediate: true }
      )

      // 监听系统主题变化
      watch(prefersDark, () => {
        if (theme.value === 'auto') {
          updateDOMTheme()
        }
      })

      // 应用当前预设
      if (selectedPreset.value !== 'zinc') {
        applyPreset()
      }
    }

    return {
      // 状态
      theme,
      selectedPreset,

      // 计算属性
      isDark,
      currentTheme,
      themePresets,

      // 方法
      setTheme,
      setPreset,
      applyPreset,
      updateDOMTheme,
      resetToDefault,
      initializeTheme
    }
  },
  {
    persist: {
      storage: localStorage,
      pick: ['theme', 'selectedPreset']
    }
  }
)
