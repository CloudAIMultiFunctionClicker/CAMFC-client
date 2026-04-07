/**
 * CAMFC Client - 路由配置
 *
 * 保留所有权利
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
 */

import { createRouter, createWebHistory } from 'vue-router'
// @ts-ignore
import { useBluetoothStore } from '../stores/bluetooth.js'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

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
      path: '/float-normal',
      name: 'float-normal',
      // 悬浮窗页面（普通窗口样式）
      component: () => import('../views/FloatNormalView.vue')
    },
    {
      path: '/screenshot',
      name: 'screenshot',
      // 截图预览页面
      component: () => import('../views/ScreenshotView.vue')
    },
    {
      path: '/note-editor',
      name: 'noteEditor',
      // 笔记编辑窗口页面
      component: () => import('../views/NoteEditorWindow.vue')
    },
    {
      path: '/recent-activities',
      name: 'recentActivities',
      // 最近活动记录页面
      component: () => import('../views/RecentActivities.vue')
    },
    {
      path: '/empty',
      name: 'empty',
      // 空白页面
      component: () => import('../views/Empty.vue')
    },
    {
      path: '/screenshot-window',
      name: 'screenshotWindow',
      // 截图窗口（独立窗口，不受路由守卫影响）
      component: () => import('../views/ScreenshotWindow.vue')
    },
    {
      path: '/float-normal-empty',
      name: 'float-normal-empty',
      // 悬浮窗空白页（独立窗口，不受路由守卫影响）
      component: () => import('../views/FloatNormalEmpty.vue')
    }
    // TODO: 可以在这里添加更多路由，比如设置页面、文件详情页等
  ]
})

// 路由守卫：蓝牙未连接时阻止跳转到其他路由
// 简单粗暴：只要不是首页，就检查蓝牙连接状态
router.beforeEach(async (to, _from, next) => {
  // 获取当前窗口标签
  let windowLabel = ''
  try {
    const currentWindow = await getCurrentWebviewWindow()
    windowLabel = currentWindow?.label || ''
  } catch (e) {
    // 获取失败时按主窗口处理
    console.warn('[路由守卫] 获取窗口标签失败，按主窗口处理:', e)
    windowLabel = 'main'
  }

  console.log(`[路由守卫] 当前窗口：${windowLabel}, 目标路由：${to.path}`)

  // ========== 截图窗口白名单 ==========
  // 截图窗口只允许访问截图相关路由
  if (windowLabel.startsWith('screenshot-')) {
    const allowedPaths = ['/screenshot-window', '/screenshot']
    if (!allowedPaths.includes(to.path)) {
      console.warn(`[路由守卫] 截图窗口禁止访问 ${to.path}，强制跳转到 /screenshot-window`)
      next('/screenshot-window')
      return
    }
    // 截图窗口不需要蓝牙检查，直接放行
    next()
    return
  }

  // ========== 笔记编辑窗口白名单 ==========
  // 笔记编辑窗口只允许访问笔记编辑路由
  if (windowLabel.startsWith('note-editor-')) {
    const allowedPaths = ['/note-editor']
    if (!allowedPaths.includes(to.path)) {
      console.warn(`[路由守卫] 笔记编辑窗口禁止访问 ${to.path}，强制跳转到 /note-editor`)
      next('/note-editor')
      return
    }
    // 笔记编辑窗口不需要蓝牙检查，直接放行
    next()
    return
  }

  // ========== 悬浮窗相关窗口白名单 ==========
  // 悬浮窗页面不需要蓝牙连接，直接放行
  if (to.path === '/float' || to.path === '/float-normal' || to.path === '/float-normal-empty') {
    next()
    return
  }

  // ========== 空白窗口白名单 ==========
  // 空白窗口不需要蓝牙连接，直接放行
  if (to.path === '/empty') {
    next()
    return
  }

  // ========== 主窗口黑名单 ==========
  // 主窗口禁止进入截图展示和笔记编辑路由
  if (windowLabel === 'main') {
    const forbiddenPaths = ['/screenshot-window', '/note-editor', '/screenshot']
    if (forbiddenPaths.includes(to.path)) {
      console.warn(`[路由守卫] 主窗口禁止访问 ${to.path}，强制跳转到 /`)
      next('/')
      return
    }
  }

  // ========== 以下是首页和蓝牙检查逻辑 ==========
  
  // 如果是首页，直接放行
  if (to.path === '/') {
    next()
    return
  }

  // 截图页面不需要蓝牙连接，直接放行
  if (to.path === '/screenshot') {
    next()
    return
  }

  // 笔记页面不需要蓝牙连接，直接放行
  if (to.path === '/notes') {
    next()
    return
  }

  // 笔记编辑窗口不需要蓝牙连接，直接放行
  if (to.path === '/note-editor') {
    next()
    return
  }

  // 截图窗口不需要蓝牙连接，直接放行
  if (to.path === '/screenshot-window') {
    next()
    return
  }

  // 获取蓝牙 store
  const bluetoothStore = useBluetoothStore()

  // 检查蓝牙是否已连接
  const connected = bluetoothStore.isConnected()
  console.log(`[路由守卫] 目标：${to.path}, 蓝牙状态：${bluetoothStore.bluetoothStatus}, 连接：${connected}`)
  
  if (connected) {
    console.log('[路由守卫] 蓝牙已连接，允许跳转')
    next()
  } else {
    // 未连接，但有可能是状态同步延迟
    // 检查是否是从连接页面跳转过来的
    if (_from.path === '/' && bluetoothStore.bluetoothStatus === 'connected') {
      console.log('[路由守卫] 检测到状态同步延迟，等待 200ms 后重试')
      // 等待一小段时间让状态同步完成
      setTimeout(() => {
        const retryConnected = bluetoothStore.isConnected()
        if (retryConnected) {
          console.log('[路由守卫] 重试成功，蓝牙已连接，允许跳转')
          next()
        } else {
          console.warn('[路由守卫] 重试失败，蓝牙未连接，阻止跳转到:', to.path)
          next('/')
        }
      }, 200)
    } else {
      // 未连接，强制跳回首页
      console.warn('[路由守卫] 蓝牙未连接，阻止跳转到:', to.path)
      next('/')
    }
  }
})

// 导出路由实例
export default router
