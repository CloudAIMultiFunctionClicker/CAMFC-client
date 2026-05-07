<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh09@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<template>
  <div class="group-detail-container">
    <div class="page-header">
      <button class="back-btn" @click="goBack" title="返回">
        <i class="ri-arrow-left-line"></i>
      </button>
      <div class="group-title-section">
        <div class="group-avatar">
          <i class="ri-group-fill"></i>
        </div>
        <div class="group-title-info">
          <h1 class="page-title">{{ groupName }}</h1>
          <p class="group-uid">UID: {{ groupUid }}</p>
        </div>
      </div>
      <button 
        class="refresh-btn" 
        @click="loadData"
        :disabled="isLoading"
        title="刷新数据"
      >
        <i class="ri-refresh-line" :class="{ 'spinning': isLoading }"></i>
      </button>
    </div>

    <div class="tab-navigation">
      <button 
        class="tab-btn" 
        :class="{ active: currentTab === 'notes' }"
        @click="switchTab('notes')"
      >
        <i class="ri-sticky-note-line"></i>
        共享笔记
        <span v-if="sharedNotes.length > 0" class="count-badge">{{ sharedNotes.length }}</span>
      </button>
      <button 
        class="tab-btn" 
        :class="{ active: currentTab === 'files' }"
        @click="switchTab('files')"
      >
        <i class="ri-file-text-line"></i>
        共享文件
        <span v-if="sharedFiles.length > 0" class="count-badge">{{ sharedFiles.length }}</span>
      </button>
    </div>

    <div v-if="currentTab === 'notes'" class="tab-content notes-tab">
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载共享笔记...</p>
      </div>

      <div v-else-if="sharedNotes.length === 0" class="empty-state">
        <i class="ri-sticky-note-off-line empty-icon"></i>
        <p>暂无共享笔记</p>
        <p class="empty-desc">群组成员分享的笔记将显示在这里</p>
      </div>

      <div v-else class="notes-list">
        <div 
          v-for="note in sharedNotes" 
          :key="note.share_uuid"
          class="note-item"
          @click="openNoteViewer(note)"
        >
          <div class="note-icon-wrapper">
            <i :class="note.type === 'personal' ? 'ri-sticky-note-line' : 'ri-calendar-line'" class="note-type-icon"></i>
          </div>
          <div class="note-content-wrapper">
            <div class="note-header">
              <span class="note-type-tag" :class="note.type">
                {{ note.type === 'personal' ? '个人笔记' : '会议记录' }}
              </span>
              <span class="note-shared-by">
                <i class="ri-user-line"></i>
                {{ note.shared_by }}
              </span>
            </div>
            <h3 class="note-title">{{ note.title }}</h3>
            <p class="note-preview">{{ getNotePreview(note.content) }}</p>
            <div class="note-meta">
              <span class="meta-item">
                <i class="ri-time-line"></i>
                {{ formatTime(note.shared_at) }}
              </span>
            </div>
          </div>
          <i class="ri-arrow-right-s-line note-arrow"></i>
        </div>
      </div>
    </div>

    <div v-else-if="currentTab === 'files'" class="tab-content files-tab">
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载共享文件...</p>
      </div>

      <div v-else-if="sharedFiles.length === 0" class="empty-state">
        <i class="ri-file-off-line empty-icon"></i>
        <p>暂无共享文件</p>
        <p class="empty-desc">群组成员分享的文件将显示在这里</p>
      </div>

      <div v-else class="files-timeline">
        <div 
          v-for="file in sortedFiles" 
          :key="file.share_uuid"
          class="timeline-item"
        >
          <div class="timeline-marker">
            <i :class="getFileIcon(file.file_name)" class="marker-icon"></i>
          </div>
          <div class="timeline-content">
            <div class="timeline-header">
              <h4 class="file-name">{{ file.file_name }}</h4>
              <span class="file-size">{{ formatFileSize(file.file_size) }}</span>
            </div>
            <div class="timeline-meta">
              <span class="meta-item">
                <i class="ri-user-line"></i>
                {{ file.shared_by }}
              </span>
              <span class="meta-item">
                <i class="ri-time-line"></i>
                {{ formatTime(file.shared_at) }}
              </span>
            </div>
            <div class="timeline-actions">
              <button 
                class="action-btn download-btn"
                @click="downloadFile(file)"
                title="下载文件"
              >
                <i class="ri-download-line"></i>
                下载
              </button>
              <button 
                v-if="isCurrentUser(file.shared_by)"
                class="action-btn delete-btn"
                @click="confirmDeleteFile(file)"
                title="删除文件"
              >
                <i class="ri-delete-bin-line"></i>
                删除
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { getSharedNotes, getSharedFiles, getSharedFileDetail, getSharedFileDownloadInfo, deleteSharedFile, getAuthHeader } from '../components/data/group.js'
import { showToast } from '../components/layout/showToast.js'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { invoke } from '@tauri-apps/api/core'
import { getBackendUrl } from '../config/backend.js'

const route = useRoute()
const router = useRouter()

const groupUid = ref('')
const groupName = ref('')
const currentTab = ref('notes')
const isLoading = ref(false)
const sharedNotes = ref([])
const sharedFiles = ref([])
const currentUserId = ref('')

// 按时间排序的文件列表（最新的在前）
const sortedFiles = computed(() => {
  return [...sharedFiles.value].sort((a, b) => {
    const timeA = new Date(a.shared_at || 0).getTime()
    const timeB = new Date(b.shared_at || 0).getTime()
    return timeB - timeA
  })
})

function getFileExtension(filename) {
  const parts = filename.split('.')
  return parts.length > 1 ? parts.pop() : ''
}

function getFileIcon(filename) {
  const ext = getFileExtension(filename).toLowerCase()
  
  if (ext === 'pdf') return 'ri-file-pdf-line'
  if (['doc', 'docx'].includes(ext)) return 'ri-file-word-line'
  if (['xls', 'xlsx'].includes(ext)) return 'ri-file-excel-line'
  if (['ppt', 'pptx'].includes(ext)) return 'ri-file-ppt-line'
  if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp'].includes(ext)) return 'ri-image-line'
  if (['mp4', 'avi', 'mov', 'wmv'].includes(ext)) return 'ri-movie-line'
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) return 'ri-archive-line'
  if (['js', 'ts', 'py', 'java', 'cpp', 'c', 'h', 'vue', 'html', 'css', 'json', 'md'].includes(ext)) return 'ri-code-s-slash-line'
  
  return 'ri-file-text-line'
}

function getFileTypeInfo(type) {
  const typeInfo = {
    pdf: { label: 'PDF 文档', icon: 'ri-file-pdf-line' },
    doc: { label: 'Word 文档', icon: 'ri-file-word-line' },
    xls: { label: 'Excel 表格', icon: 'ri-file-excel-line' },
    ppt: { label: 'PowerPoint', icon: 'ri-file-ppt-line' },
    image: { label: '图片', icon: 'ri-image-line' },
    video: { label: '视频', icon: 'ri-movie-line' },
    code: { label: '代码文件', icon: 'ri-code-s-slash-line' },
    archive: { label: '压缩包', icon: 'ri-archive-line' },
    other: { label: '其他文件', icon: 'ri-file-text-line' }
  }
  
  return typeInfo[type] || typeInfo.other
}

function formatFileSize(bytes) {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

function getNotePreview(content) {
  if (!content) return '无内容预览'
  const text = content.replace(/!\[.*?\]\(.*?\)/g, '[图片]')
  const preview = text.substring(0, 100)
  return preview.length >= 100 ? preview + '...' : preview
}

function formatTime(timestamp) {
  if (!timestamp) return ''
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diff = now - date
  
  const minute = 60 * 1000
  const hour = 60 * minute
  const day = 24 * hour
  
  if (diff < minute) {
    return '刚刚'
  } else if (diff < hour) {
    return Math.floor(diff / minute) + '分钟前'
  } else if (diff < day) {
    return Math.floor(diff / hour) + '小时前'
  } else if (diff < 7 * day) {
    return Math.floor(diff / day) + '天前'
  } else {
    return date.toLocaleString('zh-CN')
  }
}

function goBack() {
  router.back()
}

function openNoteViewer(note) {
  const windowLabel = `note-viewer-${note.share_uuid}`
  const url = `/note-viewer?shareUuid=${note.share_uuid}&groupUuid=${groupUid.value}&title=${encodeURIComponent(note.title)}`

  const webview = new WebviewWindow(windowLabel, {
    url: url,
    title: note.title || '共享笔记',
    width: 900,
    height: 700,
    minWidth: 600,
    minHeight: 400,
    center: true,
    decorations: false,
    resizable: true
  })

  webview.once('tauri://created', async () => {
    console.log('笔记查看窗口创建成功:', windowLabel)
  })

  webview.once('tauri://error', async (e) => {
    console.error('笔记查看窗口创建失败:', e)
    const errorMsg = e?.payload || ''
    if (typeof errorMsg === 'string' && errorMsg.includes('already exists')) {
      try {
        const existingWindow = await WebviewWindow.getByLabel(windowLabel)
        if (existingWindow) {
          await existingWindow.setFocus()
          await existingWindow.setAlwaysOnTop(true)
          setTimeout(async () => {
            await existingWindow.setAlwaysOnTop(false)
          }, 100)
          console.log('笔记查看窗口已置顶')
        }
      } catch (err) {
        console.error('设置窗口置顶失败:', err)
      }
    } else {
      showToast('打开笔记查看窗口失败', '#ef4444')
    }
  })
}

function openFileDetail(file) {
  showToast('文件详情功能开发中', '#f59e0b')
}

// 判断是否是当前用户
function isCurrentUser(sharedBy) {
  if (!currentUserId.value || !sharedBy) return false
  return currentUserId.value === sharedBy
}

// 下载文件
async function downloadFile(file) {
  try {
    // 获取文件下载信息
    const downloadInfo = await getSharedFileDownloadInfo(file.share_uuid, groupUid.value)
    
    if (!downloadInfo || !downloadInfo.success) {
      showToast('获取下载信息失败', '#ef4444')
      return
    }
    
    const storagePath = downloadInfo.storage_path
    const groupUuid = downloadInfo.group_uuid
    const fileName = downloadInfo.file_name
    
    // 调用后端下载接口 - 使用路径参数格式
    // 正确的格式：/download/{file_path:path}?group_uuid={group_uuid}
    const encodedPath = encodeURIComponent(storagePath)
    const downloadUrl = `${getBackendUrl()}/download/${encodedPath}?group_uuid=${groupUuid}`
    
    console.log('下载 URL:', downloadUrl)
    
    // 获取认证头
    const authHeader = await getAuthHeader()
    console.log('认证头信息:', authHeader)
    
    // 检查是否有认证信息
    if (!authHeader.Username && !authHeader.Id) {
      console.error('缺少认证信息！请检查是否设置了学生用户名密码或连接了蓝牙设备')
      showToast('缺少认证信息，请先设置学生账号或连接蓝牙设备', '#f59e0b')
      return
    }
    
    // 使用 Tauri 的 fetch 下载文件
    const response = await fetch(downloadUrl, {
      headers: authHeader
    })
    
    if (!response.ok) {
      throw new Error('下载失败')
    }
    
    const arrayBuffer = await response.arrayBuffer()
    const blob = new Blob([arrayBuffer])
    
    // 使用 Tauri 的 save 对话框保存文件
    const { save } = await import('@tauri-apps/plugin-dialog')
    const filePath = await save({
      filters: [{
        name: fileName,
        extensions: [getFileExtension(fileName)]
      }],
      defaultPath: fileName
    })
    
    if (filePath) {
      const { writeBinaryFile } = await import('@tauri-apps/plugin-fs')
      await writeBinaryFile(filePath, new Uint8Array(arrayBuffer))
      showToast(`文件已保存到：${filePath}`, '#10b981')
    }
  } catch (error) {
    console.error('下载失败:', error)
    showToast(`下载失败：${error.message}`, '#ef4444')
  }
}

// 确认删除文件
function confirmDeleteFile(file) {
  const confirmed = confirm(`确定要删除文件 "${file.file_name}" 吗？此操作不可恢复。`)
  if (confirmed) {
    deleteSharedFileFromGroup(file)
  }
}

// 删除文件
async function deleteSharedFileFromGroup(file) {
  try {
    const result = await deleteSharedFile(file.share_uuid, groupUid.value)
    if (result && result.success) {
      showToast('文件删除成功', '#10b981')
      // 重新加载文件列表
      await loadData()
    } else {
      showToast('删除失败', '#ef4444')
    }
  } catch (error) {
    console.error('删除失败:', error)
    showToast(`删除失败：${error.message}`, '#ef4444')
  }
}

function switchTab(tab) {
  currentTab.value = tab
  loadData()
}

async function loadData() {
  isLoading.value = true
  
  try {
    if (currentTab.value === 'notes') {
      const notes = await getSharedNotes(groupUid.value)
      sharedNotes.value = (notes || []).sort((a, b) => {
        return (b.shared_at || 0) - (a.shared_at || 0)
      })
    } else if (currentTab.value === 'files') {
      const files = await getSharedFiles(groupUid.value)
      sharedFiles.value = files || []
    }
  } catch (error) {
    console.error('加载数据失败:', error)
    showToast('加载数据失败', '#ef4444')
  } finally {
    isLoading.value = false
  }
}

onMounted(async () => {
  groupUid.value = route.query.uid || ''
  groupName.value = route.query.name || '未知群组'
  
  // 获取当前用户 ID
  try {
    const deviceId = await invoke('get_device_id')
    currentUserId.value = deviceId
    console.log('当前用户 ID:', currentUserId.value)
  } catch (error) {
    console.error('获取用户 ID 失败:', error)
  }
  
  if (!groupUid.value) {
    showToast('缺少群组 UID 参数', '#ef4444')
    setTimeout(() => goBack(), 1500)
    return
  }
  
  console.info('群组详情页面已加载:', { uid: groupUid.value, name: groupName.value })
  loadData()
})
</script>

<style scoped>
.group-detail-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 30px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--border-color, #30363d);
}

.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  font-size: 20px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--text-primary, #f0f6fc);
  flex-shrink: 0;
}

.back-btn:hover {
  background-color: var(--bg-tertiary, #161b22);
  border-color: var(--accent-blue, #58a6ff);
}

.group-title-section {
  display: flex;
  align-items: center;
  gap: 16px;
  flex: 1;
  min-width: 0;
}

.group-avatar {
  width: 56px;
  height: 56px;
  border-radius: 12px;
  background: linear-gradient(135deg, rgba(88, 166, 255, 0.2), rgba(49, 120, 198, 0.2));
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.group-avatar i {
  font-size: 28px;
  color: var(--accent-blue, #58a6ff);
}

.group-title-info {
  flex: 1;
  min-width: 0;
}

.page-title {
  font-size: 24px;
  margin: 0 0 4px 0;
  color: var(--text-primary, #f0f6fc);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.group-uid {
  font-size: 13px;
  color: var(--text-muted, #8b949e);
  margin: 0;
}

.refresh-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  font-size: 20px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--text-primary, #f0f6fc);
  flex-shrink: 0;
}

.refresh-btn:hover:not(:disabled) {
  background-color: var(--bg-tertiary, #161b22);
  border-color: var(--accent-blue, #58a6ff);
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.refresh-btn .spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.tab-navigation {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
  padding: 8px;
  background-color: var(--bg-secondary, #0d0d0d);
  border-radius: 12px;
  border: 1px solid var(--border-color, #30363d);
}

.tab-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 20px;
  font-size: 15px;
  font-weight: 500;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: var(--text-secondary, #8b949e);
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
}

.tab-btn i {
  font-size: 18px;
}

.tab-btn.active {
  background-color: var(--accent-blue, #3178c6);
  color: white;
}

.tab-btn:hover:not(.active) {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-primary, #f0f6fc);
}

.count-badge {
  min-width: 20px;
  height: 20px;
  padding: 0 6px;
  font-size: 12px;
  font-weight: 600;
  color: white;
  background-color: rgba(255, 255, 255, 0.3);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.tab-content {
  min-height: 400px;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: var(--text-muted, #8b949e);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color, #30363d);
  border-top-color: var(--accent-blue, #3178c6);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: var(--text-muted, #8b949e);
  text-align: center;
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-message {
  font-size: 16px;
  color: var(--text-secondary, #c9d1d9);
  margin: 0 0 8px 0;
}

.empty-desc {
  font-size: 14px;
  color: var(--text-muted, #8b949e);
}

.notes-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.note-item {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.note-item:hover {
  border-color: var(--accent-blue, #58a6ff);
  background-color: var(--bg-tertiary, #161b22);
  transform: translateX(4px);
}

.note-icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: linear-gradient(135deg, rgba(88, 166, 255, 0.2), rgba(49, 120, 198, 0.2));
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.note-type-icon {
  font-size: 24px;
  color: var(--accent-blue, #58a6ff);
}

.note-content-wrapper {
  flex: 1;
  min-width: 0;
}

.note-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.note-type-tag {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
}

.note-type-tag.personal {
  background-color: rgba(88, 166, 255, 0.2);
  color: var(--accent-blue, #58a6ff);
}

.note-type-tag.meeting {
  background-color: rgba(139, 92, 246, 0.2);
  color: #a78bfa;
}

.note-shared-by {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-muted, #8b949e);
}

.note-shared-by i {
  font-size: 14px;
}

.note-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f0f6fc);
  margin: 0 0 8px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-height: 1.4;
}

.note-preview {
  font-size: 14px;
  color: var(--text-secondary, #c9d1d9);
  margin: 0 0 12px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-height: 1.5;
}

.note-meta {
  display: flex;
  gap: 16px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-muted, #8b949e);
}

.meta-item i {
  font-size: 14px;
}

.note-arrow {
  font-size: 24px;
  color: var(--text-muted, #8b949e);
  flex-shrink: 0;
}

.files-timeline {
  display: flex;
  flex-direction: column;
  gap: 16px;
  position: relative;
  padding-left: 20px;
}

.files-timeline::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 2px;
  background: linear-gradient(to bottom, var(--accent-blue, #58a6ff), var(--border-color, #30363d));
}

.timeline-item {
  display: flex;
  gap: 16px;
  position: relative;
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.timeline-marker {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: linear-gradient(135deg, rgba(88, 166, 255, 0.2), rgba(49, 120, 198, 0.2));
  border: 2px solid var(--accent-blue, #58a6ff);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  z-index: 1;
}

.marker-icon {
  font-size: 20px;
  color: var(--accent-blue, #58a6ff);
}

.timeline-content {
  flex: 1;
  min-width: 0;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 12px;
  padding: 16px 20px;
  transition: all 0.2s;
}

.timeline-content:hover {
  border-color: var(--accent-blue, #58a6ff);
  background-color: var(--bg-tertiary, #161b22);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.timeline-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 12px;
}

.file-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f0f6fc);
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.file-size {
  font-size: 13px;
  color: var(--text-muted, #8b949e);
  padding: 4px 10px;
  background-color: var(--bg-tertiary, #161b22);
  border-radius: 12px;
  flex-shrink: 0;
}

.timeline-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 12px;
}

.timeline-meta .meta-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-muted, #8b949e);
}

.timeline-meta .meta-item i {
  font-size: 14px;
}

.timeline-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.download-btn {
  background-color: rgba(16, 185, 129, 0.15);
  color: #10b981;
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.download-btn:hover {
  background-color: rgba(16, 185, 129, 0.25);
  border-color: #10b981;
  transform: translateY(-1px);
}

.delete-btn {
  background-color: rgba(239, 68, 68, 0.15);
  color: #ef4444;
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.delete-btn:hover {
  background-color: rgba(239, 68, 68, 0.25);
  border-color: #ef4444;
  transform: translateY(-1px);
}

.files-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.file-type-section {
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 12px;
  overflow: hidden;
}

.file-type-header {
  padding: 16px 20px;
  background-color: var(--bg-tertiary, #161b22);
  border-bottom: 1px solid var(--border-color, #30363d);
}

.file-type-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f0f6fc);
}

.file-type-title .type-icon {
  font-size: 20px;
  color: var(--accent-blue, #58a6ff);
}

.file-count {
  margin-left: auto;
  padding: 2px 10px;
  background-color: rgba(88, 166, 255, 0.2);
  color: var(--accent-blue, #58a6ff);
  border-radius: 12px;
  font-size: 13px;
  font-weight: 600;
}

.file-list {
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background-color: var(--bg-primary, #0d1117);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
}

.file-item:hover {
  border-color: var(--accent-blue, #58a6ff);
  background-color: var(--bg-tertiary, #161b22);
  transform: translateX(4px);
}

.file-icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: linear-gradient(135deg, rgba(88, 166, 255, 0.15), rgba(49, 120, 198, 0.15));
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.file-type-icon {
  font-size: 24px;
  color: var(--accent-blue, #58a6ff);
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary, #f0f6fc);
  margin: 0 0 8px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-height: 1.4;
}

.file-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.file-meta .meta-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-muted, #8b949e);
}

.file-meta .meta-item i {
  font-size: 14px;
}

.file-arrow {
  font-size: 24px;
  color: var(--text-muted, #8b949e);
  flex-shrink: 0;
}

@media (max-width: 768px) {
  .group-detail-container {
    padding: 16px;
  }
  
  .page-header {
    gap: 12px;
  }
  
  .page-title {
    font-size: 20px;
  }
  
  .note-item,
  .file-item {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .note-arrow,
  .file-arrow {
    display: none;
  }
}

@media (max-aspect-ratio: 1/1) {
  .group-detail-container {
    padding: 16px;
  }
  
  .page-title {
    font-size: 20px;
  }
}
</style>
