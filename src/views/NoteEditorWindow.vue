<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<template>
  <div class="editor-window" :class="{ 'light-mode': isLightMode }">
    <div class="editor-header" data-tauri-drag-region>
      <input 
        v-model="noteTitle" 
        class="editor-title-input" 
        placeholder="未命名笔记"
        type="text"
      />
      <div class="editor-actions">
        <button class="action-btn window-btn" @click="minimizeWindow" title="最小化">
          <i class="ri-subtract-line"></i>
        </button>
        <button class="action-btn window-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
          <i :class="isMaximized ? 'ri-fullscreen-exit-line' : 'ri-fullscreen-line'"></i>
        </button>
        <button class="action-btn save-btn" @click="saveAndClose" title="保存">
          <i class="ri-check-line"></i>
        </button>
        <button class="action-btn close-btn" @click="handleClose" title="关闭">
          <i class="ri-close-line"></i>
        </button>
      </div>
    </div>
    
    <div class="editor-body-wrapper">
      <GenericNoteEditor
        ref="genericEditor"
        v-model="noteContent"
        v-model:title="noteTitle"
        :show-title="false"
        :is-light-mode="isLightMode"
        @save="handleSaveShortcut"
      />
    </div>
    
    <!-- 保存确认弹窗 -->
    <Transition name="modal">
      <div v-if="showSaveConfirmModal" class="modal-overlay" @click="cancelClose">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-save-line"></i> 保存更改</h3>
            <button class="close-btn" @click="cancelClose">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body save-modal-body">
            <p>您对笔记做了更改，是否保存？</p>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="discardChanges">不保存</button>
            <button class="confirm-btn" @click="confirmSave">保存</button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { emit, listen } from '@tauri-apps/api/event'
import { showToast } from '../components/layout/showToast.js'
import { getBackendUrl } from '../config/backend.js'
import GenericNoteEditor from '../components/editor/GenericNoteEditor.vue'

const timeOut = 3000
const route = useRoute()

// 笔记数据
const noteUuid = ref('')
const noteTitle = ref('')
const noteContent = ref('')
const originalContent = ref('')

// 编辑器引用
const genericEditor = ref(null)
const showSaveConfirmModal = ref(false)

// 窗口状态
const isMaximized = ref(false)
const currentWindow = getCurrentWindow()

// 主题状态
const isLightMode = ref(false)

// 获取认证头
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

// API 请求
async function apiRequest(url, data = {}) {
  const authHeader = await getAuthHeader()
  const response = await axios.post(getBackendUrl() + url, data, {
    headers: { ...authHeader, 'Content-Type': 'application/json' },
    timeout: timeOut
  })
  return response.data
}

// 初始化
onMounted(async () => {
  // 从 URL 参数获取笔记信息
  const uuid = route.query.uuid
  const title = route.query.title
  
  if (!uuid) {
    showToast('笔记信息不完整', '#ef4444')
    setTimeout(() => closeWindow(), 1500)
    return
  }
  
  noteUuid.value = uuid
  noteTitle.value = title || '未命名笔记'
  noteContent.value = ''
  originalContent.value = ''
  
  // 获取笔记内容
  const fetchNoteContent = async () => {
    const isMeetingNote = route.query.isMeetingNote === 'true'
    
    if (isMeetingNote) {
      noteContent.value = ''
      originalContent.value = ''
      return
    }
    
    try {
      const noteData = await apiRequest('/note/query_by_uuid', { uuid })
      let content = ''
      if (noteData && typeof noteData === 'object') {
        content = noteData.content || noteData.data?.content || noteData.note?.content || ''
      }
      
      noteContent.value = content
      originalContent.value = content
    } catch (e) {
      console.error('获取笔记内容失败:', e)
      showToast('获取笔记内容失败: ' + (e.message || '网络错误'), '#ef4444')
    }
  }
  
  await fetchNoteContent()
  
  // 监听主窗口发送的内容（作为备用方案）
  const unlistenContent = await listen('load-note-content', (event) => {
    const content = event.payload?.content || ''
    noteContent.value = content
    originalContent.value = content
    if (genericEditor.value) {
      genericEditor.value.setContent(content)
    }
  })
  
  window._unlistenContent = unlistenContent
  
  // 检查窗口状态
  checkWindowState()
  
  // 初始化主题
  initTheme()
  
  // 监听主题变化
  setupThemeListener()
  
  // 通知主窗口刷新笔记列表（打开时）
  try {
    const { emit } = await import('@tauri-apps/api/event')
    await emit('note-editor-opened', { uuid })
  } catch (e) {
    console.error('发送打开事件失败:', e)
  }
})

onUnmounted(async () => {
  // 清理事件监听
  if (window._unlistenContent) {
    window._unlistenContent()
  }
})

// 检查窗口状态
async function checkWindowState() {
  try {
    isMaximized.value = await currentWindow.isMaximized()
  } catch (error) {
    console.error('检查窗口状态失败:', error)
  }
}

// 最小化窗口
async function minimizeWindow() {
  try {
    await currentWindow.minimize()
  } catch (error) {
    console.error('最小化窗口失败:', error)
  }
}

// 切换最大化
async function toggleMaximize() {
  try {
    if (isMaximized.value) {
      await currentWindow.unmaximize()
    } else {
      await currentWindow.maximize()
    }
    // 等待一小段时间让窗口状态更新
    setTimeout(() => {
      checkWindowState()
    }, 50)
  } catch (error) {
    console.error('切换最大化失败:', error)
  }
}

// 初始化主题
function initTheme() {
  try {
    const savedTheme = localStorage.getItem('theme-preference')
    if (savedTheme === 'light' || savedTheme === 'dark') {
      isLightMode.value = savedTheme === 'light'
    } else {
      // 检测系统偏好
      isLightMode.value = window.matchMedia('(prefers-color-scheme: light)').matches
    }
  } catch (e) {
    console.error('初始化主题失败:', e)
    isLightMode.value = false
  }
}

// 监听主题变化
function setupThemeListener() {
  try {
    // 监听来自主窗口的主题变化事件
    listen('theme-changed', (event) => {
      const theme = event.payload
      isLightMode.value = theme === 'light'
      console.log('收到主题变化事件:', theme)
    })
  } catch (e) {
    console.error('设置主题监听失败:', e)
  }
}

// 关闭窗口
async function closeWindow() {
  try {
    // 先通知主窗口刷新笔记列表
    try {
      const { emit } = await import('@tauri-apps/api/event')
      await emit('note-editor-closed', { uuid: noteUuid.value })
    } catch (e) {
      console.error('发送关闭事件失败:', e)
    }
    // 等待 0.1s 让主窗口刷新
    await new Promise(resolve => setTimeout(resolve, 100))
    // 再关闭子窗口
    const appWindow = getCurrentWindow()
    await appWindow.close()
  } catch (e) {
    console.error('关闭窗口失败:', e)
    showToast('关闭失败', '#ef4444')
  }
}

// 保存并关闭
async function saveAndClose() {
  await saveNote()
  await closeWindow()
}

// 处理关闭按钮
function handleClose() {
  if (noteContent.value !== originalContent.value) {
    showSaveConfirmModal.value = true
  } else {
    closeWindow()
  }
}

// 确认保存
async function confirmSave() {
  await saveNote()
  showSaveConfirmModal.value = false
  await closeWindow()
}

// 放弃更改
async function discardChanges() {
  showSaveConfirmModal.value = false
  await closeWindow()
}

// 取消关闭
function cancelClose() {
  showSaveConfirmModal.value = false
}

// 处理保存快捷键
function handleSaveShortcut() {
  saveNote()
}

// 保存笔记
async function saveNote() {
  if (!noteUuid.value) return
  
  try {
    // 检查是否是会议笔记
    const isMeetingNote = route.query.isMeetingNote === 'true'
    
    if (isMeetingNote) {
      // 会议笔记，发送到后端接口
      await sendMeetingNoteToBackend()
    } else {
      // 普通笔记，调用原有接口
      await apiRequest('/note/update', { 
        uuid: noteUuid.value, 
        content: noteContent.value || '',
        title: noteTitle.value 
      })
    }
    
    originalContent.value = noteContent.value
    showToast('保存成功', '#10b981')
    
    // 通知主窗口刷新笔记列表
    await emit('note-saved', { uuid: noteUuid.value })
  } catch (e) {
    console.error('保存笔记失败:', e)
    showToast('保存失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

// 发送会议笔记到后端
async function sendMeetingNoteToBackend() {
  try {
    const authHeader = await getAuthHeader()
    const response = await axios.post(getBackendUrl() + '/meeting/note/add', {
      title: noteTitle.value,
      content: noteContent.value || ''
    }, {
      headers: authHeader,
      timeout: 10000
    })
    console.log('会议笔记发送成功:', response.data)
  } catch (error) {
    console.error('发送会议笔记失败:', error)
    throw error
  }
}
</script>

<style scoped>
.editor-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary, #0d1117);
  color: var(--text-primary, #f8fafc);
}

.editor-window.light-mode {
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #1e293b);
}

/* 窗口标题栏 - 与主窗口 TitleBar 保持一致 */
.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 48px;
  padding: 0 24px;
  background: var(--bg-header, #161b22);
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  -webkit-app-region: drag;
  flex-shrink: 0;
}

.light-mode .editor-header {
  background: var(--bg-header, #ffffff);
  border-bottom-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.editor-title-input {
  flex: 1;
  font-size: 16px;
  font-weight: 600;
  background: transparent;
  border: none;
  color: var(--text-primary, #f8fafc);
  outline: none;
  padding: 4px 8px;
  -webkit-app-region: no-drag;
  min-width: 0;
  letter-spacing: 0.5px;
}

.light-mode .editor-title-input {
  color: var(--text-primary, #1e293b);
}

.editor-title-input::placeholder {
  color: var(--text-secondary, #94a3b8);
}

.editor-actions {
  display: flex;
  align-items: center;
  gap: 0;
  -webkit-app-region: no-drag;
  margin-right: -16px;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 0.375rem;
  background: transparent;
  color: var(--text-secondary, #cbd5e1);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 18px;
}

.light-mode .action-btn {
  color: var(--text-secondary, #64748b);
}

.action-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-primary, #f8fafc);
}

.light-mode .action-btn:hover {
  background-color: var(--hover-bg, rgba(0, 0, 0, 0.05));
  color: var(--text-primary, #1e293b);
}

.action-btn.save-btn:hover {
  background-color: #10b981;
  color: white;
}

.action-btn.close-btn:hover {
  background-color: #ef4444;
  color: white;
}

/* 编辑器主体包装器 */
.editor-body-wrapper {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 模态框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal-content {
  background: #252525;
  border-radius: 12px;
  width: 100%;
  max-width: 400px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.light-mode .modal-content {
  background: #ffffff;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #333;
}

.light-mode .modal-header {
  border-bottom-color: #e0e0e0;
}

.modal-header h3 {
  font-size: 16px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}

.modal-header h3 i {
  color: #3b82f6;
}

.close-btn {
  background: none;
  border: none;
  color: #666;
  cursor: pointer;
  font-size: 20px;
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #333;
  color: #e0e0e0;
}

.light-mode .close-btn:hover {
  background: #f0f0f0;
  color: #333;
}

.modal-body {
  padding: 20px;
}

.save-modal-body p {
  text-align: center;
  color: #aaa;
  font-size: 14px;
}

.light-mode .save-modal-body p {
  color: #666;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid #333;
}

.light-mode .modal-footer {
  border-top-color: #e0e0e0;
}

.cancel-btn {
  padding: 8px 16px;
  border: 1px solid #444;
  border-radius: 6px;
  background: transparent;
  color: #e0e0e0;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.light-mode .cancel-btn {
  border-color: #ddd;
  color: #333;
}

.cancel-btn:hover {
  background: #333;
}

.light-mode .cancel-btn:hover {
  background: #f0f0f0;
}

.confirm-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  background: #3b82f6;
  color: white;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.confirm-btn:hover {
  background: #2563eb;
}

/* 过渡动画 */
.modal-enter-active,
.modal-leave-active {
  transition: all 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

.modal-enter-to,
.modal-leave-from {
  opacity: 1;
  transform: scale(1);
}

/* 覆盖通用编辑器的样式以适配窗口 */
:deep(.generic-note-editor) {
  height: 100%;
}

:deep(.editor-title-section) {
  display: none;
}
</style>
