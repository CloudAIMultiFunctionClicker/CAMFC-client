<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup>
import { ref, provide } from 'vue'

// 主题状态管理 - 简单实现亮色/暗色切换
// 用ref存储当前主题，默认暗色（跟现有一致）
// TODO: 可以持久化到localStorage，下次打开记住选择
const isLightMode = ref(false)

// 切换主题函数
const toggleTheme = () => {
  isLightMode.value = !isLightMode.value
  // 给body添加/移除类名，用于全局样式切换
  if (isLightMode.value) {
    document.body.classList.add('light-mode')
  } else {
    document.body.classList.remove('light-mode')
  }
}

// 把主题状态和切换函数提供给子组件使用
provide('theme', {
  isLightMode,
  toggleTheme
})

// 在组件挂载时设置初始主题
// 这里用onMounted确保在客户端执行
import { onMounted } from 'vue'
onMounted(() => {
  // 初始时确保body有正确的类
  if (isLightMode.value) {
    document.body.classList.add('light-mode')
  } else {
    document.body.classList.remove('light-mode')
  }
})
</script>

<template>
  <!-- router-view用来显示路由组件 -->
  <!-- 整个应用的主题通过body类名控制 -->
  <router-view></router-view>
</template>

<style>
/* 全局主题样式 - 通过body.light-mode类切换 */
/* 暗色主题（默认） */
body {
  --bg-primary: #0f172a;
  --bg-secondary: #1e293b;
  --bg-sidebar: #1e293b;
  --bg-header: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  --text-primary: #f8fafc;
  --text-secondary: #cbd5e1;
  --text-muted: #94a3b8;
  --border-color: rgba(255, 255, 255, 0.1);
  --accent-blue: #3b82f6;
  --accent-blue-rgb: 59, 130, 246; /* RGB值，用于rgba() */
  --accent-red: #dc3545;
  --accent-red-rgb: 220, 53, 69; /* RGB值，用于rgba() */
  --hover-bg: rgba(255, 255, 255, 0.08);
  transition: background-color 0.3s ease, color 0.3s ease;
  /* 平滑过渡效果 */
}

/* 亮色主题 */
body.light-mode {
  --bg-primary: #f8fafc;
  --bg-secondary: #ffffff;
  --bg-sidebar: #ffffff;
  --bg-header: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%);
  --text-primary: #0f172a;
  --text-secondary: #475569;
  --text-muted: #64748b;
  --border-color: rgba(0, 0, 0, 0.1);
  --accent-blue: #2563eb;
  --accent-blue-rgb: 37, 99, 235; /* 亮色模式下的RGB值 */
  --accent-red: #dc2626;
  --accent-red-rgb: 220, 38, 38; /* 亮色模式下的RGB值 */
  --hover-bg: rgba(0, 0, 0, 0.05);
}

/* 应用基础样式 */
body {
  margin: 0;
  padding: 0;
  font-family: system-ui, -apple-system, sans-serif;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

/* 全局滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-secondary);
}

::-webkit-scrollbar-thumb {
  background: var(--text-muted);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>