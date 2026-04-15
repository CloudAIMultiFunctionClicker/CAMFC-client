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

        <div v-if="aiAnalysisData" class="ai-analysis-container" :class="{ 'light-mode': isLightMode }">
          <div class="ai-header">
            <i class="ri-robot-2-line"></i>
            <span>AI 分析</span>
            <button v-if="canRegenerate" class="regenerate-btn" @click="regenerateAnalysis">
              <i class="ri-refresh-line"></i>重新生成
            </button>
          </div>
          <div class="ai-content" v-html="aiAnalysisContent"></div>
        </div>

        <div v-if="!aiAnalysisData && !isAnalyzing" class="analyze-action">
          <button class="analyze-btn" @click="startAnalysis">
            <i class="ri-robot-2-line"></i>AI 分析会议内容
          </button>
        </div>

        <div v-if="isAnalyzing" class="analyzing-state">
          <i class="ri-loader-4-line spin"></i>
          <span>AI 正在分析中...</span>
        </div>
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

// AI 分析相关
const aiAnalysisData = ref(null)
const aiAnalysisContent = ref('')
const isAnalyzing = ref(false)

// 是否可以重新生成（有缓存内容）
const canRegenerate = computed(() => {
  return aiAnalysisData.value && aiAnalysisData.value.cached
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
    let html = '<div class="meeting-table">'

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
          <div class="meeting-item-placeholder" data-index="${index}"></div>
        `
      } else if (item.type === 'note' && item.content) {
        // 笔记
        const noteHtml = renderMarkdown(item.content)
        html += `
          <div class="meeting-item note-item">
            <div class="item-timestamp">${time}</div>
            <div class="note-content">${noteHtml}</div>
          </div>
          <div class="meeting-item-placeholder" data-index="${index}"></div>
        `
      }
    })

    html += '</div>'
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

// 纯文本 Markdown 渲染（不解析标题）
function renderMarkdownText(text) {
  if (!text) return ''

  let html = text
    .replace(/\*\*(.*)\*\*/gim, '<strong>$1</strong>')
    .replace(/\*(.*)\*/gim, '<em>$1</em>')
    .replace(/~~(.*)~~/gim, '<del>$1</del>')
    .replace(/`([^`]+)`/gim, '<code>$1</code>')
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

// 开始 AI 分析
async function startAnalysis() {
  try {
    isAnalyzing.value = true

    const response = await axios.post(
      getBackendUrl() + '/meeting/ai_analyze',
      {
        meeting_uuid: meetingUuid.value
      },
      {
        headers: await getAuthHeader(),
        timeout: 60000
      }
    )

    console.log('AI 分析响应完整数据:', response)
    console.log('AI 分析响应数据:', response.data)

    if (response.data && response.data.status === 'success') {
      aiAnalysisData.value = response.data

      // 解析 analysis 字段（可能是对象或 JSON 字符串）
      if (response.data.analysis) {
        try {
          let analysisData = response.data.analysis

          // 如果是字符串，先解析
          if (typeof analysisData === 'string') {
            analysisData = JSON.parse(analysisData.replace(/'/g, '"'))
          }

          // 格式化 individual_analyses 到对应位置
          if (analysisData.individual_analyses && Array.isArray(analysisData.individual_analyses)) {
            // 先按时间戳排序
            const sortedAnalyses = [...analysisData.individual_analyses].sort((a, b) => {
              const timeA = new Date(a.timestamp || 0).getTime()
              const timeB = new Date(b.timestamp || 0).getTime()
              return timeA - timeB
            })

            // 按顺序处理每个分析项
            sortedAnalyses.forEach((item, idx) => {
              const placeholder = document.querySelector(`.meeting-item-placeholder[data-index="${idx}"]`)
              if (placeholder) {
                placeholder.innerHTML = `
                  <div class="ai-note-analysis">
                    <div class="ai-summary"><strong>概括：</strong>${renderMarkdownText(item.summary) || '无'}</div>
                    <div class="ai-detail"><strong>解析：</strong>${renderMarkdownText(item.analysis) || '无'}</div>
                  </div>
                `
              }
            })
          }

          // 整体分析结果还是显示在底部
          let analysisContent = `## ${analysisData.title || '分析结果'}\n\n`
          if (analysisData.type) analysisContent += `**类型**: ${analysisData.type}\n\n`
          if (analysisData.key_topics && analysisData.key_topics.length > 0) {
            analysisContent += `**主要议题**:\n${analysisData.key_topics.map(t => `- ${t}`).join('\n')}\n\n`
          }
          if (analysisData.key_knowledge_points && analysisData.key_knowledge_points.length > 0) {
            analysisContent += `**关键知识点**:\n${analysisData.key_knowledge_points.map(p => `- ${p}`).join('\n')}\n\n`
          }
          if (analysisData.decisions && analysisData.decisions.length > 0) {
            analysisContent += `**决策**:\n${analysisData.decisions.map(d => `- ${d}`).join('\n')}\n\n`
          }
          if (analysisData.action_items && analysisData.action_items.length > 0) {
            analysisContent += `**行动项**:\n${analysisData.action_items.map(a => `- ${a}`).join('\n')}\n\n`
          }
          if (analysisData.summary) analysisContent += `**总结**:\n${analysisData.summary}\n\n`
          if (analysisData.key_points && analysisData.key_points.length > 0) {
            analysisContent += `**重点内容**:\n${analysisData.key_points.map(p => `- ${p}`).join('\n')}\n\n`
          }
          if (analysisData.review_points && analysisData.review_points.length > 0) {
            analysisContent += `**复习要点**:\n${analysisData.review_points.map(p => `- ${p}`).join('\n')}\n\n`
          }
          if (analysisData.progress) analysisContent += `**进展**:\n${analysisData.progress}\n\n`
          if (analysisData.confidence !== undefined) {
            analysisContent += `**置信度**: ${(analysisData.confidence * 100).toFixed(1)}%\n\n`
          }

          aiAnalysisContent.value = renderMarkdown(analysisContent)
        } catch (e) {
          console.error('解析 analysis 失败:', e)

          console.info('原始 analysis:', response.data.analysis)
          aiAnalysisContent.value = typeof response.data.analysis === 'string'
            ? response.data.analysis
            : JSON.stringify(response.data.analysis, null, 2)
        }
      }
    } else {
      showToast('分析失败:' + (response.data?.message || '未知错误'), '#ef4444')
    }
  } catch (e) {
    console.error('AI 分析失败:', e)
    showToast('分析失败:' + (e.message || '网络错误'), '#ef4444')
  } finally {
    isAnalyzing.value = false
  }
}

// 重新生成分析
async function regenerateAnalysis() {
  aiAnalysisData.value = null
  aiAnalysisContent.value = ''
  await startAnalysis()
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
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

/* 查看器内容 */
.viewer-content {
  width: 100%;
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

/* 会议项目样式 - 2 列表格布局 */
.viewer-content :deep(.meeting-table) {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px 32px;
  width: 100%;
  position: relative;
}

/* 中间竖向分割线 */
.viewer-content :deep(.meeting-table)::after {
  content: '';
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 1px;
  background: var(--border-color, rgba(255, 255, 255, 0.1));
  transform: translateX(-50%);
}

.light-mode .viewer-content :deep(.meeting-table)::after {
  background: var(--border-color, rgba(0, 0, 0, 0.1));
}

.viewer-content :deep(.meeting-item) {
  display: flex;
  flex-direction: column;
  padding: 20px;
  background: var(--bg-secondary, #161b22);
  border-radius: 10px;
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

/* 第二列占位区域 */
.viewer-content :deep(.meeting-item-placeholder) {
  padding: 20px;
  background: transparent;
  border-radius: 10px;
  border: none;
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  color: var(--text-primary, #f8fafc);
  font-size: 14px;
  min-height: 120px;
}

.light-mode .viewer-content :deep(.meeting-item-placeholder) {
  background: transparent;
  border: none;
  color: var(--text-primary, #1e293b);
}

/* AI 分析内容样式 */
.viewer-content :deep(.ai-note-analysis) {
  width: 100%;
  line-height: 1.6;
}

.viewer-content :deep(.ai-summary) {
  margin-bottom: 12px;
  padding: 12px;
  background: var(--bg-secondary, #161b22);
  border-radius: 8px;
  border-left: 3px solid #3b82f6;
}

.light-mode .viewer-content :deep(.ai-summary) {
  background: var(--bg-secondary, #f1f5f9);
}

.viewer-content :deep(.ai-detail) {
  padding: 12px;
  background: var(--bg-tertiary, #0d1117);
  border-radius: 8px;
  border-left: 3px solid #10b981;
}

.light-mode .viewer-content :deep(.ai-detail) {
  background: var(--bg-tertiary, #e2e8f0);
}

.viewer-content :deep(.ai-summary strong),
.viewer-content :deep(.ai-detail strong) {
  color: #3b82f6;
  display: block;
  margin-bottom: 6px;
}

.light-mode .viewer-content :deep(.ai-summary strong),
.light-mode .viewer-content :deep(.ai-detail strong) {
  color: #2563eb;
}

/* AI 分析容器 */
.ai-analysis-container {
  margin-top: 24px;
  padding: 20px;
  background: var(--bg-secondary, #161b22);
  border-radius: 8px;
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.light-mode .ai-analysis-container {
  background: var(--bg-secondary, #f1f5f9);
  border-color: rgba(59, 130, 246, 0.2);
}

.ai-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.light-mode .ai-header {
  border-bottom-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.ai-header i {
  color: #3b82f6;
  font-size: 20px;
  margin-right: 8px;
}

.ai-header span {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f8fafc);
}

.light-mode .ai-header span {
  color: var(--text-primary, #1e293b);
}

.regenerate-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 12px;
  font-size: 12px;
  border: 1px solid #3b82f6;
  border-radius: 4px;
  background: transparent;
  color: #3b82f6;
  cursor: pointer;
  transition: all 0.2s;
}

.regenerate-btn:hover {
  background: rgba(59, 130, 246, 0.1);
}

.ai-content {
  line-height: 1.8;
  font-size: 14px;
  color: var(--text-primary, #f8fafc);
}

.light-mode .ai-content {
  color: var(--text-primary, #1e293b);
}

.ai-content :deep(h1),
.ai-content :deep(h2),
.ai-content :deep(h3) {
  margin: 16px 0 12px;
  font-weight: 600;
  line-height: 1.4;
}

.ai-content :deep(h1) {
  font-size: 18px;
  border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  padding-bottom: 8px;
}

.light-mode .ai-content :deep(h1) {
  border-bottom-color: var(--border-color, rgba(0, 0, 0, 0.1));
}

.ai-content :deep(h2) {
  font-size: 16px;
}

.ai-content :deep(h3) {
  font-size: 14px;
}

.ai-content :deep(code) {
  background: var(--bg-primary, #0d1117);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
}

.light-mode .ai-content :deep(code) {
  background: var(--bg-primary, #ffffff);
}

.ai-content :deep(strong) {
  font-weight: 600;
  color: #3b82f6;
}

.ai-content :deep(em) {
  font-style: italic;
}

.ai-content :deep(del) {
  text-decoration: line-through;
  opacity: 0.7;
}

.ai-content :deep(li) {
  margin: 4px 0;
  padding-left: 20px;
  position: relative;
}

.ai-content :deep(li)::before {
  content: '•';
  position: absolute;
  left: 4px;
  color: #3b82f6;
}

.ai-content :deep(.markdown-image) {
  max-width: 100%;
  border-radius: 8px;
  display: block;
  margin: 16px 0;
}

/* 分析操作按钮 */
.analyze-action {
  display: flex;
  justify-content: center;
  margin-top: 24px;
}

.analyze-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  font-size: 15px;
  font-weight: 500;
  border: none;
  border-radius: 8px;
  background: linear-gradient(135deg, #3b82f6, #2563eb);
  color: white;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.analyze-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4);
}

.analyze-btn:active {
  transform: translateY(0);
}

/* 分析中状态 */
.analyzing-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px;
  gap: 12px;
  color: var(--text-secondary, #94a3b8);
}

.light-mode .analyzing-state {
  color: var(--text-secondary, #64748b);
}

.analyzing-state i {
  font-size: 28px;
}
</style>
