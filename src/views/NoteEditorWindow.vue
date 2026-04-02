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
  <div class="editor-window">
    <div class="editor-header" data-tauri-drag-region>
      <input 
        v-model="noteTitle" 
        class="editor-title-input" 
        placeholder="未命名笔记"
        type="text"
      />
      <div class="editor-actions">
        <button class="action-btn save-btn" @click="saveAndClose" title="保存">
          <i class="ri-check-line"></i>
        </button>
        <button class="action-btn close-btn" @click="handleClose" title="关闭">
          <i class="ri-close-line"></i>
        </button>
      </div>
    </div>
    
    <div class="editor-body">
      <div 
        class="editor-container" 
        :class="{ 'dragging-over': isDraggingOver }" 
        data-tauri-drag-region
        @paste="handlePaste" 
        @dragenter="handleDragEnter" 
        @dragleave="handleDragLeave" 
        @drop="handleDrop"
      >
        <div
          ref="editorTextarea"
          class="note-editor-content"
          contenteditable="true"
          placeholder="使用 Markdown 格式书写... 支持 Ctrl+V 粘贴图片、拖拽图片到此处"
          @input="handleEditorInput"
          @keydown="handleEditorKeydown"
        ></div>
      </div>
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

const timeOut = 3000
const route = useRoute()

// 笔记数据
const noteUuid = ref('')
const noteTitle = ref('')
const noteContent = ref('')
const originalContent = ref('')

// 编辑器引用
const editorTextarea = ref(null)
const isDraggingOver = ref(false)
const showSaveConfirmModal = ref(false)

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
  // 从 URL 参数获取笔记信息（content 不再从 URL 获取）
  const uuid = route.query.uuid
  const title = route.query.title
  
  if (!uuid) {
    showToast('笔记信息不完整', '#ef4444')
    setTimeout(() => closeWindow(), 1500)
    return
  }
  
  noteUuid.value = uuid
  noteTitle.value = title || '未命名笔记'
  // content 初始为空，等待主窗口发送
  noteContent.value = ''
  originalContent.value = ''
  
  // 监听主窗口发送的内容
  const unlistenContent = await listen('load-note-content', (event) => {
    const content = event.payload?.content || ''
    noteContent.value = content
    originalContent.value = content
    // 初始化编辑器内容
    if (editorTextarea.value) {
      editorTextarea.value.innerHTML = renderMarkdown(content)
    }
  })
  
  // 保存 unlisten 函数用于卸载
  window._unlistenContent = unlistenContent
  
  // 监听保存快捷键
  document.addEventListener('keydown', handleGlobalKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleGlobalKeydown)
  // 清理事件监听
  if (window._unlistenContent) {
    window._unlistenContent()
  }
})

// 全局键盘事件（Ctrl+S 保存）
function handleGlobalKeydown(e) {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    saveNote()
  }
}

// 关闭窗口
async function closeWindow() {
  try {
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

// 保存笔记
async function saveNote() {
  if (!noteUuid.value) return
  
  try {
    await apiRequest('/note/update', { 
      uuid: noteUuid.value, 
      content: noteContent.value || '',
      title: noteTitle.value 
    })
    originalContent.value = noteContent.value
    showToast('保存成功', '#10b981')
    
    // 通知主窗口刷新笔记列表
    await emit('note-saved', { uuid: noteUuid.value })
  } catch (e) {
    console.error('保存笔记失败:', e)
    showToast('保存失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

// 编辑器输入处理
function handleEditorInput() {
  if (editorTextarea.value) {
    const html = editorTextarea.value.innerHTML
    noteContent.value = convertHtmlToMarkdown(html)
  }
}

// 编辑器键盘事件
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
}

// Markdown 渲染（用于初始化显示）
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

// HTML 转 Markdown
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

// 粘贴处理
async function handlePaste(event) {
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

// 从 Blob 插入图片
function insertImageFromBlob(blob) {
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

// 插入 Markdown 图片
function insertMarkdownImage(imageData) {
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
  showToast('图片已插入', '#10b981')
}

// 拖拽处理
function handleDragEnter(e) {
  e.preventDefault()
  isDraggingOver.value = true
}

function handleDragLeave(e) {
  e.preventDefault()
  isDraggingOver.value = false
}

async function handleDrop(e) {
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

// 图片按钮点击
function handleImageClick() {
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

// 插入 Markdown
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
}
</script>

<style scoped>
.editor-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-primary, #0d1117);
  color: var(--text-primary, #c9d1d9);
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: var(--bg-secondary, #161b22);
  border-bottom: 1px solid var(--border-color, #30363d);
  -webkit-app-region: drag;
}

.editor-title-input {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary, #c9d1d9);
  background-color: transparent;
  border: none;
  outline: none;
  padding: 4px 8px;
  border-radius: .375rem;
  flex: 1;
  min-width: 0;
  transition: all 0.2s;
}

.editor-title-input:focus {
  background-color: var(--bg-primary, #0d1117);
}

.editor-title-input::placeholder {
  color: var(--text-muted, #6e7681);
}

.editor-actions {
  display: flex;
  gap: 6px;
  -webkit-app-region: no-drag;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  background: none;
  border: none;
  border-radius: .375rem;
  color: var(--text-secondary, #8b949e);
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.save-btn {
  color: #238636;
}

.save-btn:hover {
  background-color: rgba(35, 134, 54, 0.15);
}

.close-btn:hover {
  color: #f85149;
  background-color: rgba(248, 81, 73, 0.15);
}

.editor-body {
  flex: 1;
  overflow: hidden;
  padding: 16px;
}

.editor-container {
  width: 100%;
  height: 100%;
  overflow-y: auto;
}

.editor-container.dragging-over {
  border: 2px dashed var(--accent-blue, #58a6ff);
  background-color: rgba(88, 166, 255, 0.1);
  border-radius: .375rem;
}

.note-editor-content {
  width: 100%;
  min-height: 100%;
  background: none;
  border: none;
  color: var(--text-primary, #c9d1d9);
  font-size: 15px;
  line-height: 1.7;
  outline: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.note-editor-content:empty::before {
  content: attr(placeholder);
  color: var(--text-muted, #6e7681);
  pointer-events: none;
  display: block;
}

.note-editor-content :deep(h1),
.note-editor-content :deep(h2),
.note-editor-content :deep(h3) {
  margin: 16px 0 10px;
  color: var(--text-primary, #c9d1d9);
  font-weight: 600;
}

.note-editor-content :deep(h1) {
  font-size: 24px;
  border-bottom: 1px solid var(--border-color, #30363d);
  padding-bottom: 8px;
}

.note-editor-content :deep(h2) {
  font-size: 20px;
}

.note-editor-content :deep(h3) {
  font-size: 16px;
}

.note-editor-content :deep(code) {
  background-color: rgba(110, 118, 129, 0.4);
  padding: 2px 6px;
  border-radius: .375rem;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 14px;
  color: #ff7b72;
}

.note-editor-content :deep(strong) {
  font-weight: 600;
}

.note-editor-content :deep(del) {
  color: var(--text-muted, #6e7681);
}

.note-editor-content :deep(li) {
  margin-left: 20px;
  margin-bottom: 4px;
}

.note-editor-content :deep(.markdown-image) {
  max-width: 100%;
  border-radius: .375rem;
  margin: 12px 0;
  border: 1px solid var(--border-color, #30363d);
}

.note-editor-content :deep(.markdown-image-wrapper) {
  display: inline-block;
  width: 100%;
}

.note-editor-content :deep(.markdown-image-error) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 20px;
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  color: var(--text-muted, #6e7681);
  margin: 12px 0;
}

.editor-toolbar {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
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
  border-radius: .375rem;
  color: var(--text-secondary, #8b949e);
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
  color: var(--text-primary, #c9d1d9);
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background-color: var(--border-color, #30363d);
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
  background-color: var(--bg-primary, #0d1117);
  color: var(--text-primary, #c9d1d9);
  font-size: 12px;
  white-space: nowrap;
  border-radius: .375rem;
  border: 1px solid var(--border-color, #30363d);
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
  border-top-color: var(--bg-primary, #0d1117);
}

.tooltip-syntax {
  display: block;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px dashed var(--border-color, #30363d);
  font-family: 'Monaco', 'Menlo', monospace;
  color: var(--accent-blue, #58a6ff);
  font-size: 11px;
}

/* 弹窗样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-secondary, #161b22);
  border-radius: .375rem;
  width: 90%;
  max-width: 400px;
  border: 1px solid var(--border-color, #30363d);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, #30363d);
}

.modal-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--text-primary, #c9d1d9);
  display: flex;
  align-items: center;
  gap: 8px;
}

.modal-body {
  padding: 20px;
}

.save-modal-body p {
  margin: 0;
  color: var(--text-secondary, #8b949e);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color, #30363d);
}

.cancel-btn {
  padding: 8px 16px;
  background-color: transparent;
  color: var(--text-primary, #c9d1d9);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.cancel-btn:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.1));
}

.confirm-btn {
  padding: 8px 16px;
  background-color: #238636;
  color: white;
  border: none;
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.confirm-btn:hover {
  background-color: #2ea043;
}

/* 动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

/* 竖屏适配 */
@media (max-width: 600px) {
  .editor-header {
    padding: 10px 12px;
  }
  
  .editor-title {
    font-size: 13px;
    max-width: 50%;
  }
  
  .editor-body {
    padding: 12px;
  }
  
  .editor-toolbar {
    bottom: 12px;
    right: 12px;
    padding: 6px 8px;
    gap: 4px;
  }
  
  .toolbar-btn {
    width: 28px;
    height: 28px;
    font-size: 14px;
  }
  
  .toolbar-divider {
    height: 16px;
    margin: 0 2px;
  }
}
</style>
