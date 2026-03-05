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
      <button class="float-btn" @click.stop="openMainPage('/fileView')" title="云盘">
        <i class="ri-hard-drive-2-line"></i>
      </button>
      <button class="float-btn note-btn" @click.stop="toggleNoteMenu" title="笔记">
        <i class="ri-sticky-note-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/settings')" title="设置">
        <i class="ri-settings-3-line"></i>
      </button>
    </div>

    <!-- 笔记功能菜单 - 水平排布 -->
    <div v-if="showNoteMenu" class="note-menu" @click.stop>
      <div class="menu-item" @click="handleScreenshot">
        <i class="ri-screenshot-line"></i>
        <span>屏幕截图</span>
      </div>
      <div class="menu-item" @click="handleNoteManager">
        <i class="ri-folder-open-line"></i>
        <span>笔记管理</span>
      </div>
      <div class="menu-item back" @click="closeNoteMenu">
        <i class="ri-close-line"></i>
        <span>关闭</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, onBeforeUnmount } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'

const isConnected = ref(false)

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

// 笔记菜单显示状态
const showNoteMenu = ref(false)

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
  // 排除所有可点击元素，包括按钮、菜单项等
  if (e.target.closest('.float-btn') || 
      e.target.closest('.connection-status') ||
      e.target.closest('.menu-item') ||
      e.target.closest('.note-menu')) {
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

    // 检查主窗口是否存在且没有被关闭
    if (mainWindow) {
      try {
        // 尝试获取窗口状态，如果窗口已关闭会抛出错误
        await mainWindow.isVisible()

        console.log('主窗口存在，聚焦并导航')
        await mainWindow.setFocus()
        await mainWindow.emit('navigate', path)
        console.log('发送导航事件:', path)
      } catch (windowError) {
        // 窗口已关闭，需要重新创建
        console.log('主窗口已关闭，重新创建')
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
 * 切换笔记菜单显示
 */
function toggleNoteMenu() {
  showNoteMenu.value = !showNoteMenu.value
  console.log('笔记菜单状态:', showNoteMenu.value)
}

/**
 * 关闭笔记菜单
 */
function closeNoteMenu() {
  showNoteMenu.value = false
}

/**
 * 处理屏幕截图
 * 占位功能，后续实现
 */
function handleScreenshot() {
  console.log('屏幕截图功能（占位）')
  alert('屏幕截图功能开发中...')
  closeNoteMenu()
}

/**
 * 处理笔记管理
 * 打开主窗口的笔记管理页面
 */
async function handleNoteManager() {
  console.log('打开笔记管理')
  closeNoteMenu()
  await openMainPage('/notes')
}

/**
 * 处理点击菜单外部区域
 * 当菜单显示时，点击外部关闭菜单
 */
function handleClickOutside(event) {
  // 如果菜单显示，且点击的不是笔记按钮，则关闭菜单
  if (showNoteMenu.value && !event.target.closest('.note-btn')) {
    closeNoteMenu()
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

/* 笔记按钮激活状态 */
.note-btn.active {
  background-color: rgba(59, 130, 246, 0.15);
  color: #3b82f6;
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
  transform: translateY(-50%);
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
  padding: 8px 12px;
  z-index: 1001;
  display: flex;
  flex-direction: row;
  gap: 8px;
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

.menu-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: #333;
  font-size: 11px;
  min-width: 56px;
}

.menu-item:hover {
  background-color: #f5f5f5;
}

.menu-item i {
  font-size: 18px;
  color: #666;
}

.menu-item:hover i {
  color: #3b82f6;
}

.menu-item.back:hover {
  background-color: #fff5f5;
}

.menu-item.back:hover i {
  color: #ef4444;
}
</style>
