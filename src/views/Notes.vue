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
  <div class="notes-container">
    <div class="notes-header">
      <h1 class="page-title">
        <i class="ri-sticky-note-line page-title-icon"></i>
        笔记
      </h1>
      <div class="header-actions">
        <button class="refresh-btn" @click="refreshNotes">
          <i class="ri-refresh-line"></i>
          刷新
        </button>
        <button class="add-btn" @click="createAndOpenNote">
          <i class="ri-add-line"></i>
          新建笔记
        </button>
        <div class="dropdown-wrapper" @mouseenter="showImportExportMenu = true" @mouseleave="showImportExportMenu = false">
          <button class="action-btn">
            <i class="ri-upload-download-line"></i>
            导入/导出
            <i class="ri-arrow-down-s-line"></i>
          </button>
          <Transition name="dropdown">
            <div v-show="showImportExportMenu" class="dropdown-menu">
              <button class="dropdown-item" @click="importNotes">
                <i class="ri-upload-line"></i>
                导入笔记
              </button>
              <button class="dropdown-item" @click="exportNotes">
                <i class="ri-download-line"></i>
                导出笔记
              </button>
            </div>
          </Transition>
        </div>
      </div>
    </div>

    <div class="notes-content">
      <!-- 骨架屏：加载时显示灰色占位块 -->
      <div v-if="isLoading" class="skeleton-grid">
        <div v-for="i in 12" :key="i" class="skeleton-card">
          <div class="skeleton-title"></div>
          <div class="skeleton-preview"></div>
          <div class="skeleton-meta">
            <div class="skeleton-date"></div>
            <div class="skeleton-more"></div>
          </div>
        </div>
      </div>
      
      <div v-else-if="notes.length === 0" class="empty-state">
        <FileText :size="48" class="empty-icon" />
        <p class="empty-message">还没有笔记</p>
        <p class="empty-desc">点击上方按钮创建您的第一个笔记</p>
      </div>

      <div v-else>
        <div v-if="pageLoading" class="loading-overlay">
          <div class="loading-spinner"></div>
          <p>正在加载...</p>
        </div>
        <div v-else class="notes-grid">
          <div
            v-for="note in currentPageNotes"
            :key="note.uuid"
            class="note-card"
            :class="{ active: false }"
            @click="selectNote(note)"
          >
            <div class="note-title-wrapper">
              <input
                v-if="editingCardNote === note.uuid"
                ref="cardTitleInput"
                v-model="note.title"
                class="card-title-edit-input"
                @blur="saveCardTitleEdit(note)"
                @keyup.enter="saveCardTitleEdit(note)"
                @keyup.escape="cancelCardTitleEdit"
                @click.stop
              />
              <span 
                v-else 
                class="note-title"
                @dblclick.stop="startCardTitleEdit(note)"
                title="双击编辑标题"
              >{{ note.title }}</span>
            </div>
            <div class="note-preview">{{ (note.content || '').substring(0, 50) }}...</div>
            <div class="note-meta">
              <span class="note-date">{{ formatDate(note.updatedAt) }}</span>
              <div class="more-wrapper">
                <button class="more-btn" @click.stop="openMoreMenu(note, $event)">
                  <i class="ri-more-fill"></i>
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <div v-if="totalPages > 1" class="pagination">
          <button class="page-btn" :disabled="currentPage === 1" @click="prevPage">
            <i class="ri-arrow-left-s-line"></i>
          </button>
          <div class="page-numbers">
            <button
              v-for="page in totalPages"
              :key="page"
              class="page-num"
              :class="{ active: page === currentPage }"
              @click="goToPage(page)"
            >
              {{ page }}
            </button>
          </div>
          <button class="page-btn" :disabled="currentPage === totalPages" @click="nextPage">
            <i class="ri-arrow-right-s-line"></i>
          </button>
        </div>
      </div>
    </div>

    <Transition name="modal">
      <div v-if="showAddModal" class="modal-overlay" @click="showAddModal = false">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-sticky-note-line"></i> 新建笔记</h3>
            <button class="close-btn" @click="showAddModal = false">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body">
            <div class="input-wrapper">
              <input
                v-model="newNoteTitle"
                class="title-input"
                placeholder="请输入笔记名称"
                @keyup.enter="addNote"
              >
            </div>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="showAddModal = false">取消</button>
            <button class="confirm-btn" @click="addNote">创建</button>
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="modal">
      <div v-if="showDeleteModal" class="modal-overlay" @click="cancelDelete">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-delete-bin-line"></i> 删除笔记</h3>
            <button class="close-btn" @click="cancelDelete">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body delete-modal-body">
            <p>确定要删除这个笔记吗？此操作<span class="danger-text">不可恢复</span>。</p>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="cancelDelete">取消</button>
            <button class="delete-confirm-btn" @click="confirmDelete">删除</button>
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="modal">
      <div v-if="showMoreMenu" class="modal-overlay" @click="closeMoreMenu">
        <div class="more-menu-content" @click.stop>
          <div class="more-menu-header">
            <input
              v-if="editingNote === moreMenuNote?.uuid"
              ref="titleInput"
              v-model="moreMenuNote.title"
              class="more-menu-title-input"
              @blur="saveTitleEdit"
              @keyup.enter="saveTitleEdit"
              @keyup.escape="cancelTitleEdit"
            />
            <span v-else class="more-menu-title">{{ moreMenuNote?.title }}</span>
            <span class="more-menu-date">{{ moreMenuNote ? formatDate(moreMenuNote.updatedAt) : '' }}</span>
          </div>
          <div class="more-menu-actions">
            <button class="more-menu-item" @click="startTitleEdit">
              <i class="ri-edit-line"></i>
              <span>重命名</span>
            </button>
            <button class="more-menu-item danger" :class="{ 'confirming': isConfirmingDelete }" @click="handleDeleteClick">
              <i :class="isConfirmingDelete ? 'ri-question-line' : 'ri-delete-bin-line'"></i>
              <span>{{ isConfirmingDelete ? '确认删除' : '删除' }}</span>
            </button>
          </div>
        </div>
      </div>
    </Transition>

  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue'
import axios from 'axios'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { listen } from '@tauri-apps/api/event'
import { showToast } from '../components/layout/showToast.js'
import { getBackendUrl } from '../config/backend.js'
import { FileText } from 'lucide-vue-next'

const timeOut = 10000 // 10 秒超时

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
const notes = ref([])
const showAddModal = ref(false)
const newNoteTitle = ref('')
const showDeleteModal = ref(false)
const noteToDelete = ref(null)
const showMoreMenu = ref(false)
const moreMenuNote = ref(null)
const editingNote = ref(null)
const editingCardNote = ref(null)
const titleInput = ref(null)
const cardTitleInput = ref(null)
const showImportExportMenu = ref(false)
const isConfirmingDelete = ref(false)
const refreshBtnSpinning = ref(false)

const pageSize = 9
const currentPage = ref(1)
const isLoading = ref(false)
const pageLoading = ref(false)

const totalPages = computed(() => Math.ceil(notes.value.length / pageSize) || 1)

const currentPageNotes = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  const end = start + pageSize
  return notes.value.slice(start, end)
})

const loadedNotes = ref({})
let unlistenNoteSaved = null
let unlistenCreateNewNote = null
let unlistenRefreshNotes = null

onMounted(async () => {
  loadNotes()
  
  // 监听笔记保存事件，刷新列表
  unlistenNoteSaved = await listen('note-saved', () => {
    loadNotes()
  })
  
  // 监听蓝牙新建笔记命令，调用 createAndOpenNote 方法
  const { listen } = await import('@tauri-apps/api/event')
  unlistenCreateNewNote = await listen('create-new-note', () => {
    createAndOpenNote()
  })
  
  // 监听刷新笔记列表事件（来自编辑器窗口）
  unlistenRefreshNotes = await listen('refresh-notes', () => {
    loadNotes()
  })
})

onUnmounted(() => {
  if (unlistenNoteSaved) {
    unlistenNoteSaved()
  }
  if (unlistenCreateNewNote) {
    unlistenCreateNewNote()
  }
  if (unlistenRefreshNotes) {
    unlistenRefreshNotes()
  }
})

async function loadNotes() {
  isLoading.value = true
  try {
    const data = await apiRequest('/note/query', { num: 100 })
    let notesList = data
    if (data && typeof data === 'object' && !Array.isArray(data)) {
      notesList = data.data || data.notes || data.result || []
    }
    notes.value = Array.isArray(notesList) ? notesList : []
    if (notes.value.length === 0) {
      await createDefaultNote()
    }
  } catch (e) {
    console.error('加载笔记失败:', e)
    showToast('加载笔记失败: ' + (e.message || '网络错误'), '#ef4444')
    notes.value = []
  }
  isLoading.value = false
  loadCurrentPageNotes()
}

async function createDefaultNote() {
  const uuid = crypto.randomUUID()
  try {
    await apiRequest('/note/add', { uuid, title: 'Hello' })
    notes.value = [{
      uuid,
      title: 'Hello',
      content: '你好！这是你的第一个笔记。\n\n开始记录你的想法吧！',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString()
    }]
  } catch (e) {
    console.error('创建默认笔记失败:', e)
  }
}

function loadCurrentPageNotes() {
  pageLoading.value = true
  loadedNotes.value = {}
  const pageNotes = currentPageNotes.value
  for (let i = 0; i < pageNotes.length; i++) {
    loadedNotes.value[pageNotes[i].uuid] = true
  }
  pageLoading.value = false
  setTimeout(() => {
    pageLoading.value = false
  }, 50)
}

function goToPage(page) {
  if (page < 1 || page > totalPages.value) return
  currentPage.value = page
  loadCurrentPageNotes()
}

function prevPage() {
  goToPage(currentPage.value - 1)
}

function nextPage() {
  goToPage(currentPage.value + 1)
}

// 打开笔记编辑窗口
async function openNoteEditorWindow(note) {
  const windowLabel = `note-editor-${note.uuid}`
  // 不传 content，避免 URL 过长导致 431 错误
  const url = `/note-editor?uuid=${note.uuid}&title=${encodeURIComponent(note.title)}`
  
  const webview = new WebviewWindow(windowLabel, {
    url: url,
    title: note.title || '编辑笔记',
    width: 900,
    height: 600,
    minWidth: 400,
    minHeight: 300,
    center: true,
    decorations: false,
    resizable: true
  })
  
  webview.once('tauri://created', async () => {
    console.log('笔记编辑窗口创建成功:', windowLabel)
    // 窗口创建后再发送内容，避免 URL 过长
    // 延迟一点确保窗口已加载
    setTimeout(async () => {
      try {
        await webview.emit('load-note-content', { content: note.content || '' })
      } catch (e) {
        console.error('发送笔记内容失败:', e)
      }
    }, 300)
  })
  
  webview.once('tauri://error', (e) => {
    console.error('笔记编辑窗口创建失败:', e)
    showToast('打开编辑窗口失败', '#ef4444')
  })
}

async function createAndOpenNote() {
  const uuid = crypto.randomUUID()
  const now = new Date()
  const timestamp = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}${String(now.getSeconds()).padStart(2, '0')}`
  const defaultTitle = `未命名笔记_${timestamp}`
  
  // 先在云端创建空白笔记
  try {
    await apiRequest('/note/add', { uuid, title: defaultTitle, content: '' })
  } catch (e) {
    console.error('创建云端笔记失败:', e)
    showToast('创建笔记失败: ' + (e.message || '网络错误'), '#ef4444')
    return
  }
  
  // 本地添加笔记
  const newNote = {
    uuid,
    title: defaultTitle,
    content: '',
    createdAt: now.toISOString(),
    updatedAt: now.toISOString()
  }
  notes.value.unshift(newNote)
  
  // 打开编辑窗口
  openNoteEditorWindow(newNote)
}

async function addNote() {
  if (!newNoteTitle.value.trim()) return

  const uuid = crypto.randomUUID()
  try {
    await apiRequest('/note/add', { uuid, title: newNoteTitle.value })
    const newNote = {
      uuid,
      title: newNoteTitle.value,
      content: '',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString()
    }
    notes.value.unshift(newNote)
    newNoteTitle.value = ''
    showAddModal.value = false
    
    // 创建后直接打开编辑窗口
    openNoteEditorWindow(newNote)
  } catch (e) {
    console.error('添加笔记失败:', e)
    showToast('添加笔记失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

function selectNote(note) {
  // 打开独立编辑窗口
  openNoteEditorWindow(note)
}

function deleteNote(id) {
  noteToDelete.value = id
  showDeleteModal.value = true
}

function confirmDelete() {
  if (noteToDelete.value) {
    deleteNoteApi(noteToDelete.value)
  }
  showDeleteModal.value = false
  noteToDelete.value = null
}

async function deleteNoteApi(uuid) {
  try {
    await apiRequest('/note/delete', { uuid })
    notes.value = notes.value.filter(n => n.uuid !== uuid)
  } catch (e) {
    console.error('删除笔记失败:', e)
    showToast('删除笔记失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

function cancelDelete() {
  showDeleteModal.value = false
  noteToDelete.value = null
}

function openMoreMenu(note, event) {
  moreMenuNote.value = note
  showMoreMenu.value = true
}

function closeMoreMenu() {
  showMoreMenu.value = false
  moreMenuNote.value = null
  isConfirmingDelete.value = false
}

function startTitleEdit() {
  if (moreMenuNote.value) {
    editingNote.value = moreMenuNote.value.uuid
    setTimeout(() => {
      if (titleInput.value) {
        titleInput.value.focus()
        titleInput.value.select()
      }
    }, 100)
  }
}

function saveTitleEdit() {
  if (moreMenuNote.value && moreMenuNote.value.title.trim()) {
    moreMenuNote.value.title = moreMenuNote.value.title.trim()
    moreMenuNote.value.updatedAt = new Date().toISOString()
    syncNoteToCloud(moreMenuNote.value)
  }
  editingNote.value = null
}

function cancelTitleEdit() {
  editingNote.value = null
}

function startCardTitleEdit(note) {
  editingCardNote.value = note.uuid
  setTimeout(() => {
    if (cardTitleInput.value) {
      cardTitleInput.value.focus()
      cardTitleInput.value.select()
    }
  }, 100)
}

function saveCardTitleEdit(note) {
  if (note && note.title.trim()) {
    note.title = note.title.trim()
    note.updatedAt = new Date().toISOString()
    syncNoteToCloud(note)
  }
  editingCardNote.value = null
}

function cancelCardTitleEdit() {
  editingCardNote.value = null
}

function handleDeleteClick() {
  if (isConfirmingDelete.value) {
    // 确认删除，执行删除操作
    if (moreMenuNote.value) {
      noteToDelete.value = moreMenuNote.value.uuid
      confirmDelete()
      isConfirmingDelete.value = false
      closeMoreMenu()
    }
  } else {
    // 第一次点击，进入确认状态
    isConfirmingDelete.value = true
  }
}



function formatDate(dateStr) {
  if (!dateStr) {
    return ''
  }
  const date = new Date(dateStr)
  if (isNaN(date.getTime())) {
    return ''
  }
  return `${date.getMonth() + 1}月${date.getDate()}日 ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
}

function exportNotes() {
  showImportExportMenu.value = false
  try {
    const data = JSON.stringify(notes.value, null, 2)
    const blob = new Blob([data], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `camfc-notes-${new Date().toISOString().slice(0, 10)}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
    showToast('笔记导出成功', '#10b981')
  } catch (err) {
    console.error('导出失败:', err)
    showToast('导出失败', '#ef4444')
  }
}

async function refreshNotes() {
  refreshBtnSpinning.value = true
  try {
    await loadNotes()
    showToast('刷新成功', '#10b981')
  } catch (e) {
    console.error('刷新失败:', e)
    showToast('刷新失败: ' + (e.message || '网络错误'), '#ef4444')
  } finally {
    refreshBtnSpinning.value = false
  }
}

function importNotes() {
  showImportExportMenu.value = false
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.json'
  input.onchange = (e) => {
    const file = e.target.files[0]
    if (!file) return
    const reader = new FileReader()
    reader.onload = (event) => {
      try {
        const importedNotes = JSON.parse(event.target.result)
        if (Array.isArray(importedNotes)) {
          notes.value = importedNotes
          currentPage.value = 1
          loadCurrentPageNotes()
          showToast(`已导入 ${importedNotes.length} 条笔记（本地导入，云端未同步）`, '#f59e0b')
        } else {
          showToast('文件格式不正确', '#ef4444')
        }
      } catch (err) {
        console.error('导入失败:', err)
        showToast('导入失败，请检查文件格式', '#ef4444')
      }
    }
    reader.readAsText(file)
  }
  input.click()
}
</script>

<style scoped>
.notes-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 0 30px;
  max-width: 1200px;
  margin: 0 auto;
}

.notes-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 30px 0;
  flex-shrink: 0; /* 防止被压缩 */
  position: sticky;
  top: 0;
  background-color: var(--bg-primary);
  z-index: 10;
}

.notes-content {
  flex: 1;
  overflow-y: auto;
  padding-bottom: 30px;
}

.page-title {
  font-size: 28px;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.page-title-icon {
  font-size: 28px;
  color: var(--accent-blue, #3178c6);
}

.add-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.add-btn:hover {
  background-color: var(--accent-blue-bright, #1f6feb);
}

.add-btn i {
  font-size: 18px;
}

.refresh-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.refresh-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
  border-color: var(--accent-blue);
}

.refresh-btn i {
  font-size: 16px;
}

.refresh-btn.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.header-actions {
  display: flex;
  gap: 10px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 16px;
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}

.action-btn i {
  font-size: 16px;
}

.dropdown-wrapper {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  min-width: 150px;
  z-index: 100;
  overflow: hidden;
}

.dropdown-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.dropdown-item:hover {
  background-color: var(--hover-bg);
}

.dropdown-item i {
  font-size: 16px;
  color: var(--text-secondary);
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.empty-state {
  text-align: center;
  padding: 80px 20px;
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  border: 1px solid var(--border-color);
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.empty-message {
  font-size: 20px;
  color: var(--text-primary);
  margin-bottom: 10px;
}

.empty-desc {
  color: var(--text-muted);
}

.loading-state {
  text-align: center;
  padding: 80px 20px;
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  border: 1px solid var(--border-color);
}

.loading-state p {
  color: var(--text-secondary);
  margin-top: 16px;
}

/* 骨架屏样式 */
.skeleton-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.skeleton-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  padding: 20px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-title {
  height: 24px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  margin-bottom: 10px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-preview {
  height: 42px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  margin-bottom: 15px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.skeleton-date {
  height: 14px;
  width: 80px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-more {
  height: 20px;
  width: 20px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

@keyframes skeleton-pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}

.loading-overlay {
  text-align: center;
  padding: 40px 20px;
}

.loading-overlay p {
  color: var(--text-secondary);
  margin-top: 12px;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color);
  border-top-color: var(--accent-blue, #3178c6);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 12px;
  margin-top: 30px;
  padding: 20px 0;
}

.page-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.page-btn:hover:not(:disabled) {
  background-color: var(--hover-bg);
  border-color: var(--accent-blue, #3178c6);
}

.page-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.page-btn i {
  font-size: 20px;
}

.page-numbers {
  display: flex;
  gap: 6px;
}

.page-num {
  min-width: 36px;
  height: 36px;
  padding: 0 10px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.page-num:hover {
  background-color: var(--hover-bg);
}

.page-num.active {
  background-color: var(--accent-blue, #3178c6);
  border-color: var(--accent-blue, #3178c6);
  color: white;
}

.notes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.note-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  padding: 20px;
  cursor: pointer;
  transition: all 0.2s;
}

.note-card:hover {
  border-color: var(--accent-blue);
}

.note-card.active {
  border-color: var(--accent-blue);
  box-shadow: 0 4px 12px rgba(var(--accent-blue-rgb), 0.15);
}

.note-title-wrapper {
  margin-bottom: 10px;
  position: relative;
}

.note-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: pointer;
  transition: all 0.2s;
}

.note-title:hover {
  color: var(--accent-blue);
}

.card-title-edit-input {
  width: 100%;
  font-size: 16px;
  font-weight: 600;
  padding: 6px 10px;
  background-color: var(--bg-primary);
  border: 1px solid var(--accent-blue);
  border-radius: .375rem;
  color: var(--text-primary);
  outline: none;
  box-sizing: border-box;
}

.card-title-edit-input:focus {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.note-preview {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 15px;
  line-height: 1.5;
  min-height: 42px;
  max-height: 63px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
}

.note-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.note-date {
  font-size: 12px;
  color: var(--text-muted);
}

.note-editor {
  position: fixed;
  right: 0;
  top: 65px;
  width: 400px;
  height: calc(100vh - 65px);
  background-color: var(--bg-secondary);
  border-left: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  z-index: 100;
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 20px;
  padding: 4px;
  border-radius: .375rem;
  transition: all 0.2s;
}

.close-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  width: 90%;
  max-width: 500px;
  border: 1px solid var(--border-color);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.modal-body {
  padding: 20px;
}

.input-wrapper {
  margin-bottom: 15px;
}

.input-wrapper .title-input {
  width: 100%;
  padding: 14px 16px;
  background: var(--bg-primary, #0f172a);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: .375rem;
  color: var(--text-primary, #f8fafc);
  font-size: 15px;
  outline: none;
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.input-wrapper .title-input::placeholder {
  color: var(--text-muted, #64748b);
}

.input-wrapper .title-input:focus {
  border-color: var(--accent-blue, #3178c6);
  box-shadow: 0 0 0 3px rgba(var(--accent-blue-rgb, 49, 120, 198), 0.2);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 20px;
  border-top: 1px solid var(--border-color);
}

.cancel-btn {
  padding: 10px 20px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn:hover {
  background-color: var(--hover-bg);
}

.confirm-btn {
  padding: 10px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.confirm-btn:hover {
  background-color: #4a8bd6;
}

.delete-confirm-btn {
  padding: 10px 20px;
  background-color: var(--danger-btn-hover-bg, #ef4444);
  color: var(--danger-btn-hover-text, white);
  border: none;
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.delete-confirm-btn:hover {
  background-color: var(--danger-btn-hover-bg, #dc2626);
  opacity: 0.9;
}

/* 删除确认按钮图标 */
.delete-confirm-btn i,
.delete-confirm-btn svg {
  color: inherit;
}

.delete-modal-body {
  padding-left: 24px;
}

.danger-text {
  color: #ef4444;
  font-weight: 500;
}

.more-wrapper {
  position: relative;
}

.more-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px 8px;
  border-radius: .375rem;
  font-size: 16px;
  transition: all 0.2s;
}

.more-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.more-menu-content {
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  width: 280px;
  border: 1px solid var(--border-color);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.more-menu-header {
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
}

.more-menu-title {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.more-menu-title-input {
  display: block;
  width: 100%;
  font-size: 14px;
  font-weight: 600;
  padding: 6px 10px;
  margin-bottom: 4px;
  background-color: var(--bg-primary);
  border: 1px solid var(--accent-blue);
  border-radius: .375rem;
  color: var(--text-primary);
  outline: none;
  box-sizing: border-box;
}

.more-menu-title-input:focus {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.more-menu-date {
  display: block;
  font-size: 12px;
  color: var(--text-muted);
}

.more-menu-actions {
  padding: 8px;
}

.more-menu-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  background: none;
  border: none;
  border-radius: .375rem;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.more-menu-item:hover {
  background-color: var(--hover-bg);
}

.more-menu-item.danger {
  color: var(--danger-btn-text, #ef4444);
}

.more-menu-item.danger:hover {
  background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.1));
  color: var(--danger-btn-hover-text, #ffffff);
}

.more-menu-item.danger.confirming {
  background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.2));
  border: 1px solid var(--danger-btn-hover-border, #ef4444);
  color: var(--danger-btn-hover-text, #ffffff);
  animation: pulse-confirm 1s ease-in-out infinite;
}

/* 更多菜单危险项图标 - 继承颜色 */
.more-menu-item.danger i,
.more-menu-item.danger svg {
  color: inherit;
}

@keyframes pulse-confirm {
  0%, 100% {
    background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.2));
  }
  50% {
    background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.35));
  }
}

@media (max-width: 768px) {
  .notes-container {
    padding: 20px;
  }

  .note-editor {
    width: 100%;
  }

  .notes-header {
    flex-direction: column;
    gap: 15px;
    align-items: flex-start;
  }
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.9);
  opacity: 0;
}
</style>
