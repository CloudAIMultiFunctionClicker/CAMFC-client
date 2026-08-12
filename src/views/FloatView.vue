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
      <!-- <button v-if="!isMainWindowVisible" class="float-btn open-main-btn" @click.stop="openMainWindow" title="打开主窗口">
        <i class="ri-home-2-line"></i>
        <span class="btn-text">主窗口</span>
      </button> -->
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
      <button class="float-btn meeting-btn" @click.stop="toggleMeeting" :title="meetingActive ? '下课' : '上课'">
        <i :class="meetingActive ? 'ri-stop-circle-line' : 'ri-play-circle-line'"></i>
        <span class="btn-text">{{ meetingActive ? '下课' : '上课' }}</span>
      </button>
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
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getCurrentWindow, Window } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../components/layout/showToast.js'
import axios from 'axios'
import { getBackendUrl } from '../config/backend.js'
import { loadAppData } from '../components/data/storage.js'

const isConnected = ref(false)
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

let keepOnTopInterval = null
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

  // 监听截图命令（来自 0x05 按键）
  const unlistenScreenshot = await listen('screenshot-command', async () => {
    console.log('悬浮窗收到截图命令（0x05）')
    // 直接触发截图
    await handleScreenshot()
  })

  // 监听会议切换命令（来自 0x02 按键）
  const unlistenToggleMeeting = await listen('toggle-meeting', async () => {
    console.log('悬浮窗收到会议切换命令（0x02）')
    await toggleMeeting()
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

  // 保持置顶（每 5 秒执行一次）
  keepOnTopInterval = setInterval(async () => {
    try {
      const floatWindow = await getCurrentWindow()
      await floatWindow.setAlwaysOnTop(true)
    } catch (e) {
      console.error('保持置顶失败:', e)
    }
  }, 5000)

  // 监听窗口大小变化并强制恢复（保持位置和大小）
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
    if (unlistenToggleMeeting) unlistenToggleMeeting()
    if (unlistenFloatToggle) unlistenFloatToggle()
    if (keepOnTopInterval) {
      clearInterval(keepOnTopInterval)
    }
  })
})

function handleConnectionClick() {
  if (!isConnected.value) {
    openMainPage('/')
  }
}

async function startDrag(e) {
  // 排除所有可点击元素
  if (e.target.closest('.float-btn') || 
      e.target.closest('.connection-status')) {
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
 * 切换课堂状态（上课/下课）
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

    const newState = !meetingActive.value
    meetingActive.value = newState
    console.log('课堂状态已切换:', newState ? '课堂开始' : '课堂结束')
    showToast(newState ? '课堂已开始' : '课堂已结束', '#10b981')
    
    // 显示 Windows 原生通知
    if (newState) {
      try {
        const { invoke } = await import('@tauri-apps/api/core')
        await invoke('show_windows_notification', {
          title: '上课模式已开启',
          message: '课堂记录功能已启动，系统将自动记录课堂内容'
        })
        console.log('Windows 通知显示成功')
      } catch (error) {
        console.error('显示 Windows 通知失败:', error)
      }
    }
  } catch (error) {
    console.error('切换课堂状态失败:', error)
    showToast('切换课堂状态失败', '#ef4444')
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
 * 根据设置决定是否隐藏主窗口
 */
async function handleScreenshot() {
  console.log('开始截图流程')

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

    // 从设置中读取是否隐藏主窗口
    const hideWindowSetting = await loadAppData('screenshot_hide_window')
    const shouldHideWindow = hideWindowSetting ? JSON.parse(hideWindowSetting) : true

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
      console.log('截图成功')
      
      // 如果会议进行中，直接发送截图到后端（非阻塞）
      if (meetingActive) {
        console.log('会议进行中，发送截图到后端（非阻塞）')
        sendScreenshotToBackend(result.image_data)
        return
      }
      
      // 课堂未进行，打开截图窗口显示
      console.log('打开截图窗口')
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
    
    console.log('课堂截图发送成功:', response.data)
    showToast('课堂截图已保存', '#10b981')
  } catch (error) {
    console.error('发送课堂截图失败:', error)
    showToast('截图保存失败', '#ef4444')
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
 * 开会模式下直接新建会议笔记，否则打开主窗口的笔记管理页面
 */
async function handleNoteManager() {
  console.log('处理笔记按钮点击')
  
  // 检查会议状态
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
    // 如果接口调用失败，默认会议未进行
    meetingActive = false
  }
  
  if (meetingActive) {
    // 课堂进行中，直接创建课堂笔记
    console.log('课堂进行中，创建课堂笔记')
    await createMeetingNote()
  } else {
    // 会议未进行，打开笔记管理页面
    console.log('会议未进行，打开笔记管理页面')
    await openMainPage('/notes')
  }
}

/**
 * 创建会议笔记
 * 调用 /meeting/note/add API 创建笔记并打开编辑窗口
 */
async function createMeetingNote() {
  const uuid = crypto.randomUUID()
  const now = new Date()
  const timestamp = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}${String(now.getSeconds()).padStart(2, '0')}`
  const defaultTitle = `会议笔记_${timestamp}`
  
  try {
    // 调用后端 API 创建课堂笔记
    const authHeader = await getAuthHeader()
    const response = await axios.post(getBackendUrl() + '/meeting/note/add', {
      title: defaultTitle,
      content: ''
    }, {
      headers: authHeader,
      timeout: 10000
    })
    console.log('会议笔记创建成功:', response.data)
    showToast('会议笔记已创建', '#10b981')
    
    // 打开笔记编辑窗口
    openNoteEditorWindow({
      uuid,
      title: defaultTitle,
      content: '',
      isMeetingNote: true
    })
  } catch (error) {
    console.error('创建会议笔记失败:', error)
    showToast('创建会议笔记失败', '#ef4444')
  }
}

/**
 * 打开笔记编辑窗口
 */
function openNoteEditorWindow(note) {
  const windowLabel = `note-editor-${note.uuid}`
  let url = `/note-editor?uuid=${note.uuid}&title=${encodeURIComponent(note.title)}`
  if (note.isMeetingNote) {
    url += '&isMeetingNote=true'
  }
  
  const webview = new WebviewWindow(windowLabel, {
    url: url,
    title: note.title || '编辑笔记',
    width: 900,
    height: 600,
    minWidth: 400,
    minHeight: 300,
    center: true,
    decorations: false,
    resizable: true
  })
  
  webview.once('tauri://created', async () => {
    console.log('笔记编辑窗口创建成功:', windowLabel)
    await new Promise(resolve => setTimeout(resolve, 300))
    
    try {
      if (note.isMeetingNote) {
        await webview.emit('load-note-content', { 
          content: note.content || '' 
        })
      }
    } catch (e) {
      console.error('发送笔记内容失败:', e)
    }
  })

  webview.once('tauri://error', (e) => {
    console.error('笔记编辑窗口创建失败:', e)
    const errorMsg = e?.payload || ''
    if (typeof errorMsg === 'string' && errorMsg.includes('already exists')) {
      showToast('该笔记编辑窗口已打开', '#f59e0b')
    } else {
      showToast('打开编辑窗口失败', '#ef4444')
    }
  })
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
      } catch (windowError) {
        // 窗口出错，重新创建
        console.log('主窗口出错，重新创建:', windowError)
        await createMainWindow('/')
      }
    } else {
      console.log('主窗口不存在，创建新窗口')
      await createMainWindow('/')
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
  border-radius: 2px;
  background-color: var(--accent-red);
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
  background-color: var(--accent-green);
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
  color: var(--accent-green);
}

.open-main-btn:hover {
  background-color: rgba(76, 175, 80, 0.2);
  color: var(--accent-green);
}

/* 课堂按钮样式 */
.meeting-btn {
  background-color: rgba(255, 152, 0, 0.1);
  color: var(--accent-yellow);
}

.meeting-btn:hover {
  background-color: rgba(255, 152, 0, 0.2);
  color: var(--accent-yellow);
}

.meeting-btn.active {
  background-color: rgba(244, 67, 54, 0.1);
  color: var(--accent-red);
}

.meeting-btn.active:hover {
  background-color: rgba(244, 67, 54, 0.2);
  color: var(--accent-red);
}

/* 按钮文字样式 */
.btn-text {
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
}

/* 未连接提示框 - 在 float-container 内居中显示 */
.connect-tip {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  background-color: var(--float-tip-bg, #ffffff);
  border: 1px solid var(--float-menu-border, #e5e5e5);
  border-radius: 2px;
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
