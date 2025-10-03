import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: () => import('@/views/index.vue')
    },
    {
      path: '/dashboard',
      name: 'dashboard-alt',
      component: () => import('@/components/Dashboard.vue')
    }
  ]
})

export default router
