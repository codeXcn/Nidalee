import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useThemeStore = defineStore(
  'theme',
  () => {
    // 状态
    const selectedColor = ref('neutral')
    const selectedRadius = ref(0.5)
    const selectedStyle = ref('new-york')
    const isDark = ref(false)

    // 颜色主题配置
    const colorThemes = {
      zinc: {
        light: {
          primary: 'oklch(0.243 0 0)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.985 0 0)',
          'primary-foreground': 'oklch(0.243 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      slate: {
        light: {
          primary: 'oklch(0.305 0.015 247.858)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.966 0.009 247.858)',
          'secondary-foreground': 'oklch(0.145 0.015 247.858)',
          accent: 'oklch(0.966 0.009 247.858)',
          'accent-foreground': 'oklch(0.145 0.015 247.858)',
          muted: 'oklch(0.966 0.009 247.858)',
          'muted-foreground': 'oklch(0.556 0.015 247.858)'
        },
        dark: {
          primary: 'oklch(0.985 0 0)',
          'primary-foreground': 'oklch(0.243 0.015 247.858)',
          secondary: 'oklch(0.269 0.015 247.858)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0.015 247.858)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0.015 247.858)',
          'muted-foreground': 'oklch(0.708 0.015 247.858)'
        }
      },
      stone: {
        light: {
          primary: 'oklch(0.305 0.008 41.116)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.966 0.005 41.116)',
          'secondary-foreground': 'oklch(0.145 0.008 41.116)',
          accent: 'oklch(0.966 0.005 41.116)',
          'accent-foreground': 'oklch(0.145 0.008 41.116)',
          muted: 'oklch(0.966 0.005 41.116)',
          'muted-foreground': 'oklch(0.556 0.008 41.116)'
        },
        dark: {
          primary: 'oklch(0.985 0 0)',
          'primary-foreground': 'oklch(0.243 0.008 41.116)',
          secondary: 'oklch(0.269 0.008 41.116)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0.008 41.116)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0.008 41.116)',
          'muted-foreground': 'oklch(0.708 0.008 41.116)'
        }
      },
      gray: {
        light: {
          primary: 'oklch(0.305 0.003 286.38)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.966 0.002 286.38)',
          'secondary-foreground': 'oklch(0.145 0.003 286.38)',
          accent: 'oklch(0.966 0.002 286.38)',
          'accent-foreground': 'oklch(0.145 0.003 286.38)',
          muted: 'oklch(0.966 0.002 286.38)',
          'muted-foreground': 'oklch(0.556 0.003 286.38)'
        },
        dark: {
          primary: 'oklch(0.985 0 0)',
          'primary-foreground': 'oklch(0.243 0.003 286.38)',
          secondary: 'oklch(0.269 0.003 286.38)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0.003 286.38)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0.003 286.38)',
          'muted-foreground': 'oklch(0.708 0.003 286.38)'
        }
      },
      neutral: {
        light: {
          primary: 'oklch(0.243 0 0)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.985 0 0)',
          'primary-foreground': 'oklch(0.243 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      red: {
        light: {
          primary: 'oklch(0.577 0.245 27.325)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.637 0.237 25.331)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      rose: {
        light: {
          primary: 'oklch(0.627 0.265 3.9)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.645 0.246 16.439)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      orange: {
        light: {
          primary: 'oklch(0.646 0.222 41.116)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.769 0.188 70.08)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      green: {
        light: {
          primary: 'oklch(0.6 0.118 145.704)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.696 0.17 162.48)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      blue: {
        light: {
          primary: 'oklch(0.488 0.243 264.376)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.488 0.243 264.376)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      yellow: {
        light: {
          primary: 'oklch(0.828 0.189 84.429)',
          'primary-foreground': 'oklch(0.243 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.769 0.188 70.08)',
          'primary-foreground': 'oklch(0.243 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      },
      violet: {
        light: {
          primary: 'oklch(0.627 0.265 303.9)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.97 0 0)',
          'secondary-foreground': 'oklch(0.145 0 0)',
          accent: 'oklch(0.97 0 0)',
          'accent-foreground': 'oklch(0.145 0 0)',
          muted: 'oklch(0.97 0 0)',
          'muted-foreground': 'oklch(0.556 0 0)'
        },
        dark: {
          primary: 'oklch(0.627 0.265 303.9)',
          'primary-foreground': 'oklch(0.985 0 0)',
          secondary: 'oklch(0.269 0 0)',
          'secondary-foreground': 'oklch(0.985 0 0)',
          accent: 'oklch(0.269 0 0)',
          'accent-foreground': 'oklch(0.985 0 0)',
          muted: 'oklch(0.269 0 0)',
          'muted-foreground': 'oklch(0.708 0 0)'
        }
      }
    }

    // 颜色选项
    const colors = [
      { name: 'zinc', label: 'Zinc', bgClass: 'bg-zinc-500' },
      { name: 'slate', label: 'Slate', bgClass: 'bg-slate-500' },
      { name: 'stone', label: 'Stone', bgClass: 'bg-stone-500' },
      { name: 'gray', label: 'Gray', bgClass: 'bg-gray-500' },
      { name: 'neutral', label: 'Neutral', bgClass: 'bg-neutral-500' },
      { name: 'red', label: 'Red', bgClass: 'bg-red-500' },
      { name: 'rose', label: 'Rose', bgClass: 'bg-rose-500' },
      { name: 'orange', label: 'Orange', bgClass: 'bg-orange-500' },
      { name: 'green', label: 'Green', bgClass: 'bg-green-500' },
      { name: 'blue', label: 'Blue', bgClass: 'bg-blue-500' },
      { name: 'yellow', label: 'Yellow', bgClass: 'bg-yellow-500' },
      { name: 'violet', label: 'Violet', bgClass: 'bg-violet-500' }
    ]

    // 圆角选项
    const radiusOptions = [
      { value: 0, label: '0' },
      { value: 0.25, label: '0.25' },
      { value: 0.5, label: '0.5' },
      { value: 0.75, label: '0.75' },
      { value: 1, label: '1' }
    ]

    // 风格选项
    const styles = [
      { name: 'default', label: '默认' },
      { name: 'new-york', label: 'New York' }
    ]

    // 应用颜色主题
    const applyColorTheme = (colorName: string) => {
      const theme = colorThemes[colorName as keyof typeof colorThemes]
      if (!theme) return

      // 移除现有的样式元素
      const existingStyle = document.getElementById('theme-variables')
      if (existingStyle) {
        existingStyle.remove()
      }

      // 创建新的样式元素
      const style = document.createElement('style')
      style.id = 'theme-variables'

      // 生成亮色和暗色模式的 CSS
      const lightColors = theme.light
      const darkColors = theme.dark

      let css = ':root {\n'
      Object.entries(lightColors).forEach(([key, value]) => {
        css += `  --${key}: ${value};\n`
      })
      css += '}\n\n'

      css += '.dark {\n'
      Object.entries(darkColors).forEach(([key, value]) => {
        css += `  --${key}: ${value};\n`
      })
      css += '}\n'

      style.textContent = css
      document.head.appendChild(style)

      // 保存到 localStorage
      localStorage.setItem('color', colorName)
    }

    // 设置颜色
    const setColor = (colorName: string) => {
      selectedColor.value = colorName
      applyColorTheme(colorName)
    }

    // 设置圆角
    const setRadius = (radius: number) => {
      selectedRadius.value = radius
      document.documentElement.style.setProperty('--radius', `${radius}rem`)
      localStorage.setItem('radius', radius.toString())
    }

    // 设置风格
    const setStyle = (styleName: string) => {
      selectedStyle.value = styleName
      localStorage.setItem('style', styleName)
    }

    // 切换主题
    const toggleTheme = (newValue: boolean) => {
      isDark.value = newValue
      if (newValue) {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }
      localStorage.setItem('theme', newValue ? 'dark' : 'light')
      applyColorTheme(selectedColor.value)
    }

    // 重置主题
    const resetTheme = () => {
      selectedColor.value = 'neutral'
      selectedRadius.value = 0.5
      selectedStyle.value = 'new-york'
      isDark.value = false

      document.documentElement.classList.remove('dark')
      document.documentElement.style.setProperty('--radius', '0.5rem')

      localStorage.removeItem('theme')
      localStorage.removeItem('color')
      localStorage.removeItem('radius')
      localStorage.removeItem('style')

      applyColorTheme('neutral')
    }

    // 初始化主题
    const initTheme = () => {
      // 检查系统主题偏好
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      if (!localStorage.getItem('theme')) {
        isDark.value = mediaQuery.matches
        if (mediaQuery.matches) {
          document.documentElement.classList.add('dark')
        }
      }

      // 监听系统主题变化
      mediaQuery.addEventListener('change', (e) => {
        if (!localStorage.getItem('theme')) {
          isDark.value = e.matches
          if (e.matches) {
            document.documentElement.classList.add('dark')
          } else {
            document.documentElement.classList.remove('dark')
          }
          applyColorTheme(selectedColor.value)
        }
      })

      // 应用初始颜色主题
      applyColorTheme(selectedColor.value)
    }

    return {
      // 状态
      selectedColor,
      selectedRadius,
      selectedStyle,
      isDark,
      // 选项
      colors,
      radiusOptions,
      styles,
      // 方法
      setColor,
      setRadius,
      setStyle,
      toggleTheme,
      resetTheme,
      initTheme
    }
  },
  {
    persist: true
  }
)
