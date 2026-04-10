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
  <div class="float-container" @mousedown="startDrag" :style="themeVars">
    <span class="float-title">CAMFC Cloud</span>
    <span
      class="connection-status"
      :class="{ connected: isConnected }"
      @click="handleConnectionClick"
    >
      {{ isConnected ? '已连接' : '未连接' }}
    </span>
    <div class="float-buttons">
      <button v-if="!isMainWindowVisible" class="float-btn open-main-btn" @click.stop="openMainWindow" title="打开主窗口">
        <i class="ri-home-2-line"></i>
        <span class="btn-text">主窗口</span>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/fileView')" title="云盘">
        <i class="ri-cloud-line"></i>
      </button>
      <button class="float-btn note-btn" @click.stop="handleNoteManager" title="笔记">
        <i class="ri-sticky-note-line"></i>
      </button>
      <button class="float-btn screenshot-btn" @click.stop="toggleScreenshotMenu" title="截图">
        <i class="ri-screenshot-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/settings')" title="设置">
        <i class="ri-settings-3-line"></i>
      </button>
      <button class="float-btn meeting-btn" @click.stop="toggleMeeting" :title="meetingActive ? '下会' : '开会'">
        <i :class="meetingActive ? 'ri-stop-circle-line' : 'ri-play-circle-line'"></i>
        <span class="btn-text">{{ meetingActive ? '下会' : '开会' }}</span>
      </button>
    </div>



    <!-- 截图功能菜单 - 水平排布 -->
    <div v-if="showScreenshotMenu" class="screenshot-menu" @click.stop>
      <div class="menu-item toggle-item" @click="toggleHideWindowOption">
        <i :class="hideWindowBeforeScreenshot ? 'ri-checkbox-circle-line' : 'ri-circle-line'"></i>
        <span>隐藏主窗口</span>
      </div>
      <div class="menu-item" @click="handleScreenshot">
        <i class="ri-screenshot-line"></i>
        <span>屏幕截图</span>
      </div>
      <div class="menu-item back" @click="closeScreenshotMenu">
        <i class="ri-close-line"></i>
        <span>关闭</span>
      </div>
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
import axios from 'axios'
import { getBackendUrl } from '../config/backend.js'

const isConnected = ref(false)
const isMainWindowVisible = ref(true)
const meetingActive = ref(false)
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

// 点击外部指令的处理函数
let clickOutsideHandler = null

onMounted(() => {
  // 添加全局点击监听，用于点击外部关闭菜单
  clickOutsideHandler = (event) => {
    handleClickOutside(event)
  }
  document.addEventListener('click', clickOutsideHandler)
})

onBeforeUnmount(() => {
  // 移除全局点击监听
  if (clickOutsideHandler) {
    document.removeEventListener('click', clickOutsideHandler)
  }
})

// 截图菜单显示状态
const showScreenshotMenu = ref(false)

// 截图前是否隐藏主窗口选项
const hideWindowBeforeScreenshot = ref(true)

let keepOnTopInterval = null
let visibilityCheckInterval = null
let unlistenTheme = null
let unlistenConnection = null

onMounted(async () => {
  console.log('FloatView mounted')
  
  try {
    const { getFloatWindowEnabled } = await import('../components/data/storage.js')
    const enabled = await getFloatWindowEnabled()
    if (!enabled) {
      console.log('悬浮窗功能已禁用，隐藏窗口')
      const currentWindow = await getCurrentWindow()
      await currentWindow.hide()
      return
    }
  } catch (e) {
    console.warn('检查悬浮窗状态失败:', e)
  }

  unlistenTheme = await listen('theme-changed', (event) => {
    const newTheme = event.payload
    console.log('[悬浮窗] 收到主题变化事件:', newTheme)
    isLightMode.value = newTheme === 'light'
    console.log('[悬浮窗] 主题已更新为:', isLightMode.value ? '浅色' : '深色')
    // 不再写入 localStorage，由主窗口统一管理主题状态
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

  // 监听悬浮窗开关状态变化
  let unlistenFloatToggle = null
  try {
    unlistenFloatToggle = await listen('float-window-toggled', async (event) => {
      const enabled = event.payload
      console.log('[悬浮窗] 收到悬浮窗状态变化事件:', enabled)
      if (!enabled) {
        console.log('[悬浮窗] 悬浮窗已被禁用，正在隐藏窗口...')
        const currentWindow = await getCurrentWindow()
        await currentWindow.hide()
        console.log('[悬浮窗] 窗口已隐藏')
      } else {
        console.log('[悬浮窗] 悬浮窗已启用，正在显示窗口...')
        const currentWindow = await getCurrentWindow()
        await currentWindow.center()
        await currentWindow.show()
        console.log('[悬浮窗] 窗口已显示')
      }
    })
    console.log('[悬浮窗] 成功监听悬浮窗状态变化事件')
  } catch (e) {
    console.error('[悬浮窗] 监听悬浮窗状态变化失败:', e)
  }

  // 不再主动查询主题，由主窗口在主题切换时主动通知悬浮窗
  // 避免了双向通信导致的主题状态循环更新问题

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

  // 保持置顶（每 5 秒执行一次）
  keepOnTopInterval = setInterval(async () => {
    try {
      const floatWindow = await getCurrentWindow()
      await floatWindow.setAlwaysOnTop(true)
    } catch (e) {
      console.error('保持置顶失败:', e)
    }
  }, 5000)

  // 监听窗口大小变化并强制恢复（只限制宽高，不管位置）
  const limitWindowSize = async () => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('set_window_size_by_label', {
        label: 'float-normal-empty',
        width: 450,
        height: 60
      })
    } catch (e) {
      console.error('限制窗口大小失败:', e)
    }
  }
  
  // 初始设置一次
  limitWindowSize()
  
  // 使用 ResizeObserver 监听窗口大小变化
  const resizeObserver = new ResizeObserver(() => {
    // 窗口大小改变时立即恢复
    limitWindowSize()
  })
  resizeObserver.observe(document.body)
  
  // 备用方案：定时检查（每 500ms）
  const sizeCheckInterval = setInterval(limitWindowSize, 500)

  onUnmounted(() => {
    if (sizeCheckInterval) {
      clearInterval(sizeCheckInterval)
    }
    if (resizeObserver) {
      resizeObserver.disconnect()
    }
    if (unlistenTheme) unlistenTheme()
    if (unlistenConnection) unlistenConnection()
    if (unlistenScreenshot) unlistenScreenshot()
    if (unlistenFloatToggle) unlistenFloatToggle()
    if (keepOnTopInterval) {
      clearInterval(keepOnTopInterval)
    }
    if (visibilityCheckInterval) {
      clearInterval(visibilityCheckInterval)
    }
  })
})

function handleConnectionClick() {
  if (!isConnected.value) {
    openMainPage('/')
  }
}

async function startDrag(e) {
  // 排除所有可点击元素，包括按钮、菜单项等
  if (e.target.closest('.float-btn') || 
      e.target.closest('.connection-status') ||
      e.target.closest('.menu-item') ||
      e.target.closest('.screenshot-menu')) {
    return
  }
  try {
    const floatWindow = await getCurrentWindow()
    await floatWindow.startDragging()
  } catch (e) {
    console.error('拖动失败:', e)
  }
}

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
 * 切换截图菜单显示
 */
function toggleScreenshotMenu() {
  showScreenshotMenu.value = !showScreenshotMenu.value
  console.log('截图菜单状态:', showScreenshotMenu.value)
}

/**
 * 关闭截图菜单
 */
function closeScreenshotMenu() {
  showScreenshotMenu.value = false
}

/**
 * 切换隐藏主窗口选项
 */
function toggleHideWindowOption() {
  hideWindowBeforeScreenshot.value = !hideWindowBeforeScreenshot.value
  console.log('隐藏主窗口选项:', hideWindowBeforeScreenshot.value)
}

/**
 * 切换会议状态（开会/下会）
 */
async function toggleMeeting() {
  console.log('切换会议状态，当前状态:', meetingActive.value)
  
  try {
    const authHeader = await getAuthHeader()
    const url = getBackendUrl() + '/meeting/' + (meetingActive.value ? 'stop' : 'start')
    
    const response = await axios.get(url, {
      headers: authHeader,
      timeout: 5000
    })
    
    meetingActive.value = !meetingActive.value
    console.log('会议状态已切换:', meetingActive.value ? '会议开始' : '会议结束')
    showToast(meetingActive.value ? '会议已开始' : '会议已结束', '#10b981')
  } catch (error) {
    console.error('切换会议状态失败:', error)
    showToast('切换会议状态失败', '#ef4444')
  }
}

/**
 * 获取认证头信息
 */
async function getAuthHeader() {
  try {
    const { getDeviceId, getTotp } = await import('../components/data/bluetooth.js')
    const deviceId = await getDeviceId()
    const currentTotp = await getTotp()
    return { "Id": deviceId, "Totp": currentTotp }
  } catch {
    return {}
  }
}

/**
 * 处理屏幕截图
 * 根据用户选择决定是否隐藏主窗口，截图完成后再打开显示截图结果
 */
async function handleScreenshot() {
  console.log('开始截图流程')
  closeScreenshotMenu()

  try {
    // 先调用后端接口检查会议状态
    console.log('检查会议状态...')
    let meetingActive = false
    
    try {
      const authHeader = await getAuthHeader()
      const response = await axios.get(getBackendUrl() + '/meeting/status', {
        headers: authHeader,
        timeout: 5000
      })
      
      meetingActive = response.data.in_meeting === true
      console.log('会议状态:', meetingActive ? '进行中' : '未进行')
    } catch (error) {
      console.error('获取会议状态失败:', error)
      // 如果接口调用失败，默认允许截图
      meetingActive = true
    }

    // 获取主窗口
    const mainWindow = await Window.getByLabel('main')
    let wasVisible = false

    // 如果用户选择隐藏主窗口且主窗口存在且可见，先隐藏它
    if (hideWindowBeforeScreenshot.value && mainWindow) {
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
      console.log('截图成功')
      
      // 如果会议进行中，直接发送截图到后端（非阻塞）
      if (meetingActive) {
        console.log('会议进行中，发送截图到后端（非阻塞）')
        sendScreenshotToBackend(result.image_data)
        return
      }
      
      // 会议未进行，打开截图窗口显示
      console.log('打开主窗口显示截图')
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
 * 发送截图到后端
 */
async function sendScreenshotToBackend(imageData) {
  try {
    const authHeader = await getAuthHeader()
    const response = await axios.post(getBackendUrl() + '/meeting/screenshot/add', {
      image: imageData
    }, {
      headers: authHeader,
      timeout: 10000
    })
    
    console.log('截图发送成功:', response.data)
    showToast('截图已发送到后端', '#10b981')
  } catch (error) {
    console.error('发送截图失败:', error)
    showToast('发送截图失败', '#ef4444')
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
 * 处理点击菜单外部区域
 * 当截图菜单显示时，点击外部关闭菜单
 */
function handleClickOutside(event) {
  // 如果截图菜单显示，且点击的不是截图按钮，则关闭菜单
  if (showScreenshotMenu.value && !event.target.closest('.screenshot-btn')) {
    closeScreenshotMenu()
  }
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
}

#app {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
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
  border-radius: .375rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  cursor: move;
  user-select: none;
  overflow: hidden;
  -webkit-app-region: drag;
  position: relative;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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

/* 会议按钮样式 */
.meeting-btn {
  background-color: rgba(255, 152, 0, 0.1);
  color: #ff9800;
}

.meeting-btn:hover {
  background-color: rgba(255, 152, 0, 0.2);
  color: #f57c00;
}

.meeting-btn.active {
  background-color: rgba(244, 67, 54, 0.1);
  color: #f44336;
}

.meeting-btn.active:hover {
  background-color: rgba(244, 67, 54, 0.2);
  color: #d32f2f;
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
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 扩散动画效果 - 扁平化设计 */
.ripple-animation {
  position: absolute;
  width: 30px;
  height: 30px;
  background-color: rgba(59, 130, 246, 0.4);
  border-radius: 50%;
  animation: ripple-expand 0.4s ease-out forwards;
  right: 45px;
  top: 50%;
  transform: translateY(-50%);
}

@keyframes ripple-expand {
  0% {
    width: 30px;
    height: 30px;
    opacity: 1;
  }
  100% {
    width: 120px;
    height: 120px;
    opacity: 0;
  }
}

/* 笔记功能菜单 - 水平排布，从右到左滑入 */
.note-menu {
  position: fixed;
  right: 8px;
  top: 50%;
  background-color: var(--float-menu-bg, white);
  border: 1px solid var(--float-menu-border, #e5e5e5);
  border-radius: .375rem;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
  padding: 6px 8px;
  z-index: 1001;
  display: flex;
  flex-direction: row;
  gap: 4px;
  animation: menu-slide-in 0.25s cubic-bezier(0.25, 0.46, 0.45, 0.94) forwards;
}

.screenshot-menu {
  position: fixed;
  right: 8px;
  top: 50%;
  background-color: var(--float-menu-bg, white);
  border: 1px solid var(--float-menu-border, #e5e5e5);
  border-radius: .375rem;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
  padding: 6px 8px;
  z-index: 1001;
  display: flex;
  flex-direction: row;
  gap: 4px;
  animation: menu-slide-in 0.25s cubic-bezier(0.25, 0.46, 0.45, 0.94) forwards;
}

/* 切换选项样式 */
.screenshot-menu .toggle-item i {
  font-size: 16px;
}

.screenshot-menu .toggle-item:hover i {
  color: var(--accent-blue, #3178c6);
}

@keyframes menu-slide-in {
  0% {
    opacity: 0;
    transform: translateY(-50%) translateX(50px);
  }
  100% {
    opacity: 1;
    transform: translateY(-50%) translateX(0);
  }
}

.menu-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
  gap: 6px;
  padding: 6px 10px;
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--float-menu-text, #333);
  font-size: 11px;
  white-space: nowrap;
}

.menu-item:hover {
  background-color: var(--float-btn-hover-bg, rgba(0, 0, 0, 0.06));
}

.menu-item i {
  font-size: 16px;
  color: #666;
  flex-shrink: 0;
}

.menu-item:hover i {
  color: var(--accent-blue, #3178c6);
}

.menu-item span {
  font-size: 11px;
  color: var(--float-menu-text, #333);
}

.menu-item.back:hover {
  background-color: var(--float-btn-hover-bg, rgba(0, 0, 0, 0.06));
}

.menu-item.back:hover i {
  color: #ef4444;
}

/* 未连接提示框 - 在 float-container 内居中显示 */
.connect-tip {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  background-color: var(--float-tip-bg, #ffffff);
  border: 1px solid var(--float-menu-border, #e5e5e5);
  border-radius: .375rem;
  padding: 8px 12px 8px 16px;
  display: flex;
  align-items: center;
  gap: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 1002;
}

/* 进入动画 */
.tip-fade-enter-active {
  animation: tip-fade-in 0.2s ease-out forwards;
}

/* 离开动画 */
.tip-fade-leave-active {
  animation: tip-fade-out 0.2s ease-in forwards;
}

.connect-tip i {
  font-size: 16px;
  color: var(--float-menu-text, #737373);
}

.connect-tip span {
  font-size: 13px;
  font-weight: 500;
  color: var(--float-menu-text, #171717);
  white-space: nowrap;
  pointer-events: none;
}

@keyframes tip-fade-in {
  0% {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.9);
  }
  100% {
    opacity: 1;
  }
}

@keyframes tip-fade-out {
  0% {
    opacity: 1;
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.9);
  }
}
</style>
