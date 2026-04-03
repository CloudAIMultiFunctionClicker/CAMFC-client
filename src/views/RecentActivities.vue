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

// 获取用户 UUID
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
  // 清理资源
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
            <div class="activity-icon" :style="{ backgroundColor: getTypeColorCode(activity.type) }">
              <svg v-if="activity.type === 'upload'" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="17 8 12 3 7 8"/>
                <line x1="12" y1="3" x2="12" y2="15"/>
              </svg>
              <svg v-else-if="activity.type === 'download'" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              <svg v-else-if="activity.type === 'access'" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                <circle cx="12" cy="12" r="3"/>
              </svg>
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
      
      <div class="loading-state" v-if="loading">
        <div class="spinner-container">
          <svg class="spinner" viewBox="0 0 24 24">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none" opacity="0.25"/>
            <path d="M4 12a8 8 0 018-8" stroke="currentColor" stroke-width="4" stroke-linecap="round"/>
          </svg>
          <p>正在加载活动记录...</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 参考 Notes.vue 的样式规范，使用 CSS 变量 */

.recent-activities-container {
  display: flex;
  height: 100vh;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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

.activity-icon {
  width: 32px;
  height: 32px;
  border-radius: .375rem;
  display: flex;
  align-items: center;
  justify-content: center;
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
  padding-left: 44px;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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
  border-radius: .375rem;
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

/* 响应式 - 竖屏适配 */
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

