/*
 * Copyright (C) 2026 Jiale Xu (ANTmmmmm) (ant-cave)
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import TrashView from '@/views/TrashView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      meta: {
        title: '全部文件'
      }
    },
    {
      path: '/trash',
      name: 'trash',
      component: TrashView,
      meta: {
        title: '回收站'
      }
    },
    {
      path: '/recent',
      name: 'recent',
      component: HomeView,
      meta: {
        title: '最近文件'
      }
    },
    {
      path: '/starred',
      name: 'starred',
      component: HomeView,
      meta: {
        title: '收藏'
      }
    },
    {
      path: '/shared',
      name: 'shared',
      component: HomeView,
      meta: {
        title: '共享文件'
      }
    },
    {
      path: '/folder/:path*',
      name: 'folder',
      component: HomeView,
      meta: {
        title: '文件夹'
      }
    },
    // 404 页面
    {
      path: '/:pathMatch(.*)*',
      redirect: '/'
    }
  ]
})

// 路由守卫：更新页面标题
router.beforeEach((to, from, next) => {
  const title = to.meta.title as string || '云盘'
  document.title = `${title} - CAMFC Cloud`
  next()
})

export default router
