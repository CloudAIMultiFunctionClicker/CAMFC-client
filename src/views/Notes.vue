<template>
  <div class="notes-container">
    <div class="notes-header">
      <h1 class="page-title">笔记</h1>
      <button class="add-btn" @click="showAddModal = true">
        <i class="ri-add-line"></i>
        新建笔记
      </button>
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
            <button class="delete-btn" @click.stop="deleteNote(note.id)">
              <i class="ri-delete-bin-line"></i>
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="selectedNote" class="note-editor">
      <div class="editor-header">
        <input
          v-model="selectedNote.title"
          class="title-input"
          placeholder="笔记标题"
          @input="saveNote"
        >
        <button class="close-btn" @click="selectedNote = null">
          <i class="ri-close-line"></i>
        </button>
      </div>
      <textarea
        v-model="selectedNote.content"
        class="content-input"
        placeholder="开始写笔记..."
        @input="saveNote"
      ></textarea>
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
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const notes = ref([])
const selectedNote = ref(null)
const showAddModal = ref(false)
const newNoteTitle = ref('')

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
  notes.value = notes.value.filter(n => n.id !== id)
  if (selectedNote.value?.id === id) {
    selectedNote.value = null
  }
  saveNotes()
}

function saveNote() {
  if (selectedNote.value) {
    selectedNote.value.updatedAt = new Date().toISOString()
    saveNotes()
  }
}

function formatDate(dateStr) {
  const date = new Date(dateStr)
  return `${date.getMonth() + 1}/${date.getDate()}`
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

.delete-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
}

.delete-btn:hover {
  color: #ef4444;
  background-color: rgba(239, 68, 68, 0.1);
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

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
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
