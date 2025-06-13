import { createPinia } from 'pinia'
import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import Toast from 'vue-toastification'
import 'vue-toastification/dist/index.css'

import App from './App.vue'
import './assets/main.css'
import routes from './router/index'

// Create router
const router = createRouter({
  history: createWebHistory(),
  routes,
})

// Create Pinia store
const pinia = createPinia()

// Create and mount the app
const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(Toast)

app.mount('#app') 