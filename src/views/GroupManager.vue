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
  <div class="group-manager-container">
    <div class="page-header">
      <h1 class="page-title">班级管理</h1>
      <button 
        class="refresh-btn" 
        @click="loadData"
        :disabled="isLoading"
        title="刷新数据"
      >
        <i class="ri-refresh-line" :class="{ 'spinning': isLoading }"></i>
      </button>
    </div>

    <div class="create-section">
      <div class="input-group">
        <input
          v-model="newGroupName"
          type="text"
          placeholder="输入群组名称（1-15 字符）"
          class="group-name-input"
          @keyup.enter="handleCreateGroup"
          maxlength="15"
        />
        <button 
          class="create-btn" 
          @click="handleCreateGroup"
          :disabled="!newGroupName.trim()"
        >
          创建群组
        </button>
      </div>
    </div>

    <div class="tab-navigation">
      <button 
        class="tab-btn" 
        :class="{ active: currentTab === 'groups' }"
        @click="currentTab = 'groups'"
      >
        <i class="ri-group-line"></i>
        我的群组
      </button>
      <button 
        class="tab-btn" 
        :class="{ active: currentTab === 'applications' }"
        @click="currentTab = 'applications'"
      >
        <i class="ri-notification-badge-line"></i>
        待处理申请
        <span v-if="pendingApplicationsCount > 0" class="badge">{{ pendingApplicationsCount }}</span>
      </button>
    </div>

    <div v-if="currentTab === 'groups'" class="tab-content groups-tab">
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载群组列表...</p>
      </div>

      <div v-else-if="groups.length === 0" class="empty-state">
        <i class="ri-group-line empty-icon"></i>
        <p>暂无群组，创建一个吧！</p>
      </div>

      <div v-else class="groups-grid">
        <div 
          v-for="group in groups" 
          :key="group.uid"
          class="group-card"
          @click="goToGroupDetail(group)"
        >
          <div class="group-avatar">
            <i class="ri-group-fill"></i>
          </div>
          <div class="group-info">
            <h3 class="group-name">{{ group.name }}</h3>
            <p class="group-uid">UID: {{ group.uid }}</p>
            <div class="group-stats">
              <span class="stat-item">
                <i class="ri-user-line"></i>
                {{ group.member_count || 0 }} 人
              </span>
              <span class="stat-item">
                <i class="ri-file-text-line"></i>
                {{ group.note_count || 0 }} 篇笔记
              </span>
            </div>
          </div>
          <div class="group-actions">
            <button 
              class="enter-btn" 
              @click.stop="goToGroupDetail(group)"
            >
              进入群组
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="currentTab === 'applications'" class="tab-content applications-tab">
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载申请列表...</p>
      </div>

      <div v-else-if="messages.length === 0" class="empty-state">
        <i class="ri-notification-off-line empty-icon"></i>
        <p>暂无待处理申请</p>
      </div>

      <div v-else class="applications-list">
        <div 
          v-for="message in messages" 
          :key="message.uuid"
          class="application-item"
          :class="message.type"
        >
          <div class="application-info">
            <div class="application-header">
              <span class="application-type" :class="message.type">
                <i :class="message.type === 'join' ? 'ri-user-add-line' : 'ri-user-unfollow-line'"></i>
                {{ message.type === 'join' ? '入群申请' : '退群申请' }}
              </span>
              <span class="application-status" :class="message.status">
                <i :class="message.status === 'pending' ? 'ri-time-line' : 'ri-checkbox-circle-line'"></i>
                {{ message.status === 'pending' ? '待处理' : '已批准' }}
              </span>
            </div>
            <div class="application-details">
              <p class="student-email">
                <i class="ri-user-line"></i>
                申请人：{{ message.student_email }}
              </p>
              <p class="group-name-detail">
                <i class="ri-group-line"></i>
                群组：{{ message.group_name }}
              </p>
              <p class="reason">
                <i class="ri-file-text-line"></i>
                申请理由：{{ message.text || '无' }}
              </p>
              <p class="time">
                <i class="ri-time-line"></i>
                {{ formatTime(message.timestamp) }}
              </p>
            </div>
          </div>
          <div v-if="message.status === 'pending'" class="action-buttons">
            <button 
              class="approve-btn" 
              @click="handleApprove(message)"
              :title="message.type === 'join' ? '批准入群' : '批准退群'"
            >
              <i class="ri-checkbox-line"></i>
              批准
            </button>
            <button 
              v-if="message.type === 'join'"
              class="reject-btn" 
              @click="handleReject(message.uuid)"
              title="拒绝申请"
            >
              <i class="ri-close-line"></i>
              拒绝
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { createGroup, deleteGroup, approveJoin, rejectJoin, approveQuit, getGroupList, getMessageList } from '../components/data/group.js'
import { showToast } from '../components/layout/showToast.js'

const router = useRouter()

const newGroupName = ref('')
const groups = ref([])
const messages = ref([])
const isLoading = ref(false)
const currentTab = ref('groups')

const pendingApplicationsCount = computed(() => {
  return messages.value.filter(m => m.status === 'pending').length
})

async function handleCreateGroup() {
  if (!newGroupName.value.trim()) {
    showToast('请输入群组名称', '#f59e0b')
    return
  }

  if (newGroupName.value.trim().length > 15) {
    showToast('群组名称不能超过 15 个字符', '#f59e0b')
    return
  }

  try {
    const result = await createGroup(newGroupName.value.trim())
    if (result && result.uid) {
      showToast('群组创建成功', '#10b981')
      groups.value.push({
        uid: result.uid,
        name: newGroupName.value.trim(),
        member_count: 1,
        note_count: 0
      })
      newGroupName.value = ''
    }
  } catch (error) {
    const errorMsg = error.response?.data?.message || '创建失败'
    showToast(errorMsg, '#ef4444')
  }
}

async function handleDeleteGroup(uid) {
  const confirmed = confirm('确定要删除这个群组吗？此操作不可逆！')
  
  if (!confirmed) {
    console.info('用户取消了删除操作')
    return
  }
  
  try {
    const result = await deleteGroup(uid)
    if (result && result.success) {
      showToast('群组已删除', '#10b981')
      groups.value = groups.value.filter(g => g.uid !== uid)
    }
  } catch (error) {
    const errorMsg = error.response?.data?.message || '删除失败'
    showToast(errorMsg, '#ef4444')
  }
}

async function handleApprove(message) {
  try {
    let result;
    if (message.type === 'join') {
      result = await approveJoin(message.uuid)
    } else if (message.type === 'quit') {
      result = await approveQuit(message.uuid)
    }
    
    if (result && result.success) {
      showToast('申请已批准', '#10b981')
      messages.value = messages.value.filter(m => m.uuid !== message.uuid)
    }
  } catch (error) {
    const errorMsg = error.response?.data?.detail || error.message || '批准失败'
    showToast(errorMsg, '#ef4444')
  }
}

async function handleReject(uuid) {
  try {
    const result = await rejectJoin(uuid)
    if (result && result.success) {
      showToast('申请已拒绝', '#10b981')
      messages.value = messages.value.filter(m => m.uuid !== uuid)
    }
  } catch (error) {
    const errorMsg = error.response?.data?.detail || error.message || '拒绝失败'
    showToast(errorMsg, '#ef4444')
  }
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
    return date.toLocaleDateString('zh-CN')
  }
}

function goToGroupDetail(group) {
  router.push({
    path: '/group-detail',
    query: {
      uid: group.uid,
      name: group.name
    }
  })
}

async function loadData() {
  isLoading.value = true
  
  try {
    const groupData = await getGroupList()
    groups.value = Array.isArray(groupData) ? groupData : []
    
    const messageData = await getMessageList()
    messages.value = Array.isArray(messageData) ? messageData : []
    
    if (groups.value.length > 0 && currentTab.value === 'groups') {
    }
  } catch (error) {
    console.error('加载数据失败:', error)
    showToast('加载数据失败', '#ef4444')
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  console.info('班级管理页面已加载')
  loadData()
})
</script>

<style scoped>
.group-manager-container {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 30px;
}

.page-title {
  font-size: 28px;
  margin: 0;
  color: var(--text-primary, #f0f6fc);
  text-align: center;
  flex: 1;
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

.create-section {
  margin-bottom: 30px;
}

.input-group {
  display: flex;
  gap: 12px;
  max-width: 600px;
  margin: 0 auto;
}

.group-name-input {
  flex: 1;
  padding: 12px 16px;
  font-size: 14px;
  border: 1px solid var(--border-color, #30363d);
  border-radius: 8px;
  background-color: var(--input-bg, #000000);
  color: var(--text-primary, #f0f6fc);
  outline: none;
  transition: border-color 0.2s;
}

.group-name-input:focus {
  border-color: var(--accent-blue, #3178c6);
}

.create-btn {
  padding: 12px 24px;
  font-size: 14px;
  font-weight: 500;
  color: #fff;
  background-color: var(--accent-blue, #3178c6);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.create-btn:hover:not(:disabled) {
  background-color: var(--accent-blue-bright, #1f6feb);
}

.create-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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

.badge {
  position: absolute;
  top: 4px;
  right: 8px;
  min-width: 18px;
  height: 18px;
  padding: 0 6px;
  font-size: 11px;
  font-weight: 600;
  color: white;
  background-color: #ef4444;
  border-radius: 9px;
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
}

.groups-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
}

.group-card {
  display: flex;
  flex-direction: column;
  padding: 20px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.group-card:hover {
  border-color: var(--accent-blue, #58a6ff);
  background-color: var(--bg-tertiary, #161b22);
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
}

.group-avatar {
  width: 64px;
  height: 64px;
  border-radius: 12px;
  background: linear-gradient(135deg, rgba(88, 166, 255, 0.2), rgba(49, 120, 198, 0.2));
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 16px;
}

.group-avatar i {
  font-size: 32px;
  color: var(--accent-blue, #58a6ff);
}

.group-info {
  flex: 1;
  margin-bottom: 16px;
}

.group-name {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #f0f6fc);
  margin: 0 0 8px 0;
}

.group-uid {
  font-size: 12px;
  color: var(--text-muted, #8b949e);
  margin: 0 0 12px 0;
}

.group-stats {
  display: flex;
  gap: 16px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-secondary, #c9d1d9);
}

.stat-item i {
  font-size: 14px;
  color: var(--accent-blue, #58a6ff);
}

.group-actions {
  display: flex;
  gap: 8px;
}

.enter-btn {
  flex: 1;
  padding: 10px 16px;
  font-size: 14px;
  font-weight: 500;
  color: #fff;
  background-color: var(--accent-blue, #3178c6);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.enter-btn:hover {
  background-color: var(--accent-blue-bright, #1f6feb);
}

.applications-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.application-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 20px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 12px;
  transition: all 0.2s;
}

.application-item:hover {
  border-color: var(--accent-blue, #58a6ff);
}

.application-item.join {
  border-left: 4px solid var(--accent-blue, #58a6ff);
}

.application-item.quit {
  border-left: 4px solid var(--accent-orange, #f59e0b);
}

.application-info {
  flex: 1;
}

.application-header {
  display: flex;
  gap: 12px;
  align-items: center;
  margin-bottom: 12px;
}

.application-type,
.application-status {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
}

.application-type.join {
  background-color: rgba(88, 166, 255, 0.2);
  color: var(--accent-blue, #58a6ff);
}

.application-type.quit {
  background-color: rgba(245, 158, 11, 0.2);
  color: var(--accent-orange, #f59e0b);
}

.application-status.pending {
  background-color: rgba(245, 158, 11, 0.2);
  color: var(--accent-orange, #f59e0b);
}

.application-status.approved {
  background-color: rgba(34, 197, 94, 0.2);
  color: var(--accent-green, #4ade80);
}

.application-details {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.application-details p {
  margin: 0;
  font-size: 14px;
  color: var(--text-secondary, #c9d1d9);
  display: flex;
  align-items: center;
  gap: 8px;
}

.application-details i {
  font-size: 16px;
  color: var(--text-muted, #8b949e);
}

.student-email {
  font-weight: 500;
}

.reason {
  color: var(--text-muted, #8b949e);
}

.time {
  font-size: 12px;
  color: var(--text-muted, #8b949e);
}

.action-buttons {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
  margin-left: 16px;
}

.approve-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  color: #fff;
  background-color: var(--accent-green, #3fb950);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.approve-btn:hover {
  background-color: #2ea043;
}

.reject-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  color: var(--danger-btn-text, #f85149);
  background-color: transparent;
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.reject-btn:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, #ffffff);
  border-color: var(--danger-btn-hover-border, #f85149);
}

@media (max-width: 768px) {
  .group-manager-container {
    padding: 16px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .input-group {
    flex-direction: column;
  }
  
  .groups-grid {
    grid-template-columns: 1fr;
  }
  
  .application-item {
    flex-direction: column;
    gap: 16px;
  }
  
  .action-buttons {
    width: 100%;
    margin-left: 0;
  }
  
  .action-buttons button {
    flex: 1;
  }
}

@media (max-aspect-ratio: 1/1) {
  .group-manager-container {
    padding: 16px;
  }
  
  .page-title {
    font-size: 24px;
  }
  
  .groups-grid {
    grid-template-columns: 1fr;
  }
}
</style>
