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
  <div class="note-viewer-window" :class="{ 'light-mode': isLightMode }">
    <div class="viewer-header" data-tauri-drag-region>
      <div class="viewer-title-section">
        <i class="ri-sticky-note-line viewer-icon"></i>
        <span class="viewer-title">{{ noteTitle || '笔记查看' }}</span>
      </div>
      <div class="viewer-actions">
        <button class="action-btn window-btn" @click="minimizeWindow" title="最小化">
          <i class="ri-subtract-line"></i>
        </button>
        <button class="action-btn window-btn" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
          <i :class="isMaximized ? 'ri-fullscreen-exit-line' : 'ri-fullscreen-line'"></i>
        </button>
        <button class="action-btn close-btn" @click="closeWindow" title="关闭">
          <i class="ri-close-line"></i>
        </button>
      </div>
    </div>

    <div class="viewer-body">
      <div v-if="loading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载笔记内容...</p>
      </div>

      <div v-else-if="error" class="error-state">
        <i class="ri-error-warning-line error-icon"></i>
        <p>{{ error }}</p>
        <button class="retry-btn" @click="loadNoteData">重试</button>
      </div>

      <div v-else-if="noteData" class="note-content-wrapper">
        <!-- 笔记元信息 -->
        <div class="note-meta">
          <div class="meta-item">
            <i class="ri-file-text-line meta-icon"></i>
            <span class="meta-label">类型：</span>
            <span class="meta-value">{{ noteData.type === 'personal' ? '个人笔记' : '会议记录' }}</span>
          </div>
          <div class="meta-item">
            <i class="ri-user-line meta-icon"></i>
            <span class="meta-label">分享者：</span>
            <span class="meta-value">{{ noteData.shared_by }}</span>
          </div>
          <div class="meta-item">
            <i class="ri-time-line meta-icon"></i>
            <span class="meta-label">分享时间：</span>
            <span class="meta-value">{{ formatTime(noteData.shared_at) }}</span>
          </div>
        </div>

        <!-- 个人笔记内容 -->
        <div v-if="noteData.type === 'personal'" class="note-section">
          <h3 class="section-title">
            <i class="ri-file-text-line"></i> 笔记内容
          </h3>
          <div class="note-content" v-html="formatNoteContent(noteData.content)"></div>
        </div>

        <!-- 会议记录内容 -->
        <div v-if="noteData.type === 'meeting'" class="note-section">
          <h3 class="section-title">
            <i class="ri-calendar-line"></i> 会议笔记
          </h3>
          <div v-if="noteData.meeting_notes && noteData.meeting_notes.length > 0" class="meeting-notes-list">
            <div v-for="(note, index) in noteData.meeting_notes" :key="index" class="meeting-note-item">
              <p class="meeting-note-time">{{ note.formatted_time }}</p>
              <div class="meeting-note-content" v-html="formatNoteContent(note.content)"></div>
            </div>
          </div>
          <div v-else class="empty-content">
            <p>暂无会议笔记</p>
          </div>
        </div>

        <!-- AI 标题 -->
        <div v-if="noteData.ai_title" class="ai-section">
          <h3 class="section-title ai-title-heading">
            <i class="ri-robot-line"></i> AI 标题
          </h3>
          <div class="ai-content-box">
            <p class="ai-title-text">{{ noteData.ai_title.title }}</p>
          </div>
        </div>

        <!-- AI 关键词 -->
        <div v-if="noteData.ai_keywords && noteData.ai_keywords.key_words && noteData.ai_keywords.key_words.length > 0" class="ai-section">
          <h3 class="section-title ai-keywords-heading">
            <i class="ri-tag-line"></i> AI 关键词
          </h3>
          <div class="ai-content-box keywords-box">
            <span v-for="(keyword, index) in noteData.ai_keywords.key_words" :key="index" class="keyword-tag">
              {{ keyword }}
            </span>
          </div>
        </div>

        <!-- AI 分析 -->
        <div v-if="noteData.ai_analysis" class="ai-section ai-analysis-section">
          <h3 class="section-title ai-analysis-heading">
            <i class="ri-brain-line"></i> AI 分析
          </h3>
          <div class="ai-content-box analysis-box" v-html="formatAIAnalysis(noteData.ai_analysis)"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { getSharedNoteDetail } from '../components/data/group.js'
import { getBackendUrl, initBackendConfig } from '../config/backend.js'

const route = useRoute()
const timeOut = 5000

// 窗口状态
const currentWindow = getCurrentWindow()
const isMaximized = ref(false)
const isLightMode = ref(false)

// 笔记数据
const noteData = ref(null)
const noteTitle = ref('')
const loading = ref(true)
const error = ref('')

// 获取认证头
async function getAuthHeader(retryCount = 0) {
  const maxRetries = 3;
  const retryDelay = 300;
  
  try {
    const { getDeviceId, getTotp } = await import('../components/data/bluetooth.js')
    const deviceId = await getDeviceId()
    const currentTotp = await getTotp()
    
    if (currentTotp && deviceId) {
      return { "Id": deviceId, "Totp": currentTotp }
    }
    
    // 如果是教师端但没获取到，尝试重试等待蓝牙模块准备好
    if (retryCount < maxRetries) {
      console.log(`等待蓝牙模块准备中... (${retryCount + 1}/${maxRetries})`)
      await new Promise(resolve => setTimeout(resolve, retryDelay))
      return await getAuthHeader(retryCount + 1)
    }
  } catch {
    // 重试失败后尝试返回空 header
  }
  
  return {}
}

// 格式化时间戳
function formatTime(timestamp) {
  if (!timestamp) return ''
  
  let date
  // 尝试判断时间戳格式
  if (typeof timestamp === 'number') {
    // Unix 时间戳（秒），转换为毫秒
    date = new Date(timestamp * 1000)
  } else if (typeof timestamp === 'string') {
    // ISO 格式字符串
    date = new Date(timestamp)
  } else {
    return ''
  }
  
  // 检查日期是否有效
  if (isNaN(date.getTime())) return ''
  
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 格式化笔记内容（处理换行和图片）
function formatNoteContent(content) {
  if (!content) return ''
  
  let formatted = content
  
  // 处理 base64 图片数据
  // 匹配 data:image/xxx;base64,xxxx 格式的 base64 数据，并包裹成 img 标签
  const base64Regex = /(data:image\/(?:png|jpg|jpeg|gif|webp);base64,[A-Za-z0-9+/=]+)/g
  formatted = formatted.replace(base64Regex, (match, base64Data) => {
    return `<img src="${base64Data}" class="note-image" style="max-width:100%;max-height:50vh;width:auto;height:auto;display:block;margin:16px auto;border-radius: 2px;object-fit:contain;" loading="lazy" />`
  })
  
  // 处理 Markdown 图片格式 ![alt](url)
  formatted = formatted.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, (match, alt, url) => {
    return `<img src="${url}" alt="${alt}" class="note-image" style="max-width:100%;max-height:50vh;width:auto;height:auto;display:block;margin:16px auto;border-radius: 2px;object-fit:contain;" loading="lazy" />`
  })
  
  // 处理 HTML img 标签，添加样式类
  formatted = formatted.replace(/<img([^>]*)>/gi, (match, attrs) => {
    if (!attrs.includes('class=')) {
      return `<img${attrs} class="note-image" style="max-width:100%;max-height:50vh;width:auto;height:auto;display:block;margin:16px auto;border-radius: 2px;object-fit:contain;" loading="lazy" />`
    }
    if (!attrs.includes('loading=')) {
      return match.replace(/>/, ' style="max-width:100%;max-height:50vh;width:auto;height:auto;display:block;margin:16px auto;border-radius: 2px;object-fit:contain;" loading="lazy" />')
    }
    return match
  })
  
  // 将换行符替换为 <br>
  formatted = formatted.replace(/\n/g, '<br>')
  
  return formatted
}

// 格式化 AI 分析内容
function formatAIAnalysis(analysis) {
  if (!analysis) return ''

  if (typeof analysis === 'string') {
    try {
      const parsed = JSON.parse(analysis)
      analysis = parsed
    } catch (e) {
      return analysis.replace(/\n/g, '<br>')
    }
  }

  if (typeof analysis === 'object') {
    let html = ''

    if (analysis.summary) {
      const summaryText = typeof analysis.summary === 'string' 
        ? analysis.summary 
        : JSON.stringify(analysis.summary)
      html += `<p><strong>总结：</strong>${summaryText.replace(/\n/g, '<br>')}</p>`
    }

    if (analysis.individual_analyses && Array.isArray(analysis.individual_analyses)) {
      html += '<h5>详细分析：</h5><ul>'
      analysis.individual_analyses.forEach(item => {
        if (item.summary) {
          const itemText = typeof item.summary === 'string' 
            ? item.summary 
            : JSON.stringify(item.summary)
          html += `<li>${itemText.replace(/\n/g, '<br>')}</li>`
        }
      })
      html += '</ul>'
    }

    return html
  }

  return String(analysis).replace(/\n/g, '<br>')
}

// 加载笔记数据
async function loadNoteData() {
  const shareUuid = route.query.shareUuid
  const groupUuid = route.query.groupUuid

  if (!shareUuid || !groupUuid) {
    error.value = '缺少必要参数'
    loading.value = false
    return
  }

  loading.value = true
  error.value = ''

  try {
    const result = await getSharedNoteDetail(shareUuid, groupUuid)
    if (result && result.success && result.note) {
      noteData.value = result.note
      noteTitle.value = result.note.title || '共享笔记'
    } else {
      error.value = '获取笔记详情失败'
    }
  } catch (e) {
    console.error('获取笔记详情失败:', e)
    error.value = '获取笔记详情失败: ' + (e.response?.data?.detail || e.message || '网络错误')
  } finally {
    loading.value = false
  }
}

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
    setTimeout(() => {
      checkWindowState()
    }, 50)
  } catch (error) {
    console.error('切换最大化失败:', error)
  }
}

// 关闭窗口
async function closeWindow() {
  try {
    const appWindow = getCurrentWindow()
    await appWindow.close()
  } catch (e) {
    console.error('关闭窗口失败:', e)
  }
}

// 初始化主题
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

// 监听主题变化
async function setupThemeListener() {
  try {
    await listen('theme-changed', (event) => {
      const theme = event.payload
      isLightMode.value = theme === 'light'
    })
  } catch (e) {
    console.warn('设置主题监听失败（非关键功能）:', e)
  }
}

onMounted(async () => {
  // 从 URL 参数获取笔记信息
  const shareUuid = route.query.shareUuid
  const groupUuid = route.query.groupUuid
  const title = route.query.title

  // 初始化后端配置
  await initBackendConfig()

  if (!shareUuid || !groupUuid) {
    error.value = '缺少必要参数：shareUuid 和 groupUuid'
    loading.value = false
    setTimeout(() => closeWindow(), 2000)
    return
  }

  noteTitle.value = title || '共享笔记'

  // 监听主窗口发送的数据（作为备用方案）
  try {
    const unlistenData = await listen('load-note-viewer-data', (event) => {
      const data = event.payload
      if (data && data.note) {
        noteData.value = data.note
        noteTitle.value = data.note.title || title || '共享笔记'
        loading.value = false
        error.value = ''
      }
    })
    window._unlistenData = unlistenData
  } catch (e) {
    console.warn('设置数据监听失败（非关键功能）:', e)
  }

  // 检查窗口状态（静默处理权限错误）
  checkWindowState()

  // 初始化主题
  initTheme()

  // 监听主题变化（静默处理权限错误）
  await setupThemeListener()

  // 加载笔记数据
  await loadNoteData()
})

onUnmounted(async () => {
  // 清理事件监听
  if (window._unlistenData) {
    try {
      window._unlistenData()
    } catch (e) {
      console.warn('清理数据监听失败:', e)
    }
    window._unlistenData = null
  }
})
</script>

<style scoped>
.note-viewer-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: var(--bg-primary, #0d1117);
  color: var(--text-primary, #f8fafc);
}

.note-viewer-window.light-mode {
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #1e293b);
}

/* 隐藏 TitleBar 的 title-bar-content */
:deep(.title-bar-content) {
  display: none !important;
}

/* 窗口标题栏 */
.viewer-header {
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

.light-mode .viewer-header {
  background: var(--bg-header, #ffffff);
  border-bottom-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.viewer-title-section {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.viewer-icon {
  font-size: 20px;
  color: var(--accent-blue, #3b82f6);
  flex-shrink: 0;
}

.viewer-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f8fafc);
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.light-mode .viewer-title {
  color: var(--text-primary, #1e293b);
}

.viewer-actions {
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

.action-btn.close-btn:hover {
  background-color: #ef4444;
  color: white;
}

/* 内容区域 */
.viewer-body {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px 0;
}

/* 加载状态 */
.loading-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 16px;
  color: var(--text-muted, #8b949e);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color, #30363d);
  border-top-color: var(--accent-blue, #3b82f6);
  border-radius: 2px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-icon {
  font-size: 48px;
  color: #ef4444;
}

.retry-btn {
  padding: 8px 20px;
  background: var(--accent-blue, #3b82f6);
  color: white;
  border: none;
  border-radius: 2px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.retry-btn:hover {
  background: #2563eb;
}

/* 笔记内容包装器 */
.note-content-wrapper {
  max-width: min(900px, 100vw - 48px);
  margin: 0 auto;
  padding: 0 24px;
  box-sizing: border-box;
  width: 100%;
}

/* 元信息区域 */
.note-meta {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 12px;
  padding: 16px;
  margin-bottom: 24px;
  background: var(--bg-secondary, #161b22);
  border-radius: 2px;
  border: 1px solid var(--border-color, #30363d);
  width: 100%;
  box-sizing: border-box;
}

.light-mode .note-meta {
  background: var(--bg-secondary, #f8fafc);
  border-color: var(--border-color, #e2e8f0);
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  flex-wrap: wrap;
}

.meta-icon {
  color: var(--accent-blue, #3b82f6);
  font-size: 16px;
  flex-shrink: 0;
}

.meta-label {
  font-weight: 500;
  color: var(--text-secondary, #c9d1d9);
}

.light-mode .meta-label {
  color: var(--text-secondary, #64748b);
}

.meta-value {
  color: var(--text-primary, #f8fafc);
}

.light-mode .meta-value {
  color: var(--text-primary, #1e293b);
}

/* 笔记章节 */
.note-section,
.ai-section {
  margin-bottom: 28px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  padding-bottom: 10px;
  border-bottom: 2px solid var(--border-color, #30363d);
  color: var(--text-primary, #f8fafc);
  display: flex;
  align-items: center;
  gap: 8px;
}

.light-mode .section-title {
  color: var(--text-primary, #1e293b);
  border-color: var(--border-color, #e2e8f0);
}

.section-title i {
  color: var(--accent-blue, #3b82f6);
}

/* 笔记内容 */
.note-content,
.meeting-note-content {
  padding: 20px;
  background: var(--bg-secondary, #161b22);
  border-radius: 2px;
  font-size: 15px;
  line-height: 1.8;
  color: var(--text-secondary, #c9d1d9);
  word-wrap: break-word;
  overflow-wrap: break-word;
  width: 100%;
  box-sizing: border-box;
  max-width: 100%;
}

.light-mode .note-content,
.light-mode .meeting-note-content {
  background: var(--bg-secondary, #f8fafc);
  color: var(--text-secondary, #475569);
}

/* 图片样式 - 响应式显示 */
.note-image {
  max-width: 100%;
  max-height: 50vh;
  width: auto;
  height: auto;
  display: block;
  margin: 16px auto;
  border-radius: 2px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  transition: transform 0.2s ease;
  object-fit: contain;
}

.note-image:hover {
  transform: scale(1.02);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.4);
}

/* 会议笔记列表 */
.meeting-notes-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.meeting-note-item {
  padding: 16px;
  background: var(--bg-secondary, #161b22);
  border-radius: 2px;
  border-left: 3px solid var(--accent-blue, #3b82f6);
}

.light-mode .meeting-note-item {
  background: var(--bg-secondary, #f8fafc);
}

.meeting-note-time {
  font-size: 13px;
  color: var(--text-muted, #8b949e);
  margin-bottom: 10px;
  font-weight: 500;
}

/* AI 内容区域 */
.ai-content-box {
  padding: 16px 20px;
  background: var(--bg-secondary, #161b22);
  border-radius: 2px;
  font-size: 14px;
  line-height: 1.7;
}

.light-mode .ai-content-box {
  background: var(--bg-secondary, #f8fafc);
}

.ai-title-text {
  font-size: 18px;
  font-weight: 600;
  color: var(--accent-blue, #58a6ff);
  margin: 0;
}

/* 关键词标签 */
.keywords-box {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  padding: 16px;
}

.keyword-tag {
  padding: 8px 16px;
  background: rgba(139, 92, 246, 0.2);
  color: #a78bfa;
  border-radius: 2px;
  font-size: 13px;
  font-weight: 500;
  border: 1px solid rgba(139, 92, 246, 0.3);
  transition: all 0.2s;
}

.keyword-tag:hover {
  background: rgba(139, 92, 246, 0.3);
  transform: translateY(-1px);
}

/* AI 分析内容 */
.analysis-box {
  color: var(--text-secondary, #c9d1d9);
}

.light-mode .analysis-box {
  color: var(--text-secondary, #475569);
}

.analysis-box h5 {
  font-size: 15px;
  margin: 14px 0 10px;
  color: var(--text-primary, #f8fafc);
}

.light-mode .analysis-box h5 {
  color: var(--text-primary, #1e293b);
}

.analysis-box ul {
  margin-left: 24px;
  padding-left: 0;
}

.analysis-box li {
  margin-bottom: 8px;
}

.analysis-box p {
  margin: 10px 0;
}

.analysis-box strong {
  color: var(--text-primary, #f8fafc);
}

.light-mode .analysis-box strong {
  color: var(--text-primary, #1e293b);
}

/* 空内容提示 */
.empty-content {
  text-align: center;
  padding: 40px 20px;
  color: var(--text-muted, #8b949e);
  font-size: 14px;
  background: var(--bg-secondary, #161b22);
  border-radius: 2px;
}

.light-mode .empty-content {
  background: var(--bg-secondary, #f8fafc);
}

/* 滚动条样式 */
.viewer-body::-webkit-scrollbar {
  width: 8px;
}

.viewer-body::-webkit-scrollbar-track {
  background: transparent;
}

.viewer-body::-webkit-scrollbar-thumb {
  background: var(--border-color, #30363d);
  border-radius: 2px;
}

.viewer-body::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted, #8b949e);
}

/* 响应式布局 */
@media (max-width: 768px) {
  .viewer-body {
    padding: 16px 0;
  }

  .note-content-wrapper {
    max-width: 100%;
    padding: 0 16px;
  }

  .section-title {
    font-size: 16px;
  }

  .note-content,
  .meeting-note-content {
    padding: 14px;
    font-size: 14px;
  }

  .note-image {
    margin: 12px 0;
  }
}
</style>
