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
import { ref, provide, onMounted, onUnmounted } from 'vue'
// 导入Pinia store来获取蓝牙状态
import { useBluetoothStore } from './stores/bluetooth.js'

import {showToast} from './components/layout/showToast.js'

// 导入应用头部组件
import AppHeader from './components/layout/AppHeader.vue'

// 导入后端配置初始化函数
import { initBackendConfig } from './config/backend.js'

// 注意：现在不直接导入蓝牙函数了
// 根据计划，除了bluetooth.js中，其他地方不要调用TOTP有关函数
// 通过Pinia store获取数据

// 处理Ctrl+R等快捷键
document.addEventListener('keydown', (e) => {
  if (e.ctrlKey && (e.key === 'r' || e.key === 'p'|| e.key === 'h'|| e.key === 'z' || e.key === 'f')) {
    e.preventDefault(); // 阻止浏览器默认行为
  }
});



// 主题状态管理 - 默认跟随系统配色
// 先尝试从localStorage读取用户之前的选择
// 如果没保存过，就检测系统偏好
const getInitialTheme = () => {
  // 先看看localStorage有没有保存用户的选择
  const savedTheme = localStorage.getItem('theme-preference')
  if (savedTheme === 'light' || savedTheme === 'dark') {
    return savedTheme === 'light'
  }
  
  // 没有保存过的话，检测系统偏好
  // 优先检测用户明确设置的系统主题
  // matchMedia返回的是MediaQueryList对象，matches属性表示是否匹配
  const prefersLight = window.matchMedia('(prefers-color-scheme: light)').matches
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  
  // 如果系统明确设置了亮色主题，就用亮色
  if (prefersLight) {
    return true
  }
  
  // 如果系统明确设置了暗色主题，就用暗色
  // 注意：有些浏览器可能同时返回false（比如no-preference），那我们就默认暗色
  // 之前想过默认暗色会不会不友好？但项目原来就是暗色主题，保持一致性吧
  return false
}

const isLightMode = ref(getInitialTheme())

// 切换主题函数
const toggleTheme = () => {
  isLightMode.value = !isLightMode.value
  // 给body添加/移除类名，用于全局样式切换
  updateBodyClass()
  
  // 保存用户选择到localStorage
  // 存为字符串，方便下次读取
  localStorage.setItem('theme-preference', isLightMode.value ? 'light' : 'dark')
}

// 更新body类名的辅助函数
const updateBodyClass = () => {
  if (isLightMode.value) {
    document.body.classList.add('light-mode')
  } else {
    document.body.classList.remove('light-mode')
  }
}

// 注意：现在不直接调用蓝牙函数了，通过Pinia store管理状态
// InitialView.vue会处理蓝牙连接和TOTP获取
// 这里只提供基础的工具函数，如果需要的话

// 创建bluetooth store实例
const bluetoothStore = useBluetoothStore()

// 扫描蓝牙设备的函数（保留兼容性，但通过store状态反馈）
// 这个函数现在主要给其他组件用，如果它们需要手动扫描
const scanBluetooth = async () => {
  try {
    showToast('开始扫描蓝牙设备...')
    // 动态导入蓝牙模块，避免循环依赖
    const { scanDevices, findCpenDevices } = await import('./components/data/bluetooth')
    const devices = await scanDevices()
    const cpenDevices = findCpenDevices(devices)
    
    showToast(`扫描完成，发现 ${devices.length} 个设备，其中 ${cpenDevices.length} 个Cpen设备`)
    
    // 如果发现Cpen设备，可以尝试自动连接（可选）
    // 但根据设计，连接应该由InitialView.vue处理
    if (cpenDevices.length > 0) {
      showToast(`发现Cpen设备: ${cpenDevices[0].displayInfo}`)
    }
    
    return { devices, cpenDevices }
  } catch (error) {
    console.error('蓝牙扫描失败:', error)
    showToast('蓝牙扫描失败')
    return { devices: [], cpenDevices: [] }
  }
}

// 把主题状态和切换函数提供给子组件使用
provide('theme', {
  isLightMode,
  toggleTheme
})

// 在组件挂载时设置初始主题
onMounted(async () => {
  // 初始时确保body有正确的类
  updateBodyClass()
  
  // 初始化后端配置（只会在应用启动时调用一次）
  await initBackendConfig()
  
  // 监听系统主题变化，如果用户没有手动设置过，就跟着系统变
  // 这里监听亮色主题的变化，因为我们的逻辑是基于亮色/暗色来判断的
  const lightMediaQuery = window.matchMedia('(prefers-color-scheme: light)')
  
  const handleSystemThemeChange = (e) => {
    // 只有当用户没有保存过偏好时才跟随系统变化
    // 检查localStorage里有没有保存过主题偏好
    const hasUserPreference = localStorage.getItem('theme-preference') !== null
    if (!hasUserPreference) {
      // e.matches为true表示现在系统是亮色主题
      isLightMode.value = e.matches
      updateBodyClass()
      
      // 这里有个问题：如果系统从light变成dark，e.matches就是false
      // 但如果系统从dark变成light，e.matches就是true
      // 我们的逻辑应该没问题，因为getInitialTheme里也是用light匹配来判断
    }
  }
  
  // 添加监听
  lightMediaQuery.addEventListener('change', handleSystemThemeChange)
  
  // 在组件卸载时清理监听器
  onUnmounted(() => {
    lightMediaQuery.removeEventListener('change', handleSystemThemeChange)
  })
  
// 窗口启动后，不再自动连接Cpen设备
// 因为InitialView.vue现在是专门的连接界面，它会处理连接
// 这里只显示启动提示
setTimeout(() => {
  console.log('应用启动完成，InitialView将处理蓝牙连接')
  // 可以显示一个简单的启动提示
  // showToast('CAMFC客户端已启动')
}, 1000)
})
</script>

<template>
  <!-- router-view用来显示路由组件 -->
  <!-- 整个应用的主题通过body类名控制 -->
  <AppHeader/>

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
