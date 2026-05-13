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

<template>
  <div class="float-container" :style="themeVars">
    <span class="float-title">CAMFC Cloud</span>
    <span
      class="connection-status"
      :class="{ connected: isConnected }"
      @click="handleConnectionClick"
    >
      {{ isConnected ? '已连接' : '未连接' }}
    </span>
    <div class="float-buttons">
      <button class="float-btn" @click.stop="openMainPage('/fileView')" title="云盘">
        <i class="ri-cloud-line"></i>
      </button>
      <button class="float-btn note-btn" @click.stop="handleNoteManager" title="笔记">
        <i class="ri-sticky-note-line"></i>
      </button>
      <button class="float-btn screenshot-btn" @click.stop="handleScreenshot" title="截图">
        <i class="ri-screenshot-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/settings')" title="设置">
        <i class="ri-settings-3-line"></i>
      </button>
      <!-- <button v-if="!isMainWindowVisible" class="float-btn open-main-btn" @click.stop="openMainWindow" title="打开主窗口">
        <i class="ri-home-2-line"></i>
        <span class="btn-text">主窗口</span>
      </button> -->
    </div>





    <!-- 未连接提示框 -->
    <Transition name="tip-fade">
      <div v-if="showConnectTip" class="connect-tip">
        <i class="ri-information-line"></i>
        <span>请先连接设备</span>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, onBeforeUnmount, computed } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getCurrentWindow, Window } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData } from '../components/data/storage.js'

const isConnected = ref(false)
const isMainWindowVisible = ref(true)
const showConnectTip = ref(false)
let connectTipTimer = null

const getInitialTheme = () => {
  const savedTheme = localStorage.getItem('theme-preference')
  if (savedTheme === 'light') return true
  if (savedTheme === 'dark') return false
  return window.matchMedia('(prefers-color-scheme: light)').matches
}

const isLightMode = ref(getInitialTheme())

const themeVars = computed(() => ({
  '--float-bg': isLightMode.value ? '#f5f5f5' : '#0d0d0d',
  '--float-text': isLightMode.value ? '#333' : '#e0e0e0',
  '--float-btn-color': isLightMode.value ? '#666' : '#a0a0a0',
  '--float-btn-hover-bg': isLightMode.value ? 'rgba(0, 0, 0, 0.06)' : 'rgba(255, 255, 255, 0.1)',
  '--float-btn-hover-color': isLightMode.value ? '#333' : '#fff',
  '--float-menu-bg': isLightMode.value ? 'white' : '#1a1a1a',
  '--float-menu-text': isLightMode.value ? '#333' : '#e0e0e0',
  '--float-menu-border': isLightMode.value ? '#e5e5e5' : '#333333',
  '--float-tip-bg': isLightMode.value ? '#ffffff' : '#1a1a1a',
}))





let visibilityCheckInterval = null
let themeCheckInterval = null
let unlistenTheme = null
let unlistenConnection = null

onMounted(async () => {
  console.log('FloatNormalView mounted')
  
  unlistenTheme = await listen('theme-changed', (event) => {
    const newTheme = event.payload
    isLightMode.value = newTheme === 'light'
    localStorage.setItem('theme-preference', newTheme)
  })

  unlistenConnection = await listen('connection-status', (event) => {
    console.log('收到连接状态事件:', event.payload)
    isConnected.value = event.payload
  })

  // 监听截图命令（来自 0x12 按键）
  const unlistenScreenshot = await listen('screenshot-command', async () => {
    console.log('悬浮窗收到截图命令（0x12）')
    // 直接触发截图
    await handleScreenshot()
  })

  const checkMainWindowTheme = async () => {
    try {
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        const webview = await WebviewWindow.getByLabel('main')
        if (webview) {
          await webview.emit('get-theme')
        }
      }
    } catch (e) {
      console.error('检查主窗口主题失败:', e)
    }
  }

  checkMainWindowTheme()
  themeCheckInterval = setInterval(checkMainWindowTheme, 2000)

  // 检查主窗口可见性状态
  const checkMainWindowVisibility = async () => {
    try {
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        isMainWindowVisible.value = await mainWindow.isVisible()
      } else {
        isMainWindowVisible.value = false
      }
    } catch (e) {
      console.error('检查主窗口可见性失败:', e)
      isMainWindowVisible.value = false
    }
  }

  // 初始检查
  await checkMainWindowVisibility()

  // 定期检查主窗口可见性（每500毫秒检查一次，响应更快）
  visibilityCheckInterval = setInterval(async () => {
    await checkMainWindowVisibility()
  }, 500)

  onUnmounted(() => {
    if (unlistenTheme) unlistenTheme()
    if (unlistenConnection) unlistenConnection()
    if (unlistenScreenshot) unlistenScreenshot()
    if (visibilityCheckInterval) {
      clearInterval(visibilityCheckInterval)
    }
    if (themeCheckInterval) {
      clearInterval(themeCheckInterval)
    }
  })
})

function handleConnectionClick() {
  if (!isConnected.value) {
    openMainPage('/')
  }
}

// 普通窗口不需要自定义拖拽（有窗口装饰，可以通过标题栏拖动）
// function startDrag(e) {
//   // 排除所有可点击元素，包括按钮、菜单项等
//   if (e.target.closest('.float-btn') || 
//       e.target.closest('.connection-status') ||
//       e.target.closest('.menu-item') ||
//       e.target.closest('.screenshot-menu')) {
//     return
//   }
//   try {
//     const floatWindow = await getCurrentWindow()
//     await floatWindow.startDragging()
//   } catch (e) {
//     console.error('拖动失败:', e)
//   }
// }

/**
 * 显示未连接提示
 */
function showTip() {
  showConnectTip.value = true
  
  // 清除之前的定时器
  if (connectTipTimer) {
    clearTimeout(connectTipTimer)
  }
  
  // 1 秒后自动关闭
  connectTipTimer = setTimeout(() => {
    hideTip()
  }, 1000)
}

/**
 * 隐藏提示
 */
function hideTip() {
  showConnectTip.value = false
  if (connectTipTimer) {
    clearTimeout(connectTipTimer)
    connectTipTimer = null
  }
}

/**
 * 打开主页面（检查连接状态）
 */
async function openMainPage(path) {
  console.log('点击按钮，目标是:', path)

  // 检查是否需要连接设备（云盘、设置、笔记管理需要连接）
  const needConnection = ['/fileView', '/settings', '/notes'].includes(path)
  if (needConnection && !isConnected.value) {
    console.log('设备未连接，显示提示并打开首页')
    showTip()
    // 自动打开首页让用户连接设备
    path = '/'
  }

  try {
    // 使用 Window.getByLabel 获取主窗口
    const mainWindow = await Window.getByLabel('main')

    // 检查主窗口是否存在且没有被关闭
    if (mainWindow) {
      try {
        // 检查窗口是否可见（包括是否在托盘）
        const isVisible = await mainWindow.isVisible()
        
        if (isVisible) {
          console.log('主窗口可见，聚焦并导航')
          // 取消最小化（如果窗口被最小化）
          await mainWindow.unminimize()
          // 将窗口提到前台并聚焦
          await mainWindow.show()
          await mainWindow.center()
          await mainWindow.setFocus()
          // 发送导航事件
          const webview = await WebviewWindow.getByLabel('main')
          if (webview) {
            await webview.emit('navigate', path)
          }
          console.log('发送导航事件:', path)
        } else {
          // 窗口存在但不可见（可能在托盘），显示窗口
          console.log('主窗口在托盘，显示并聚焦')
          await mainWindow.show()
          await mainWindow.unminimize()
          await mainWindow.center()
          await mainWindow.setFocus()
          // 发送导航事件
          const webview = await WebviewWindow.getByLabel('main')
          if (webview) {
            await webview.emit('navigate', path)
          }
          console.log('显示窗口并发送导航事件:', path)
        }
      } catch (windowError) {
        // 窗口已关闭或出错，需要重新创建
        console.log('主窗口出错，重新创建:', windowError)
        await createMainWindow(path)
      }
    } else {
      console.log('主窗口不存在，创建新窗口')
      await createMainWindow(path)
    }

  } catch (e) {
    console.error('打开主窗口失败:', e)
    alert('打开主窗口失败: ' + e)
  }
}

// 创建主窗口的辅助函数
async function createMainWindow(path) {
  console.log('创建新主窗口，路径:', path)
  const webview = new WebviewWindow('main', {
    url: path,
    title: 'CAMFC Cloud',
    width: 1152,
    height: 648,
    center: true
  })

  webview.once('tauri://created', () => {
    console.log('主窗口创建成功')
  })

  webview.once('tauri://error', (e) => {
    console.error('主窗口创建失败:', e)
  })
}

/**
 * 处理屏幕截图
 * 根据设置决定是否隐藏主窗口
 */
async function handleScreenshot() {
  console.log('开始截图流程')

  try {
    // 从设置中读取是否隐藏主窗口
    const hideWindowSetting = await loadAppData('screenshot_hide_window')
    const shouldHideWindow = hideWindowSetting ? JSON.parse(hideWindowSetting) : false

    // 获取主窗口
    const mainWindow = await Window.getByLabel('main')
    let wasVisible = false

    // 如果设置要求隐藏主窗口且主窗口存在且可见，先隐藏它
    if (shouldHideWindow && mainWindow) {
      wasVisible = await mainWindow.isVisible()
      if (wasVisible) {
        console.log('隐藏主窗口以便截图')
        await mainWindow.hide()
        // 等待一段时间确保窗口完全隐藏
        await new Promise(resolve => setTimeout(resolve, 300))
      }
    }

    // 调用截图命令
    console.log('执行截图')
    const result = await invoke('capture_screen')

    if (result.success) {
      console.log('截图成功，打开截图窗口')
      // 截图成功，打开截图窗口
      await openScreenshotWindow(result)
    } else {
      console.error('截图失败:', result.error)
      // 截图失败，如果之前窗口是可见的且被隐藏了，恢复显示
      if (wasVisible && mainWindow) {
        await mainWindow.show()
      }
    }
  } catch (e) {
    console.error('截图过程出错:', e)
  }
}

/**
 * 打开截图窗口显示截图结果
 */
async function openScreenshotWindow(screenshotData) {
  try {
    console.log('创建独立的截图窗口')
    
    const screenshotWindowLabel = `screenshot-${Date.now()}`
    
    const screenshotWindow = new WebviewWindow(screenshotWindowLabel, {
      url: '/screenshot-window',
      title: '截图',
      width: 1152,
      height: 648,
      center: true,
      decorations: false,
      resizable: true
    })

    screenshotWindow.once('tauri://created', async () => {
      console.log('截图窗口创建成功')
      
      // 等待窗口完全加载
      await new Promise(resolve => setTimeout(resolve, 500))
      
      // 检查窗口是否仍然存在
      try {
        const windowExists = await Window.getByLabel(screenshotWindowLabel)
        if (windowExists) {
          console.log('发送截图数据')
          await screenshotWindow.emit('screenshot-data', screenshotData)
          console.log('截图数据已发送')
        }
      } catch (e) {
        console.error('检查或发送数据失败:', e)
      }
    })

    screenshotWindow.once('tauri://error', (e) => {
      console.error('截图窗口创建失败:', e)
    })
    
    console.log('截图窗口已创建，标签:', screenshotWindowLabel)
  } catch (e) {
    console.error('打开截图窗口失败:', e)
  }
}

/**
 * 处理笔记管理
 * 打开主窗口的笔记管理页面
 */
async function handleNoteManager() {
  console.log('打开笔记管理')
  await openMainPage('/notes')
}

/**
 * 打开主窗口
 * 专门用于显示主窗口，不指定具体页面路径
 */
async function openMainWindow() {
  console.log('打开主窗口')

  try {
    // 使用 Window.getByLabel 获取主窗口
    const mainWindow = await Window.getByLabel('main')

    if (mainWindow) {
      try {
        // 检查窗口是否可见
        const isVisible = await mainWindow.isVisible()

        if (isVisible) {
          console.log('主窗口已可见，聚焦')
          // 取消最小化（如果窗口被最小化）
          await mainWindow.unminimize()
          // 将窗口提到前台并聚焦
          await mainWindow.show()
          await mainWindow.center()
          await mainWindow.setFocus()
        } else {
          // 窗口存在但不可见（可能在托盘），显示窗口
          console.log('主窗口在托盘，显示并聚焦')
          await mainWindow.show()
          await mainWindow.unminimize()
          await mainWindow.center()
          await mainWindow.setFocus()
        }
        // 更新状态为可见
        isMainWindowVisible.value = true
      } catch (windowError) {
        // 窗口出错，重新创建
        console.log('主窗口出错，重新创建:', windowError)
        await createMainWindow('/')
        isMainWindowVisible.value = true
      }
    } else {
      console.log('主窗口不存在，创建新窗口')
      await createMainWindow('/')
      isMainWindowVisible.value = true
    }

  } catch (e) {
    console.error('打开主窗口失败:', e)
    alert('打开主窗口失败: ' + e)
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  /* 防止文本缩放导致的布局变化 */
  text-size-adjust: none;
  -webkit-text-size-adjust: none;
  -moz-text-size-adjust: none;
  /* 防止 DPI 缩放影响 */
  image-rendering: -webkit-optimize-contrast;
  image-rendering: crisp-edges;
}

#app {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  /* 确保内容不受系统缩放影响 */
  zoom: 1;
}
</style>

<style scoped>
.float-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  padding: 0 12px;
  gap: 8px;
  background-color: var(--float-bg, #f5f5f5);
  border-radius: 2px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  cursor: move;
  user-select: none;
  overflow: hidden;
}

.float-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--float-text, #333);
  white-space: nowrap;
  line-height: 1;
}

.connection-status {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 2px;
  background-color: #ff6b6b;
  color: white;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
  line-height: 1.4;
}

.connection-status:hover {
  opacity: 0.85;
}

.connection-status.connected {
  background-color: #52c41a;
}

.connection-status.connected:hover {
  opacity: 0.85;
}

.float-buttons {
  display: flex;
  gap: 4px;
  margin-left: auto;
}

.float-btn {
  min-width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 6px;
  font-size: 13px;
  background-color: transparent;
  color: var(--float-btn-color, #666);
  border: none;
  border-radius: 2px;
  cursor: pointer;
  transition: all 0.2s;
  line-height: 1;
  gap: 4px;
}

.float-btn:hover {
  background-color: var(--float-btn-hover-bg, rgba(0, 0, 0, 0.06));
  color: var(--float-btn-hover-color, #333);
}

/* 笔记按钮激活状态 */
.note-btn.active {
  background-color: rgba(var(--accent-blue-rgb, 49, 120, 198), 0.15);
  color: var(--accent-blue, #3178c6);
}

/* 打开主窗口按钮 - 特殊样式 */
.open-main-btn {
  background-color: rgba(76, 175, 80, 0.1);
  color: #4caf50;
}

.open-main-btn:hover {
  background-color: rgba(76, 175, 80, 0.2);
  color: #2e7d32;
}

/* 按钮文字样式 */
.btn-text {
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
}

/* 扩散动画遮罩 - 适配扁平悬浮窗 */
.ripple-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.3);
  z-index: 99999;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 扩散动画效果 - 扁平化设计 */
.ripple-animation {
  position: absolute;
  width: 30px;
  height: 30px;
  background-color: rgba(59, 130, 246, 0.4);}
  </style>