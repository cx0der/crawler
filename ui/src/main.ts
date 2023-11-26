import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

// Have to chain it like this for VueDevtools to recognize pinia!
const app = createApp(App).use(createPinia())

app.use(router)

app.mount('#app')
