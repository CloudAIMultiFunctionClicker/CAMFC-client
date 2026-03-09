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
  <div class="notes-container">
    <div class="notes-header">
      <h1 class="page-title">
        <i class="ri-sticky-note-line page-title-icon"></i>
        笔记
      </h1>
      <div class="header-actions">
        <button class="add-btn" @click="showAddModal = true">
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
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载笔记...</p>
      </div>
      
      <div v-else-if="notes.length === 0" class="empty-state">
        <div class="empty-icon">📝</div>
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
            :class="{ active: selectedNote?.uuid === note.uuid }"
            @click="selectNote(note)"
          >
            <div class="note-title">{{ note.title }}</div>
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
      <div v-if="selectedNote" class="note-modal-overlay" @click.self="selectedNote = null; isEditing = false">
        <div class="note-modal-content" @click.stop>
          <div class="note-modal-header">
            <span class="note-title-display">{{ selectedNote.title }}</span>
            <div class="note-modal-actions">
              <button v-if="!isEditing" class="edit-btn" @click="startEditing">
                <i class="ri-edit-line"></i>
              </button>
              <template v-else>
                <button class="save-btn" @click="saveAndClose">
                  <i class="ri-check-line"></i>
                </button>
              </template>
              <button class="close-btn" @click="handleCloseNote">
                <i class="ri-close-line"></i>
              </button>
            </div>
          </div>
          <div class="note-modal-body">
            <div v-if="!isEditing">
              <div v-if="selectedNote.content" class="preview-text" v-html="renderMarkdown(selectedNote.content)"></div>
              <div v-else class="preview-text empty">暂无内容</div>
            </div>
            <div v-else class="editor-container">
              <textarea
                ref="editorTextarea"
                v-model="selectedNote.content"
                class="note-editor-textarea"
                placeholder="使用 Markdown 格式书写..."
                @input="saveNote"
              ></textarea>
            </div>
          </div>
          <div v-if="isEditing" class="editor-toolbar">
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('h1')">
                <i class="ri-h-1"></i>
              </button>
              <span class="tooltip">一级标题<span class="tooltip-syntax">语法: # 标题</span></span>
            </div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('h2')">
                <i class="ri-h-2"></i>
              </button>
              <span class="tooltip">二级标题<span class="tooltip-syntax">语法: ## 标题</span></span>
            </div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('h3')">
                <i class="ri-h-3"></i>
              </button>
              <span class="tooltip">三级标题<span class="tooltip-syntax">语法: ### 标题</span></span>
            </div>
            <div class="toolbar-divider"></div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('bold')">
                <i class="ri-bold"></i>
              </button>
              <span class="tooltip">加粗<span class="tooltip-syntax">语法: **文本**</span></span>
            </div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('italic')">
                <i class="ri-italic"></i>
              </button>
              <span class="tooltip">斜体<span class="tooltip-syntax">语法: *文本*</span></span>
            </div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('strike')">
                <i class="ri-strikethrough"></i>
              </button>
              <span class="tooltip">删除线<span class="tooltip-syntax">语法: ~~文本~~</span></span>
            </div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('code')">
                <i class="ri-code-line"></i>
              </button>
              <span class="tooltip">行内代码<span class="tooltip-syntax">语法: `代码`</span></span>
            </div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('list')">
                <i class="ri-list-unordered"></i>
              </button>
              <span class="tooltip">列表<span class="tooltip-syntax">语法: - 项目</span></span>
            </div>
            <div class="toolbar-btn-wrapper">
              <button class="toolbar-btn" @click="insertMarkdown('image')">
                <i class="ri-image-line"></i>
              </button>
              <span class="tooltip">图片<span class="tooltip-syntax">语法: ![描述](地址)</span></span>
            </div>
          </div>
        </div>
      </div>
    </Transition>

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
            <span class="more-menu-title">{{ moreMenuNote?.title }}</span>
            <span class="more-menu-date">{{ moreMenuNote ? formatDate(moreMenuNote.updatedAt) : '' }}</span>
          </div>
          <div class="more-menu-actions">
            <button class="more-menu-item" @click="openRenameModal">
              <i class="ri-edit-line"></i>
              <span>重命名</span>
            </button>
            <button class="more-menu-item danger" @click="openDeleteFromMenu">
              <i class="ri-delete-bin-line"></i>
              <span>删除</span>
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="modal">
      <div v-if="showRenameModal" class="modal-overlay" @click="cancelRename">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-pencil-line"></i> 重命名笔记</h3>
            <button class="close-btn" @click="cancelRename">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body">
            <div class="input-wrapper">
              <input
                v-model="newNoteName"
                class="title-input"
                placeholder="请输入新名称"
                @keyup.enter="confirmRename"
              >
            </div>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="cancelRename">取消</button>
            <button class="confirm-btn" @click="confirmRename">确定</button>
          </div>
        </div>
      </div>
    </Transition>

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
import { ref, onMounted, computed } from 'vue'
import axios from 'axios'
import { showToast } from '../components/layout/showToast.js'
import { getBackendUrl } from '../config/backend.js'

const timeOut = 3000

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
const selectedNote = ref(null)
const showAddModal = ref(false)
const newNoteTitle = ref('')
const showDeleteModal = ref(false)
const noteToDelete = ref(null)
const showMoreMenu = ref(false)
const moreMenuNote = ref(null)
const showRenameModal = ref(false)
const renameNote = ref(null)
const newNoteName = ref('')
const isEditing = ref(false)
const showSaveConfirmModal = ref(false)
const showImportExportMenu = ref(false)
let originalContent = ''

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

onMounted(() => {
  loadNotes()
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

async function addNote() {
  if (!newNoteTitle.value.trim()) return

  const uuid = crypto.randomUUID()
  try {
    await apiRequest('/note/add', { uuid, title: newNoteTitle.value })
    notes.value.unshift({
      uuid,
      title: newNoteTitle.value,
      content: '',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString()
    })
    newNoteTitle.value = ''
    showAddModal.value = false
  } catch (e) {
    console.error('添加笔记失败:', e)
    showToast('添加笔记失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

function selectNote(note) {
  selectedNote.value = note
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
    if (selectedNote.value?.uuid === uuid) {
      selectedNote.value = null
    }
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
}

function openRenameModal() {
  if (moreMenuNote.value) {
    renameNote.value = moreMenuNote.value
    newNoteName.value = moreMenuNote.value.title
    showRenameModal.value = true
    closeMoreMenu()
  }
}

async function confirmRename() {
  if (renameNote.value && newNoteName.value.trim()) {
    renameNote.value.title = newNoteName.value.trim()
    renameNote.value.updatedAt = new Date().toISOString()
    await syncNoteToCloud(renameNote.value)
    showRenameModal.value = false
    renameNote.value = null
    newNoteName.value = ''
  }
}

function cancelRename() {
  showRenameModal.value = false
  renameNote.value = null
  newNoteName.value = ''
}

function openDeleteFromMenu() {
  if (moreMenuNote.value) {
    noteToDelete.value = moreMenuNote.value.uuid
    showDeleteModal.value = true
    closeMoreMenu()
  }
}

function startEditing() {
  if (selectedNote.value) {
    originalContent = selectedNote.value.content
  }
  isEditing.value = true
}

function handleCloseNote() {
  if (isEditing.value) {
    if (selectedNote.value && selectedNote.value.content !== originalContent) {
      showSaveConfirmModal.value = true
    } else {
      selectedNote.value = null
      isEditing.value = false
    }
  } else {
    selectedNote.value = null
    isEditing.value = false
  }
}

async function saveAndClose() {
  await syncNoteToCloud(selectedNote.value)
  selectedNote.value = null
  isEditing.value = false
}

async function confirmSave() {
  await syncNoteToCloud(selectedNote.value)
  showSaveConfirmModal.value = false
  selectedNote.value = null
  isEditing.value = false
}

function discardChanges() {
  if (selectedNote.value) {
    selectedNote.value.content = notes.value.find(n => n.uuid === selectedNote.value?.uuid)?.content || ''
  }
  showSaveConfirmModal.value = false
  selectedNote.value = null
  isEditing.value = false
}

function cancelClose() {
  showSaveConfirmModal.value = false
}

function renderMarkdown(text) {
  if (!text) return ''
  
  let html = text
    .replace(/^### (.*$)/gim, '<h3>$1</h3>')
    .replace(/^## (.*$)/gim, '<h2>$1</h2>')
    .replace(/^# (.*$)/gim, '<h1>$1</h1>')
    .replace(/\*\*(.*)\*\*/gim, '<strong>$1</strong>')
    .replace(/\*(.*)\*/gim, '<em>$1</em>')
    .replace(/~~(.*)~~/gim, '<del>$1</del>')
    .replace(/`([^`]+)`/gim, '<code>$1</code>')
    .replace(/^- (.*$)/gim, '<li>$1</li>')
    .replace(/!\[([^\]]*)\]\(([^)]+)\)/gim, '<img src="$2" alt="$1" class="markdown-image" onerror="this.style.display=\'none\'">')
    .replace(/\n/gim, '<br>')
  
  return html
}

function saveNote() {
  if (selectedNote.value) {
    selectedNote.value.updatedAt = new Date().toISOString()
  }
}

async function syncNoteToCloud(note) {
  try {
    await apiRequest('/note/update', { 
      uuid: note.uuid, 
      content: note.content || '',
      title: note.title 
    })
  } catch (e) {
    console.error('同步笔记失败:', e)
    showToast('同步失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

const editorTextarea = ref(null)

function insertMarkdown(type) {
  if (!editorTextarea.value) return
  
  const textarea = editorTextarea.value
  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const text = selectedNote.value.content || ''
  const before = text.substring(0, start)
  const selected = text.substring(start, end)
  const after = text.substring(end)
  
  let insert = ''
  switch (type) {
    case 'h1':
      insert = selected ? `# ${selected}` : '# 标题'
      break
    case 'h2':
      insert = selected ? `## ${selected}` : '## 标题'
      break
    case 'h3':
      insert = selected ? `### ${selected}` : '### 标题'
      break
    case 'bold':
      insert = selected ? `**${selected}**` : '**加粗文本**'
      break
    case 'italic':
      insert = selected ? `*${selected}*` : '*斜体文本*'
      break
    case 'strike':
      insert = selected ? `~~${selected}~~` : '~~删除线~~'
      break
    case 'code':
      insert = selected ? `\`${selected}\`` : '`代码`'
      break
    case 'list':
      insert = selected ? `- ${selected}` : '- 列表项'
      break
    case 'image':
      insert = '![图片描述](图片地址)'
      break
  }
  
  selectedNote.value.content = before + insert + after
  saveNote()
  
  setTimeout(() => {
    const newCursor = start + insert.length
    textarea.setSelectionRange(newCursor, newCursor)
  }, 0)
}

function formatDate(dateStr) {
  const date = new Date(dateStr)
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
  padding: 60px;
  max-width: 800px;
  margin: 0 auto;
  min-height: calc(100vh - 100px);
}

.notes-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.page-title {
  font-size: 2.5rem;
  color: #333;
  margin: 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.page-title-icon {
  font-size: 2.5rem;
  color: #333;
}

.add-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 24px;
  background-color: #333;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.add-btn:hover {
  background-color: #555;
}

.add-btn i {
  font-size: 18px;
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
  background-color: #fff;
  color: #666;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.action-btn:hover {
  background-color: #f5f5f5;
  color: #333;
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
  background-color: #fff;
  border: 1px solid #eee;
  border-radius: 4px;
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
  color: #333;
  font-size: 14px;
  cursor: pointer;
  text-align: left;
}

.dropdown-item:hover {
  background-color: #f9f9f9;
}

.dropdown-item i {
  font-size: 16px;
  color: #666;
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
  background-color: #fff;
  border-radius: 4px;
  border: 1px solid #eee;
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.empty-message {
  font-size: 20px;
  color: #333;
  margin-bottom: 10px;
}

.empty-desc {
  color: #999;
}

.loading-state {
  text-align: center;
  padding: 80px 20px;
  background-color: #fff;
  border-radius: 4px;
  border: 1px solid #eee;
}

.loading-state p {
  color: #666;
  margin-top: 16px;
}

.loading-overlay {
  text-align: center;
  padding: 40px 20px;
}

.loading-overlay p {
  color: #666;
  margin-top: 12px;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #eee;
  border-top-color: #333;
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
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
  color: #333;
  cursor: pointer;
}

.page-btn:hover:not(:disabled) {
  background-color: #f5f5f5;
  border-color: #333;
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
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
  color: #333;
  font-size: 14px;
  cursor: pointer;
}

.page-num:hover {
  background-color: #f5f5f5;
}

.page-num.active {
  background-color: #333;
  border-color: #333;
  color: white;
}

.notes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.note-card {
  background-color: #fff;
  border: 1px solid #eee;
  border-radius: 4px;
  padding: 24px;
  cursor: pointer;
}

.note-card:hover {
  border-color: #333;
}

.note-card.active {
  border-color: #333;
}

.note-title {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.note-preview {
  font-size: 14px;
  color: #666;
  margin-bottom: 15px;
  line-height: 1.5;
  min-height: 42px;
}

.note-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.note-date {
  font-size: 12px;
  color: #999;
}

.note-editor {
  position: fixed;
  right: 0;
  top: 65px;
  width: 400px;
  height: calc(100vh - 65px);
  background-color: #fff;
  border-left: 1px solid #eee;
  display: flex;
  flex-direction: column;
  z-index: 100;
}

.note-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.note-modal-content {
  background-color: #fff;
  border-radius: 4px;
  width: 90%;
  max-width: 900px;
  max-height: 85vh;
  min-height: 500px;
  border: 1px solid #eee;
  display: flex;
  flex-direction: column;
}

.note-modal-overlay,
.note-modal-overlay .note-modal-content {
  animation-duration: 0.3s;
  animation-timing-function: ease;
}

.note-modal-overlay.modal-enter-active .note-modal-content {
  animation-name: modalScaleIn;
}

.note-modal-overlay.modal-leave-active .note-modal-content {
  animation-name: modalScaleOut;
}

@keyframes modalScaleIn {
  from {
    transform: scale(0.9);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes modalScaleOut {
  from {
    transform: scale(1);
    opacity: 1;
  }
  to {
    transform: scale(0.9);
    opacity: 0;
  }
}

.note-modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #eee;
}

.note-modal-actions {
  display: flex;
  gap: 8px;
}

.note-modal-body {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
  min-height: 400px;
}

.title-input {
  flex: 1;
  background: none;
  border: none;
  font-size: 20px;
  font-weight: 600;
  color: #333;
  outline: none;
}

.note-title-display {
  flex: 1;
  font-size: 20px;
  font-weight: 600;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title-input::placeholder {
  color: #999;
}

.close-btn {
  background: none;
  border: none;
  color: #999;
  cursor: pointer;
  font-size: 20px;
  padding: 4px;
  border-radius: 4px;
}

.close-btn:hover {
  color: #333;
  background-color: #f5f5f5;
}

.editor-actions {
  display: flex;
  gap: 8px;
}

.edit-btn {
  background: none;
  border: none;
  color: #999;
  cursor: pointer;
  font-size: 18px;
  padding: 4px 8px;
  border-radius: 4px;
}

.edit-btn:hover {
  color: #333;
  background-color: #f5f5f5;
}

.save-btn {
  background: none;
  border: none;
  color: #2a7;
  cursor: pointer;
  font-size: 18px;
  padding: 4px 8px;
  border-radius: 4px;
}

.save-btn:hover {
  background-color: #f5f5f5;
}

.preview-text {
  font-size: 15px;
  color: #333;
  line-height: 1.6;
}

.preview-text.empty {
  color: #999;
}

.preview-text :deep(h1),
.preview-text :deep(h2),
.preview-text :deep(h3) {
  margin: 16px 0 10px;
  color: #333;
}

.preview-text :deep(code) {
  background-color: #f5f5f5;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 14px;
}

.preview-text :deep(del) {
  color: #999;
}

.preview-text :deep(li) {
  margin-left: 20px;
  margin-bottom: 4px;
}

.preview-text :deep(.markdown-image) {
  max-width: 100%;
  border-radius: 4px;
  margin: 12px 0;
}

.note-editor-textarea {
  width: 100%;
  height: 100%;
  min-height: 400px;
  background: none;
  border: none;
  color: #333;
  font-size: 15px;
  line-height: 1.7;
  resize: none;
  outline: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.note-editor-textarea::placeholder {
  color: #999;
}

.editor-container {
  width: 100%;
  min-height: 400px;
}

.editor-toolbar {
  position: absolute;
  bottom: 20px;
  right: 20px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background-color: #fff;
  border: 1px solid #eee;
  border-radius: 4px;
  z-index: 10;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: none;
  border: none;
  border-radius: 4px;
  color: #666;
  font-size: 16px;
  cursor: pointer;
}

.toolbar-btn:hover {
  background-color: #f5f5f5;
  color: #333;
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background-color: #eee;
  margin: 0 4px;
}

.toolbar-btn-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toolbar-btn-wrapper:hover .tooltip {
  opacity: 1;
  visibility: visible;
  transform: translateX(-50%) translateY(0);
}

.tooltip {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%) translateY(5px);
  padding: 8px 12px;
  background-color: #333;
  color: #fff;
  font-size: 12px;
  white-space: nowrap;
  border-radius: 4px;
  border: 1px solid #eee;
  opacity: 0;
  visibility: hidden;
  transition: all 0.2s ease;
  pointer-events: none;
  margin-bottom: 8px;
  z-index: 100;
}

.tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 6px solid transparent;
  border-top-color: #333;
}

.tooltip-syntax {
  display: block;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px dashed #666;
  font-family: 'Monaco', 'Menlo', monospace;
  color: #ccc;
  font-size: 11px;
}

.content-input {
  flex: 1;
  background: none;
  border: none;
  padding: 20px;
  font-size: 15px;
  color: #333;
  line-height: 1.6;
  resize: none;
  outline: none;
}

.content-input::placeholder {
  color: #999;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: #fff;
  border-radius: 4px;
  width: 90%;
  max-width: 500px;
  border: 1px solid #eee;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #eee;
}

.modal-header h3 {
  margin: 0;
  color: #333;
}

.modal-body {
  padding: 20px;
}

.save-modal-body {
  padding-left: 24px;
}

.input-wrapper {
  margin-bottom: 15px;
}

.input-wrapper .title-input {
  width: 100%;
  padding: 12px 16px;
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
  color: #333;
  font-size: 15px;
  outline: none;
  box-sizing: border-box;
}

.input-wrapper .title-input::placeholder {
  color: #999;
}

.input-wrapper .title-input:focus {
  border-color: #333;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 20px;
  border-top: 1px solid #eee;
}

.cancel-btn {
  padding: 10px 20px;
  background-color: #fff;
  color: #333;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
}

.cancel-btn:hover {
  background-color: #f5f5f5;
}

.confirm-btn {
  padding: 10px 20px;
  background-color: #333;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.confirm-btn:hover {
  background-color: #555;
}

.delete-confirm-btn {
  padding: 10px 20px;
  background-color: #c00;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.delete-confirm-btn:hover {
  background-color: #a00;
}

.delete-modal-body {
  padding-left: 24px;
}

.danger-text {
  color: #c00;
  font-weight: 500;
}

.more-wrapper {
  position: relative;
}

.more-btn {
  background: none;
  border: none;
  color: #999;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 16px;
}

.more-btn:hover {
  color: #333;
  background-color: #f5f5f5;
}

.more-menu-content {
  background-color: #fff;
  border-radius: 4px;
  width: 280px;
  border: 1px solid #eee;
  overflow: hidden;
}

.more-menu-header {
  padding: 16px;
  border-bottom: 1px solid #eee;
}

.more-menu-title {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: #333;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.more-menu-date {
  display: block;
  font-size: 12px;
  color: #999;
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
  border-radius: 4px;
  color: #333;
  cursor: pointer;
  font-size: 14px;
}

.more-menu-item:hover {
  background-color: #f9f9f9;
}

.more-menu-item.danger {
  color: #c00;
}

.more-menu-item.danger:hover {
  background-color: #fff0f0;
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
