import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '../views/DashboardView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: DashboardView
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
      path: '/auto-functions',
      name: 'auto-functions',
      component: () => import('../views/AutoFunctionsView.vue')
    },
    {
      path: '/security',
      name: 'security',
      component: () => import('../views/SecurityView.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsView.vue')
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue')
    }
  ]
})

export default router
