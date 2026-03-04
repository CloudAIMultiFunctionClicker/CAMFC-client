<template>
  <div class="float-container" @mousedown="startDrag">
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
        <i class="ri-hard-drive-2-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/notes')" title="笔记">
        <i class="ri-sticky-note-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/settings')" title="设置">
        <i class="ri-settings-3-line"></i>
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'

const isConnected = ref(false)

let keepOnTopInterval = null

onMounted(async () => {
  console.log('FloatView mounted')
  
  const unlisten = await listen('connection-status', (event) => {
    console.log('收到连接状态事件:', event.payload)
    isConnected.value = event.payload
  })
  
  keepOnTopInterval = setInterval(async () => {
    try {
      const floatWindow = await getCurrentWindow()
      await floatWindow.setAlwaysOnTop(true)
    } catch (e) {
      console.error('保持置顶失败:', e)
    }
  }, 5000)
  
  onUnmounted(() => {
    unlisten()
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
  if (e.target.closest('.float-btn') || e.target.closest('.connection-status')) {
    return
  }
  try {
    const floatWindow = await getCurrentWindow()
    await floatWindow.startDragging()
  } catch (e) {
    console.error('拖动失败:', e)
  }
}

async function openMainPage(path) {
  console.log('点击按钮，目标是:', path)
  
  try {
    const mainWindow = await WebviewWindow.getByLabel('main')
    
    if (mainWindow) {
      console.log('聚焦主窗口')
      await mainWindow.setFocus()
      
      await mainWindow.emit('navigate', path)
      console.log('发送导航事件:', path)
    } else {
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
  background-color: #f5f5f5;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  cursor: move;
  user-select: none;
}

.float-title {
  font-size: 13px;
  font-weight: 500;
  color: #333;
  white-space: nowrap;
  line-height: 1;
}

.connection-status {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
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
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  font-size: 13px;
  background-color: transparent;
  color: #666;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  line-height: 1;
}

.float-btn:hover {
  background-color: rgba(0, 0, 0, 0.06);
  color: #333;
}
</style>
