<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<script setup>
import { ref, provide, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
// 导入 Pinia store 来获取蓝牙状态
import { useBluetoothStore } from './stores/bluetooth.js'

import {showToast} from './components/layout/showToast.js'
import TitleBar from './components/layout/TitleBar.vue'

const route = useRoute()
const router = useRouter()

const isFloatPage = computed(() => route.path === '/float')

// 判断是否在笔记编辑器子窗口
const isNoteEditorPage = computed(() => route.path === '/note-editor')

// 判断是否在空白窗口
const isEmptyPage = computed(() => route.path === '/empty')

// 判断是否需要隐藏标题栏（笔记编辑器和空白窗口）
const shouldHideTitleBar = computed(() => isNoteEditorPage.value || isEmptyPage.value)

// TOTP定时刷新
let totpRefreshInterval = null
const TOTP_REFRESH_INTERVAL = 30000

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
const toggleTheme = async () => {
  isLightMode.value = !isLightMode.value
  updateBodyClass()
  
  localStorage.setItem('theme-preference', isLightMode.value ? 'light' : 'dark')
  
  try {
    const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
    const floatWindow = await WebviewWindow.getByLabel('float')
    if (floatWindow) {
      await floatWindow.emit('theme-changed', isLightMode.value ? 'light' : 'dark')
    }
  } catch (e) {
    console.log('发送主题变化事件失败:', e)
  }
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

// TOTP定时刷新函数
const startTotpRefresh = async () => {
  if (totpRefreshInterval) {
    clearInterval(totpRefreshInterval)
  }

  totpRefreshInterval = setInterval(async () => {
    try {
      const { getTotp } = await import('./components/data/bluetooth')
      await getTotp()
      console.log('[TOTP] 后台TOTP缓存刷新成功')
    } catch (error) {
      console.warn('[TOTP] TOTP缓存刷新失败:', error.message)
    }
  }, TOTP_REFRESH_INTERVAL)

  console.log(`[TOTP] 已启动定时刷新，每${TOTP_REFRESH_INTERVAL / 1000}秒刷新一次`)
}

const stopTotpRefresh = () => {
  if (totpRefreshInterval) {
    clearInterval(totpRefreshInterval)
    totpRefreshInterval = null
    console.log('[TOTP] 已停止TOTP定时刷新')
  }
}

// 监听蓝牙连接状态，启动/停止TOTP刷新
watch(() => bluetoothStore.isConnected(), (connected) => {
  if (connected) {
    console.log('[TOTP] 设备已连接，启动TOTP定时刷新')
    startTotpRefresh()
  } else {
    console.log('[TOTP] 设备已断开，停止TOTP定时刷新')
    stopTotpRefresh()
  }
})

// 在组件挂载时设置初始主题
onMounted(async () => {
  // 初始时确保body有正确的类
  updateBodyClass()
  
  // 先显示窗口，再异步检测服务器
  // 使用setTimeout延迟执行，让窗口先渲染出来
  setTimeout(async () => {
    await initBackendConfig()
  }, 100)
  
  // 蓝牙按键事件监听器引用
  let buttonEventUnlisten = null
  
  // 蓝牙断开事件监听器引用
  let bluetoothDisconnectUnlisten = null
  
  // 导航事件监听器引用
  let navigateEventUnlisten = null
  
  // 监听蓝牙按键事件
  const { listen } = await import('@tauri-apps/api/event')
  buttonEventUnlisten = await listen('button-event', async (event) => {
    // 悬浮窗不处理按键事件
    if (route.path === '/float') {
      return
    }
    
    const eventType = event.payload.event_type
    
    // GPIO10 处理 -> 右箭头
    if (eventType === 'button_press') {
      showToast('GPIO10 按下', '#3b82f6')
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: true } }))
    } else if (eventType === 'button_release') {
      showToast('GPIO10 松开', '#10b981')
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: false } }))
      // 模拟右箭头键
      try {
        const { pressWinKey } = await import('./components/data/bluetooth')
        await pressWinKey()
        console.log('右箭头键模拟成功')
      } catch (e) {
        console.error('右箭头键模拟失败:', e)
      }
    }
    
    // GPIO9 处理 -> 左箭头
    else if (eventType === 'button_press_left') {
      showToast('GPIO9 按下', '#8b5cf6')
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: true } }))
    } else if (eventType === 'button_release_left') {
      showToast('GPIO9 松开', '#f59e0b')
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: false } }))
      // 模拟左箭头键
      try {
        const { pressLeftKey } = await import('./components/data/bluetooth')
        await pressLeftKey()
        console.log('左箭头键模拟成功')
      } catch (e) {
        console.error('左箭头键模拟失败:', e)
      }
    }
  })
  
  // 监听截图命令（0x12）- 只在非悬浮窗页面显示 toast
  const screenshotUnlisten = await listen('screenshot-command', async () => {
    if (route.path === '/float') {
      return
    }
    console.log('收到截图命令（0x12）')
    showToast('触发截图', '#3b82f6')
  })
  
  // 监听显示主窗口 + 新建笔记命令（0x10）
  const showNoteUnlisten = await listen('show-note-command', async () => {
    if (route.path === '/float') {
      return
    }
    console.log('收到显示主窗口 + 新建笔记命令（0x10）')
    showToast('新建笔记', '#10b981')
    // 显示主窗口并直接打开新建笔记窗口
    try {
      const { Window } = await import('@tauri-apps/api/window')
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        await mainWindow.show()
        await mainWindow.unminimize()
        await mainWindow.setFocus()
      }
      
      // 直接创建并打开新笔记（类似点击"新建笔记"按钮）
      const uuid = crypto.randomUUID()
      const now = new Date()
      const timestamp = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}${String(now.getSeconds()).padStart(2, '0')}`
      const defaultTitle = `未命名笔记_${timestamp}`
      
      const url = `/note-editor?uuid=${uuid}&title=${encodeURIComponent(defaultTitle)}`
      const windowLabel = `note-editor-${uuid}`
      
      const webview = new WebviewWindow(windowLabel, {
        url: url,
        title: defaultTitle,
        width: 900,
        height: 600,
        minWidth: 400,
        minHeight: 300,
        center: true,
        decorations: false,
        resizable: true
      })
      
      webview.once('tauri://created', () => {
        console.log('笔记编辑窗口创建成功:', windowLabel)
      })
      
      webview.once('tauri://error', (e) => {
        console.error('笔记编辑窗口创建失败:', e)
        showToast('打开编辑窗口失败', '#ef4444')
      })
    } catch (e) {
      console.error('新建笔记失败:', e)
    }
  })
  
  // 监听打开云盘页面命令（0x08）
  const openCloudUnlisten = await listen('open-cloud-command', async () => {
    if (route.path === '/float') {
      return
    }
    console.log('收到打开云盘命令（0x08）')
    showToast('打开云盘', '#8b5cf6')
    // 显示主窗口并导航到云盘页面
    try {
      const { Window } = await import('@tauri-apps/api/window')
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        await mainWindow.show()
        await mainWindow.unminimize()
        await mainWindow.setFocus()
      }
      // 发送导航事件
      const webview = await WebviewWindow.getByLabel('main')
      if (webview) {
        await webview.emit('navigate', '/fileView')
      }
    } catch (e) {
      console.error('打开云盘页面失败:', e)
    }
  })
  
  // 定时检查蓝牙连接状态（每 10 秒）
  let connectionCheckInterval = null
  
  // 蓝牙断开提示框是否显示中，避免重复显示
  let isShowingDisconnectDialog = false
  
  const checkConnectionStatus = async () => {
    try {
      // 动态导入蓝牙模块
      const { isConnected } = await import('./components/data/bluetooth')
      const connected = await isConnected()
      
      // 如果从已连接变为未连接，跳转到初始页面
      if (bluetoothStore.isConnected() && !connected) {
        console.log('蓝牙连接已断开，准备显示提示')
        
        // 避免重复显示对话框
        if (isShowingDisconnectDialog) {
          console.log('对话框已在显示中，跳过')
          return
        }
        
        isShowingDisconnectDialog = true
        
        // 重置状态
        bluetoothStore.reset()
        
        // 显示确认对话框
        const userConfirmed = await showDisconnectConfirm()
        
        if (userConfirmed) {
          console.log('用户确认断开，刷新页面')
          // 刷新整个页面
          window.location.reload()
        }
      }
    } catch (error) {
      // 静默处理错误，避免刷屏
      console.log('检查连接状态出错:', error)
    }
  }
  
  // 监听蓝牙断开事件（实时检测）
  bluetoothDisconnectUnlisten = await listen('bluetooth-disconnect', async () => {
    console.log('收到蓝牙断开事件')
    
    // 如果当前是已连接状态，显示提示
    if (bluetoothStore.isConnected()) {
      console.log('蓝牙设备已断开，准备显示提示')
      
      // 避免重复显示对话框
      if (isShowingDisconnectDialog) {
        console.log('对话框已在显示中，跳过')
        return
      }
      
      isShowingDisconnectDialog = true
      
      // 重置状态
      bluetoothStore.reset()
      
      // 显示确认对话框
      const userConfirmed = await showDisconnectConfirm()
      
      if (userConfirmed) {
        console.log('用户确认断开，刷新页面')
        // 刷新整个页面
        window.location.reload()
      }
    }
  })
  
  // 显示蓝牙断开确认对话框
  const showDisconnectConfirm = async () => {
    return new Promise((resolve) => {
      // 创建对话框
      const dialog = document.createElement('div')
      dialog.className = 'disconnect-dialog'
      dialog.innerHTML = `
        <div class="disconnect-dialog-content">
          <h3>设备已断开连接</h3>
          <p>当前蓝牙设备已断开，点击确认后重新连接</p>
          <div class="disconnect-dialog-actions">
            <button class="disconnect-btn confirm">确认</button>
          </div>
        </div>
      `
      
      // 添加样式
      const style = document.createElement('style')
      style.textContent = `
        .disconnect-dialog {
          position: fixed;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background-color: rgba(0, 0, 0, 0.6);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 99999;
          backdrop-filter: blur(4px);
        }
        
        .disconnect-dialog-content {
  background-color: var(--bg-secondary, #1e293b);
  border-radius: .375rem;
  padding: 32px;
  max-width: 400px;
  text-align: center;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}
        
        .disconnect-dialog-content h3 {
          font-size: 20px;
          font-weight: 600;
          color: var(--text-primary, #f1f5f9);
          margin: 0 0 12px 0;
        }
        
        .disconnect-dialog-content p {
          font-size: 14px;
          color: var(--text-secondary, #94a3b8);
          margin: 0 0 24px 0;
        }
        
        .disconnect-dialog-actions {
          display: flex;
          gap: 12px;
          justify-content: center;
        }
        
        .disconnect-btn {
          padding: 10px 32px;
          border-radius: .375rem;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
          transition: all 0.2s;
          border: none;
        }
        
        .disconnect-btn.confirm {
          background-color: var(--accent-blue, #3b82f6);
          color: #fff;
        }
        
        .disconnect-btn.confirm:hover {
          background-color: #2563eb;
        }
      `
      
      document.head.appendChild(style)
      document.body.appendChild(dialog)
      
      // 绑定按钮事件
      const confirmBtn = dialog.querySelector('.confirm')
      
      const closeDialog = () => {
        dialog.remove()
        style.remove()
      }
      
      confirmBtn.addEventListener('click', () => {
        closeDialog()
        resolve(true)
      })
    })
  }
  
  // 开始定时检查（每 2 秒，快速响应断开）
  connectionCheckInterval = setInterval(checkConnectionStatus, 2000)
  
  // 监听系统主题变化，如果用户没有手动设置过，就跟着系统变
  const lightMediaQuery = window.matchMedia('(prefers-color-scheme: light)')
  
  const handleSystemThemeChange = (e) => {
    const hasUserPreference = localStorage.getItem('theme-preference') !== null
    if (!hasUserPreference) {
      isLightMode.value = e.matches
      updateBodyClass()
    }
  }
  
  lightMediaQuery.addEventListener('change', handleSystemThemeChange)
  
  // 监听悬浮窗发来的导航事件
  try {
    navigateEventUnlisten = await listen('navigate', (event) => {
      console.log('收到导航事件:', event.payload)
      const path = event.payload
      // 只在非悬浮窗页面时跳转
      if (path && router && route.path !== '/float') {
        router.push(path)
      }
    })
  } catch (e) {
    console.log('监听导航事件失败（非Tauri环境）:', e)
  }
  
  // 监听悬浮窗发来的主题查询请求
  try {
    await listen('get-theme', async () => {
      const floatWindow = await WebviewWindow.getByLabel('float')
      if (floatWindow) {
        await floatWindow.emit('theme-changed', isLightMode.value ? 'light' : 'dark')
      }
    })
  } catch (e) {
    console.log('监听主题查询事件失败:', e)
  }
  
  // 在组件卸载时清理监听器
  onUnmounted(() => {
    // 停止TOTP定时刷新
    stopTotpRefresh()
    
    lightMediaQuery.removeEventListener('change', handleSystemThemeChange)
    if (connectionCheckInterval) {
      clearInterval(connectionCheckInterval)
    }
    if (buttonEventUnlisten) {
      buttonEventUnlisten()
    }
    if (bluetoothDisconnectUnlisten) {
      bluetoothDisconnectUnlisten()
    }
    if (navigateEventUnlisten) {
      navigateEventUnlisten()
    }
    if (screenshotUnlisten) {
      screenshotUnlisten()
    }
    if (showNoteUnlisten) {
      showNoteUnlisten()
    }
    if (openCloudUnlisten) {
      openCloudUnlisten()
    }
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
  <!-- router-view 用来显示路由组件 -->
  <!-- 整个应用的主题通过 body 类名控制 -->
  <div class="app-container" v-if="!isFloatPage">
    <!-- 自定义顶栏 -->
    <TitleBar v-if="!shouldHideTitleBar" />
    <div class="main-content" :style="shouldHideTitleBar ? 'padding-top: 0;' : ''">
      <router-view></router-view>
    </div>
  </div>
  <router-view v-else></router-view>
</template>

<style>
/* 全局主题样式 - 优化后的深色主题配色 */
body {
  /* 背景色 - 使用纯黑色/深黑色 */
  --bg-primary: #000000;
  --bg-secondary: #0d0d0d;
  --bg-sidebar: #0d0d0d;
  --bg-header: #0d0d0d;
  --bg-tertiary: #1a1a1a;
  
  /* 文字色 - 高可读性 */
  --text-primary: #f0f6fc;
  --text-secondary: #c9d1d9;
  --text-muted: #8b949e;
  
  /* 边框色 - 低对比度 */
  --border-color: #30363d;
  
  /* 强调色 - 根据图片配色 */
  --accent-blue: #3178c6;
  --accent-blue-rgb: 49, 120, 198;
  --accent-blue-bright: #1f6feb;
  --accent-blue-dark: #3572a5;
  --accent-green: #3fb950;
  --accent-green-rgb: 63, 185, 80;
  --accent-red: #f85149;
  --accent-red-rgb: 248, 81, 73;
  --accent-purple: #bc8cff;
  --accent-yellow: #d29922;
  
  /* 交互色 */
  --hover-bg: rgba(255, 255, 255, 0.08);
  --selected-bg: rgba(255, 255, 255, 0.12);
  --input-bg: #000000;
  
  /* 警告按钮配色（暗色模式） */
  --danger-btn-bg: #212830;
  --danger-btn-text: #f85149;
  --danger-btn-border: rgba(248, 81, 73, 0.4);
  --danger-btn-hover-bg: #f85149;
  --danger-btn-hover-text: #ffffff;
  --danger-btn-hover-border: #f85149;
  
  transition: background-color 0.3s ease, color 0.3s ease;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

input, textarea, [contenteditable="true"] {
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  user-select: text;
}

/* 亮色主题（GitHub 风格） */
body.light-mode {
  --bg-primary: #ffffff;
  --bg-secondary: #ffffff;
  --bg-sidebar: #ffffff;
  --bg-header: #f6f8fa;
  --bg-tertiary: #f6f8fa;
  --text-primary: #24292f;
  --text-secondary: #57606a;
  --text-muted: #8c959f;
  --border-color: #d0d7de;
  --accent-blue: #0969da;
  --accent-blue-rgb: 9, 105, 218;
  --accent-blue-bright: #0550ae;
  --accent-blue-dark: #0a3069;
  --accent-green: #2da44e;
  --accent-green-rgb: 45, 164, 78;
  --accent-red: #cf222e;
  --accent-red-rgb: 207, 34, 46;
  --accent-purple: #8250df;
  --accent-yellow: #9a6700;
  --hover-bg: #f3f4f6;
  --selected-bg: #ddf4ff;
  --input-bg: #ffffff;
  --danger-bg: #ffebe9;
  --danger-border: #ffcccc;
  
  /* 警告按钮配色（亮色模式） */
  --danger-btn-bg: #f6f8fa;
  --danger-btn-text: #cf222e;
  --danger-btn-border: rgba(207, 34, 46, 0.4);
  --danger-btn-hover-bg: #cf222e;
  --danger-btn-hover-text: #ffffff;
  --danger-btn-hover-border: #cf222e;
}

/* 应用基础样式 */
body {
  margin: 0;
  padding: 0;
  font-family: system-ui, -apple-system, sans-serif;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

/* 应用容器布局 - 固定标题栏，内容可滚动 */
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

/* 主内容区域 - 支持滚动，留出标题栏空间 */
.main-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding-top: 48px; /* 留出标题栏高度 */
}

/* 全局滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-secondary, #161b22);
}

::-webkit-scrollbar-thumb {
  background: var(--border-color, #30363d);
  border-radius: .375rem;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted, #8b949e);
}

/* 全局警告按钮样式 - GitHub 风格 */
.btn-danger {
  background-color: var(--danger-btn-bg, #212830);
  color: var(--danger-btn-text, #f85149);
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
}

.btn-danger:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, #ffffff);
  border-color: var(--danger-btn-hover-border, #f85149);
}

.btn-danger:active {
  transform: scale(0.98);
}

/* 警告按钮内的图标 - 确保对比度 */
.btn-danger i,
.btn-danger svg {
  color: inherit;
}
</style>
