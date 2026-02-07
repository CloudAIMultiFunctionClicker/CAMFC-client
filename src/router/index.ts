import { createRouter, createWebHistory } from 'vue-router'
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
    }
    // TODO: 可以在这里添加更多路由，比如设置页面、文件详情页等
  ]
})

// 路由守卫：蓝牙未连接时阻止跳转到其他路由
// 简单粗暴：只要不是首页，就检查蓝牙连接状态
router.beforeEach((to, from, next) => {
  // 如果是首页，直接放行
  if (to.path === '/') {
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
