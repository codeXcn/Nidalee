import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  // @ts-expect-error 跳过检测
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/forbidden',
      name: 'Forbidden',
      component: () => import('@/views/ForbiddenView.vue')
    },
    {
      path: '/',
      name: 'dashboard',
      component: () => import('../views/DashboardView.vue')
    },
    {
      path: '/game-helper',
      name: 'game-helper',
      component: () => import('../views/GameHelperView.vue')
    },
    {
      path: '/match-analysis',
      name: 'match-analysis',
      component: () => import('../views/MatchAnalysisView.vue')
    },
    {
      path: '/match-search',
      name: 'match-search',
      component: () => import('../views/MatchSearchView.vue')
    },
    {
      path: '/auto-functions',
      name: 'auto-functions',
      component: () => import('../views/AutoFunctionsView.vue')
    },
    {
      path: '/test-api',
      name: 'security',
      component: () => import('../views/TestView.vue')
    },

    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsView.vue')
    },
    {
      path: '/opgg',
      name: 'opgg',
      component: () => import('../views/OpggView.vue')
    }
  ]
})

export default router
