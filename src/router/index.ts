/**
 * CAMFC Client - 路由配置
 * 
 * Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
 * Email: abc.cxh2009@foxmail.com
 *
 * Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
 * Email: 1220594170@qq.com
 *
 * Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
 * Email: admin@mc666.top
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { createRouter, createWebHistory } from 'vue-router'
// @ts-ignore
import { useBluetoothStore } from '../stores/bluetooth.js'

/**
 * 应用路由配置
 * 这是最简陋的路由配置，只包含三个页面
 * 采用懒加载优化包大小，只有首页是直接导入的
 */
const router = createRouter({
  // 使用web history模式，URL看起来更干净（没有#号）
  // 不过需要在生产环境服务器配置rewrite规则
  history: createWebHistory(),
  
  // 路由定义
  routes: [
    {
      path: '/',
      name: 'initialView',
      component: ()=>import('../views/InitialView.vue'),  // 首页直接导入，保证首次加载速度
    },
    {path:'/fileView',
      name: 'fileView',
      component: () => import('../views/FileView.vue')
    },
    {path:'/main',
      name: 'main',
      component: () => import('../views/Main.vue')
    },
    {
      path: '/about',
      name: 'about',
      // 懒加载：关于页面按需加载，减少初始包大小
      // TODO: 如果页面很多，可以考虑分组打包（webpack chunk）
      component: () => import('../views/AboutView.vue')
    },
    {
      path: '/contact',
      name: 'contact',
      // 懒加载：联系页面
      component: () => import('../views/ContactView.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      // 懒加载：联系页面
      component: () => import('../views/Settings.vue')
    },
    // 新增仪表板相关路由
    {
      path: '/hardware-settings',
      name: 'hardwareSettings',
      // 硬件设置占位页面
      // TODO: 这个页面还没具体功能，先放个占位
      component: () => import('../views/HardwareSettings.vue')
    },
    {
      path: '/software-settings',
      name: 'softwareSettings',
      // 软件设置占位页面
      component: () => import('../views/SoftwareSettings.vue')
    },
    {
      path: '/more-info',
      name: 'moreInfo',
      // 更多信息占位页面
      component: () => import('../views/MoreInfo.vue')
    },
    {
      path: '/notes',
      name: 'notes',
      // 笔记页面
      component: () => import('../views/Notes.vue')
    },
    {
      path: '/transfer',
      name: 'transfer',
      // 传输页面
      component: () => import('../views/TransferView.vue')
    },
    {
      path: '/float',
      name: 'float',
      // 悬浮窗页面
      component: () => import('../views/FloatView.vue')
    },
    {
      path: '/screenshot',
      name: 'screenshot',
      // 截图预览页面
      component: () => import('../views/ScreenshotView.vue')
    }
    // TODO: 可以在这里添加更多路由，比如设置页面、文件详情页等
  ]
})

// 路由守卫：蓝牙未连接时阻止跳转到其他路由
// 简单粗暴：只要不是首页，就检查蓝牙连接状态
router.beforeEach((to, _from, next) => {
  // 如果是首页，直接放行
  if (to.path === '/') {
    next()
    return
  }

  // 悬浮窗页面不需要蓝牙连接，直接放行
  if (to.path === '/float') {
    next()
    return
  }

  // 截图页面不需要蓝牙连接，直接放行
  if (to.path === '/screenshot') {
    next()
    return
  }

  // 获取蓝牙store
  const bluetoothStore = useBluetoothStore()

  // 检查蓝牙是否已连接
  if (bluetoothStore.isConnected()) {
    next()
  } else {
    // 未连接，强制跳回首页
    console.warn('蓝牙未连接，阻止跳转到:', to.path)
    next('/')
  }
})

// 导出路由实例
export default router
