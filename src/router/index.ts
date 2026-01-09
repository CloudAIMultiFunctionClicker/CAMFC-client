import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

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
      name: 'home',
      component: HomeView  // 首页直接导入，保证首次加载速度
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
    }
    // TODO: 可以在这里添加更多路由，比如设置页面、文件详情页等
  ]
})

// 导出路由实例
export default router
