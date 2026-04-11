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
  <div class="generic-note-editor" :class="{ 'light-mode': isLightMode }">
    <!-- 标题输入区 -->
    <div v-if="showTitle" class="editor-title-section">
      <input 
        v-model="localTitle" 
        class="editor-title-input" 
        :placeholder="titlePlaceholder"
        type="text"
        @input="handleTitleInput"
      />
    </div>
    
    <!-- 编辑器主体 -->
    <div class="editor-body">
      <div 
        class="editor-container" 
        :class="{ 'dragging-over': isDraggingOver }" 
        @paste="handlePaste" 
        @dragenter="handleDragEnter" 
        @dragleave="handleDragLeave" 
        @drop="handleDrop"
      >
        <div
          ref="editorTextarea"
          class="note-editor-content"
          contenteditable="true"
          :placeholder="contentPlaceholder"
          @input="handleEditorInput"
          @keydown="handleEditorKeydown"
        ></div>
      </div>
    </div>
    
    <!-- 工具栏 -->
    <div v-if="showToolbar" class="editor-toolbar">
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
      <div v-if="enableImage" class="toolbar-btn-wrapper">
        <button class="toolbar-btn" @click="handleImageClick">
          <i class="ri-image-line"></i>
        </button>
        <span class="tooltip">图片<span class="tooltip-syntax">Ctrl+V 粘贴或拖拽图片</span></span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue'

const props = defineProps({
  // 标题相关
  modelValueTitle: { type: String, default: '' },
  showTitle: { type: Boolean, default: true },
  titlePlaceholder: { type: String, default: '未命名笔记' },
  
  // 内容相关
  modelValue: { type: String, default: '' },
  contentPlaceholder: { type: String, default: '使用 Markdown 格式书写... 支持 Ctrl+V 粘贴图片、拖拽图片到此处' },
  
  // 功能开关
  showToolbar: { type: Boolean, default: true },
  enableImage: { type: Boolean, default: true },
  enablePaste: { type: Boolean, default: true },
  enableDragDrop: { type: Boolean, default: true },
  
  // 外观
  isLightMode: { type: Boolean, default: false },
  
  // 快捷键
  enableSaveShortcut: { type: Boolean, default: true }
})

const emit = defineEmits([
  'update:modelValue',
  'update:modelValueTitle',
  'input',
  'titleInput',
  'save',
  'imageInserted'
])

// 本地状态
const localTitle = ref('')
const localContent = ref('')
const editorTextarea = ref(null)
const isDraggingOver = ref(false)

// 初始化
onMounted(() => {
  localTitle.value = props.modelValueTitle
  localContent.value = props.modelValue

  if (editorTextarea.value) {
    editorTextarea.value.innerHTML = renderMarkdown(props.modelValue)
    // 添加图片块点击事件委托
    editorTextarea.value.addEventListener('click', handleImageBlockClick)
  }

  // 监听保存快捷键
  if (props.enableSaveShortcut) {
    document.addEventListener('keydown', handleGlobalKeydown)
  }
})

onUnmounted(() => {
  if (props.enableSaveShortcut) {
    document.removeEventListener('keydown', handleGlobalKeydown)
  }
  // 清理图片块点击事件
  if (editorTextarea.value) {
    editorTextarea.value.removeEventListener('click', handleImageBlockClick)
  }
})

// 处理图片块点击 - 选中图片块或删除
function handleImageBlockClick(e) {
  const imageBlock = e.target.closest('.markdown-image-block')
  if (imageBlock) {
    e.preventDefault()
    e.stopPropagation()

    // 检查是否点击了删除按钮区域（右上角）
    const rect = imageBlock.getBoundingClientRect()
    const clickX = e.clientX - rect.left
    const clickY = e.clientY - rect.top
    const isDeleteClick = clickX > rect.width - 20 && clickY < 20

    if (isDeleteClick && imageBlock.classList.contains('selected')) {
      // 删除图片块
      imageBlock.remove()
      localContent.value = convertHtmlToMarkdown(editorTextarea.value.innerHTML)
      emit('update:modelValue', localContent.value)
      return
    }

    // 高亮选中的图片块
    document.querySelectorAll('.markdown-image-block.selected').forEach(el => {
      el.classList.remove('selected')
    })
    imageBlock.classList.add('selected')
  } else {
    // 点击图片块外部，取消选中
    document.querySelectorAll('.markdown-image-block.selected').forEach(el => {
      el.classList.remove('selected')
    })
  }
}

// 监听外部值变化
watch(() => props.modelValue, (newVal) => {
  if (newVal !== localContent.value && editorTextarea.value) {
    localContent.value = newVal
    editorTextarea.value.innerHTML = renderMarkdown(newVal)
  }
})

watch(() => props.modelValueTitle, (newVal) => {
  if (newVal !== localTitle.value) {
    localTitle.value = newVal
  }
})

// 全局键盘事件（Ctrl+S 保存）
function handleGlobalKeydown(e) {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    emit('save', { title: localTitle.value, content: localContent.value })
  }
}

// 标题输入处理
function handleTitleInput() {
  emit('update:modelValueTitle', localTitle.value)
  emit('titleInput', localTitle.value)
}

// 编辑器输入处理
function handleEditorInput() {
  if (editorTextarea.value) {
    const html = editorTextarea.value.innerHTML
    const markdown = convertHtmlToMarkdown(html)
    localContent.value = markdown
    emit('update:modelValue', markdown)
    emit('input', markdown)
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
    .replace(/<div class="markdown-image-block"[^>]*>/g, '')
    .replace(/<\/div><div class="markdown-image-block"[^>]*>/g, '')
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
  if (!props.enablePaste) return
  
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

  // 使用 contenteditable="false" 禁止光标进入图片区域，作为块元素处理
  const imgHtml = `<div class="markdown-image-block" contenteditable="false"><div class="markdown-image-wrapper"><img src="${imageData}" alt="截图_${Date.now()}" class="markdown-image" onerror="this.style.display='none'; this.nextSibling && (this.nextSibling.style.display='flex')"></div><div class="markdown-image-error" style="display:none"><i class="ri-image-line"></i><span>图片加载失败</span></div></div><div><br></div>`

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

  localContent.value = convertHtmlToMarkdown(editor.innerHTML)
  emit('update:modelValue', localContent.value)
  emit('imageInserted', imageData)
}

// 拖拽处理
function handleDragEnter(e) {
  if (!props.enableDragDrop) return
  e.preventDefault()
  isDraggingOver.value = true
}

function handleDragLeave(e) {
  if (!props.enableDragDrop) return
  e.preventDefault()
  isDraggingOver.value = false
}

async function handleDrop(e) {
  if (!props.enableDragDrop) return
  e.preventDefault()
  isDraggingOver.value = false

  const files = e.dataTransfer.files
  if (files.length === 0) return

  const file = files[0]
  if (!file.type.startsWith('image/')) return

  await insertImageFromBlob(file)
}

// 图片按钮点击
function handleImageClick() {
  if (!props.enableImage) return
  
  navigator.clipboard.read().then(items => {
    for (const item of items) {
      if (item.types.some(type => type.startsWith('image/'))) {
        item.getType('image/').then(blob => {
          insertImageFromBlob(blob)
        })
        return
      }
    }
  }).catch(() => {
    // 静默失败
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

  localContent.value = convertHtmlToMarkdown(editor.innerHTML)
  emit('update:modelValue', localContent.value)
}

// 暴露方法给父组件
defineExpose({
  getContent: () => localContent.value,
  getTitle: () => localTitle.value,
  setContent: (content) => {
    localContent.value = content
    if (editorTextarea.value) {
      editorTextarea.value.innerHTML = renderMarkdown(content)
    }
    emit('update:modelValue', content)
  },
  setTitle: (title) => {
    localTitle.value = title
    emit('update:modelValueTitle', title)
  },
  insertImage: insertMarkdownImage,
  focus: () => {
    if (editorTextarea.value) {
      editorTextarea.value.focus()
    }
  }
})
</script>

<style scoped>
.generic-note-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary, #0d1117);
  color: var(--text-primary, #f8fafc);
}

.generic-note-editor.light-mode {
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #1e293b);
}

/* 标题区域 */
.editor-title-section {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.light-mode .editor-title-section {
  border-bottom-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.editor-title-input {
  width: 100%;
  font-size: 20px;
  font-weight: 600;
  background: transparent;
  border: none;
  color: var(--text-primary, #f8fafc);
  outline: none;
  padding: 0;
}

.light-mode .editor-title-input {
  color: var(--text-primary, #1e293b);
}

.editor-title-input::placeholder {
  color: var(--text-secondary, #94a3b8);
}

/* 编辑器主体 */
.editor-body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.editor-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  transition: background-color 0.2s;
}

.editor-container.dragging-over {
  background-color: rgba(59, 130, 246, 0.1);
}

.note-editor-content {
  min-height: 100%;
  outline: none;
  line-height: 1.8;
  font-size: 15px;
}

.note-editor-content:empty::before {
  content: attr(placeholder);
  color: var(--text-secondary, #94a3b8);
  pointer-events: none;
}

/* 工具栏 */
.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 12px 16px;
  border-top: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  background: var(--bg-primary, #0d1117);
  overflow-x: auto;
}

.light-mode .editor-toolbar {
  border-top-color: var(--border-color, rgba(0, 0, 0, 0.1));
  background: var(--bg-primary, #ffffff);
}

.toolbar-btn-wrapper {
  position: relative;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary, #cbd5e1);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 18px;
}

.light-mode .toolbar-btn {
  color: var(--text-secondary, #64748b);
}

.toolbar-btn:hover {
  background: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-primary, #f8fafc);
}

.light-mode .toolbar-btn:hover {
  background: var(--hover-bg, rgba(0, 0, 0, 0.05));
  color: var(--text-primary, #1e293b);
}

.toolbar-divider {
  width: 1px;
  height: 24px;
  background: var(--border-color, rgba(255, 255, 255, 0.1));
  margin: 0 8px;
}

.light-mode .toolbar-divider {
  background: var(--border-color, rgba(0, 0, 0, 0.1));
}

/* Tooltip */
.tooltip {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%) translateY(-8px);
  padding: 6px 10px;
  background: var(--bg-secondary, #161b22);
  color: var(--text-primary, #f8fafc);
  font-size: 12px;
  border-radius: 4px;
  white-space: nowrap;
  opacity: 0;
  visibility: hidden;
  transition: all 0.2s;
  z-index: 10000;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border: 4px solid transparent;
  border-top-color: var(--bg-secondary, #161b22);
}

.tooltip-syntax {
  display: block;
  color: var(--text-secondary, #94a3b8);
  font-size: 11px;
  margin-top: 2px;
}

.toolbar-btn-wrapper:hover .tooltip {
  opacity: 1;
  visibility: visible;
}

/* Markdown 样式 */
.note-editor-content :deep(h1),
.note-editor-content :deep(h2),
.note-editor-content :deep(h3) {
  margin: 16px 0 12px;
  font-weight: 600;
  line-height: 1.4;
}

.note-editor-content :deep(h1) {
  font-size: 24px;
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  padding-bottom: 8px;
}

.light-mode .note-editor-content :deep(h1) {
  border-bottom-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.note-editor-content :deep(h2) {
  font-size: 20px;
}

.note-editor-content :deep(h3) {
  font-size: 18px;
}

.note-editor-content :deep(code) {
  background: var(--bg-secondary, #161b22);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 13px;
}

.light-mode .note-editor-content :deep(code) {
  background: var(--bg-secondary, #f1f5f9);
}

.note-editor-content :deep(strong) {
  font-weight: 600;
}

.note-editor-content :deep(del) {
  text-decoration: line-through;
  opacity: 0.7;
}

.note-editor-content :deep(li) {
  margin: 4px 0;
  padding-left: 20px;
  position: relative;
}

.note-editor-content :deep(li)::before {
  content: '•';
  position: absolute;
  left: 4px;
  color: var(--text-secondary, #94a3b8);
}

.note-editor-content :deep(.markdown-image) {
  max-width: 100%;
  border-radius: 8px;
  display: block;
  pointer-events: none;
}

/* 图片块容器 - 作为块元素，禁止光标进入 */
.note-editor-content :deep(.markdown-image-block) {
  display: block;
  margin: 12px 0;
  user-select: none;
  cursor: default;
  position: relative;
  transition: outline 0.2s ease;
}

.note-editor-content :deep(.markdown-image-block):hover,
.note-editor-content :deep(.markdown-image-block.selected) {
  outline: 2px solid var(--accent-blue, #3b82f6);
  outline-offset: 4px;
  border-radius: 4px;
}

/* 选中状态下的删除按钮 */
.note-editor-content :deep(.markdown-image-block.selected)::after {
  content: '×';
  position: absolute;
  top: -8px;
  right: -8px;
  width: 20px;
  height: 20px;
  background: #ef4444;
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: bold;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.note-editor-content :deep(.markdown-image-wrapper) {
  display: block;
  max-width: 100%;
}

.note-editor-content :deep(.markdown-image-error) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 20px;
  background: var(--bg-secondary, #161b22);
  border-radius: 8px;
  color: var(--text-secondary, #94a3b8);
}

.light-mode .note-editor-content :deep(.markdown-image-error) {
  background: var(--bg-secondary, #f1f5f9);
}
</style>
