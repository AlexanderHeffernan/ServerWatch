import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import DashboardView from '../views/DashboardView.vue';
import FileExplorerView from '../views/FileExplorerView.vue';
import SettingsView from '../views/SettingsView.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'dashboard',
    component: DashboardView
  },
  {
    path: '/file-explorer',
    name: 'file-explorer',
    component: FileExplorerView
  },
  {
    path: '/settings',
    name: 'settings',
    component: SettingsView
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router