<script setup>
import { ref, provide, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useBluetoothStore } from './stores/bluetooth.js'
import { showToast } from './components/layout/showToast.js'
import TitleBar from './components/layout/TitleBar.vue'
import { initBackendConfig } from './config/backend.js'

const route = useRoute()
const router = useRouter()
const bluetoothStore = useBluetoothStore()

const isFloatPage = computed(() => route.path === '/float')

// TOTP 刷新
let totpRefreshInterval = null
const TOTP_REFRESH_INTERVAL = 30000

// 禁用 Ctrl+R 等快捷键
document.addEventListener('keydown', (e) => {
  if (e.ctrlKey && (e.key === 'r' || e.key === 'p' || e.key === 'h' || e.key === 'z' || e.key === 'f')) {
    e.preventDefault()
  }
})

// 主题管理 - 默认跟随系统
const getInitialTheme = () => {
  const savedTheme = localStorage.getItem('theme-preference')
  if (savedTheme === 'light' || savedTheme === 'dark') {
    return savedTheme === 'light'
  }
  
  const prefersLight = window.matchMedia('(prefers-color-scheme: light)').matches
  return prefersLight
}

const isLightMode = ref(getInitialTheme())

// 切换主题
const toggleTheme = async () => {
  isLightMode.value = !isLightMode.value
  updateBodyClass()
  localStorage.setItem('theme-preference', isLightMode.value ? 'light' : 'dark')
  
  try {
    const floatWindow = await WebviewWindow.getByLabel('float')
    if (floatWindow) {
      await floatWindow.emit('theme-changed', isLightMode.value ? 'light' : 'dark')
    }
  } catch (e) {
    console.log('发送主题变化事件失败:', e)
  }
}

const updateBodyClass = () => {
  document.body.classList.toggle('light-mode', isLightMode.value)
}

// 蓝牙扫描（保留兼容性）
const scanBluetooth = async () => {
  try {
    showToast('开始扫描蓝牙设备...')
    const { scanDevices, findCpenDevices } = await import('./components/data/bluetooth')
    const devices = await scanDevices()
    const cpenDevices = findCpenDevices(devices)
    
    showToast(`扫描完成，发现 ${devices.length} 个设备，${cpenDevices.length} 个 Cpen 设备`)
    
    if (cpenDevices.length > 0) {
      showToast(`发现 Cpen 设备：${cpenDevices[0].displayInfo}`)
    }
    
    return { devices, cpenDevices }
  } catch (error) {
    console.error('蓝牙扫描失败:', error)
    showToast('蓝牙扫描失败')
    return { devices: [], cpenDevices: [] }
  }
}

provide('theme', { isLightMode, toggleTheme })

// TOTP 定时刷新
const startTotpRefresh = async () => {
  if (totpRefreshInterval) clearInterval(totpRefreshInterval)

  totpRefreshInterval = setInterval(async () => {
    try {
      const { getTotp } = await import('./components/data/bluetooth')
      await getTotp()
      console.log('[TOTP] 后台缓存刷新成功')
    } catch (error) {
      console.warn('[TOTP] 缓存刷新失败:', error.message)
    }
  }, TOTP_REFRESH_INTERVAL)

  console.log(`[TOTP] 已启动定时刷新，每${TOTP_REFRESH_INTERVAL / 1000}秒一次`)
}

const stopTotpRefresh = () => {
  if (totpRefreshInterval) {
    clearInterval(totpRefreshInterval)
    totpRefreshInterval = null
    console.log('[TOTP] 已停止刷新')
  }
}

// 监听蓝牙连接状态
watch(() => bluetoothStore.isConnected(), (connected) => {
  if (connected) {
    console.log('[TOTP] 设备已连接，启动刷新')
    startTotpRefresh()
  } else {
    console.log('[TOTP] 设备已断开，停止刷新')
    stopTotpRefresh()
  }
})

// 组件挂载
onMounted(async () => {
  updateBodyClass()
  
  // 延迟初始化后端
  setTimeout(async () => {
    await initBackendConfig()
  }, 100)
  
  let buttonEventUnlisten = null
  let bluetoothDisconnectUnlisten = null
  let navigateEventUnlisten = null
  
  // 监听蓝牙按键事件
  const { listen } = await import('@tauri-apps/api/event')
  buttonEventUnlisten = await listen('button-event', async (event) => {
    if (route.path === '/float') return
    
    const eventType = event.payload.event_type
    
    if (eventType === 'button_press') {
      showToast('GPIO10 按下', '#3b82f6')
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: true } }))
    } else if (eventType === 'button_release') {
      showToast('GPIO10 松开', '#10b981')
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: false } }))
      try {
        const { pressWinKey } = await import('./components/data/bluetooth')
        await pressWinKey()
      } catch (e) {
        console.error('右箭头键模拟失败:', e)
      }
    } else if (eventType === 'button_press_left') {
      showToast('GPIO9 按下', '#8b5cf6')
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: true } }))
    } else if (eventType === 'button_release_left') {
      showToast('GPIO9 松开', '#f59e0b')
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: false } }))
      try {
        const { pressLeftKey } = await import('./components/data/bluetooth')
        await pressLeftKey()
      } catch (e) {
        console.error('左箭头键模拟失败:', e)
      }
    }
  })
  
  // 监听截图命令
  const screenshotUnlisten = await listen('screenshot-command', async () => {
    if (route.path === '/float') return
    console.log('收到截图命令')
    showToast('触发截图', '#3b82f6')
  })
  
  // 监听打开 note 页面
  const showNoteUnlisten = await listen('show-note-command', async () => {
    if (route.path === '/float') return
    console.log('收到打开 note 命令')
    showToast('打开笔记', '#10b981')
    
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
        await webview.emit('navigate', '/notes')
      }
    } catch (e) {
      console.error('打开 note 页面失败:', e)
    }
  })
  
  // 监听打开云盘页面
  const openCloudUnlisten = await listen('open-cloud-command', async () => {
    if (route.path === '/float') return
    console.log('收到打开云盘命令')
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
      console.error('打开云盘失败:', e)
    }
  })
  
  // 蓝牙断开处理
  let isShowingDisconnectDialog = false
  
  const checkConnectionStatus = async () => {
    try {
      const { isConnected } = await import('./components/data/bluetooth')
      const connected = await isConnected()
      
      if (bluetoothStore.isConnected() && !connected) {
        if (isShowingDisconnectDialog) return
        isShowingDisconnectDialog = true
        
        bluetoothStore.reset()
        const userConfirmed = await showDisconnectConfirm()
        
        if (userConfirmed) {
          window.location.reload()
        }
      }
    } catch (error) {
      console.log('检查连接状态出错:', error)
    }
  }
  
  bluetoothDisconnectUnlisten = await listen('bluetooth-disconnect', async () => {
    console.log('收到蓝牙断开事件')
    
    if (bluetoothStore.isConnected()) {
      if (isShowingDisconnectDialog) return
      isShowingDisconnectDialog = true
      
      bluetoothStore.reset()
      const userConfirmed = await showDisconnectConfirm()
      
      if (userConfirmed) {
        window.location.reload()
      }
    }
  })
  
  const showDisconnectConfirm = async () => {
    return new Promise((resolve) => {
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
      
      const style = document.createElement('style')
      style.textContent = `
        .disconnect-dialog {
          position: fixed;
          top: 0; left: 0; right: 0; bottom: 0;
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
  
  let connectionCheckInterval = null
  connectionCheckInterval = setInterval(checkConnectionStatus, 2000)
  
  // 监听系统主题变化
  const lightMediaQuery = window.matchMedia('(prefers-color-scheme: light)')
  const handleSystemThemeChange = (e) => {
    const hasUserPreference = localStorage.getItem('theme-preference') !== null
    if (!hasUserPreference) {
      isLightMode.value = e.matches
      updateBodyClass()
    }
  }
  lightMediaQuery.addEventListener('change', handleSystemThemeChange)
  
  // 监听悬浮窗导航事件
  try {
    navigateEventUnlisten = await listen('navigate', (event) => {
      const path = event.payload
      if (path && router && route.path !== '/float') {
        router.push(path)
      }
    })
  } catch (e) {
    console.log('监听导航事件失败:', e)
  }
  
  // 监听主题查询
  try {
    await listen('get-theme', async () => {
      const floatWindow = await WebviewWindow.getByLabel('float')
      if (floatWindow) {
        await floatWindow.emit('theme-changed', isLightMode.value ? 'light' : 'dark')
      }
    })
  } catch (e) {
    console.log('监听主题查询失败:', e)
  }
  
  // 清理
  onUnmounted(() => {
    stopTotpRefresh()
    lightMediaQuery.removeEventListener('change', handleSystemThemeChange)
    if (connectionCheckInterval) clearInterval(connectionCheckInterval)
    if (buttonEventUnlisten) buttonEventUnlisten()
    if (bluetoothDisconnectUnlisten) bluetoothDisconnectUnlisten()
    if (navigateEventUnlisten) navigateEventUnlisten()
    if (screenshotUnlisten) screenshotUnlisten()
    if (showNoteUnlisten) showNoteUnlisten()
    if (openCloudUnlisten) openCloudUnlisten()
  })
  
  console.log('应用启动完成')
})
</script>

<template>
  <div class="app-container" v-if="!isFloatPage">
    <TitleBar />
    <div class="main-content">
      <router-view></router-view>
    </div>
  </div>
  <router-view v-else></router-view>
</template>

<style>
/* GitHub 风格深色主题 */
body {
  --bg-primary: #0d1117;
  --bg-secondary: #161b22;
  --bg-sidebar: #161b22;
  --bg-header: #161b22;
  --bg-tertiary: #21262d;
  
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
  
  --hover-bg: rgba(240, 246, 252, 0.1);
  --selected-bg: rgba(56, 139, 253, 0.15);
  --input-bg: #0d1117;
  
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

/* GitHub 风格亮色主题 */
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
  overflow: hidden;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow: hidden;
}

/* 滚动条 */
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

/* 警告按钮 */
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

.btn-danger i,
.btn-danger svg {
  color: inherit;
}
</style>
