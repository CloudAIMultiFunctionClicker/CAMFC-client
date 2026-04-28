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
              <p class="group-name-detail">群组：{{ message.group_name }}</p>
              <p class="reason">申请理由：{{ message.text || '无' }}</p>
              <p class="time">{{ formatTime(message.timestamp) }}</p>
            </div>
          </div>
          <div class="action-buttons">
            <button 
              v-if="message.status === 'pending'"
              class="approve-btn" 
              @click="handleApprove(message)"
              :title="message.type === 'join' ? '批准入群' : '批准退群'"
            >
              批准
            </button>
            <button 
              v-if="message.status === 'pending' && message.type === 'join'"
              class="reject-btn" 
              @click="handleReject(message.uuid)"
              title="拒绝申请"
            >
              拒绝
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 群组共享笔记区域 -->
    <div class="shared-notes-section">
      <div class="section-header">
        <h2 class="section-title">群组共享笔记</h2>
        <select v-model="selectedGroupForNotes" class="group-select" @change="handleGroupSelect">
          <option value="">选择群组</option>
          <option v-for="group in groups" :key="group.uid" :value="group.uid">
            {{ group.name }} ({{ group.uid }})
          </option>
        </select>
      </div>
      
      <div v-if="!selectedGroupForNotes" class="empty-state">
        <p>请先选择一个群组</p>
      </div>
      
      <div v-else-if="sharedNotesLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>正在加载共享笔记...</p>
      </div>
      
      <div v-else-if="sharedNotes.length === 0" class="empty-state">
        <p>该群组暂无共享笔记</p>
      </div>
      
      <div v-else class="shared-notes-list">
        <div 
          v-for="note in sharedNotes" 
          :key="note.share_uuid"
          class="shared-note-item"
          @click="viewSharedNoteDetail(note.share_uuid)"
        >
          <div class="note-info">
            <div class="note-header">
              <span class="note-type" :class="note.type">
                {{ note.type === 'personal' ? '个人笔记' : '会议记录' }}
              </span>
              <span class="note-shared-by">分享者：{{ note.shared_by }}</span>
            </div>
            <div class="note-details">
              <p class="note-title">{{ note.title }}</p>
              <p class="note-time">{{ formatTime(note.shared_at) }}</p>
            </div>
          </div>
          <i class="ri-arrow-right-s-line"></i>
        </div>
      </div>
    </div>

    <!-- 共享笔记详情弹窗 -->
    <Transition name="modal">
      <div v-if="showNoteDetailModal" class="modal-overlay" @click="closeNoteDetailModal">
        <div class="modal-content note-detail-modal" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-sticky-note-line"></i> {{ selectedNoteDetail?.title || '共享笔记详情' }}</h3>
            <button class="close-btn" @click="closeNoteDetailModal">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body note-detail-body">
            <div v-if="noteDetailLoading" class="loading-state">
              <div class="loading-spinner"></div>
              <p>正在加载笔记详情...</p>
            </div>
            <div v-else-if="selectedNoteDetail">
              <div class="note-meta-info">
                <p><strong>类型：</strong>{{ selectedNoteDetail.type === 'personal' ? '个人笔记' : '会议记录' }}</p>
                <p><strong>分享者：</strong>{{ selectedNoteDetail.shared_by }}</p>
                <p><strong>分享时间：</strong>{{ formatTime(selectedNoteDetail.shared_at) }}</p>
              </div>
              
              <div v-if="selectedNoteDetail.type === 'personal'" class="note-content-section">
                <h4>笔记内容</h4>
                <div class="note-content" v-html="formatNoteContent(selectedNoteDetail.content)"></div>
              </div>
              
              <div v-if="selectedNoteDetail.type === 'meeting'" class="meeting-content-section">
                <h4>会议笔记</h4>
                <div v-if="selectedNoteDetail.meeting_notes && selectedNoteDetail.meeting_notes.length > 0">
                  <div v-for="(note, index) in selectedNoteDetail.meeting_notes" :key="index" class="meeting-note-item">
                    <p class="meeting-note-time">{{ note.formatted_time }}</p>
                    <div class="meeting-note-content" v-html="formatNoteContent(note.content)"></div>
                  </div>
                </div>
                <div v-else>
                  <p class="empty-message">暂无会议笔记</p>
                </div>
              </div>
              
              <div v-if="selectedNoteDetail.ai_title" class="ai-section">
                <h4><i class="ri-robot-line"></i> AI 标题</h4>
                <p class="ai-title">{{ selectedNoteDetail.ai_title.title }}</p>
              </div>
              
              <div v-if="selectedNoteDetail.ai_keywords && selectedNoteDetail.ai_keywords.key_words && selectedNoteDetail.ai_keywords.key_words.length > 0" class="ai-section">
                <h4><i class="ri-tag-line"></i> AI 关键词</h4>
                <div class="ai-keywords">
                  <span v-for="(keyword, index) in selectedNoteDetail.ai_keywords.key_words" :key="index" class="keyword-tag">
                    {{ keyword }}
                  </span>
                </div>
              </div>
              
              <div v-if="selectedNoteDetail.ai_analysis" class="ai-section ai-analysis-section">
                <h4><i class="ri-brain-line"></i> AI 分析</h4>
                <div class="ai-analysis-content" v-html="formatAIAnalysis(selectedNoteDetail.ai_analysis)"></div>
              </div>
              
              <!-- 学生互动数据 -->
              <div v-if="noteInteractionsData && !interactionsLoading" class="interactions-section">
                <h4><i class="ri-user-star-line"></i> 学生互动数据</h4>
                
                <!-- 阅读记录 -->
                <div class="interaction-stats">
                  <div class="stat-item read-record">
                    <span class="stat-label">📖 已读：</span>
                    <span class="stat-value" :class="{ 'has-data': noteInteractionsData.read_by && noteInteractionsData.read_by.length > 0 }">
                      {{ noteInteractionsData.read_by && noteInteractionsData.read_by.length > 0 ? noteInteractionsData.read_by.join(', ') : '暂无' }}
                    </span>
                  </div>
                </div>
                
                <!-- 全文收藏统计 -->
                <div class="interaction-stats">
                  <div class="stat-item full-text">
                    <span class="stat-label">⭐ 收藏：</span>
                    <span class="stat-value" :class="{ 'has-data': noteInteractionsData.note_favorited_by && noteInteractionsData.note_favorited_by.length > 0 }">
                      {{ noteInteractionsData.note_favorited_by && noteInteractionsData.note_favorited_by.length > 0 ? noteInteractionsData.note_favorited_by.join(', ') : '暂无' }}
                    </span>
                  </div>
                </div>
                
                <!-- 分块互动数据 -->
                <div v-if="noteInteractionsData.blocks && Object.keys(noteInteractionsData.blocks).length > 0" class="block-interactions">
                  <div v-for="(blockData, blockIndex) in noteInteractionsData.blocks" :key="blockIndex" class="block-interaction-item">
                    <div class="block-header">
                      <span class="block-index">第{{ parseInt(blockIndex) + 1 }}个内容块</span>
                    </div>
                    <div class="block-stats">
                      <div class="stat-item">
                        <span class="stat-label">⭐ 收藏：</span>
                        <span class="stat-value" :class="{ 'has-data': blockData.favorited_by && blockData.favorited_by.length > 0 }">
                          {{ blockData.favorited_by && blockData.favorited_by.length > 0 ? blockData.favorited_by.join(', ') : '暂无' }}
                        </span>
                      </div>
                      <div class="stat-item">
                        <span class="stat-label">❓ 提问：</span>
                        <span class="stat-value" :class="{ 'has-data': blockData.question_by && blockData.question_by.length > 0 }">
                          {{ blockData.question_by && blockData.question_by.length > 0 ? blockData.question_by.join(', ') : '暂无' }}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
                
                <div v-else-if="!noteInteractionsData.note_favorited_by || noteInteractionsData.note_favorited_by.length === 0" class="no-interactions">
                  <p>暂无学生互动数据</p>
                </div>
              </div>
            </div>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="closeNoteDetailModal">关闭</button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
// 班级管理页面
// 功能：创建/删除群组、处理入群/退群申请
// 注：所有请求都会 console.info 输出

import { ref, onMounted } from 'vue'
import { createGroup, deleteGroup, queryMessage, approveJoin, rejectJoin, approveQuit, getGroupList, getMessageList, getSharedNotes, getSharedNoteDetail, getNoteInteractions, recordNoteRead } from '../components/data/group.js'
import { showToast } from '../components/layout/showToast.js'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

// 班级管理页面现在不需要蓝牙连接也能访问（和笔记页面一样）
// 但实际 API 调用需要 TOTP 认证

const newGroupName = ref('')
const groups = ref([])
const messages = ref([])
const isLoading = ref(false)

// 共享笔记相关
const selectedGroupForNotes = ref('')
const sharedNotes = ref([])
const sharedNotesLoading = ref(false)
const showNoteDetailModal = ref(false)
const selectedNoteDetail = ref(null)
const noteDetailLoading = ref(false)

// 笔记互动数据相关
const noteInteractionsData = ref(null)
const interactionsLoading = ref(false)

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
      // 从列表中移除该消息
      messages.value = messages.value.filter(m => m.uuid !== message.uuid)
    }
  } catch (error) {
    const errorMsg = error.response?.data?.detail || error.message || '批准失败'
    showToast(errorMsg, '#ef4444')
  }
}

// 拒绝申请
async function handleReject(uuid) {
  try {
    const result = await rejectJoin(uuid)
    if (result && result.success) {
      showToast('申请已拒绝', '#10b981')
      // 从列表中移除该消息
      messages.value = messages.value.filter(m => m.uuid !== uuid)
    }
  } catch (error) {
    const errorMsg = error.response?.data?.detail || error.message || '拒绝失败'
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

// 处理群组选择
function handleGroupSelect() {
  if (selectedGroupForNotes.value) {
    loadSharedNotes()
  } else {
    sharedNotes.value = []
  }
}

// 加载共享笔记
async function loadSharedNotes() {
  if (!selectedGroupForNotes.value) {
    sharedNotes.value = []
    return
  }
  
  sharedNotesLoading.value = true
  try {
    const notes = await getSharedNotes(selectedGroupForNotes.value)
    sharedNotes.value = notes || []
  } catch (error) {
    console.error('加载共享笔记失败:', error)
    showToast('加载共享笔记失败', '#ef4444')
    sharedNotes.value = []
  } finally {
    sharedNotesLoading.value = false
  }
}

// 查看共享笔记详情（打开独立窗口）
async function viewSharedNoteDetail(shareUuid) {
  if (!selectedGroupForNotes.value || !shareUuid) {
    console.error('参数错误:', { shareUuid, groupUuid: selectedGroupForNotes.value })
    showToast('参数错误', '#ef4444')
    return
  }

  console.info('打开笔记查看窗口:', { shareUuid, groupUuid: selectedGroupForNotes.value })

  // 获取笔记标题（用于窗口标题）
  const note = sharedNotes.value.find(n => n.share_uuid === shareUuid)
  const noteTitle = note?.title || '共享笔记'

  const windowLabel = `note-viewer-${shareUuid}`
  const url = `/note-viewer?shareUuid=${shareUuid}&groupUuid=${selectedGroupForNotes.value}&title=${encodeURIComponent(noteTitle)}`

  try {
    const webview = new WebviewWindow(windowLabel, {
      url: url,
      title: noteTitle,
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
        // 窗口已存在，获取并置顶
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
  } catch (error) {
    console.error('创建笔记查看窗口失败:', error)
    showToast('打开笔记查看窗口失败: ' + (error.message || '未知错误'), '#ef4444')
  }
}

// 关闭笔记详情弹窗
function closeNoteDetailModal() {
  showNoteDetailModal.value = false
  selectedNoteDetail.value = null
  noteDetailLoading.value = false
  noteInteractionsData.value = null
  interactionsLoading.value = false
}

// 格式化笔记内容（处理换行）
function formatNoteContent(content) {
  if (!content) return ''
  // 将换行符替换为 <br>
  return content.replace(/\n/g, '<br>')
}

// 格式化 AI 分析内容
function formatAIAnalysis(analysis) {
  if (!analysis) return ''
  
  // 如果是字符串，尝试解析为 JSON
  if (typeof analysis === 'string') {
    try {
      const parsed = JSON.parse(analysis)
      analysis = parsed
    } catch (e) {
      // 解析失败，直接返回原文本
      return analysis.replace(/\n/g, '<br>')
    }
  }
  
  // 如果是对象，格式化为 HTML
  if (typeof analysis === 'object') {
    let html = ''
    
    // 处理 summary
    if (analysis.summary) {
      html += `<p><strong>总结：</strong>${analysis.summary.replace(/\n/g, '<br>')}</p>`
    }
    
    // 处理 individual_analyses
    if (analysis.individual_analyses && Array.isArray(analysis.individual_analyses)) {
      html += '<h5>详细分析：</h5><ul>'
      analysis.individual_analyses.forEach(item => {
        if (item.summary) {
          html += `<li>${item.summary.replace(/\n/g, '<br>')}</li>`
        }
      })
      html += '</ul>'
    }
    
    return html
  }
  
  return String(analysis).replace(/\n/g, '<br>')
}

// 加载群组和消息数据
async function loadData() {
  isLoading.value = true
  console.info('========== 开始加载班级管理数据 ==========')
  
  try {
    // 获取群组列表（如果后端未实现，不会报错）
    console.info('请求群组列表...')
    const groupData = await getGroupList()
    console.info('群组列表响应:', groupData)
    groups.value = Array.isArray(groupData) ? groupData : []
    console.info(`加载了 ${groups.value.length} 个群组`)
    
    // 如果有群组，自动选择第一个群组并加载共享笔记
    if (groups.value.length > 0 && !selectedGroupForNotes.value) {
      selectedGroupForNotes.value = groups.value[0].uid
      sharedNotesLoading.value = true
      try {
        const notes = await getSharedNotes(selectedGroupForNotes.value)
        sharedNotes.value = notes || []
      } catch (error) {
        console.error('加载默认群组共享笔记失败:', error)
      } finally {
        sharedNotesLoading.value = false
      }
    }
    
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
  console.info('班级管理页面已加载')
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
}

.refresh-btn:hover:not(:disabled) {
  background-color: var(--bg-tertiary, #161b22);
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
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 群组选择器 */
.group-select {
  padding: 8px 12px;
  background-color: var(--bg-secondary, #0d0d0d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 6px;
  color: var(--text-primary, #f0f6fc);
  font-size: 14px;
  cursor: pointer;
  outline: none;
}

.group-select:focus {
  border-color: var(--accent-blue, #58a6ff);
}

.group-select option {
  background-color: var(--bg-primary, #0d1117);
  color: var(--text-primary, #f0f6fc);
}

/* 共享笔记区域 */
.shared-notes-section {
  margin-top: 30px;
  padding: 20px;
  background-color: var(--bg-secondary, #0d0d0d);
  border-radius: 8px;
  border: 1px solid var(--border-color, #30363d);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.section-title {
  font-size: 20px;
  margin: 0;
  color: var(--text-primary, #f0f6fc);
}

.shared-notes-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.shared-note-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  background-color: var(--bg-primary, #0d1117);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.shared-note-item:hover {
  border-color: var(--accent-blue, #58a6ff);
  background-color: var(--bg-secondary, #161b22);
  transform: translateX(4px);
}

.note-info {
  flex: 1;
}

.note-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.note-type {
  padding: 4px 8px;
  background-color: var(--accent-blue, rgba(88, 166, 255, 0.2));
  color: var(--accent-blue, #58a6ff);
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.note-type.meeting {
  background-color: rgba(139, 92, 246, 0.2);
  color: #a78bfa;
}

.note-shared-by {
  font-size: 13px;
  color: var(--text-muted, #8b949e);
}

.note-details {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.note-title {
  font-size: 15px;
  color: var(--text-primary, #f0f6fc);
  margin: 0;
}

.note-time {
  font-size: 12px;
  color: var(--text-muted, #8b949e);
}

.shared-note-item i {
  font-size: 24px;
  color: var(--text-muted, #8b949e);
}

/* 笔记详情弹窗 */
.note-detail-modal {
  max-width: 800px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
}

.note-detail-body {
  max-height: 60vh;
  overflow-y: auto;
}

.note-meta-info {
  margin-bottom: 20px;
  padding: 12px;
  background-color: var(--bg-secondary, #0d0d0d);
  border-radius: 6px;
}

.note-meta-info p {
  margin: 8px 0;
  font-size: 14px;
  color: var(--text-secondary, #c9d1d9);
}

.note-content-section,
.meeting-content-section,
.ai-section {
  margin-top: 20px;
}

.note-content-section h4,
.meeting-content-section h4,
.ai-section h4 {
  font-size: 16px;
  margin-bottom: 12px;
  color: var(--text-primary, #f0f6fc);
  display: flex;
  align-items: center;
  gap: 8px;
}

.note-content,
.meeting-note-content {
  padding: 12px;
  background-color: var(--bg-secondary, #0d0d0d);
  border-radius: 6px;
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-secondary, #c9d1d9);
  white-space: pre-wrap;
  word-break: break-word;
}

.meeting-note-item {
  margin-bottom: 16px;
  padding: 12px;
  background-color: var(--bg-secondary, #0d0d0d);
  border-radius: 6px;
}

.meeting-note-time {
  font-size: 12px;
  color: var(--text-muted, #8b949e);
  margin-bottom: 8px;
}

.ai-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--accent-blue, #58a6ff);
  padding: 12px;
  background-color: var(--bg-secondary, #0d0d0d);
  border-radius: 6px;
}

.ai-keywords {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.keyword-tag {
  padding: 6px 12px;
  background-color: rgba(139, 92, 246, 0.2);
  color: #a78bfa;
  border-radius: 16px;
  font-size: 13px;
}

.ai-analysis-content {
  padding: 16px;
  background-color: var(--bg-secondary, #0d0d0d);
  border-radius: 6px;
  font-size: 14px;
  line-height: 1.8;
  color: var(--text-secondary, #c9d1d9);
}

.ai-analysis-content h5 {
  font-size: 15px;
  margin: 12px 0 8px;
  color: var(--text-primary, #f0f6fc);
}

.ai-analysis-content ul {
  margin-left: 20px;
}

.ai-analysis-content li {
  margin-bottom: 8px;
}

.ai-analysis-section {
  margin-bottom: 20px;
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

.reject-btn {
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  color: var(--danger-btn-text, #f85149);
  background-color: transparent;
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  margin-left: 8px;
}

.reject-btn:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, #ffffff);
  border-color: var(--danger-btn-hover-border, #f85149);
}

.action-buttons {
  display: flex;
  gap: 8px;
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

/* 学生互动数据区域 */
.interactions-section {
  border-top: 1px solid var(--border-color, #30363d);
  padding-top: 16px;
  margin-top: 16px;
}

.interactions-section h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #f0f6fc);
  margin-bottom: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.interaction-stats {
  margin-bottom: 16px;
}

.stat-item {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin-bottom: 8px;
}

.stat-item.read-record {
        padding: 10px 12px;
        background-color: var(--bg-tertiary, rgba(240, 246, 252, 0.08));
        border-radius: 6px;
      }
      
      .stat-item.full-text {
        padding: 10px 12px;
        background-color: var(--bg-tertiary, rgba(240, 246, 252, 0.08));
        border-radius: 6px;
      }

.stat-label {
  font-weight: 500;
  color: var(--text-secondary, #c9d1d9);
  min-width: 80px;
}

.stat-value {
  color: var(--text-muted, #8b949e);
  font-size: 14px;
}

.stat-value.has-data {
  color: #a78bfa;
  font-weight: 500;
}

.block-interactions {
  margin-top: 12px;
}

.block-interaction-item {
  margin-bottom: 12px;
  padding: 12px;
  background-color: var(--bg-tertiary, rgba(240, 246, 252, 0.08));
  border-radius: 8px;
  border-left: 3px solid var(--border-color, #30363d);
}

.block-header {
  margin-bottom: 8px;
}

.block-index {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary, #c9d1d9);
}

.block-stats {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.no-interactions {
  text-align: center;
  padding: 20px;
  color: var(--text-muted, #8b949e);
  font-size: 14px;
}
</style>
