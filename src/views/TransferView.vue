<script setup>
import Sidebar from '../components/layout/Sidebar.vue'
import { ref, onMounted, onUnmounted } from 'vue'
import { getUploadProgress, pauseUpload, resumeUpload } from '../components/data/upload.js'
import { getDownloadProgress, pauseDownload, resumeDownload } from '../components/data/download.js'
import { getActiveUploads, setActiveUploads, getActiveDownloads, setActiveDownloads, openFile, openFolder } from '../components/data/storage.js'
import { invoke } from '@tauri-apps/api/core'

const isSidebarCollapsed = ref(false)

const handleCollapseChange = (collapsed) => {
  isSidebarCollapsed.value = collapsed
}

const activeTab = ref('upload')

const uploadList = ref([])
const downloadList = ref([])
let pollTimer = null

const formatSize = (bytes) => {
  if (!bytes || bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return (bytes / Math.pow(1024, i)).toFixed(1) + ' ' + units[i]
}

const refreshUploads = async () => {
  const uploadIds = await getActiveUploads()
  if (!uploadIds || uploadIds.length === 0) {
    uploadList.value = []
    return
  }

  const validIds = []
  try {
    const newList = []

    for (const id of uploadIds) {
      try {
        const progress = await getUploadProgress(id)
        if (progress && progress.status !== 'Error') {
          validIds.push(id)
          newList.push({
            id: progress.upload_id,
            name: progress.filename || '未知文件',
            size: formatSize(progress.total_size),
            progress: progress.progress_percentage || 0,
            status: mapStatus(progress.status),
            speed: progress.speed_kbps ? (progress.speed_kbps / 1024).toFixed(1) + ' MB/s' : '-',
            uploaded: formatSize(progress.uploaded),
            total: formatSize(progress.total_size),
            uploadId: progress.upload_id
          })
        }
      } catch (e) {
        console.error('获取上传进度失败:', e)
      }
    }

    uploadList.value = newList
    if (validIds.length !== uploadIds.length) {
      await setActiveUploads(validIds)
    }
  } catch (e) {
    console.error('刷新上传列表失败:', e)
  }
}

const refreshDownloads = async () => {
  const fileIds = await getActiveDownloads()
  if (!fileIds || fileIds.length === 0) {
    downloadList.value = []
    return
  }

  const validIds = []
  try {
    const newList = []

    for (const id of fileIds) {
      try {
        const progress = await getDownloadProgress(id)
        if (progress && progress.status !== 'Error') {
          validIds.push(id)
          newList.push({
            id: progress.file_id,
            name: progress.file_name || '未知文件',
            size: formatSize(progress.total_size),
            progress: progress.progress_percentage || 0,
            status: mapStatus(progress.status),
            speed: progress.speed_kbps ? (progress.speed_kbps / 1024).toFixed(1) + ' MB/s' : '-',
            downloaded: formatSize(progress.downloaded),
            total: formatSize(progress.total_size),
            fileId: progress.file_id
          })
        }
      } catch (e) {
        console.error('获取下载进度失败:', e)
      }
    }

    downloadList.value = newList
    if (validIds.length !== fileIds.length) {
      await setActiveDownloads(validIds)
    }
  } catch (e) {
    console.error('刷新下载列表失败:', e)
  }
}

const mapStatus = (status) => {
  const map = {
    'Uploading': 'uploading',
    'Downloading': 'downloading',
    'Paused': 'paused',
    'Waiting': 'waiting',
    'Completed': 'completed',
    'Error': 'failed'
  }
  return map[status] || status
}

const refreshAll = async () => {
  await Promise.all([refreshUploads(), refreshDownloads()])
}

const handlePause = async (item, type) => {
  try {
    if (type === 'upload') {
      if (item.status === 'uploading' || item.status === 'downloading') {
        await pauseUpload(item.uploadId)
        item.status = 'paused'
      } else if (item.status === 'paused') {
        await resumeUpload(item.uploadId)
        item.status = 'uploading'
      }
    } else {
      if (item.status === 'uploading' || item.status === 'downloading') {
        await pauseDownload(item.fileId)
        item.status = 'paused'
      } else if (item.status === 'paused') {
        await resumeDownload(item.fileId)
        item.status = 'downloading'
      }
    }
  } catch (e) {
    console.error('操作失败:', e)
  }
}

const handleCancel = async (item, type) => {
  if (type === 'upload') {
    const stored = await getActiveUploads()
    const newList = stored.filter(id => id !== item.id)
    await setActiveUploads(newList)
    uploadList.value = uploadList.value.filter(i => i.id !== item.id)
  } else {
    const stored = await getActiveDownloads()
    const newList = stored.filter(id => id !== item.id)
    await setActiveDownloads(newList)
    downloadList.value = downloadList.value.filter(i => i.id !== item.id)
  }
}

const handleRetry = (item, type) => {
  item.status = type === 'upload' ? 'uploading' : 'downloading'
  item.progress = 0
}

const handleOpenFile = async (item) => {
  try {
    const filePath = await invoke('get_download_file_path', { fileId: item.fileId })
    await openFile(filePath)
  } catch (error) {
    console.error('打开文件失败:', error)
  }
}

const handleOpenFolder = async (item) => {
  try {
    const filePath = await invoke('get_download_file_path', { fileId: item.fileId })
    await openFolder(filePath)
  } catch (error) {
    console.error('打开文件夹失败:', error)
  }
}

const clearCompleted = async (type) => {
  if (type === 'upload') {
    const completedIds = uploadList.value
      .filter(i => i.status === 'completed' || i.status === 'failed')
      .map(i => i.id)
    if (completedIds.length > 0) {
      const stored = await getActiveUploads()
      const newList = stored.filter(id => !completedIds.includes(id))
      await setActiveUploads(newList)
    }
    uploadList.value = uploadList.value.filter(i => i.status !== 'completed' && i.status !== 'failed')
  } else {
    const completedIds = downloadList.value
      .filter(i => i.status === 'completed' || i.status === 'failed')
      .map(i => i.id)
    if (completedIds.length > 0) {
      const stored = await getActiveDownloads()
      const newList = stored.filter(id => !completedIds.includes(id))
      await setActiveDownloads(newList)
    }
    downloadList.value = downloadList.value.filter(i => i.status !== 'completed' && i.status !== 'failed')
  }
}

const getStatusText = (status) => {
  const map = {
    uploading: '上传中',
    downloading: '下载中',
    paused: '已暂停',
    waiting: '等待中',
    completed: '已完成',
    failed: '失败'
  }
  return map[status] || status
}

const getStatusClass = (status) => {
  return {
    'status-uploading': status === 'uploading',
    'status-downloading': status === 'downloading',
    'status-paused': status === 'paused',
    'status-waiting': status === 'waiting',
    'status-completed': status === 'completed',
    'status-failed': status === 'failed'
  }
}

onMounted(() => {
  refreshAll()
  pollTimer = setInterval(refreshAll, 500)
})

onUnmounted(() => {
  if (pollTimer) {
    clearInterval(pollTimer)
  }
})
</script>

<template>
  <div class="main-container">
    <Sidebar @collapse-change="handleCollapseChange"/>

    <div class="content-area" :class="{ 'expanded': isSidebarCollapsed }">
      <div class="transfer-container">
        <h1 class="page-title">传输</h1>

        <div class="tab-bar">
          <button
            class="tab-btn"
            :class="{ active: activeTab === 'upload' }"
            @click="activeTab = 'upload'"
          >
            上传
            <span class="tab-count" v-if="uploadList.length">{{ uploadList.length }}</span>
          </button>
          <button
            class="tab-btn"
            :class="{ active: activeTab === 'download' }"
            @click="activeTab = 'download'"
          >
            下载
            <span class="tab-count" v-if="downloadList.length">{{ downloadList.length }}</span>
          </button>
        </div>

        <div class="transfer-list" v-if="activeTab === 'upload'">
          <div class="list-header">
            <span class="header-name">文件名</span>
            <span class="header-size">大小</span>
            <span class="header-progress">进度</span>
            <span class="header-status">状态</span>
            <span class="header-action">操作</span>
          </div>

          <div class="list-content">
            <div
              v-for="item in uploadList"
              :key="item.id"
              class="transfer-item"
            >
              <div class="item-name">
                <span class="file-icon">📄</span>
                <span class="file-name">{{ item.name }}</span>
              </div>
              <div class="item-size">{{ item.size }}</div>
              <div class="item-progress">
                <div class="progress-bar">
                  <div
                    class="progress-fill"
                    :style="{ width: item.progress + '%' }"
                  ></div>
                </div>
                <span class="progress-text">{{ item.uploaded }} / {{ item.total }}</span>
              </div>
              <div class="item-status">
                <span class="status-badge" :class="getStatusClass(item.status)">
                  {{ getStatusText(item.status) }}
                </span>
                <span class="speed-text" v-if="item.status === 'uploading'">{{ item.speed }}</span>
              </div>
              <div class="item-action">
                <button
                  class="action-btn"
                  @click="handlePause(item, 'upload')"
                  v-if="item.status !== 'completed' && item.status !== 'failed'"
                  :title="item.status === 'paused' ? '继续' : '暂停'"
                >
                  {{ item.status === 'paused' ? '▶' : '⏸' }}
                </button>
                <button
                  class="action-btn retry"
                  @click="handleRetry(item, 'upload')"
                  v-if="item.status === 'failed'"
                  title="重试"
                >
                  ↻
                </button>
                <button
                  class="action-btn cancel"
                  @click="handleCancel(item, 'upload')"
                  v-if="item.status !== 'completed'"
                  title="取消"
                >
                  ✕
                </button>
              </div>
            </div>

            <div class="empty-state" v-if="uploadList.length === 0">
              <span class="empty-icon">📤</span>
              <p>暂无上传任务</p>
            </div>
          </div>

          <div class="list-footer" v-if="uploadList.length > 0">
            <button class="clear-btn" @click="clearCompleted('upload')">
              清除已完成
            </button>
          </div>
        </div>

        <div class="transfer-list" v-if="activeTab === 'download'">
          <div class="list-header">
            <span class="header-name">文件名</span>
            <span class="header-size">大小</span>
            <span class="header-progress">进度</span>
            <span class="header-status">状态</span>
            <span class="header-action">操作</span>
          </div>

          <div class="list-content">
            <div
              v-for="item in downloadList"
              :key="item.id"
              class="transfer-item"
            >
              <div class="item-name">
                <span class="file-icon">📄</span>
                <span class="file-name">{{ item.name }}</span>
              </div>
              <div class="item-size">{{ item.size }}</div>
              <div class="item-progress">
                <div class="progress-bar">
                  <div
                    class="progress-fill"
                    :style="{ width: item.progress + '%' }"
                  ></div>
                </div>
                <span class="progress-text">{{ item.downloaded }} / {{ item.total }}</span>
              </div>
              <div class="item-status">
                <span class="status-badge" :class="getStatusClass(item.status)">
                  {{ getStatusText(item.status) }}
                </span>
                <span class="speed-text" v-if="item.status === 'downloading'">{{ item.speed }}</span>
              </div>
              <div class="item-action">
                <button
                  class="action-btn"
                  @click="handleOpenFolder(item)"
                  v-if="item.status === 'completed'"
                  title="打开文件夹"
                >
                  📁
                </button>
                <button
                  class="action-btn"
                  @click="handleOpenFile(item)"
                  v-if="item.status === 'completed'"
                  title="打开文件"
                >
                  📄
                </button>
                <button
                  class="action-btn"
                  @click="handlePause(item, 'download')"
                  v-if="item.status !== 'completed' && item.status !== 'failed'"
                  :title="item.status === 'paused' ? '继续' : '暂停'"
                >
                  {{ item.status === 'paused' ? '▶' : '⏸' }}
                </button>
                <button
                  class="action-btn retry"
                  @click="handleRetry(item, 'download')"
                  v-if="item.status === 'failed'"
                  title="重试"
                >
                  ↻
                </button>
                <button
                  class="action-btn cancel"
                  @click="handleCancel(item, 'download')"
                  v-if="item.status !== 'completed'"
                  title="取消"
                >
                  ✕
                </button>
              </div>
            </div>

            <div class="empty-state" v-if="downloadList.length === 0">
              <span class="empty-icon">📥</span>
              <p>暂无下载任务</p>
            </div>
          </div>

          <div class="list-footer" v-if="downloadList.length > 0">
            <button class="clear-btn" @click="clearCompleted('download')">
              清除已完成
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.main-container {
  display: flex;
  width: 100%;
  height: calc(100vh - 65px);
  overflow: hidden;
}

.content-area {
  flex: 1;
  background: var(--bg-primary, #0f172a);
  padding: 24px;
  margin-left: 0;
  box-sizing: border-box;
  overflow-y: auto;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.content-area.expanded {
  margin-left: -240px;
}

.transfer-container {
  max-width: 1000px;
  margin: 0 auto;
}

.page-title {
  font-size: 28px;
  margin-bottom: 24px;
  color: var(--text-primary);
}

.tab-bar {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 8px;
}

.tab-btn {
  padding: 10px 24px;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-size: 15px;
  cursor: pointer;
  border-radius: 8px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
}

.tab-btn:hover {
  background: var(--bg-secondary);
}

.tab-btn.active {
  background: var(--accent-blue);
  color: white;
}

.tab-count {
  background: rgba(255,255,255,0.2);
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 12px;
}

.tab-btn:not(.active) .tab-count {
  background: var(--bg-secondary);
  color: var(--text-secondary);
}

.transfer-list {
  background: var(--bg-secondary);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.list-header {
  display: grid;
  grid-template-columns: 2fr 1fr 2fr 1fr 150px;
  gap: 16px;
  padding: 14px 20px;
  background: rgba(0,0,0,0.2);
  font-size: 13px;
  color: var(--text-muted);
  font-weight: 500;
}

.list-content {
  max-height: 500px;
  overflow-y: auto;
}

.transfer-item {
  display: grid;
  grid-template-columns: 2fr 1fr 2fr 1fr 150px;
  gap: 16px;
  padding: 16px 20px;
  align-items: center;
  border-bottom: 1px solid var(--border-color);
  transition: background 0.2s;
}

.transfer-item:last-child {
  border-bottom: none;
}

.transfer-item:hover {
  background: rgba(255,255,255,0.02);
}

.item-name {
  display: flex;
  align-items: center;
  gap: 10px;
  overflow: hidden;
}

.file-icon {
  font-size: 20px;
}

.file-name {
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-size {
  color: var(--text-secondary);
  font-size: 14px;
}

.item-progress {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.progress-bar {
  height: 6px;
  background: var(--bg-primary);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--accent-blue);
  border-radius: 3px;
  transition: width 0.3s;
}

.progress-text {
  font-size: 12px;
  color: var(--text-muted);
}

.item-status {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.status-badge {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  width: fit-content;
}

.status-uploading, .status-downloading {
  background: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
}

.status-paused {
  background: rgba(245, 158, 11, 0.2);
  color: #f59e0b;
}

.status-waiting {
  background: rgba(100, 116, 139, 0.2);
  color: #64748b;
}

.status-completed {
  background: rgba(16, 185, 129, 0.2);
  color: #10b981;
}

.status-failed {
  background: rgba(239, 68, 68, 0.2);
  color: #ef4444;
}

.speed-text {
  font-size: 12px;
  color: var(--text-muted);
}

.item-action {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.action-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.action-btn:hover {
  background: var(--accent-blue);
  color: white;
}

.action-btn.cancel:hover {
  background: #ef4444;
}

.action-btn.retry:hover {
  background: #f59e0b;
}

.list-footer {
  padding: 12px 20px;
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
}

.clear-btn {
  padding: 8px 16px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.clear-btn:hover {
  background: var(--bg-primary);
  color: var(--text-primary);
}

.empty-state {
  padding: 60px 20px;
  text-align: center;
  color: var(--text-muted);
}

.empty-icon {
  font-size: 48px;
  display: block;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-state p {
  font-size: 15px;
}

@media (max-width: 768px) {
  .list-header {
    display: none;
  }

  .transfer-item {
    grid-template-columns: 1fr;
    gap: 12px;
    padding: 16px;
  }

  .item-name {
    font-weight: 500;
  }

  .item-progress, .item-status, .item-action {
    padding-left: 30px;
  }
}
</style>
