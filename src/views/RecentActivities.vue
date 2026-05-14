

<script setup>
import Sidebar from '../components/layout/Sidebar.vue'
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  getRecentActivities,
  getRecentUploads,
  getRecentDownloads,
  getRecentAccesses,
  formatActivity,
  formatActivityTimestamp,
  getTypeLabel,
  getTypeColor,
  formatFileSize
} from '../components/data/activityLog.js'
import { getFileIcon } from '../utils/fileIcon.js'
import { showToast } from '../components/layout/showToast.js'

const isSidebarCollapsed = ref(false)

const handleCollapseChange = (collapsed) => {
  isSidebarCollapsed.value = collapsed
}

const activeTab = ref('all')
const limit = ref(10)
const activities = ref([])
const loading = ref(false)
const total = ref(0)
const userUuid = ref('')

const activityTypes = [
  { value: 'all', label: '全部' },
  { value: 'upload', label: '上传' },
  { value: 'download', label: '下载' },
  { value: 'access', label: '访问' }
]

const loadUserUuid = async () => {
  try {
    userUuid.value = await invoke('get_user_uuid')
    console.info('获取用户 UUID 成功:', userUuid.value)
  } catch (error) {
    console.error('获取用户 UUID 失败:', error)
    showToast(`获取用户 UUID 失败：${error}`, '#ef4444')
    throw error
  }
}

const loadActivities = async () => {
  if (!userUuid.value) {
    console.warn('用户 UUID 未加载，跳过活动记录加载')
    return
  }

  loading.value = true
  try {
    let result
    switch (activeTab.value) {
      case 'upload':
        result = await getRecentUploads(userUuid.value, limit.value)
        break
      case 'download':
        result = await getRecentDownloads(userUuid.value, limit.value)
        break
      case 'access':
        result = await getRecentAccesses(userUuid.value, limit.value)
        break
      default:
        result = await getRecentActivities({ userUuid: userUuid.value, limit: limit.value })
    }

    activities.value = result.activities || []
    total.value = result.total || 0

    console.info(`加载了 ${activities.value.length} 条活动记录，总计 ${total.value} 条`)
  } catch (error) {
    console.error('加载活动记录失败:', error)
    showToast(`加载活动记录失败：${error}`, '#ef4444')
    activities.value = []
    total.value = 0
  } finally {
    loading.value = false
  }
}

const refreshActivities = () => {
  loadActivities()
}

const getFormattedActivity = (activity) => {
  return formatActivity(activity)
}

const formatTime = (timestamp) => {
  return formatActivityTimestamp(timestamp)
}

const getTypeColorCode = (type) => {
  return getTypeColor(type)
}

const getFileIconClass = (filename) => {
  return getFileIcon(filename)
}

const onLimitChange = (event) => {
  const newLimit = parseInt(event.target.value)
  if (newLimit > 0 && newLimit <= 100) {
    limit.value = newLimit
    loadActivities()
  }
}

onMounted(async () => {
  try {
    await loadUserUuid()
    await loadActivities()
  } catch (error) {
    console.error('初始化失败:', error)
  }
})

onUnmounted(() => {

})

watch([activeTab, limit], () => {
  loadActivities()
})

</script>

<template>
  <div class="recent-activities-container">
    <Sidebar
      :collapsed="isSidebarCollapsed"
      @collapse="handleCollapseChange"
    />

    <div class="content" :class="{ 'sidebar-collapsed': isSidebarCollapsed }">
      <div class="header">
        <h1>最近活动记录</h1>
        <p class="subtitle">查看您最近操作过的云端文件记录</p>
      </div>

      <div class="controls">
        <div class="filter-group">
          <label>活动类型：</label>
          <div class="filter-buttons">
            <button
              v-for="type in activityTypes"
              :key="type.value"
              :class="['filter-btn', { active: activeTab === type.value }]"
              @click="activeTab = type.value"
            >
              {{ type.label }}
            </button>
          </div>
        </div>

        <div class="limit-group">
          <label>显示数量：</label>
          <input
            type="number"
            v-model.number="limit"
            min="1"
            max="100"
            @change="onLimitChange"
            class="limit-input"
          />
          <span class="limit-hint">（最多 100 条）</span>
        </div>

        <button
          class="refresh-btn"
          @click="refreshActivities"
          :disabled="loading"
        >
          <svg v-if="loading" class="spinner" viewBox="0 0 24 24">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none" opacity="0.25"/>
            <path d="M4 12a8 8 0 018-8" stroke="currentColor" stroke-width="4" stroke-linecap="round"/>
          </svg>
          <span v-else>刷新</span>
        </button>
      </div>

      <div class="stats" v-if="!loading && activities.length > 0">
        <span class="stat-item">
          <span class="stat-value">{{ total }}</span>
          <span class="stat-label">条记录</span>
        </span>
        <span class="stat-item">
          <span class="stat-value">{{ activities.length }}</span>
          <span class="stat-label">本次显示</span>
        </span>
      </div>

      <div class="activities-list" v-if="!loading && activities.length > 0">
        <div
          v-for="(activity, index) in activities"
          :key="index"
          class="activity-item"
        >
          <div class="activity-header">
            <div class="file-icon-wrapper">
              <i :class="getFileIconClass(activity.file_name || activity.file_path)"></i>
            </div>
            <div class="activity-info">
              <span class="activity-type" :style="{ color: getTypeColorCode(activity.type) }">
                {{ getTypeLabel(activity.type) }}
              </span>
              <span class="activity-time">
                {{ formatTime(activity.timestamp) }}
              </span>
            </div>
          </div>

          <div class="activity-body">
            <div class="file-name">
              {{ activity.file_name || activity.file_path }}
            </div>
            <div class="file-path">
              {{ activity.file_path }}
            </div>
            <div class="file-size">
              {{ formatFileSize(activity.file_size) }}
            </div>
          </div>
        </div>
      </div>

      <div class="empty-state" v-if="!loading && activities.length === 0">
        <div class="empty-icon">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#9ca3af" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="16" y1="13" x2="8" y2="13"/>
            <line x1="16" y1="17" x2="8" y2="17"/>
            <polyline points="10 9 9 9 8 9"/>
          </svg>
        </div>
        <p class="empty-text">
          {{
            total > 0
              ? `当前筛选条件下没有活动记录（总计 ${total} 条）`
              : '暂无活动记录，请先进行上传、下载或文件访问操作'
          }}
        </p>
        <button
          v-if="total > 0"
          class="clear-filters-btn"
          @click="activeTab = 'all'"
        >
          清除筛选
        </button>
      </div>

      <div class="skeleton-activities-list" v-if="loading">
        <div v-for="i in 5" :key="i" class="skeleton-activity-item">
          <div class="skeleton-activity-header">
            <div class="skeleton-icon"></div>
            <div class="skeleton-info">
              <div class="skeleton-type"></div>
              <div class="skeleton-time"></div>
            </div>
          </div>
          <div class="skeleton-activity-body">
            <div class="skeleton-file-name"></div>
            <div class="skeleton-file-path"></div>
            <div class="skeleton-file-size"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

.recent-activities-container {
  display: flex;
  height: calc(100vh - 48px);
  background-color: var(--bg-primary);
}

.content {
  flex: 1;
  margin-left: 240px;
  padding: 30px;
  overflow-y: auto;
  transition: margin-left 0.3s ease;
  max-width: 1200px;
}

.content.sidebar-collapsed {
  margin-left: 64px;
}

.header {
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--border-color);
}

.header h1 {
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.subtitle {
  font-size: 14px;
  color: var(--text-muted);
  margin: 0;
}

.controls {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
  flex-wrap: wrap;
  padding: 20px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
}

.filter-group,
.limit-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-group label,
.limit-group label {
  font-size: 14px;
  color: var(--text-secondary);
  font-weight: 500;
}

.filter-buttons {
  display: flex;
  gap: 4px;
}

.filter-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 2px;
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.filter-btn:hover {
  background-color: var(--hover-bg);
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.filter-btn.active {
  background-color: var(--accent-blue);
  border-color: var(--accent-blue);
  color: white;
}

.limit-input {
  width: 80px;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 2px;
  font-size: 14px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.limit-input:focus {
  outline: none;
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 2px rgba(var(--accent-blue-rgb, 49, 120, 198), 0.2);
}

.limit-hint {
  font-size: 12px;
  color: var(--text-muted);
}

.refresh-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 2px;
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.refresh-btn:hover:not(:disabled) {
  background-color: var(--hover-bg);
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.spinner {
  width: 16px;
  height: 16px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.stats {
  display: flex;
  gap: 24px;
  margin-bottom: 24px;
  padding: 16px 20px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.stat-value {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.stat-label {
  font-size: 13px;
  color: var(--text-muted);
}

.activities-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.activity-item {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
  padding: 16px;
  transition: all 0.2s;
}

.activity-item:hover {
  border-color: var(--accent-blue);
}

.activity-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.file-icon-wrapper {
  width: 40px;
  height: 40px;
  border-radius: 2px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
}

.file-icon-wrapper i {
  font-size: 20px;
  color: var(--accent-blue);
}

.activity-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.activity-type {
  font-size: 13px;
  font-weight: 500;
}

.activity-time {
  font-size: 12px;
  color: var(--text-muted);
}

.activity-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-left: 52px;
}

.file-name {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  word-break: break-all;
}

.file-path {
  font-size: 13px;
  color: var(--text-secondary);
  word-break: break-all;
}

.file-size {
  font-size: 13px;
  color: var(--text-muted);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 24px;
  text-align: center;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
}

.empty-icon {
  margin-bottom: 24px;
  opacity: 0.5;
  color: var(--text-muted);
}

.empty-text {
  font-size: 16px;
  color: var(--text-secondary);
  margin: 0 0 24px 0;
}

.clear-filters-btn {
  padding: 10px 20px;
  border: 1px solid var(--border-color);
  border-radius: 2px;
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.clear-filters-btn:hover {
  background-color: var(--hover-bg);
  border-color: var(--accent-blue);
  color: var(--text-primary);
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 24px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
}

.spinner-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.spinner-container p {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.skeleton-activities-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.skeleton-activity-item {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 2px;
  padding: 16px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-activity-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.skeleton-icon {
  width: 40px;
  height: 40px;
  border-radius: 2px;
  background-color: rgba(128, 128, 128, 0.2);
}

.skeleton-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.skeleton-type {
  height: 16px;
  width: 60px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 2px;
}

.skeleton-time {
  height: 12px;
  width: 80px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 2px;
}

.skeleton-activity-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-left: 52px;
}

.skeleton-file-name {
  height: 18px;
  width: 60%;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 2px;
}

.skeleton-file-path {
  height: 14px;
  width: 80%;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 2px;
}

.skeleton-file-size {
  height: 14px;
  width: 50px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 2px;
}

@keyframes skeleton-pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}

@media (max-width: 768px) {
  .content {
    padding: 20px;
    margin-left: 0;
  }

  .content.sidebar-collapsed {
    margin-left: 0;
  }

  .controls {
    flex-direction: column;
    align-items: stretch;
  }

  .filter-group,
  .limit-group {
    flex-direction: column;
    align-items: flex-start;
  }

  .filter-buttons {
    flex-wrap: wrap;
  }
}
</style>

