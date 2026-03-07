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
        <i class="ri-cloud-line"></i>
      </button>
      <button class="float-btn note-btn" @click.stop="toggleNoteMenu" title="笔记">
        <i class="ri-sticky-note-line"></i>
      </button>
      <button class="float-btn" @click.stop="openMainPage('/settings')" title="设置">
        <i class="ri-settings-3-line"></i>
      </button>
      <button v-if="!isMainWindowVisible" class="float-btn open-main-btn" @click.stop="openMainWindow" title="打开主窗口">
        <i class="ri-home-2-line"></i>
        <span class="btn-text">主窗口</span>
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
import { ref, onMounted, onUnmounted, onBeforeUnmount } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getCurrentWindow, Window } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../components/layout/showToast.js'

const isConnected = ref(false)
const isMainWindowVisible = ref(true)
const showConnectTip = ref(false)
let connectTipTimer = null

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
let visibilityCheckInterval = null

onMounted(async () => {
  console.log('FloatView mounted')

  const unlisten = await listen('connection-status', (event) => {
    console.log('收到连接状态事件:', event.payload)
    isConnected.value = event.payload
  })

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

  // 保持置顶（每5秒执行一次）
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
 * 先隐藏主窗口，截图完成后再打开显示截图结果
 */
async function handleScreenshot() {
  console.log('开始截图流程')
  closeNoteMenu()

  try {
    // 获取主窗口
    const mainWindow = await Window.getByLabel('main')
    let wasVisible = false

    // 如果主窗口存在且可见，先隐藏它
    if (mainWindow) {
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
      console.log('截图成功，打开主窗口显示截图')
      // 截图成功，打开主窗口并传递截图数据
      await openScreenshotWindow(result)
    } else {
      console.error('截图失败:', result.error)
      // 截图失败，如果之前窗口是可见的，恢复显示
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
    // 获取主窗口
    let mainWindow = await Window.getByLabel('main')

    if (!mainWindow) {
      // 创建新窗口
      console.log('创建新的主窗口来显示截图')
      const webview = new WebviewWindow('main', {
        url: '/screenshot',
        title: 'CAMFC Cloud',
        width: 1152,
        height: 648,
        center: true
      })

      // 等待窗口创建完成后再发送截图数据
      webview.once('tauri://created', async () => {
        console.log('截图窗口创建成功，发送截图数据')
        const webviewWindow = await WebviewWindow.getByLabel('main')
        if (webviewWindow) {
          await webviewWindow.emit('screenshot-data', screenshotData)
        }
      })

      webview.once('tauri://error', (e) => {
        console.error('截图窗口创建失败:', e)
      })
    } else {
      // 窗口已存在，显示并导航到截图页面
      console.log('使用现有主窗口显示截图')
      await mainWindow.show()
      await mainWindow.unminimize()
      await mainWindow.center()
      await mainWindow.setFocus()

      // 发送导航事件
      const webview = await WebviewWindow.getByLabel('main')
      if (webview) {
        await webview.emit('navigate', '/screenshot')
        // 发送截图数据
        await webview.emit('screenshot-data', screenshotData)
      }
    }
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
  min-width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 6px;
  font-size: 13px;
  background-color: transparent;
  color: #666;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  line-height: 1;
  gap: 4px;
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

.menu-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
  gap: 6px;
  padding: 6px 10px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: #333;
  font-size: 11px;
  white-space: nowrap;
}

.menu-item:hover {
  background-color: #f5f5f5;
}

.menu-item i {
  font-size: 16px;
  color: #666;
  flex-shrink: 0;
}

.menu-item:hover i {
  color: #3b82f6;
}

.menu-item span {
  font-size: 11px;
  color: #333;
}

.menu-item.back:hover {
  background-color: #fff5f5;
}

.menu-item.back:hover i {
  color: #ef4444;
}

/* 未连接提示框 - 居中显示，白底黑字 */
.connect-tip {
  position: fixed;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  background-color: #ffffff;
  border: 1px solid #e5e5e5;
  border-radius: 8px;
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
  color: #737373;
}

.connect-tip span {
  font-size: 13px;
  font-weight: 500;
  color: #171717;
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
    transform: translate(-50%, -50%) scale(1);
  }
}

@keyframes tip-fade-out {
  0% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.9);
  }
}
</style>
