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

<script setup>
import { inject, ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Moon, Sun, Minus, Square, Copy, X, Home } from 'lucide-vue-next'
import { useRouter } from 'vue-router'

const theme = inject('theme')
const router = useRouter()

const showConfirmDialog = ref(false)
const showCloseRipple = ref(false)
const closeRippleStyle = ref({})

const currentWindow = getCurrentWindow()

const isMaximized = ref(false)

const checkWindowState = async () => {
  try {
    isMaximized.value = await currentWindow.isMaximized()
  } catch (error) {
    console.error('检查窗口状态失败:', error)
  }
}

let resizeObserver = null
const setupResizeObserver = () => {
  resizeObserver = new ResizeObserver(() => {
    checkWindowState()
  })
  resizeObserver.observe(document.body)
}

onMounted(() => {
  checkWindowState()
  setupResizeObserver()
})

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect()
  }
})

const minimizeWindow = async () => {
  try {
    await currentWindow.minimize()
  } catch (error) {
    console.error('最小化窗口失败:', error)
  }
}

const toggleMaximize = async () => {
  try {
    if (isMaximized.value) {
      await currentWindow.unmaximize()
    } else {
      await currentWindow.maximize()
    }
    isMaximized.value = !isMaximized.value
  } catch (error) {
    console.error('切换最大化失败:', error)
  }
}

const closeApp = async () => {
  try {
    await invoke('exit_app')
  } catch (error) {
    console.error('退出应用失败:', error)
  }
}

const requestClose = () => {
  // 获取关闭按钮的位置
  const closeBtn = document.querySelector('.close-btn')
  if (closeBtn) {
    const rect = closeBtn.getBoundingClientRect()
    // 计算波纹中心位置（相对于视口）
    const centerX = rect.left + rect.width / 2
    const centerY = rect.top + rect.height / 2
    
    // 计算需要的最大扩散半径（覆盖整个窗口）
    const windowWidth = window.innerWidth
    const windowHeight = window.innerHeight
    const maxRadius = Math.sqrt(Math.pow(windowWidth, 2) + Math.pow(windowHeight, 2))
    
    // 设置波纹样式
    closeRippleStyle.value = {
      left: centerX + 'px',
      top: centerY + 'px',
      '--ripple-radius': maxRadius + 'px'
    }
    
    // 触发扩散动画
    showCloseRipple.value = true
    
    // 延迟显示确认对话框
    setTimeout(() => {
      showConfirmDialog.value = true
    }, 100)
  } else {
    showConfirmDialog.value = true
  }
}

const confirmClose = async () => {
  showConfirmDialog.value = false
  await closeApp()
}

const cancelClose = () => {
  showConfirmDialog.value = false
  showCloseRipple.value = false
}

const hideToTray = async () => {
  showConfirmDialog.value = false
  try {
    await currentWindow.hide()
  } catch (error) {
    console.error('隐藏窗口失败:', error)
  }
}

const startWindowDrag = async (event) => {
  if (event.button !== 0) return
  if (event.target.closest('button, .window-controls, .home-btn')) return
  
  try {
    await currentWindow.startDragging()
  } catch (error) {
    console.error('拖动窗口失败:', error)
  }
}

const goHome = () => {
  router.push('/main')
}

const showDevelopingToast = () => {
  alert('下载进度功能开发中')
}
</script>

<template>
  <header class="title-bar">
    <div class="title-bar-content" @mousedown="startWindowDrag">
      <div class="title-left">
        <span class="app-title">CAMFC Cloud</span>
        <button class="icon-btn home-btn" @click="goHome" title="主页">
          <Home :size="18" :stroke-width="3" />
        </button>
      </div>
      
      <div class="title-right">
        <button class="icon-btn tray-btn" @click="hideToTray" title="隐藏到托盘">
          <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-picture-in-picture-icon lucide-picture-in-picture"><path d="M2 10h6V4"/><path d="m2 4 6 6"/><path d="M21 10V7a2 2 0 0 0-2-2h-7"/><path d="M3 14v2a2 2 0 0 0 2 2h3"/><rect x="12" y="14" width="10" height="7" rx="1"/></svg>
        </button>
        
        <button class="icon-btn download-btn" @click="showDevelopingToast" title="下载记录">
          <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-download-icon lucide-download"><path d="M12 15V3"/><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/></svg>
        </button>
        
        <button class="icon-btn theme-btn" @click="theme?.toggleTheme" title="切换主题">
          <Moon v-if="theme?.isLightMode.value" :size="18" :stroke-width="2.5" />
          <Sun v-else :size="18" :stroke-width="2.5" />
        </button>
        
        <div class="divider"></div>
        
        <div class="window-controls">
          <button class="icon-btn window-btn" @click="minimizeWindow" title="最小化">
            <Minus :size="18" :stroke-width="3" />
          </button>
          <button class="icon-btn window-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
            <Copy v-if="isMaximized" :size="15" :stroke-width="3" class="restore-icon" />
            <Square v-else :size="15" :stroke-width="3" />
          </button>
          <button ref="closeBtnRef" class="icon-btn window-btn close-btn" @click="requestClose" title="关闭">
            <X :size="18" :stroke-width="3" />
          </button>
        </div>
      </div>
    </div>
  </header>
  
  <!-- 窗口关闭扩散波纹 -->
  <Transition name="ripple-fade">
    <div v-if="showCloseRipple" class="close-ripple" :style="closeRippleStyle"></div>
  </Transition>
  
  <Transition name="confirm">
    <div v-if="showConfirmDialog" class="confirm-container" @click="cancelClose">
      <div class="ripple-effect" @click.stop></div>
      <div class="confirm-dialog" @click.stop>
        <div class="confirm-body">
          <p>你点击了关闭按钮，你希望？</p>
          <div class="close-options">
            <button class="option-btn" @click="hideToTray">
              <div class="option-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-picture-in-picture-icon lucide-picture-in-picture"><path d="M2 10h6V4"/><path d="m2 4 6 6"/><path d="M21 10V7a2 2 0 0 0-2-2h-7"/><path d="M3 14v2a2 2 0 0 0 2 2h3"/><rect x="12" y="14" width="10" height="7" rx="1"/></svg>
              </div>
              <div class="option-content">
                <h4>隐藏到托盘</h4>
                <p>应用将在后台运行，可从托盘重新打开</p>
              </div>
            </button>
            <button class="option-btn" @click="confirmClose">
              <div class="option-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-circle-x-icon lucide-circle-x"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>
              </div>
              <div class="option-content">
                <h4>完全关闭</h4>
                <p>应用将完全退出，需要重新启动</p>
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.title-bar {
  width: 100%;
  height: 48px;
  flex-shrink: 0;
  background: var(--bg-header, linear-gradient(135deg, #0f172a 0%, #1e293b 100%));
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  position: relative;
  z-index: 1000;
  transition: background 0.3s ease, border-color 0.3s ease;
  -webkit-app-region: drag;
}

.title-bar-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  height: 100%;
  padding: 0 24px;
  box-sizing: border-box;
}

.title-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 0 0 auto;
  min-width: 200px;
}

.app-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f8fafc);
  letter-spacing: 0.5px;
  white-space: nowrap;
}

.title-right {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  justify-content: flex-end;
}

.icon-btn {
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  color: var(--text-secondary, #cbd5e1);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  font-size: 16px;
  border-radius: 6px;
  -webkit-app-region: no-drag;
}

.icon-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-primary, #f8fafc);
}

.theme-btn:hover {
  background-color: var(--accent-blue, #3b82f6);
  color: white;
}

.tray-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--accent-blue, #3b82f6);
}

.download-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--accent-green, #10b981);
}

.divider {
  width: 1px;
  height: 24px;
  background-color: var(--border-color, rgba(255, 255, 255, 0.1));
  margin: 0 4px;
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 0;
  margin-left: 8px;
  margin-right: -16px;
}

.window-btn.close-btn:hover {
  background-color: #ef4444;
  color: white;
}

.restore-icon {
  transform: scaleX(-1);
}

@media (max-width: 768px) {
  .title-bar {
    height: 36px;
  }
  
  .icon-btn {
    width: 28px;
    height: 28px;
    font-size: 14px;
  }
  
  .app-title {
    font-size: 13px;
  }
}

/* 关闭确认对话框样式 */
.confirm-container {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
}

/* 窗口关闭扩散波纹 */
.close-ripple {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 10000;
}

.close-ripple::before {
  content: '';
  position: absolute;
  width: 20px;
  height: 20px;
  background: radial-gradient(circle, rgba(59, 130, 246, 0.3) 0%, rgba(59, 130, 246, 0.15) 40%, transparent 70%);
  border-radius: 50%;
  transform: translate(-50%, -50%) scale(0);
  animation: closeRippleExpand 0.4s cubic-bezier(0.4, 0, 0.2, 1) forwards;
}

@keyframes closeRippleExpand {
  0% {
    transform: translate(-50%, -50%) scale(0);
    opacity: 1;
  }
  100% {
    transform: translate(-50%, -50%) scale(50);
    opacity: 0;
  }
}

/* 波纹淡入淡出 */
.ripple-fade-enter-active,
.ripple-fade-leave-active {
  transition: opacity 0.4s ease;
}

.ripple-fade-enter-from,
.ripple-fade-leave-to {
  opacity: 0;
}

/* 扩散波纹效果 */
.ripple-effect {
  position: absolute;
  top: 56px;
  right: 24px;
  width: 320px;
  height: 280px;
  border-radius: 12px;
  background: radial-gradient(circle, rgba(59, 130, 246, 0.12) 0%, transparent 70%);
  animation: rippleExpand 0.25s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  pointer-events: none;
}

@keyframes rippleExpand {
  0% {
    transform: scale(0);
    opacity: 0;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

.confirm-dialog {
  position: absolute;
  top: 56px;
  right: 24px;
  background-color: var(--bg-secondary);
  border-radius: 12px;
  width: 320px;
  border: 1px solid var(--border-color);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  animation: dialogFadeIn 0.2s cubic-bezier(0.4, 0, 0.2, 1) forwards;
}

@keyframes dialogFadeIn {
  0% {
    opacity: 0;
    transform: scale(0.92);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}

.confirm-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 24px 24px 16px;
  border-bottom: 1px solid var(--border-color);
}

.confirm-icon {
  color: #ef4444;
  flex-shrink: 0;
}

.confirm-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.confirm-body {
  padding: 24px;
}

.confirm-body p {
  margin: 0 0 16px;
  font-size: 15px;
  color: var(--text-primary);
  line-height: 1.5;
}

.close-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.option-btn {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  width: 100%;
  padding: 16px;
  background-color: var(--bg-primary);
  border: 2px solid var(--border-color);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
}

.option-btn:hover {
  border-color: var(--accent-blue);
  background-color: var(--hover-bg);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.option-icon {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-secondary);
  border-radius: 10px;
  color: var(--accent-blue);
}

.option-icon svg {
  width: 24px;
  height: 24px;
}

.option-content {
  flex: 1;
}

.option-content h4 {
  margin: 0 0 6px;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.option-content p {
  margin: 0;
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.4;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--border-color);
}

.cancel-btn,
.confirm-btn {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.cancel-btn {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.cancel-btn:hover {
  background-color: var(--hover-bg);
}

/* 对话框进入动画 - 干净利落的效果 */
.confirm-enter-active {
  transition: all 0.15s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.confirm-leave-active {
  transition: all 0.12s cubic-bezier(0.4, 0, 0.2, 1);
}

.confirm-enter-from,
.confirm-leave-to {
  opacity: 0;
}

.confirm-enter-from .confirm-dialog {
  opacity: 0;
  transform: scale(0.9);
}

.confirm-leave-to .confirm-dialog {
  opacity: 0;
  transform: scale(0.98);
}
</style>
