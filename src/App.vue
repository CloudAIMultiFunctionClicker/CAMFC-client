

<script setup>
import { ref, provide, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

import { useBluetoothStore } from './stores/bluetooth.js'

import {showToast} from './components/layout/showToast.js'
import TitleBar from './components/layout/TitleBar.vue'
import Sidebar from './components/layout/Sidebar.vue'

const route = useRoute()
const router = useRouter()

const isFloatPage = computed(() => route.path === '/float')

const isFloatNormalPage = computed(() => route.path === '/float-normal')

const isFloatNormalEmptyPage = computed(() => route.path === '/float-normal-empty')

const isNoteEditorPage = computed(() => route.path === '/note-editor')

const isMeetingEditorPage = computed(() => route.path === '/meeting-editor')

const isEmptyPage = computed(() => route.path === '/empty')

const isScreenshotWindowPage = computed(() => route.path === '/screenshot-window')

const isScreenshotPage = computed(() => route.path === '/screenshot')

const isNoteViewerPage = computed(() => route.path === '/note-viewer')

const isAgentWindowPage = computed(() => route.path === '/agent-window')

const shouldHideTitleBar = computed(() => isNoteEditorPage.value || isMeetingEditorPage.value || isEmptyPage.value || isScreenshotWindowPage.value || isScreenshotPage.value || isFloatNormalEmptyPage.value || isNoteViewerPage.value || isAgentWindowPage.value || isFloatNormalPage.value)

const isSidebarCollapsed = ref(false)

const handleSidebarCollapse = (collapsed) => {
  isSidebarCollapsed.value = collapsed
}

let totpRefreshInterval = null
const TOTP_REFRESH_INTERVAL = 30000

import { initBackendConfig } from './config/backend.js'

document.addEventListener('keydown', (e) => {
  if (e.ctrlKey && (e.key === 'r' || e.key === 'p'|| e.key === 'h'|| e.key === 'z' || e.key === 'f')) {
    e.preventDefault();
  }
});

const getInitialTheme = () => {

  const savedTheme = localStorage.getItem('theme-preference')
  if (savedTheme === 'light' || savedTheme === 'dark') {
    return savedTheme === 'light'
  }

  const prefersLight = window.matchMedia('(prefers-color-scheme: light)').matches
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches

  if (prefersLight) {
    return true
  }

  return false
}

const isLightMode = ref(getInitialTheme())

const toggleTheme = async () => {
  console.log('[主题切换] 用户点击切换主题，当前主题:', isLightMode.value ? '浅色' : '深色')
  isLightMode.value = !isLightMode.value
  console.log('[主题切换] 新主题:', isLightMode.value ? '浅色' : '深色')
  updateBodyClass()

  localStorage.setItem('theme-preference', isLightMode.value ? 'light' : 'dark')

  try {
    const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
    const theme = isLightMode.value ? 'light' : 'dark'

    const windowLabels = ['float', 'float-normal', 'float-normal-empty']
    for (const label of windowLabels) {
      try {
        const window = await WebviewWindow.getByLabel(label)
        if (window) {
          await window.emit('theme-changed', theme)
          console.log(`已发送主题变化事件到窗口：${label}`)
        }
      } catch (e) {
        console.log(`发送主题事件到 ${label} 失败:`, e)
      }
    }
  } catch (e) {
    console.log('发送主题变化事件失败:', e)
  }
}

const updateBodyClass = () => {
  if (isLightMode.value) {
    document.body.classList.add('light-mode')
  } else {
    document.body.classList.remove('light-mode')
  }
}

const bluetoothStore = useBluetoothStore()

const scanBluetooth = async () => {
  try {
    showToast('开始扫描蓝牙设备...')

    const { scanDevices, findCpenDevices } = await import('./components/data/bluetooth')
    const devices = await scanDevices()
    const cpenDevices = findCpenDevices(devices)

    showToast(`扫描完成，发现 ${devices.length} 个设备，其中 ${cpenDevices.length} 个Cpen设备`)

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

provide('theme', {
  isLightMode,
  toggleTheme
})

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

watch(() => bluetoothStore.isConnected(), (connected) => {
  if (connected) {
    console.log('[TOTP] 设备已连接，启动TOTP定时刷新')
    startTotpRefresh()
  } else {
    console.log('[TOTP] 设备已断开，停止TOTP定时刷新')
    stopTotpRefresh()
  }
})

onMounted(async () => {

  updateBodyClass()

  setTimeout(async () => {
    await initBackendConfig()
  }, 100)

  setTimeout(async () => {
    try {
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')

      const existingWindow = await WebviewWindow.getByLabel('float-normal-empty')
      if (existingWindow) {
        console.log('空白窗口已存在')
        return
      }

      const blankWindow = new WebviewWindow('float-normal-empty', {
        url: '/float',
        title: '',
        width: 450,
        height: 60,
        x: 100,
        y: 100,
        decorations: false,
        skipTaskbar: true
      })

      blankWindow.once('tauri://created', async () => {
        console.log('空白窗口创建成功')
        try {
          const { Window } = await import('@tauri-apps/api/window')
          const window = await Window.getByLabel('float-normal-empty')
          if (window) {
            await window.setAlwaysOnTop(true)
          }
        } catch (e) {
          console.error('设置置顶失败:', e)
        }
      })

      blankWindow.once('tauri://error', (e) => {
        console.error('空白窗口创建失败:', e)
      })
    } catch (e) {
      console.error('创建空白窗口失败:', e)
    }
  }, 600)

  let buttonEventUnlisten = null

  let bluetoothDisconnectUnlisten = null

  let navigateEventUnlisten = null

  let currentWindowLabel = 'main'
  try {
    const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow')
    const currentWindow = await getCurrentWebviewWindow()
    if (currentWindow && currentWindow.label) {
      currentWindowLabel = currentWindow.label
    }
  } catch (e) {
    console.warn('获取窗口标签失败，默认为 main:', e)
  }

  console.log('[事件监听] 当前窗口标签:', currentWindowLabel)

  const { listen } = await import('@tauri-apps/api/event')
  buttonEventUnlisten = await listen('button-event', async (event) => {
    console.log('[按钮事件] 收到事件，当前窗口:', currentWindowLabel, '路由:', route.path)

    if (currentWindowLabel !== 'main') {
      console.log('[按钮事件] 非主窗口，忽略事件')
      return
    }

    if (route.path === '/float') {
      console.log('[按钮事件] 悬浮窗页面，忽略事件')
      return
    }

    const eventType = event.payload.event_type

    if (eventType === 'button_press') {
      showToast('下一页 按下', '#3b82f6')
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: true } }))
    } else if (eventType === 'button_release') {
      showToast('下一页 松开', '#10b981')
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: false } }))

      try {
        const { pressWinKey } = await import('./components/data/bluetooth')
        await pressWinKey()
        console.log('右箭头键模拟成功')
      } catch (e) {
        console.error('右箭头键模拟失败:', e)
      }
    }

    else if (eventType === 'button_press_left') {
      showToast('上一页 按下', '#8b5cf6')
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: true } }))
    } else if (eventType === 'button_release_left') {
      showToast('上一页 松开', '#f59e0b')
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: false } }))

      try {
        const { pressLeftKey } = await import('./components/data/bluetooth')
        await pressLeftKey()
        console.log('左箭头键模拟成功')
      } catch (e) {
        console.error('左箭头键模拟失败:', e)
      }
    }
  })

  const screenshotUnlisten = await listen('screenshot-command', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到截图命令（0x12）')
    showToast('触发截图', '#3b82f6')
  })

  const openCloudUnlisten = await listen('open-cloud-command', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到打开云盘命令（0x08）')
    showToast('打开云盘', '#8b5cf6')

    try {
      const { Window } = await import('@tauri-apps/api/window')
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        await mainWindow.show()
        await mainWindow.unminimize()
        await mainWindow.setFocus()
      }

      const webview = await WebviewWindow.getByLabel('main')
      if (webview) {
        await webview.emit('navigate', '/fileView')
      }
    } catch (e) {
      console.error('打开云盘页面失败:', e)
    }
  })

  const navigateToNotesUnlisten = await listen('navigate-to-notes', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到跳转到笔记列表命令（0x10/10）')
    showToast('跳转到笔记列表', '#3b82f6')

    try {
      const { Window } = await import('@tauri-apps/api/window')
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        await mainWindow.show()
        await mainWindow.unminimize()
        await mainWindow.setFocus()
      }

      router.push('/notes')
    } catch (e) {
      console.error('跳转到笔记列表失败:', e)
    }
  })

  const navigateToMeetingsUnlisten = await listen('navigate-to-meetings', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到跳转到课堂记录命令（按钮 7 0x06）')
    showToast('跳转到课堂记录', '#8b5cf6')

    try {
      const { Window } = await import('@tauri-apps/api/window')
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        await mainWindow.show()
        await mainWindow.unminimize()
        await mainWindow.setFocus()
      }

      router.push('/notes_meetings')
    } catch (e) {
      console.error('跳转到课堂记录失败:', e)
    }
  })

  const createNoteUnlisten = await listen('create-note', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到新建笔记命令（0x02/2）')
    showToast('新建笔记', '#10b981')

    try {
      const { Window } = await import('@tauri-apps/api/window')
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        await mainWindow.show()
        await mainWindow.unminimize()
        await mainWindow.setFocus()
      }

      const { emit } = await import('@tauri-apps/api/event')
      await emit('create-new-note')
    } catch (e) {
      console.error('新建笔记失败:', e)
    }
  })

  const volumeUpUnlisten = await listen('volume-up', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到音量增加命令（按钮 1 0x0C）')
    showToast('音量增加 🔊', '#22c55e')
  })

  const volumeDownUnlisten = await listen('volume-down', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到音量减少命令（按钮 3 0x04）')
    showToast('音量减少 🔉', '#ef4444')
  })

  const openAgentWindowUnlisten = await listen('open-agent-window', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    if (route.path === '/float') {
      return
    }
    console.log('收到打开 agent 窗口命令（按钮 7 0x06）')
    showToast('打开智能体窗口', '#8b5cf6')

    try {
      // 显示主窗口
      const { Window } = await import('@tauri-apps/api/window')
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) {
        await mainWindow.show()
        await mainWindow.unminimize()
        await mainWindow.setFocus()
      }

      // 打开 agent 窗口
      const agentWindow = new WebviewWindow('agent-window', {
        url: '/agent-window',
        title: '自动执行 - CAMFC',
        width: 600,
        height: 700,
        resizable: true,
        center: true,
        decorations: true,
        maximizable: false,
        fullscreen: false,
      })

      agentWindow.once('tauri://created', () => {
        console.log('agent 窗口已创建')
      })

      agentWindow.once('tauri://error', (e) => {
        console.error('创建 agent 窗口失败:', e)
        // 如果窗口已存在，显示并聚焦
        WebviewWindow.getByLabel('agent-window').then(w => {
          if (w) {
            w.show()
            w.setFocus()
          }
        })
      })
    } catch (e) {
      console.error('打开 agent 窗口失败:', e)
    }
  })

  let connectionCheckInterval = null

  const checkConnectionStatus = async () => {
    try {

      const { isConnected } = await import('./components/data/bluetooth')
      const connected = await isConnected()

      if (bluetoothStore.isConnected() && !connected) {
        console.log('蓝牙连接已断开，跳转到蓝牙连接页面')
        bluetoothStore.reset()
        if (route.path !== '/') {
          router.push('/')
        }
      }
    } catch (error) {

      console.log('检查连接状态出错:', error)
    }
  }

  bluetoothDisconnectUnlisten = await listen('bluetooth-disconnect', async () => {
    if (currentWindowLabel !== 'main') {
      return
    }
    console.log('收到蓝牙断开事件，跳转到蓝牙连接页面')
    bluetoothStore.reset()
    if (route.path !== '/') {
      router.push('/')
    }
  })

  connectionCheckInterval = setInterval(checkConnectionStatus, 2000)

  const lightMediaQuery = window.matchMedia('(prefers-color-scheme: light)')

  const handleSystemThemeChange = async (e) => {
    const hasUserPreference = localStorage.getItem('theme-preference') !== null
    if (!hasUserPreference) {
      isLightMode.value = e.matches
      updateBodyClass()

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
  }

  lightMediaQuery.addEventListener('change', handleSystemThemeChange)

  try {
    navigateEventUnlisten = await listen('navigate', (event) => {
      console.log('收到导航事件:', event.payload)
      const path = event.payload

      if (path && router && route.path !== '/float') {
        router.push(path)
      }
    })
  } catch (e) {
    console.log('监听导航事件失败（非Tauri环境）:', e)
  }

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

  let noteEditorOpenedUnlisten = null
  try {
    noteEditorOpenedUnlisten = await listen('note-editor-opened', async () => {
      if (currentWindowLabel !== 'main') {
        return
      }

      if (route.path === '/notes') {
        console.log('笔记编辑器打开，刷新笔记列表')
        const { Window } = await import('@tauri-apps/api/window')
        const mainWindow = await Window.getByLabel('main')
        if (mainWindow) {
          await mainWindow.emit('refresh-notes')
        }
      }
    })
  } catch (e) {
    console.log('监听笔记编辑器打开事件失败:', e)
  }

  let noteEditorClosedUnlisten = null
  try {
    noteEditorClosedUnlisten = await listen('note-editor-closed', async () => {
      if (currentWindowLabel !== 'main') {
        return
      }

      if (route.path === '/notes') {
        console.log('笔记编辑器关闭，刷新笔记列表')
        const { Window } = await import('@tauri-apps/api/window')
        const mainWindow = await Window.getByLabel('main')
        if (mainWindow) {
          await mainWindow.emit('refresh-notes')
        }
      }
    })
  } catch (e) {
    console.log('监听笔记编辑器关闭事件失败:', e)
  }

  onUnmounted(() => {

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
    if (navigateToNotesUnlisten) {
      navigateToNotesUnlisten()
    }
    if (createNoteUnlisten) {
      createNoteUnlisten()
    }
    if (openCloudUnlisten) {
      openCloudUnlisten()
    }
    if (volumeUpUnlisten) {
      volumeUpUnlisten()
    }
    if (volumeDownUnlisten) {
      volumeDownUnlisten()
    }
    if (navigateToAgentUnlisten) {
      navigateToAgentUnlisten()
    }
    if (noteEditorOpenedUnlisten) {
      noteEditorOpenedUnlisten()
    }
    if (noteEditorClosedUnlisten) {
      noteEditorClosedUnlisten()
    }
  })

setTimeout(() => {
  console.log('应用启动完成，InitialView将处理蓝牙连接')

}, 1000)
})
</script>

<template>

  <div class="app-container" v-if="!isFloatPage">

    <TitleBar v-if="!shouldHideTitleBar" />

    <Sidebar v-if="!shouldHideTitleBar" @collapse-change="handleSidebarCollapse" />
    <div class="main-content" :class="{ 'sidebar-collapsed': isSidebarCollapsed }" :style="shouldHideTitleBar ? 'padding-top: 0; padding-left: 0;' : ''">
      <router-view></router-view>
    </div>
  </div>
  <router-view v-else></router-view>
</template>

<style>

body {

  --bg-primary: #000000;
  --bg-secondary: #0d0d0d;
  --bg-sidebar: #0d0d0d;
  --bg-header: #0d0d0d;
  --bg-tertiary: #1a1a1a;

  --text-primary: #f0f6fc;
  --text-secondary: #c9d1d9;
  --text-muted: #8b949e;

  --border-color: #30363d;

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

  --hover-bg: rgba(255, 255, 255, 0.08);
  --selected-bg: rgba(255, 255, 255, 0.12);
  --input-bg: #000000;

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

  --danger-btn-bg: #f6f8fa;
  --danger-btn-text: #cf222e;
  --danger-btn-border: rgba(207, 34, 46, 0.4);
  --danger-btn-hover-bg: #cf222e;
  --danger-btn-hover-text: #ffffff;
  --danger-btn-hover-border: #cf222e;
}

body {
  margin: 0;
  padding: 0;
  font-family: system-ui, -apple-system, sans-serif;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding-top: 48px;
  padding-left: 240px;
  transition: padding-left 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.main-content.sidebar-collapsed {
  padding-left: 0;
}

::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-secondary, #161b22);
}

::-webkit-scrollbar-thumb {
  background: var(--border-color, #30363d);
  border-radius: 2px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted, #8b949e);
}

.btn-danger {
  background-color: var(--danger-btn-bg, #212830);
  color: var(--danger-btn-text, #f85149);
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
  padding: 8px 16px;
  border-radius: 2px;
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

.btn-danger i,
.btn-danger svg {
  color: inherit;
}
</style>
