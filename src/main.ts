import pinia from '@/shared/stores'
import { VueQueryPlugin, QueryClient } from '@tanstack/vue-query'
import App from './App.vue'
import router from './router'
import './style.css'
const app = createApp(App)
const queryClient = new QueryClient()
app.use(VueQueryPlugin, { queryClient })
app.use(pinia)
app.use(router)

app.mount('#app')
