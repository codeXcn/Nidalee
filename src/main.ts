import pinia from '@/stores'
import App from './App.vue'
import router from './router'
import './style.css'
const app = createApp(App)

app.use(pinia)
app.use(router)

app.mount('#app')
