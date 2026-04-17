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
      <h1 class="page-title">群组管理</h1>
      <button 
        class="refresh-btn" 
        @click="loadData"
        :disabled="isLoading"
        title="刷新数据"
      >
        <span :class="{ 'spinning': isLoading }">🔄</span>
      </button>
    </div>
    
    <!-- 创建群组区域 -->
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

    <!-- 群组列表 -->
    <div class="groups-section">
      <h2 class="section-title">我的群组</h2>
      
      <div v-if="groups.length === 0" class="empty-state">
        <p>暂无群组，创建一个吧！</p>
      </div>

      <div v-else class="groups-list">
        <div 
          v-for="group in groups" 
          :key="group.uid"
          class="group-item"
        >
          <div class="group-info">
            <span class="group-name">{{ group.name }}</span>
            <span class="group-uid">{{ group.uid }}</span>
          </div>
          <button 
            class="delete-btn" 
            @click="handleDeleteGroup(group.uid)"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- 消息处理区域 -->
    <div class="messages-section">
      <h2 class="section-title">待处理申请</h2>
      
      <div v-if="messages.length === 0" class="empty-state">
        <p>暂无待处理申请</p>
      </div>

      <div v-else class="messages-list">
        <div 
          v-for="message in messages" 
          :key="message.uuid"
          class="message-item"
        >
          <div class="message-info">
            <div class="message-header">
              <span class="message-type" :class="message.type">
                {{ message.type === 'join' ? '入群申请' : '退群申请' }}
              </span>
              <span class="message-status" :class="message.status">
                {{ message.status === 'pending' ? '待处理' : '已批准' }}
              </span>
            </div>
            <div class="message-details">
              <p class="student-email">学生：{{ message.student_email }}</p>
              <p class="reason">申请理由：{{ message.text || '无' }}</p>
              <p class="time">{{ formatTime(message.created_at) }}</p>
            </div>
          </div>
          <button 
            v-if="message.status === 'pending'"
            class="approve-btn" 
            @click="handleApprove(message.uuid)"
          >
            批准
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
// 群组管理页面
// 功能：创建/删除群组、处理入群/退群申请
// 注：所有请求都会 console.info 输出

import { ref, onMounted } from 'vue'
import { createGroup, deleteGroup, queryMessage, allowApplication, getGroupList, getMessageList } from '../components/data/group.js'
import { showToast } from '../components/layout/showToast.js'

// 群组管理页面现在不需要蓝牙连接也能访问（和笔记页面一样）
// 但实际 API 调用需要 TOTP 认证

const newGroupName = ref('')
const groups = ref([])
const messages = ref([])
const isLoading = ref(false)

// 创建群组
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
        name: newGroupName.value.trim()
      })
      newGroupName.value = ''
    }
  } catch (error) {
    const errorMsg = error.response?.data?.message || '创建失败'
    showToast(errorMsg, '#ef4444')
  }
}

// 删除群组
async function handleDeleteGroup(uid) {
  // 先弹出确认框
  const confirmed = confirm('确定要删除这个群组吗？此操作不可逆！')
  
  // 用户取消则直接返回，不发送删除请求
  if (!confirmed) {
    console.info('用户取消了删除操作')
    return
  }
  
  console.info('用户确认删除，开始发送删除请求...')

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

// 批准申请
async function handleApprove(uuid) {
  try {
    const result = await allowApplication(uuid)
    if (result && result.success) {
      showToast('申请已批准', '#10b981')
      // 更新本地状态
      const msg = messages.value.find(m => m.uuid === uuid)
      if (msg) {
        msg.status = 'approved'
      }
    }
  } catch (error) {
    const errorMsg = error.response?.data?.message || '批准失败'
    showToast(errorMsg, '#ef4444')
  }
}

// 格式化时间戳
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

// 加载群组和消息数据
async function loadData() {
  isLoading.value = true
  console.info('========== 开始加载群组管理数据 ==========')
  
  try {
    // 获取群组列表（如果后端未实现，不会报错）
    console.info('请求群组列表...')
    const groupData = await getGroupList()
    console.info('群组列表响应:', groupData)
    groups.value = Array.isArray(groupData) ? groupData : []
    console.info(`加载了 ${groups.value.length} 个群组`)
    
    // 获取消息列表
    console.info('请求消息列表...')
    const messageData = await getMessageList()
    console.info('消息列表响应:', messageData)
    messages.value = Array.isArray(messageData) ? messageData : []
    console.info(`加载了 ${messages.value.length} 条消息`)
    
    console.info('========== 数据加载完成 ==========')
  } catch (error) {
    console.error('加载数据失败:', error)
    showToast('加载数据失败', '#ef4444')
  } finally {
    isLoading.value = false
  }
}

// 页面加载时自动加载数据
onMounted(() => {
  console.info('群组管理页面已加载')
  loadData()
})
</script>

<style scoped>
.group-manager-container {
  padding: 20px;
  max-width: 1200px;
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
  padding: 8px 12px;
  font-size: 18px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--text-primary, #f0f6fc);
}

.refresh-btn:hover:not(:disabled) {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  border-color: var(--accent-blue, #3178c6);
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.refresh-btn .spinning {
  display: inline-block;
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

.create-section {
  margin-bottom: 40px;
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
  border-radius: 6px;
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
  border-radius: 6px;
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

.groups-section,
.messages-section {
  margin-bottom: 40px;
}

.section-title {
  font-size: 20px;
  margin-bottom: 20px;
  color: var(--text-primary, #f0f6fc);
  border-bottom: 1px solid var(--border-color, #30363d);
  padding-bottom: 10px;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: var(--text-muted, #8b949e);
  font-size: 14px;
}

.groups-list,
.messages-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.group-item,
.message-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 6px;
  transition: border-color 0.2s;
}

.group-item:hover,
.message-item:hover {
  border-color: var(--accent-blue, #3178c6);
}

.group-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.group-name {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-primary, #f0f6fc);
}

.group-uid {
  font-size: 12px;
  color: var(--text-muted, #8b949e);
}

.delete-btn {
  padding: 8px 16px;
  font-size: 14px;
  color: var(--danger-btn-text, #f85149);
  background-color: transparent;
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.delete-btn:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, #ffffff);
  border-color: var(--danger-btn-hover-border, #f85149);
}

.message-header {
  display: flex;
  gap: 12px;
  margin-bottom: 8px;
}

.message-type,
.message-status {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
}

.message-type.join {
  background-color: rgba(59, 130, 246, 0.2);
  color: #60a5fa;
}

.message-type.quit {
  background-color: rgba(245, 158, 11, 0.2);
  color: #fbbf24;
}

.message-status.pending {
  background-color: rgba(245, 158, 11, 0.2);
  color: #fbbf24;
}

.message-status.approved {
  background-color: rgba(34, 197, 94, 0.2);
  color: #4ade80;
}

.message-details {
  font-size: 14px;
  color: var(--text-secondary, #c9d1d9);
}

.message-details p {
  margin: 4px 0;
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

.approve-btn {
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  color: #fff;
  background-color: var(--accent-green, #3fb950);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.approve-btn:hover {
  background-color: #2ea043;
}

/* 响应式 */
@media (max-width: 768px) {
  .input-group {
    flex-direction: column;
  }
  
  .group-item,
  .message-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .delete-btn,
  .approve-btn {
    width: 100%;
  }
}

/* 竖屏模式 */
@media (max-aspect-ratio: 1/1) {
  .group-manager-container {
    padding: 16px;
  }
  
  .page-title {
    font-size: 24px;
  }
}
</style>
