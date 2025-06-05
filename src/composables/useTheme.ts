import { ref, computed, watch, onMounted } from 'vue'
import { useColorMode, usePreferredDark } from '@vueuse/core'

export type ThemeColor = 'zinc' | 'rose' | 'green' | 'blue' | 'yellow'
export type ThemeMode = 'light' | 'dark' | 'auto'

// 主题配置
const THEMES = {
  zinc: { name: '锌灰色 (默认)', class: 'theme-zinc' },
  rose: { name: '玫瑰色', class: 'theme-rose' },
  green: { name: '绿色', class: 'theme-green' },
  blue: { name: '蓝色', class: 'theme-blue' },
  yellow: { name: '黄色', class: 'theme-yellow' }
} as const

// 使用 localStorage 存储主题设置
const STORAGE_KEY = {
  COLOR: 'nidalee-theme-color',
  MODE: 'nidalee-theme-mode'
}

export function useTheme() {
  // 从 localStorage 获取保存的主题设置
  const savedColor = localStorage.getItem(STORAGE_KEY.COLOR) as ThemeColor | null
  const savedMode = localStorage.getItem(STORAGE_KEY.MODE) as ThemeMode | null

  // 主题状态
  const themeColor = ref<ThemeColor>(savedColor || 'zinc')
  const colorMode = useColorMode({
    modes: { light: 'light', dark: 'dark', auto: 'auto' },
    initialValue: savedMode || 'auto'
  })
  const prefersDark = usePreferredDark()

  // 计算属性
  const isDark = computed(() => {
    if (colorMode.value === 'dark') return true
    if (colorMode.value === 'light') return false
    return prefersDark.value
  })

  const currentTheme = computed(() => THEMES[themeColor.value])
  const availableThemes = computed(() => THEMES)

  // 方法
  function setThemeColor(color: ThemeColor) {
    themeColor.value = color
    localStorage.setItem(STORAGE_KEY.COLOR, color)
    applyTheme()
  }

  function setThemeMode(mode: ThemeMode) {
    colorMode.value = mode
    localStorage.setItem(STORAGE_KEY.MODE, mode)
  }

  function applyTheme() {
    const root = document.documentElement

    // 移除所有主题类
    Object.values(THEMES).forEach(theme => {
      root.classList.remove(theme.class)
    })

    // 移除深色模式类
    root.classList.remove('dark', 'light')

    // 添加当前主题类
    root.classList.add(currentTheme.value.class)

    // 添加深色/浅色模式类
    if (isDark.value) {
      root.classList.add('dark')
    } else {
      root.classList.add('light')
    }
  }

  function resetToDefault() {
    setThemeColor('zinc')
    setThemeMode('auto')
  }

  function toggleMode() {
    if (colorMode.value === 'light') {
      setThemeMode('dark')
    } else if (colorMode.value === 'dark') {
      setThemeMode('auto')
    } else {
      setThemeMode('light')
    }
  }

  // 监听变化
  watch(
    [themeColor, isDark],
    () => {
      applyTheme()
    },
    { immediate: true }
  )

  // 监听系统主题变化
  watch(prefersDark, () => {
    if (colorMode.value === 'auto') {
      applyTheme()
    }
  })

  // 初始化 - 在组件挂载时和创建时都执行
  onMounted(() => {
    applyTheme()
  })

  // 立即应用主题（同步执行）
  applyTheme()

  return {
    // 状态
    themeColor,
    themeMode: colorMode,
    isDark,

    // 计算属性
    currentTheme,
    availableThemes,

    // 方法
    setThemeColor,
    setThemeMode,
    resetToDefault,
    toggleMode,
    applyTheme
  }
}
