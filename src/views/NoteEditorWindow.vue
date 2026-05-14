

<template>
  <div class="editor-window" :class="{ 'light-mode': isLightMode }">
    <div class="editor-header" data-tauri-drag-region>
      <input
        v-model="noteTitle"
        class="editor-title-input"
        placeholder="未命名笔记"
        type="text"
        :disabled="isMeetingNote"
        :class="{ 'disabled': isMeetingNote }"
        title="会议笔记不允许修改标题"
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
        :enable-image="!isMeetingNote"
        :enable-paste="!isMeetingNote"
        :enable-drag-drop="!isMeetingNote"
        @save="handleSaveShortcut"
        @image-blocked="handleImageBlocked"
      />
    </div>

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

const noteUuid = ref('')
const noteTitle = ref('')
const noteContent = ref('')
const originalContent = ref('')
const isMeetingNote = ref(false)

const genericEditor = ref(null)
const showSaveConfirmModal = ref(false)

const isMaximized = ref(false)
const currentWindow = getCurrentWindow()

const isLightMode = ref(false)

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

async function apiRequest(url, data = {}) {
  const authHeader = await getAuthHeader()
  const response = await axios.post(getBackendUrl() + url, data, {
    headers: { ...authHeader, 'Content-Type': 'application/json' },
    timeout: timeOut
  })
  return response.data
}

onMounted(async () => {

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
  isMeetingNote.value = route.query.isMeetingNote === 'true'

  const fetchNoteContent = async () => {
    if (isMeetingNote.value) {
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

      if (e.response?.status === 404) {
        console.log('笔记内容已通过事件加载，跳过直接请求')
        return
      }
      console.error('获取笔记内容失败:', e)
      showToast('获取笔记内容失败: ' + (e.message || '网络错误'), '#ef4444')
    }
  }

  await fetchNoteContent()

  const unlistenContent = await listen('load-note-content', (event) => {
    const content = event.payload?.content || ''
    noteContent.value = content
    originalContent.value = content
    if (genericEditor.value) {
      genericEditor.value.setContent(content)
    }
  })

  window._unlistenContent = unlistenContent

  checkWindowState()

  initTheme()

  setupThemeListener()

  try {
    const { emit } = await import('@tauri-apps/api/event')
    await emit('note-editor-opened', { uuid })
  } catch (e) {
    console.error('发送打开事件失败:', e)
  }
})

onUnmounted(async () => {

  if (window._unlistenContent) {
    window._unlistenContent()
  }
})

async function checkWindowState() {
  try {
    isMaximized.value = await currentWindow.isMaximized()
  } catch (error) {
    console.error('检查窗口状态失败:', error)
  }
}

async function minimizeWindow() {
  try {
    await currentWindow.minimize()
  } catch (error) {
    console.error('最小化窗口失败:', error)
  }
}

async function toggleMaximize() {
  try {
    if (isMaximized.value) {
      await currentWindow.unmaximize()
    } else {
      await currentWindow.maximize()
    }

    setTimeout(() => {
      checkWindowState()
    }, 50)
  } catch (error) {
    console.error('切换最大化失败:', error)
  }
}

function initTheme() {
  try {
    const savedTheme = localStorage.getItem('theme-preference')
    if (savedTheme === 'light' || savedTheme === 'dark') {
      isLightMode.value = savedTheme === 'light'
    } else {

      isLightMode.value = window.matchMedia('(prefers-color-scheme: light)').matches
    }
  } catch (e) {
    console.error('初始化主题失败:', e)
    isLightMode.value = false
  }
}

function setupThemeListener() {
  try {

    listen('theme-changed', (event) => {
      const theme = event.payload
      isLightMode.value = theme === 'light'
      console.log('收到主题变化事件:', theme)
    })
  } catch (e) {
    console.error('设置主题监听失败:', e)
  }
}

async function closeWindow() {
  try {

    try {
      const { emit } = await import('@tauri-apps/api/event')
      await emit('note-editor-closed', { uuid: noteUuid.value })
    } catch (e) {
      console.error('发送关闭事件失败:', e)
    }

    await new Promise(resolve => setTimeout(resolve, 100))

    const appWindow = getCurrentWindow()
    await appWindow.close()
  } catch (e) {
    console.error('关闭窗口失败:', e)
    showToast('关闭失败', '#ef4444')
  }
}

async function saveAndClose() {
  await saveNote()
  await closeWindow()
}

function handleClose() {
  if (noteContent.value !== originalContent.value) {
    showSaveConfirmModal.value = true
  } else {
    closeWindow()
  }
}

async function confirmSave() {
  await saveNote()
  showSaveConfirmModal.value = false
  await closeWindow()
}

async function discardChanges() {
  showSaveConfirmModal.value = false
  await closeWindow()
}

function cancelClose() {
  showSaveConfirmModal.value = false
}

function handleSaveShortcut() {
  saveNote()
}

function handleImageBlocked() {
  showToast('会议模式下不允许插入图片', '#f59e0b')
}

async function saveNote() {
  if (!noteUuid.value) return

  try {

    const isMeetingNote = route.query.isMeetingNote === 'true'

    if (isMeetingNote) {

      await sendMeetingNoteToBackend()
    } else {

      await apiRequest('/note/update', {
        uuid: noteUuid.value,
        content: noteContent.value || '',
        title: noteTitle.value
      })
    }

    originalContent.value = noteContent.value
    showToast('保存成功', '#10b981')

    await emit('note-saved', { uuid: noteUuid.value })
  } catch (e) {
    console.error('保存笔记失败:', e)
    showToast('保存失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

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
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.light-mode .editor-title-input {
  color: var(--text-primary, #1e293b);
}

.editor-title-input::placeholder {
  color: var(--text-secondary, #94a3b8);
}

.editor-title-input:disabled {
  background: rgba(0, 0, 0, 0.05);
  cursor: not-allowed;
  opacity: 0.7;
}

.light-mode .editor-title-input:disabled {
  background: rgba(0, 0, 0, 0.05);
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
  border-radius: 2px;
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

.editor-body-wrapper {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

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
  z-index: 99999;
  padding: 20px;
}

.modal-content {
  background: #252525;
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
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
  border-radius: 2px;
  background: #3b82f6;
  color: white;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.confirm-btn:hover {
  background: #2563eb;
}

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

:deep(.generic-note-editor) {
  height: 100%;
}

:deep(.editor-title-section) {
  display: none;
}
</style>
