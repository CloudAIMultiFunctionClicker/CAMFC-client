<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top

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
      <button class="float-btn" @click.stop="openMainPage('/main')">
        <i class="ri-cloud-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/notes')">
        <i class="ri-sticky-note-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/settings')">
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
  justify-content: space-between;
  padding: 0 10px;
  gap: 8px;
  background-color: var(--bg-secondary, rgba(30, 41, 59, 0.95));
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  cursor: move;
  user-select: none;
}

.float-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary, #f8fafc);
  white-space: nowrap;
  line-height: 1;
}

.connection-status {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 6px;
  background-color: rgba(255, 107, 107, 0.9);
  color: white;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
  line-height: 1.2;
}

.connection-status:hover {
  opacity: 0.8;
}

.connection-status.connected {
  background-color: rgba(85, 170, 85, 0.9);
}

.connection-status.connected:hover {
  opacity: 0.8;
}

.float-buttons {
  display: flex;
  gap: 4px;
}

.float-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  font-size: 14px;
  background-color: transparent;
  color: var(--text-secondary, #94a3b8);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  line-height: 1;
}

.float-btn:hover {
  background-color: rgba(59, 130, 246, 0.3);
  color: #3b82f6;
}
</style>
