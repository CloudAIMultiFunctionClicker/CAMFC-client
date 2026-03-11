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
import { loadAppData, saveAppData } from '../../components/data/storage.js'

const theme = inject('theme')
const router = useRouter()

const showConfirmDialog = ref(false)
const rememberChoice = ref(false)
const closePreference = ref(null) // 'minimize' | 'exit' | null

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
  loadClosePreference()
})

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect()
  }
})

const loadClosePreference = async () => {
  try {
    const saved = await loadAppData('close_preference')
    console.log('[关闭偏好] 加载原始数据:', saved)
    if (saved) {
      const pref = JSON.parse(saved)
      closePreference.value = pref.preference
      console.log('[关闭偏好] 加载成功:', closePreference.value)
    } else {
      console.log('[关闭偏好] 无保存的偏好')
    }
  } catch (error) {
    console.error('[关闭偏好] 加载失败:', error)
  }
}

const saveClosePreference = async (preference) => {
  try {
    console.log('[关闭偏好] 准备保存:', preference)
    await saveAppData('close_preference', JSON.stringify({ preference }))
    console.log('[关闭偏好] 保存成功:', preference)
    
    // 验证保存
    const verify = await loadAppData('close_preference')
    console.log('[关闭偏好] 验证保存结果:', verify)
  } catch (error) {
    console.error('[关闭偏好] 保存失败:', error)
  }
}

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

const requestClose = async () => {
  // 每次都重新读取最新的偏好设置
  await loadClosePreference()
  
  // 根据偏好执行不同的操作
  if (closePreference.value === 'minimize') {
    // 直接隐藏到托盘
    hideToTray(true)
  } else if (closePreference.value === 'exit') {
    // 直接退出
    confirmClose(true)
  } else {
    // 'ask' 或者未设置，显示确认对话框
    showConfirmDialog.value = true
    rememberChoice.value = false
  }
}

const confirmClose = async (fromPreference = false) => {
  showConfirmDialog.value = false
  if (rememberChoice.value && !fromPreference) {
    await saveClosePreference('exit')
  }
  await closeApp()
}

const confirmCloseWithRemember = async () => {
  console.log('[关闭偏好] 点击完全关闭，复选框状态:', rememberChoice.value)
  await confirmClose(false)
}

const cancelClose = () => {
  showConfirmDialog.value = false
}

const hideToTrayWithRemember = async () => {
  console.log('[关闭偏好] 点击隐藏到托盘，复选框状态:', rememberChoice.value)
  await hideToTray(false)
}

const hideToTray = async (fromPreference = false) => {
  showConfirmDialog.value = false
  if (rememberChoice.value && !fromPreference) {
    await saveClosePreference('minimize')
  }
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
</script>

<template>
  <header class="title-bar">
    <div class="title-bar-content" @mousedown="startWindowDrag">
      <div class="title-left">
        <span class="app-title">CAMFC Cloud</span>
      </div>
      
      <div class="title-right">
        <button class="icon-btn home-btn" @click="goHome" title="主页">
          <Home :size="18" :stroke-width="3" />
        </button>
        
        <button class="icon-btn theme-btn" @click="theme?.toggleTheme" title="切换主题">
          <Moon v-if="theme?.isLightMode.value" :size="18" :stroke-width="2.5" />
          <Sun v-else :size="18" :stroke-width="2.5" />
        </button>
        
        <div class="window-controls">
          <button class="icon-btn window-btn" @click="minimizeWindow" title="最小化">
            <Minus :size="18" :stroke-width="3" />
          </button>
          <button class="icon-btn window-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
            <Copy v-if="isMaximized" :size="18" :stroke-width="3" class="restore-icon" />
            <Square v-else :size="18" :stroke-width="3" />
          </button>
          <button class="icon-btn window-btn close-btn" @click="requestClose" title="关闭">
            <X :size="18" :stroke-width="3" />
          </button>
        </div>
      </div>
    </div>
  </header>
  
  <Transition name="modal">
    <div v-if="showConfirmDialog" class="confirm-overlay" @click="cancelClose">
      <div class="confirm-dialog" @click.stop>
        <div class="confirm-header">
          <X class="confirm-icon" :size="24" :stroke-width="2.5" />
          <h3>关闭应用</h3>
        </div>
        <div class="confirm-body">
          <p>请选择关闭方式：</p>
          <div class="close-options">
            <button class="option-btn" @click="hideToTrayWithRemember">
              <div class="option-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                  <line x1="8" y1="21" x2="16" y2="21"/>
                  <line x1="12" y1="17" x2="12" y2="21"/>
                </svg>
              </div>
              <div class="option-content">
                <h4>隐藏到托盘</h4>
                <p>应用将在后台运行，可从托盘重新打开</p>
              </div>
            </button>
            <button class="option-btn" @click="confirmCloseWithRemember">
              <div class="option-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M18 6 6 18M6 6l12 12"/>
                  <rect x="3" y="3" width="18" height="18" rx="2"/>
                </svg>
              </div>
              <div class="option-content">
                <h4>完全关闭</h4>
                <p>应用将完全退出，需要重新启动</p>
              </div>
            </button>
          </div>
          <div class="remember-choice">
            <label class="checkbox-label">
              <input type="checkbox" v-model="rememberChoice" />
              <span>记住此次选择</span>
            </label>
            <p class="remember-tip">下次将直接执行，可在设置中修改</p>
          </div>
        </div>
        <div class="confirm-actions">
          <button class="cancel-btn" @click="cancelClose">取消</button>
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

.window-controls {
  display: flex;
  align-items: center;
  gap: 0;
  margin-left: 8px;
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
.confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.confirm-dialog {
  background-color: var(--bg-secondary);
  border-radius: 16px;
  width: 90%;
  max-width: 400px;
  border: 1px solid var(--border-color);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
  overflow: hidden;
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

.remember-choice {
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  font-size: 14px;
  color: var(--text-primary);
  user-select: none;
  padding: 8px 12px;
  border-radius: 8px;
  transition: background-color 0.2s ease;
}

.checkbox-label:hover {
  background-color: var(--hover-bg);
}

.checkbox-label input[type="checkbox"] {
  appearance: none;
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  border: 2px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-primary);
  cursor: pointer;
  position: relative;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.checkbox-label input[type="checkbox"]:hover {
  border-color: var(--accent-blue);
}

.checkbox-label input[type="checkbox"]:checked {
  background-color: var(--accent-blue);
  border-color: var(--accent-blue);
}

.checkbox-label input[type="checkbox"]:checked::after {
  content: '';
  position: absolute;
  left: 5px;
  top: 1px;
  width: 4px;
  height: 9px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.remember-tip {
  margin: 0;
  padding: 0 0 0 28px;
  font-size: 12px;
  color: var(--text-muted);
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

/* 对话框动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .confirm-dialog,
.modal-leave-active .confirm-dialog {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.modal-enter-from .confirm-dialog,
.modal-leave-to .confirm-dialog {
  transform: scale(0.9);
  opacity: 0;
}
</style>
