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
    },
    {
      path: '/team',
      name: 'team',
      component: () => import('@/views/Team.vue')
    },
    {
      path: '/projects',
      name: 'projects',
      component: () => import('@/views/Projects.vue')
    },
    {
      path: '/calendar',
      name: 'calendar',
      component: () => import('@/views/Calendar.vue')
    },
    {
      path: '/reports',
      name: 'reports',
      component: () => import('@/views/Reports.vue')
    }
  ]
})

export default router
