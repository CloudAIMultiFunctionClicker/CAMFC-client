<template>
  <div class="notes-container">
    <div class="notes-header">
      <h1 class="page-title">笔记</h1>
      <div class="header-actions">
        <button class="add-btn" @click="showAddModal = true">
          <i class="ri-add-line"></i>
          新建笔记
        </button>
        <button class="action-btn" @click="importNotes">
          <i class="ri-upload-line"></i>
          导入
        </button>
        <button class="action-btn" @click="exportNotes">
          <i class="ri-download-line"></i>
          导出
        </button>
      </div>
    </div>

    <div class="notes-content">
      <div v-if="notes.length === 0" class="empty-state">
        <div class="empty-icon">📝</div>
        <p class="empty-message">还没有笔记</p>
        <p class="empty-desc">点击上方按钮创建您的第一个笔记</p>
      </div>

      <div v-else class="notes-grid">
        <div
          v-for="note in notes"
          :key="note.id"
          class="note-card"
          :class="{ active: selectedNote?.id === note.id }"
          @click="selectNote(note)"
        >
          <div class="note-title">{{ note.title }}</div>
          <div class="note-preview">{{ note.content.substring(0, 50) }}...</div>
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
    </div>

    <Transition name="modal">
      <div v-if="selectedNote" class="note-modal-overlay" @click.self="selectedNote = null; isEditing = false">
        <div class="note-modal-content" @click.stop>
          <div class="note-modal-header">
            <span class="note-title-display">{{ selectedNote.title }}</span>
            <div class="note-modal-actions">
              <button v-if="!isEditing" class="edit-btn" @click="isEditing = true">
                <i class="ri-edit-line"></i>
              </button>
              <template v-else>
                <button class="save-btn" @click="isEditing = false">
                  <i class="ri-check-line"></i>
                </button>
              </template>
              <button class="close-btn" @click="selectedNote = null; isEditing = false">
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
            <button class="toolbar-btn" @click="insertMarkdown('h1')" title="一级标题&#10;语法: # 标题">
              <i class="ri-h-1"></i>
            </button>
            <button class="toolbar-btn" @click="insertMarkdown('h2')" title="二级标题&#10;语法: ## 标题">
              <i class="ri-h-2"></i>
            </button>
            <button class="toolbar-btn" @click="insertMarkdown('h3')" title="三级标题&#10;语法: ### 标题">
              <i class="ri-h-3"></i>
            </button>
            <div class="toolbar-divider"></div>
            <button class="toolbar-btn" @click="insertMarkdown('bold')" title="加粗&#10;语法: **文本**">
              <i class="ri-bold"></i>
            </button>
            <button class="toolbar-btn" @click="insertMarkdown('italic')" title="斜体&#10;语法: *文本*">
              <i class="ri-italic"></i>
            </button>
            <button class="toolbar-btn" @click="insertMarkdown('strike')" title="删除线&#10;语法: ~~文本~~">
              <i class="ri-strikethrough"></i>
            </button>
            <button class="toolbar-btn" @click="insertMarkdown('code')" title="行内代码&#10;语法: `代码`">
              <i class="ri-code-line"></i>
            </button>
            <button class="toolbar-btn" @click="insertMarkdown('list')" title="列表&#10;语法: - 项目">
              <i class="ri-list-unordered"></i>
            </button>
            <button class="toolbar-btn" @click="insertMarkdown('image')" title="图片&#10;语法: ![描述](地址)">
              <i class="ri-image-line"></i>
            </button>
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
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
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

onMounted(() => {
  loadNotes()
})

function loadNotes() {
  const savedNotes = localStorage.getItem('camfc-notes')
  if (savedNotes) {
    notes.value = JSON.parse(savedNotes)
  }
}

function saveNotes() {
  localStorage.setItem('camfc-notes', JSON.stringify(notes.value))
}

function addNote() {
  if (!newNoteTitle.value.trim()) return

  const note = {
    id: Date.now(),
    title: newNoteTitle.value,
    content: '',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  }

  notes.value.unshift(note)
  saveNotes()

  newNoteTitle.value = ''
  showAddModal.value = false
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
    notes.value = notes.value.filter(n => n.id !== noteToDelete.value)
    if (selectedNote.value?.id === noteToDelete.value) {
      selectedNote.value = null
    }
    saveNotes()
  }
  showDeleteModal.value = false
  noteToDelete.value = null
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

function confirmRename() {
  if (renameNote.value && newNoteName.value.trim()) {
    renameNote.value.title = newNoteName.value.trim()
    renameNote.value.updatedAt = new Date().toISOString()
    saveNotes()
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
    noteToDelete.value = moreMenuNote.value.id
    showDeleteModal.value = true
    closeMoreMenu()
  }
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
    saveNotes()
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
  return `${date.getMonth() + 1}/${date.getDate()} ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
}

async function exportNotes() {
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
}

async function importNotes() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.json'
  input.onchange = async (e) => {
    const file = e.target.files?.[0]
    if (!file) return

    try {
      const text = await file.text()
      const importedNotes = JSON.parse(text)
      if (Array.isArray(importedNotes)) {
        notes.value = importedNotes
        saveNotes()
      }
    } catch (err) {
      console.error('导入失败:', err)
      alert('导入失败，请检查文件格式')
    }
  }
  input.click()
}
</script>

<style scoped>
.notes-container {
  padding: 30px;
  max-width: 1200px;
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
  font-size: 28px;
  color: var(--text-primary);
  margin: 0;
}

.add-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.add-btn:hover {
  background-color: #4a8bd6;
  transform: translateY(-1px);
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
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
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

.empty-state {
  text-align: center;
  padding: 80px 20px;
  background-color: var(--bg-secondary);
  border-radius: 12px;
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

.notes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.note-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.2s;
}

.note-card:hover {
  border-color: var(--accent-blue);
  transform: translateY(-2px);
}

.note-card.active {
  border-color: var(--accent-blue);
  box-shadow: 0 4px 12px rgba(var(--accent-blue-rgb), 0.15);
}

.note-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.note-preview {
  font-size: 14px;
  color: var(--text-secondary);
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

.note-modal-overlay {
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
  z-index: 1000;
}

.note-modal-content {
  background-color: var(--bg-secondary);
  border-radius: 16px;
  width: 90%;
  max-width: 900px;
  max-height: 85vh;
  min-height: 500px;
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
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
  border-bottom: 1px solid var(--border-color);
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
  color: var(--text-primary);
  outline: none;
}

.note-title-display {
  flex: 1;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title-input::placeholder {
  color: var(--text-muted);
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 20px;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.editor-actions {
  display: flex;
  gap: 8px;
}

.edit-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 18px;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.2s;
}

.edit-btn:hover {
  color: var(--accent-blue, #3b82f6);
  background-color: rgba(59, 130, 246, 0.1);
}

.save-btn {
  background: none;
  border: none;
  color: #22c55e;
  cursor: pointer;
  font-size: 18px;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.2s;
}

.save-btn:hover {
  background-color: rgba(34, 197, 94, 0.1);
}

.preview-text {
  font-size: 15px;
  color: var(--text-primary);
  line-height: 1.6;
}

.preview-text.empty {
  color: var(--text-muted);
}

.preview-text :deep(h1),
.preview-text :deep(h2),
.preview-text :deep(h3) {
  margin: 16px 0 10px;
  color: var(--text-primary);
}

.preview-text :deep(code) {
  background-color: var(--bg-primary);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 14px;
}

.preview-text :deep(del) {
  color: var(--text-muted);
}

.preview-text :deep(li) {
  margin-left: 20px;
  margin-bottom: 4px;
}

.preview-text :deep(.markdown-image) {
  max-width: 100%;
  border-radius: 8px;
  margin: 12px 0;
}

.note-editor-textarea {
  width: 100%;
  height: 100%;
  min-height: 400px;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 15px;
  line-height: 1.7;
  resize: none;
  outline: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.note-editor-textarea::placeholder {
  color: var(--text-muted);
}

.editor-container {
  width: 100%;
  min-height: 400px;
}

.note-editor-textarea {
  width: 100%;
  height: 100%;
  min-height: 400px;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 15px;
  line-height: 1.7;
  resize: none;
  outline: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.editor-toolbar {
  position: absolute;
  bottom: 20px;
  right: 20px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background-color: var(--bg-secondary, #1e293b);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: 10px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
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
  border-radius: 6px;
  color: var(--text-secondary, #94a3b8);
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
  color: var(--text-primary, #f1f5f9);
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background-color: var(--border-color, rgba(255, 255, 255, 0.2));
  margin: 0 4px;
}

.content-input {
  flex: 1;
  background: none;
  border: none;
  padding: 20px;
  font-size: 15px;
  color: var(--text-primary);
  line-height: 1.6;
  resize: none;
  outline: none;
}

.content-input::placeholder {
  color: var(--text-muted);
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
  border-radius: 12px;
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
  border-radius: 10px;
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
  border-color: var(--accent-blue, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
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
  border-radius: 8px;
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
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.confirm-btn:hover {
  background-color: #4a8bd6;
}

.delete-confirm-btn {
  padding: 10px 20px;
  background-color: #ef4444;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.delete-confirm-btn:hover {
  background-color: #dc2626;
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
  border-radius: 4px;
  font-size: 16px;
  transition: all 0.2s;
}

.more-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.more-menu-content {
  background-color: var(--bg-secondary);
  border-radius: 12px;
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
  border-radius: 8px;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.more-menu-item:hover {
  background-color: var(--hover-bg);
}

.more-menu-item.danger {
  color: #ef4444;
}

.more-menu-item.danger:hover {
  background-color: rgba(239, 68, 68, 0.1);
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
