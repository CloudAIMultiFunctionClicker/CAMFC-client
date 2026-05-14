

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
import { loadAppData } from '../components/data/storage.js'

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

let clickOutsideHandler = null

onMounted(() => {

  clickOutsideHandler = (event) => {
    handleClickOutside(event)
  }
  document.addEventListener('click', clickOutsideHandler)
})

onBeforeUnmount(() => {

  if (clickOutsideHandler) {
    document.removeEventListener('click', clickOutsideHandler)
  }
})

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

  })

  unlistenConnection = await listen('connection-status', (event) => {
    console.log('收到连接状态事件:', event.payload)
    isConnected.value = event.payload
  })

  const unlistenScreenshot = await listen('screenshot-command', async () => {
    console.log('悬浮窗收到截图命令（0x12）')

    await handleScreenshot()
  })

  const unlistenToggleMeeting = await listen('toggle-meeting', async () => {
    console.log('悬浮窗收到会议切换命令（0x02）')
    await toggleMeeting()
  })

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

  await checkMainWindowVisibility()

  visibilityCheckInterval = setInterval(async () => {
    await checkMainWindowVisibility()
  }, 500)

  keepOnTopInterval = setInterval(async () => {
    try {
      const floatWindow = await getCurrentWindow()
      await floatWindow.setAlwaysOnTop(true)
    } catch (e) {
      console.error('保持置顶失败:', e)
    }
  }, 5000)

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

  limitWindowSize()

  const resizeObserver = new ResizeObserver(() => {

    limitWindowSize()
  })
  resizeObserver.observe(document.body)

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

function showTip() {
  showConnectTip.value = true

  if (connectTipTimer) {
    clearTimeout(connectTipTimer)
  }

  connectTipTimer = setTimeout(() => {
    hideTip()
  }, 1000)
}

function hideTip() {
  showConnectTip.value = false
  if (connectTipTimer) {
    clearTimeout(connectTipTimer)
    connectTipTimer = null
  }
}

async function openMainPage(path) {
  console.log('点击按钮，目标是:', path)

  const needConnection = ['/fileView', '/settings', '/notes'].includes(path)
  if (needConnection && !isConnected.value) {
    console.log('设备未连接，显示提示并打开首页')
    showTip()

    path = '/'
  }

  try {

    const mainWindow = await Window.getByLabel('main')

    if (mainWindow) {
      try {

        const isVisible = await mainWindow.isVisible()

        if (isVisible) {
          console.log('主窗口可见，聚焦并导航')

          await mainWindow.unminimize()

          await mainWindow.show()
          await mainWindow.center()
          await mainWindow.setFocus()

          const webview = await WebviewWindow.getByLabel('main')
          if (webview) {
            await webview.emit('navigate', path)
          }
          console.log('发送导航事件:', path)
        } else {

          console.log('主窗口在托盘，显示并聚焦')
          await mainWindow.show()
          await mainWindow.unminimize()
          await mainWindow.center()
          await mainWindow.setFocus()

          const webview = await WebviewWindow.getByLabel('main')
          if (webview) {
            await webview.emit('navigate', path)
          }
          console.log('显示窗口并发送导航事件:', path)
        }
      } catch (windowError) {

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

function toggleScreenshotMenu() {
  showScreenshotMenu.value = !showScreenshotMenu.value
  console.log('截图菜单状态:', showScreenshotMenu.value)
}

function closeScreenshotMenu() {
  showScreenshotMenu.value = false
}

function toggleHideWindowOption() {
  hideWindowBeforeScreenshot.value = !hideWindowBeforeScreenshot.value
  console.log('隐藏主窗口选项:', hideWindowBeforeScreenshot.value)
}

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

async function handleScreenshot() {
  console.log('开始截图流程')

  try {

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

      meetingActive = true
    }

    const hideWindowSetting = await loadAppData('screenshot_hide_window')
    const shouldHideWindow = hideWindowSetting ? JSON.parse(hideWindowSetting) : true

    const mainWindow = await Window.getByLabel('main')
    let wasVisible = false

    if (shouldHideWindow && mainWindow) {
      wasVisible = await mainWindow.isVisible()
      if (wasVisible) {
        console.log('隐藏主窗口以便截图')
        await mainWindow.hide()

        await new Promise(resolve => setTimeout(resolve, 300))
      }
    }

    console.log('执行截图')
    const result = await invoke('capture_screen')

    if (result.success) {
      console.log('截图成功')

      if (meetingActive) {
        console.log('会议进行中，发送截图到后端（非阻塞）')
        sendScreenshotToBackend(result.image_data)
        return
      }

      console.log('打开截图窗口')
      await openScreenshotWindow(result)
    } else {
      console.error('截图失败:', result.error)

      if (wasVisible && mainWindow) {
        await mainWindow.show()
      }
    }
  } catch (e) {
    console.error('截图过程出错:', e)
  }
}

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

      await new Promise(resolve => setTimeout(resolve, 500))

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

async function handleNoteManager() {
  console.log('处理笔记按钮点击')

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

    meetingActive = false
  }

  if (meetingActive) {

    console.log('课堂进行中，创建课堂笔记')
    await createMeetingNote()
  } else {

    console.log('会议未进行，打开笔记管理页面')
    await openMainPage('/notes')
  }
}

async function createMeetingNote() {
  const uuid = crypto.randomUUID()
  const now = new Date()
  const timestamp = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}${String(now.getSeconds()).padStart(2, '0')}`
  const defaultTitle = `会议笔记_${timestamp}`

  try {

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

function handleClickOutside(event) {

}

async function openMainWindow() {
  console.log('打开主窗口')

  try {

    const mainWindow = await Window.getByLabel('main')

    if (mainWindow) {
      try {

        const isVisible = await mainWindow.isVisible()

        if (isVisible) {
          console.log('主窗口已可见，聚焦')

          await mainWindow.unminimize()

          await mainWindow.show()
          await mainWindow.center()
          await mainWindow.setFocus()
        } else {

          console.log('主窗口在托盘，显示并聚焦')
          await mainWindow.show()
          await mainWindow.unminimize()
          await mainWindow.center()
          await mainWindow.setFocus()
        }

        isMainWindowVisible.value = true
      } catch (windowError) {

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

  text-size-adjust: none;
  -webkit-text-size-adjust: none;
  -moz-text-size-adjust: none;

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

.note-btn.active {
  background-color: rgba(var(--accent-blue-rgb, 49, 120, 198), 0.15);
  color: var(--accent-blue, #3178c6);
}

.open-main-btn {
  background-color: rgba(76, 175, 80, 0.1);
  color: #4caf50;
}

.open-main-btn:hover {
  background-color: rgba(76, 175, 80, 0.2);
  color: #2e7d32;
}

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

.btn-text {
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
}

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

.ripple-animation {
  position: absolute;
  width: 30px;
  height: 30px;
  background-color: rgba(59, 130, 246, 0.4);
  border-radius: 2px;
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

.note-menu {
  position: fixed;
  right: 8px;
  top: 50%;
  background-color: var(--float-menu-bg, white);
  border: 1px solid var(--float-menu-border, #e5e5e5);
  border-radius: 2px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
  padding: 6px 8px;
  z-index: 1001;
  display: flex;
  flex-direction: row;
  gap: 4px;
  animation: menu-slide-in 0.25s cubic-bezier(0.25, 0.46, 0.45, 0.94) forwards;
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

.tip-fade-enter-active {
  animation: tip-fade-in 0.2s ease-out forwards;
}

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
