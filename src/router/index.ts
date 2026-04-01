/**
 * 路由配置
 * 懒加载页面，蓝牙未连接时限制访问
 */

import { createRouter, createWebHistory } from 'vue-router'
// @ts-ignore
import { useBluetoothStore } from '../stores/bluetooth.js'

// 不需要蓝牙连接的页面
const NO_BLUETOOTH_ROUTES = ['/float', '/screenshot', '/notes']

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'initialView',
      component: () => import('../views/InitialView.vue'),
    },
    { path: '/fileView', name: 'fileView', component: () => import('../views/FileView.vue') },
    { path: '/main', name: 'main', component: () => import('../views/Main.vue') },
    { path: '/about', name: 'about', component: () => import('../views/AboutView.vue') },
    { path: '/contact', name: 'contact', component: () => import('../views/ContactView.vue') },
    { path: '/settings', name: 'settings', component: () => import('../views/Settings.vue') },
    { path: '/more-info', name: 'moreInfo', component: () => import('../views/MoreInfo.vue') },
    { path: '/notes', name: 'notes', component: () => import('../views/Notes.vue') },
    { path: '/transfer', name: 'transfer', component: () => import('../views/TransferView.vue') },
    { path: '/float', name: 'float', component: () => import('../views/FloatView.vue') },
    { path: '/screenshot', name: 'screenshot', component: () => import('../views/ScreenshotView.vue') }
  ]
})

// 路由守卫：检查蓝牙连接
router.beforeEach((to, from, next) => {
  // 首页或不需要蓝牙的页面，直接放行
  if (to.path === '/' || NO_BLUETOOTH_ROUTES.includes(to.path)) {
    next()
    return
  }

  const bluetoothStore = useBluetoothStore()
  const connected = bluetoothStore.isConnected()
  
  console.log(`[路由守卫] ${to.path}, 状态：${bluetoothStore.bluetoothStatus}`)
  
  if (connected) {
    next()
    return
  }
  
  // 可能是状态同步延迟，等 200ms 再试
  if (from.path === '/' && bluetoothStore.bluetoothStatus === 'connected') {
    setTimeout(() => {
      if (bluetoothStore.isConnected()) {
        next()
      } else {
        next('/')
      }
    }, 200)
  } else {
    next('/')
  }
})

export default router
