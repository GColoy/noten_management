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
      path: '/search',
      name: 'search',
      component: () => import('@/views/search.vue')
    },
    {
      path: '/score/:id',
      name: 'piece',
      component: () => import('@/views/score.vue')
    }
  ]
})

export default router
