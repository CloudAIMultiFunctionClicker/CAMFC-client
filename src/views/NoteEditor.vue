<template>
  <div class="note-editor-container">
    <div class="editor-header">
      <div class="title-wrapper">
        <input
          v-model="noteTitle"
          class="title-input"
          placeholder="笔记标题"
          @input="handleTitleChange"
        >
      </div>
      <div class="header-actions">
        <button class="save-btn" @click="saveAndClose">
          <i class="ri-check-line"></i>
          保存
        </button>
      </div>
    </div>

    <div class="editor-body">
      <div
        ref="editorTextarea"
        class="note-editor-content"
        contenteditable="true"
        placeholder="使用 Markdown 格式书写... 支持 Ctrl+V 粘贴图片、拖拽图片到此处"
        @input="handleEditorInput"
        @keydown="handleEditorKeydown"
        @paste="handlePaste"
        @dragenter="handleDragEnter"
        @dragleave="handleDragLeave"
        @drop="handleDrop"
      ></div>
    </div>

    <div class="editor-toolbar">
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
        <button class="toolbar-btn" @click="handleImageClick">
          <i class="ri-image-line"></i>
        </button>
        <span class="tooltip">图片<span class="tooltip-syntax">Ctrl+V 粘贴或拖拽图片</span></span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import axios from 'axios'
import { showToast } from '../components/layout/showToast.js'
import { getBackendUrl } from '../config/backend.js'

const route = useRoute()
const router = useRouter()

const timeOut = 3000
let unlisten = null

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

const noteTitle = ref('')
const noteContent = ref('')
const noteUuid = ref('')
const editorTextarea = ref(null)
const isDraggingOver = ref(false)
const isNewNote = ref(true)

let unlistenClose = null

onMounted(async () => {
  const uuid = route.query.uuid
  if (uuid) {
    noteUuid.value = uuid
    isNewNote.value = false
    await loadNote(uuid)
  } else {
    noteUuid.value = crypto.randomUUID()
    isNewNote.value = true
  }

  unlisten = await listen('update-note-uuid', async (event) => {
    const uuid = event.payload
    if (uuid) {
      noteUuid.value = uuid
      isNewNote.value = false
      await loadNote(uuid)
    }
  })

  // 监听窗口关闭事件，实现关闭前确认
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  const win = getCurrentWindow()
  unlistenClose = await win.onCloseRequested(async (event) => {
    const tempNote = localStorage.getItem('temp_note_' + noteUuid.value)
    if (tempNote) {
      const confirmed = confirm('有未保存的更改，确定要关闭吗？')
      if (!confirmed) {
        event.preventDefault()
        return
      }
    }
    // 清理临时存储
    localStorage.removeItem('temp_note_' + noteUuid.value)
  })
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
  if (unlistenClose) {
    unlistenClose()
  }
})

async function loadNote(uuid) {
  try {
    const data = await apiRequest('/note/query', { num: 100 })
    let notesList = data
    if (data && typeof data === 'object' && !Array.isArray(data)) {
      notesList = data.data || data.notes || data.result || []
    }
    const notes = Array.isArray(notesList) ? notesList : []
    const note = notes.find(n => n.uuid === uuid)
    if (note) {
      noteTitle.value = note.title
      noteContent.value = note.content || ''
      setTimeout(() => {
        initEditorContent()
      }, 100)
    }
  } catch (e) {
    console.error('加载笔记失败:', e)
    showToast('加载笔记失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

function initEditorContent() {
  if (editorTextarea.value) {
    editorTextarea.value.innerHTML = renderMarkdown(noteContent.value)
  }
}

function handleTitleChange() {
  saveNote()
}

function handleEditorInput() {
  if (editorTextarea.value) {
    const html = editorTextarea.value.innerHTML
    noteContent.value = convertHtmlToMarkdown(html)
    saveNote()
  }
}

function handleEditorKeydown(e) {
  if (e.key === 'Enter' && !e.shiftKey) {
    const selection = window.getSelection()
    if (selection.rangeCount > 0) {
      const range = selection.getRangeAt(0)
      const container = range.startContainer
      if (container.nodeType === Node.TEXT_NODE && container.textContent.startsWith('#')) {
        e.preventDefault()
        document.execCommand('insertHTML', false, '<div><br></div>')
      }
    }
  }
  if (e.ctrlKey && e.key === 's') {
    e.preventDefault()
    saveAndClose()
  }
}

const handlePaste = async (event) => {
  const clipboardData = event.clipboardData
  if (!clipboardData) return

  const items = clipboardData.items
  for (let i = 0; i < items.length; i++) {
    if (items[i].type.startsWith('image/')) {
      event.preventDefault()
      const blob = items[i].getAsFile()
      if (blob) {
        await insertImageFromBlob(blob)
      }
      return
    }
  }
}

const insertImageFromBlob = (blob) => {
  return new Promise((resolve) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      const base64 = e.target.result
      insertMarkdownImage(base64)
      resolve()
    }
    reader.readAsDataURL(blob)
  })
}

const insertMarkdownImage = (imageData) => {
  const editor = editorTextarea.value
  if (!editor) return

  const imgHtml = `<div class="markdown-image-wrapper"><img src="${imageData}" alt="截图_${Date.now()}" class="markdown-image" onerror="this.style.display='none'; this.nextSibling && (this.nextSibling.style.display='flex')"></div><div class="markdown-image-error" style="display:none"><i class="ri-image-line"></i><span>图片加载失败</span></div><div><br></div>`

  const selection = window.getSelection()
  if (selection.rangeCount > 0) {
    const range = selection.getRangeAt(0)
    range.deleteContents()
    const tempDiv = document.createElement('div')
    tempDiv.innerHTML = imgHtml
    range.insertNode(tempDiv)
    range.collapse(false)
  } else {
    editor.innerHTML += imgHtml
  }

  noteContent.value = convertHtmlToMarkdown(editor.innerHTML)
  saveNote()
  showToast('图片已插入', '#10b981')
}

const handleDragEnter = (e) => {
  e.preventDefault()
  isDraggingOver.value = true
}

const handleDragLeave = (e) => {
  e.preventDefault()
  isDraggingOver.value = false
}

const handleDrop = async (e) => {
  e.preventDefault()
  isDraggingOver.value = false

  const files = e.dataTransfer.files
  if (files.length === 0) return

  const file = files[0]
  if (!file.type.startsWith('image/')) {
    showToast('仅支持图片文件', '#f59e0b')
    return
  }

  await insertImageFromBlob(file)
}

const handleImageClick = () => {
  navigator.clipboard.read().then(items => {
    for (const item of items) {
      if (item.types.some(type => type.startsWith('image/'))) {
        item.getType('image/').then(blob => {
          insertImageFromBlob(blob)
        })
        return
      }
    }
    showToast('剪贴板无图片，可直接 Ctrl+V 粘贴或拖拽图片', '#f59e0b')
  }).catch(() => {
    showToast('可直接 Ctrl+V 粘贴或拖拽图片到编辑器', '#f59e0b')
  })
}

function insertMarkdown(type) {
  const editor = editorTextarea.value
  if (!editor) return

  const selection = window.getSelection()
  let selectedText = ''
  if (selection.rangeCount > 0) {
    selectedText = selection.toString()
  }

  let insert = ''
  switch (type) {
    case 'h1':
      insert = selectedText ? `<h1>${selectedText}</h1>` : '<h1>标题</h1>'
      break
    case 'h2':
      insert = selectedText ? `<h2>${selectedText}</h2>` : '<h2>标题</h2>'
      break
    case 'h3':
      insert = selectedText ? `<h3>${selectedText}</h3>` : '<h3>标题</h3>'
      break
    case 'bold':
      insert = selectedText ? `<strong>${selectedText}</strong>` : '<strong>加粗文本</strong>'
      break
    case 'italic':
      insert = selectedText ? `<em>${selectedText}</em>` : '<em>斜体文本</em>'
      break
    case 'strike':
      insert = selectedText ? `<del>${selectedText}</del>` : '<del>删除线</del>'
      break
    case 'code':
      insert = selectedText ? `<code>${selectedText}</code>` : '<code>代码</code>'
      break
    case 'list':
      insert = selectedText ? `<li>${selectedText}</li>` : '<li>列表项</li>'
      break
    case 'image':
      showToast('请使用 Ctrl+V 粘贴或拖拽图片', '#f59e0b')
      return
  }

  if (selection.rangeCount > 0) {
    const range = selection.getRangeAt(0)
    range.deleteContents()
    const tempDiv = document.createElement('div')
    tempDiv.innerHTML = insert
    range.insertNode(tempDiv)
    range.collapse(false)
  } else {
    editor.innerHTML += insert
  }

  noteContent.value = convertHtmlToMarkdown(editor.innerHTML)
  saveNote()
}

function saveNote() {
  // 可以在本地存储临时副本
  localStorage.setItem('temp_note_' + noteUuid.value, JSON.stringify({
    title: noteTitle.value,
    content: noteContent.value
  }))
}

async function saveAndClose() {
  if (!noteTitle.value.trim()) {
    showToast('请输入笔记标题', '#f59e0b')
    return
  }

  try {
    if (isNewNote.value) {
      await apiRequest('/note/add', { uuid: noteUuid.value, title: noteTitle.value })
    }
    await apiRequest('/note/update', {
      uuid: noteUuid.value,
      title: noteTitle.value,
      content: noteContent.value || ''
    })

    localStorage.removeItem('temp_note_' + noteUuid.value)
    showToast('笔记保存成功', '#10b981')

    // 通知主窗口刷新
    try {
      const { emit } = await import('@tauri-apps/api/event')
      await emit('notes-updated', { uuid: noteUuid.value })
    } catch (e) {
      console.log('通知主窗口失败:', e)
    }

    // 关闭窗口
    try {
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
      const win = await WebviewWindow.getByLabel('note_editor_new')
      if (win) {
        await win.close()
      } else {
        const currentWin = await WebviewWindow.getCurrent()
        if (currentWin) {
          await currentWin.close()
        }
      }
    } catch (e) {
      console.error('关闭窗口失败:', e)
    }
  } catch (e) {
    console.error('保存笔记失败:', e)
    showToast('保存失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

async function handleClose() {
  // 检查是否有未保存的更改
  const tempNote = localStorage.getItem('temp_note_' + noteUuid.value)
  if (tempNote) {
    // 使用原生 confirm 替代 dialog 插件
    const confirmed = confirm('有未保存的更改，确定要关闭吗？')
    if (!confirmed) return
  }

  // 清理临时存储
  localStorage.removeItem('temp_note_' + noteUuid.value)

  // 使用 Tauri 的 WebviewWindow 关闭当前窗口
  try {
    const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
    const win = await WebviewWindow.getByLabel('note_editor_new')
    if (win) {
      await win.close()
    } else {
      // 如果是编辑窗口，尝试获取当前窗口标签
      const currentWin = await WebviewWindow.getCurrent()
      if (currentWin) {
        await currentWin.close()
      }
    }
  } catch (e) {
    console.error('关闭窗口失败:', e)
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
    .replace(/!\[([^\]]*)\]\(([^)]+)\)/gim, '<div class="markdown-image-wrapper"><img src="$2" alt="$1" class="markdown-image" onerror="this.style.display=\'none\'; this.nextSibling && (this.nextSibling.style.display=\'flex\')"></div><div class="markdown-image-error" style="display:none"><i class="ri-image-line"></i><span>图片加载失败</span></div>')
    .replace(/\n/gim, '<br>')

  return html
}

function convertHtmlToMarkdown(html) {
  let text = html
    .replace(/<div><br><\/div>/g, '\n')
    .replace(/<div>(.*?)<\/div>/g, '$1\n')
    .replace(/<h1>(.*?)<\/h1>/g, '# $1\n')
    .replace(/<h2>(.*?)<\/h2>/g, '## $1\n')
    .replace(/<h3>(.*?)<\/h3>/g, '### $1\n')
    .replace(/<strong>(.*?)<\/strong>/g, '**$1**')
    .replace(/<b>(.*?)<\/b>/g, '**$1**')
    .replace(/<em>(.*?)<\/em>/g, '*$1*')
    .replace(/<i>(.*?)<\/i>/g, '*$1*')
    .replace(/<del>(.*?)<\/del>/g, '~~$1~~')
    .replace(/<code>(.*?)<\/code>/g, '`$1`')
    .replace(/<li>(.*?)<\/li>/g, '- $1\n')
    .replace(/<ul>|<\/ul>|<ol>|<\/ol>/g, '')
    .replace(/<br\s*\/?>/g, '\n')
    .replace(/<img src="([^"]*)" alt="([^"]*)"[^>]*>/g, '![$2]($1)')
    .replace(/<span class="markdown-image-wrapper">/g, '')
    .replace(/<div class="markdown-image-error"[^>]*>.*?<\/div>/g, '')
    .replace(/<div class="img-error">.*?<\/div>/g, '')
    .replace(/<[^>]+>/g, '')
    .replace(/&nbsp;/g, ' ')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&amp;/g, '&')
  return text.trim()
}
</script>

<style scoped>
.note-editor-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-secondary);
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
}

.title-wrapper {
  flex: 1;
  margin-right: 20px;
}

.title-input {
  width: 100%;
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

.header-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.save-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.save-btn:hover {
  background-color: var(--accent-blue-bright);
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

.editor-body {
  flex: 1;
  padding: 20px;
  overflow: hidden;
}

.note-editor-content {
  width: 100%;
  height: 100%;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 15px;
  line-height: 1.7;
  resize: none;
  outline: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.note-editor-content:empty::before {
  content: attr(placeholder);
  color: var(--text-muted);
  pointer-events: none;
  display: block;
}

.note-editor-content :deep(h1),
.note-editor-content :deep(h2),
.note-editor-content :deep(h3) {
  margin: 16px 0 10px;
  color: var(--text-primary);
}

.note-editor-content :deep(code) {
  background-color: var(--bg-primary);
  padding: 2px 6px;
  border-radius: .375rem;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 14px;
}

.note-editor-content :deep(strong) {
  font-weight: 600;
}

.note-editor-content :deep(del) {
  color: var(--text-muted);
}

.note-editor-content :deep(li) {
  margin-left: 20px;
  margin-bottom: 4px;
}

.note-editor-content :deep(.markdown-image) {
  max-width: 100%;
  border-radius: .375rem;
  margin: 12px 0;
}

.note-editor-content :deep(.markdown-image-wrapper) {
  display: inline-block;
  width: 100%;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background-color: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: none;
  border: none;
  border-radius: .375rem;
  color: var(--text-secondary);
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background-color: var(--border-color);
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
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: 12px;
  white-space: nowrap;
  border-radius: .375rem;
  border: 1px solid var(--border-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
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
  border-top-color: var(--bg-primary);
}

.tooltip-syntax {
  display: block;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px dashed var(--border-color);
  font-family: 'Monaco', 'Menlo', monospace;
  color: var(--accent-blue);
  font-size: 11px;
}
</style>
