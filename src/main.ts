import { createApp } from 'vue'
import App from './App.vue'
import './index.css'

// 导入 Pinia
import { pinia } from '@/stores'

// 初始化主题

const app = createApp(App)

// 注册 Pinia
app.use(pinia)

app.mount('#app')

// 应用挂载后初始化主题
import { useThemeStore } from '@/stores/useThemeStore'
const themeStore = useThemeStore()
themeStore.initializeTheme()
