import type { RouteRecordRaw } from 'vue-router'

// Import views
import Assets from '@/views/Assets.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Assets',
    component: Assets,
    meta: {
      title: 'Assets'
    }
  }
]

export default routes 