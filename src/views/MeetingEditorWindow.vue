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
      <div class="editor-title-display">{{ meetingTitle }}</div>
      <div class="editor-actions">
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

    <div class="editor-body-wrapper">
      <div class="viewer-container" :class="{ 'light-mode': isLightMode }">
        <div v-if="isLoading" class="loading-state">
          <i class="ri-loader-4-line spin"></i>
          <span>加载中...</span>
        </div>
        <div v-else-if="loadError" class="error-state">
          <i class="ri-error-warning-line"></i>
          <span>{{ loadError }}</span>
        </div>
        <div v-else class="viewer-content" v-html="renderedContent"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import axios from 'axios'
import { showToast } from '../components/layout/showToast.js'
import { getBackendUrl, initBackendConfig } from '../config/backend.js'

const route = useRoute()

// 会议数据
const meetingUuid = ref('')
const meetingTitle = ref('')
const meetingData = ref(null)
const isLoading = ref(true)
const loadError = ref('')

// 窗口状态
const isMaximized = ref(false)
const currentWindow = getCurrentWindow()

// 主题状态
const isLightMode = ref(false)

// 渲染后的内容
const renderedContent = computed(() => {
  return renderMeetingContent(meetingData.value)
})

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

// 渲染会议内容（混合截图和笔记）
function renderMeetingContent(meeting) {
  if (!meeting) return '<div class="empty-content">暂无内容</div>'

  // 如果有 items 数组，按时间顺序混合渲染
  if (meeting.items && Array.isArray(meeting.items) && meeting.items.length > 0) {
    let html = ''

    meeting.items.forEach((item, index) => {
      const time = formatTime(item.timestamp)

      if (item.type === 'screenshot' && item.data) {
        // 截图
        html += `
          <div class="meeting-item screenshot-item">
            <div class="item-timestamp">${time}</div>
            <div class="screenshot-wrapper">
              <img src="${item.data}" alt="截图" class="meeting-screenshot">
            </div>
          </div>
        `
      } else if (item.type === 'note' && item.content) {
        // 笔记
        const noteHtml = renderMarkdown(item.content)
        html += `
          <div class="meeting-item note-item">
            <div class="item-timestamp">${time}</div>
            <div class="note-content">${noteHtml}</div>
          </div>
        `
      }
    })

    return html || '<div class="empty-content">暂无内容</div>'
  }

  // 兼容旧格式：使用 content 字段
  return renderMarkdown(meeting.content || '')
}

// 格式化时间
function formatTime(timestamp) {
  if (!timestamp) return ''
  try {
    const date = new Date(timestamp)
    return date.toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    })
  } catch {
    return timestamp
  }
}

// Markdown 渲染
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
    .replace(/!\[([^\]]*)\]\(([^)]+)\)/gim, '<div class="markdown-image-wrapper"><img src="$2" alt="$1" class="markdown-image"></div>')
    .replace(/\n/gim, '<br>')

  return html
}

// 加载会议内容
async function loadMeetingContent() {
  try {
    isLoading.value = true
    loadError.value = ''

    const response = await axios.get(getBackendUrl() + '/meeting/history/query_by_uuid', {
      params: {
        meeting_uuid: meetingUuid.value
      },
      headers: await getAuthHeader(),
      timeout: 10000
    })

    const data = response.data
    console.log('API 返回数据:', data)
    if (data && data.success && data.meeting) {
      console.log('会议对象:', data.meeting)
      meetingTitle.value = data.meeting.title || '会议记录'
      meetingData.value = data.meeting
    } else {
      loadError.value = '获取会议内容失败'
    }
  } catch (e) {
    console.error('加载会议内容失败:', e)
    loadError.value = '加载失败: ' + (e.message || '网络错误')
  } finally {
    isLoading.value = false
  }
}

// 初始化
onMounted(async () => {
  const uuid = route.query.uuid
  const title = route.query.title

  if (!uuid) {
    showToast('会议信息不完整', '#ef4444')
    setTimeout(() => closeWindow(), 1500)
    return
  }

  meetingUuid.value = uuid
  meetingTitle.value = title || '会议记录'

  // 初始化主题
  initTheme()

  // 初始化后端配置
  await initBackendConfig()

  // 加载会议内容
  await loadMeetingContent()

  // 检查窗口状态
  checkWindowState()
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
    showToast('关闭失败', '#ef4444')
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

/* 窗口标题栏 */
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

.editor-title-display {
  flex: 1;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f8fafc);
  padding: 4px 8px;
  -webkit-app-region: no-drag;
  min-width: 0;
  letter-spacing: 0.5px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.light-mode .editor-title-display {
  color: var(--text-primary, #1e293b);
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

.action-btn.close-btn:hover {
  background-color: #ef4444;
  color: white;
}

/* 编辑器主体 */
.editor-body-wrapper {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 查看器容器 */
.viewer-container {
  flex: 1;
  overflow-y: auto;
  padding: 24px 32px;
  background: var(--bg-primary, #0d1117);
}

.viewer-container.light-mode {
  background: var(--bg-primary, #ffffff);
}

/* 加载状态 */
.loading-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
  color: var(--text-secondary, #94a3b8);
}

.light-mode .loading-state,
.light-mode .error-state {
  color: var(--text-secondary, #64748b);
}

.loading-state i,
.error-state i {
  font-size: 32px;
}

.error-state i {
  color: #ef4444;
}

.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 查看器内容 */
.viewer-content {
  max-width: 800px;
  margin: 0 auto;
  line-height: 1.8;
  font-size: 15px;
  color: var(--text-primary, #f8fafc);
}

.light-mode .viewer-content {
  color: var(--text-primary, #1e293b);
}

.viewer-content :deep(h1),
.viewer-content :deep(h2),
.viewer-content :deep(h3) {
  margin: 24px 0 16px;
  font-weight: 600;
  line-height: 1.4;
}

.viewer-content :deep(h1) {
  font-size: 24px;
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  padding-bottom: 12px;
}

.light-mode .viewer-content :deep(h1) {
  border-bottom-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.viewer-content :deep(h2) {
  font-size: 20px;
}

.viewer-content :deep(h3) {
  font-size: 18px;
}

.viewer-content :deep(code) {
  background: var(--bg-secondary, #161b22);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 13px;
}

.light-mode .viewer-content :deep(code) {
  background: var(--bg-secondary, #f1f5f9);
}

.viewer-content :deep(strong) {
  font-weight: 600;
}

.viewer-content :deep(em) {
  font-style: italic;
}

.viewer-content :deep(del) {
  text-decoration: line-through;
  opacity: 0.7;
}

.viewer-content :deep(li) {
  margin: 4px 0;
  padding-left: 20px;
  position: relative;
}

.viewer-content :deep(li)::before {
  content: '•';
  position: absolute;
  left: 4px;
  color: var(--text-secondary, #94a3b8);
}

.viewer-content :deep(.markdown-image) {
  max-width: 100%;
  border-radius: 8px;
  display: block;
  margin: 16px 0;
}

.viewer-content :deep(.markdown-image-wrapper) {
  display: block;
  max-width: 100%;
  margin: 16px 0;
}

.viewer-content :deep(.empty-content) {
  text-align: center;
  color: var(--text-secondary, #94a3b8);
  padding: 48px 0;
  font-style: italic;
}

.light-mode .viewer-content :deep(.empty-content) {
  color: var(--text-secondary, #64748b);
}

.viewer-content :deep(br) {
  display: block;
  content: '';
  margin-top: 8px;
}

/* 会议项目样式 */
.viewer-content :deep(.meeting-item) {
  margin-bottom: 24px;
  padding: 16px;
  background: var(--bg-secondary, #161b22);
  border-radius: 8px;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.light-mode .viewer-content :deep(.meeting-item) {
  background: var(--bg-secondary, #f1f5f9);
  border-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.viewer-content :deep(.item-timestamp) {
  font-size: 12px;
  color: var(--text-secondary, #94a3b8);
  margin-bottom: 8px;
  font-family: 'Consolas', 'Monaco', monospace;
}

.light-mode .viewer-content :deep(.item-timestamp) {
  color: var(--text-secondary, #64748b);
}

.viewer-content :deep(.screenshot-wrapper) {
  display: flex;
  justify-content: center;
}

.viewer-content :deep(.meeting-screenshot) {
  max-width: 100%;
  max-height: 400px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.viewer-content :deep(.note-content) {
  line-height: 1.8;
}

.viewer-content :deep(.screenshot-item) {
  background: var(--bg-secondary, #0d1117);
}

.light-mode .viewer-content :deep(.screenshot-item) {
  background: var(--bg-secondary, #ffffff);
}
</style>
