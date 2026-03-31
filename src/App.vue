<script setup>
import { ref, provide, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useBluetoothStore } from './stores/bluetooth.js'

import TitleBar from './components/layout/TitleBar.vue'
import { useTheme } from './composables/useTheme.js'
import { useBluetooth } from './composables/useBluetooth.js'

const route = useRoute()
const router = useRouter()

const isFloatPage = computed(() => route.path === '/float')

const { isLightMode, toggleTheme, initTheme } = useTheme()
const bluetoothStore = useBluetoothStore()
const { startTotpRefresh, stopTotpRefresh } = useBluetooth()

let buttonEventUnlisten = null
let bluetoothDisconnectUnlisten = null
let navigateEventUnlisten = null
let screenshotUnlisten = null
let showNoteUnlisten = null
let openCloudUnlisten = null
let connectionCheckInterval = null
let isShowingDisconnectDialog = false

provide('theme', {
  isLightMode,
  toggleTheme
})

watch(() => bluetoothStore.isConnected(), (connected) => {
  if (connected) {
    startTotpRefresh()
  } else {
    stopTotpRefresh()
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
        position: fixed; top: 0; left: 0; right: 0; bottom: 0;
        background-color: rgba(0, 0, 0, 0.6);
        display: flex; align-items: center; justify-content: center;
        z-index: 99999; backdrop-filter: blur(4px);
      }
      .disconnect-dialog-content {
        background-color: var(--bg-secondary, #1e293b);
        border-radius: .375rem; padding: 32px;
        max-width: 400px; text-align: center;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
        border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
      }
      .disconnect-dialog-content h3 {
        font-size: 20px; font-weight: 600;
        color: var(--text-primary, #f1f5f9); margin: 0 0 12px 0;
      }
      .disconnect-dialog-content p {
        font-size: 14px; color: var(--text-secondary, #94a3b8);
        margin: 0 0 24px 0;
      }
      .disconnect-dialog-actions { display: flex; gap: 12px; justify-content: center; }
      .disconnect-btn {
        padding: 10px 32px; border-radius: .375rem;
        font-size: 14px; font-weight: 500; cursor: pointer;
        transition: all 0.2s; border: none;
      }
      .disconnect-btn.confirm { background-color: var(--accent-blue, #3b82f6); color: #fff; }
      .disconnect-btn.confirm:hover { background-color: #2563eb; }
    `

    document.head.appendChild(style)
    document.body.appendChild(dialog)

    const confirmBtn = dialog.querySelector('.confirm')
    confirmBtn.addEventListener('click', () => {
      dialog.remove()
      style.remove()
      resolve(true)
    })
  })
}

const checkConnectionStatus = async () => {
  try {
    // 只有在 store 显示已连接时才检查实际连接状态
    if (bluetoothStore.bluetoothStatus !== 'connected') {
      return
    }

    const { isConnected } = await import('./components/data/bluetooth.js')
    const connected = await isConnected()

    // 连续检查 3 次，避免偶然的检查失败
    if (!connected) {
      console.log('[连接检查] 实际连接状态为 false，等待 500ms 后重试')
      await new Promise(resolve => setTimeout(resolve, 500))
      const retryConnected = await isConnected()
      if (!retryConnected) {
        console.log('[连接检查] 重试后仍为 false，等待 500ms 后再次重试')
        await new Promise(resolve => setTimeout(resolve, 500))
        const finalConnected = await isConnected()
        if (!finalConnected) {
          console.warn('[连接检查] 3 次检查均失败，判定为断开连接')
          // 3 次检查都失败，才判定为断开
          if (isShowingDisconnectDialog) return

          isShowingDisconnectDialog = true
          bluetoothStore.reset()

          const userConfirmed = await showDisconnectConfirm()
          if (userConfirmed) {
            window.location.reload()
          }
        }
      } else {
        console.log('[连接检查] 重试成功，连接正常')
      }
    }
  } catch (error) {
    console.log('[连接检查] 检查出错:', error)
  }
}

onMounted(async () => {
  initTheme()

  const { initBackendConfig } = await import('./config/backend.js')
  setTimeout(async () => {
    await initBackendConfig()
  }, 100)

  const { listen } = await import('@tauri-apps/api/event')

  buttonEventUnlisten = await listen('button-event', async (event) => {
    if (route.path === '/float') return

    const eventType = event.payload.event_type

    if (eventType === 'button_press') {
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: true } }))
    } else if (eventType === 'button_release') {
      window.dispatchEvent(new CustomEvent('button-state', { detail: { pressed: false } }))
      try {
        const { pressWinKey } = await import('./components/data/bluetooth.js')
        await pressWinKey()
      } catch (e) {
        console.error('右箭头键模拟失败:', e)
      }
    } else if (eventType === 'button_press_left') {
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: true } }))
    } else if (eventType === 'button_release_left') {
      window.dispatchEvent(new CustomEvent('button-state-left', { detail: { pressed: false } }))
      try {
        const { pressLeftKey } = await import('./components/data/bluetooth.js')
        await pressLeftKey()
      } catch (e) {
        console.error('左箭头键模拟失败:', e)
      }
    }
  })

  screenshotUnlisten = await listen('screenshot-command', async () => {
    if (route.path === '/float') return
    console.log('收到截图命令（0x12）')
  })

  showNoteUnlisten = await listen('show-note-command', async () => {
    if (route.path === '/float') return
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

  openCloudUnlisten = await listen('open-cloud-command', async () => {
    if (route.path === '/float') return
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

  bluetoothDisconnectUnlisten = await listen('bluetooth-disconnect', async () => {
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

  connectionCheckInterval = setInterval(checkConnectionStatus, 2000)

  try {
    navigateEventUnlisten = await listen('navigate', (event) => {
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
})

onUnmounted(() => {
  stopTotpRefresh()

  if (connectionCheckInterval) {
    clearInterval(connectionCheckInterval)
  }
  if (buttonEventUnlisten) buttonEventUnlisten()
  if (bluetoothDisconnectUnlisten) bluetoothDisconnectUnlisten()
  if (navigateEventUnlisten) navigateEventUnlisten()
  if (screenshotUnlisten) screenshotUnlisten()
  if (showNoteUnlisten) showNoteUnlisten()
  if (openCloudUnlisten) openCloudUnlisten()
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
