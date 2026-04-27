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
  <div class="notes-container">
    <div class="notes-header">
      <div class="header-left">
        <h1 class="page-title">
          <i :class="currentTab === 'notes' ? 'ri-sticky-note-line' : 'ri-team-line'" class="page-title-icon"></i>
          {{ currentTab === 'notes' ? '笔记' : '课堂记录' }}
        </h1>
        <div class="tab-switch">
          <button 
            class="tab-btn" 
            :class="{ active: currentTab === 'notes' }"
            @click="switchTab('notes')"
          >
            <i class="ri-sticky-note-line"></i>
            笔记
          </button>
          <button 
            class="tab-btn" 
            :class="{ active: currentTab === 'meetings' }"
            @click="switchTab('meetings')"
          >
            <i class="ri-team-line"></i>
            课堂记录
          </button>
        </div>
      </div>
      <div class="search-wrapper">
        <i class="ri-search-line search-icon"></i>
        <input
          v-model="searchKeyword"
          type="text"
          class="search-input"
          :placeholder="currentTab === 'notes' ? '搜索笔记标题...' : '搜索课堂记录...'"
          @input="onSearchInput"
        />
        <button v-if="searchKeyword" class="search-clear-btn" @click="clearSearch">
          <i class="ri-close-line"></i>
        </button>
      </div>
      <div class="header-actions">
        <button class="refresh-btn" @click="refreshNotes">
          <i class="ri-refresh-line"></i>
          刷新
        </button>
        <button v-if="currentTab === 'notes'" class="add-btn" @click="createAndOpenNote">
          <i class="ri-add-line"></i>
          新建笔记
        </button>
        <div v-if="currentTab === 'notes'" class="dropdown-wrapper" @mouseenter="showImportExportMenu = true" @mouseleave="showImportExportMenu = false">
          <button class="action-btn">
            <i class="ri-upload-download-line"></i>
            导入/导出
            <i class="ri-arrow-down-s-line"></i>
          </button>
          <Transition name="dropdown">
            <div v-show="showImportExportMenu" class="dropdown-menu">
              <button class="dropdown-item" @click="importNotes">
                <i class="ri-upload-line"></i>
                导入笔记
              </button>
              <button class="dropdown-item" @click="exportNotes">
                <i class="ri-download-line"></i>
                导出笔记
              </button>
            </div>
          </Transition>
        </div>
      </div>
    </div>

    <div class="notes-content">
      <!-- 骨架屏：加载时显示灰色占位块 -->
      <div v-if="isLoading" class="skeleton-grid">
        <div v-for="i in 12" :key="i" class="skeleton-card">
          <div class="skeleton-title"></div>
          <div class="skeleton-preview"></div>
          <div class="skeleton-meta">
            <div class="skeleton-date"></div>
            <div class="skeleton-more"></div>
          </div>
        </div>
      </div>
      
      <!-- 网络错误提示 -->
      <div v-else-if="hasError" class="error-state">
        <AlertCircle :size="48" class="error-icon" />
        <p class="error-message">网络连接错误</p>
        <p class="error-desc">请检查网络连接是否正常</p>
        <button class="retry-btn" @click="loadNotes">
          <i class="ri-refresh-line"></i>
          重试
        </button>
      </div>
      
      <!-- 笔记列表 -->
      <template v-else-if="currentTab === 'notes'">
        <div v-if="filteredNotes.length === 0 && notes.length > 0" class="empty-state">
          <i class="ri-search-line empty-icon" style="font-size: 64px;"></i>
          <p class="empty-message">未找到匹配的笔记</p>
          <p class="empty-desc">尝试其他关键词</p>
        </div>

        <div v-else-if="notes.length === 0" class="empty-state">
          <FileText :size="48" class="empty-icon" />
          <p class="empty-message">还没有笔记</p>
          <p class="empty-desc">点击上方按钮创建您的第一个笔记</p>
        </div>

        <div v-else>
          <div v-if="pageLoading" class="loading-overlay">
            <div class="loading-spinner"></div>
            <p>正在加载...</p>
          </div>
          <div v-else class="notes-grid">
            <div
              v-for="note in filteredNotes"
              :key="note.uuid"
              class="note-card"
              :class="{ active: false }"
              @click="selectNote(note)"
            >
              <div class="note-title-wrapper">
                <input
                  v-if="editingCardNote === note.uuid"
                  ref="cardTitleInput"
                  v-model="note.title"
                  class="card-title-edit-input"
                  @blur="saveCardTitleEdit(note)"
                  @keyup.enter="saveCardTitleEdit(note)"
                  @keyup.escape="cancelCardTitleEdit"
                  @click.stop
                />
                <span 
                  v-else 
                  class="note-title"
                  @dblclick.stop="startCardTitleEdit(note)"
                  title="双击编辑标题"
                >{{ note.title }}</span>
              </div>
              <div class="note-preview">{{ (note.content || '').substring(0, 50) }}...</div>
              <div class="note-meta">
                <span class="note-date">{{ formatDate(note.updatedAt) }}</span>
                <div class="more-wrapper">
                  <button class="more-btn" @click.stop="openMoreMenu(note, $event)">
                    <i class="ri-more-fill"></i>
                  </button>
                </div>
              </div>
            </div>
          </div>
          
          <div v-if="totalPages > 1" class="pagination">
            <button class="page-btn" :disabled="currentPage === 1" @click="prevPage">
              <i class="ri-arrow-left-s-line"></i>
            </button>
            <div class="page-numbers">
              <button
                v-for="page in totalPages"
                :key="page"
                class="page-num"
                :class="{ active: page === currentPage }"
                @click="goToPage(page)"
              >
                {{ page }}
              </button>
            </div>
            <button class="page-btn" :disabled="currentPage === totalPages" @click="nextPage">
              <i class="ri-arrow-right-s-line"></i>
            </button>
          </div>
        </div>
      </template>

      <!-- 会议记录列表 -->
      <template v-else-if="currentTab === 'meetings'">
        <div v-if="filteredMeetings.length === 0 && meetings.length > 0" class="empty-state">
          <i class="ri-search-line empty-icon" style="font-size: 64px;"></i>
          <p class="empty-message">未找到匹配的课堂记录</p>
          <p class="empty-desc">尝试其他关键词</p>
        </div>

        <div v-else-if="meetings.length === 0" class="empty-state">
          <i class="ri-team-line empty-icon" style="font-size: 64px;"></i>
          <p class="empty-message">还没有课堂记录</p>
          <p class="empty-desc">课堂中创建的笔记将自动保存为课堂记录</p>
        </div>

        <div v-else>
          <div v-if="pageLoading" class="loading-overlay">
            <div class="loading-spinner"></div>
            <p>正在加载...</p>
          </div>
          <div v-else class="notes-grid">
            <div
              v-for="meeting in filteredMeetings"
              :key="meeting.meeting_uuid"
              class="note-card meeting-card"
              @click="selectMeeting(meeting)"
            >
              <div class="note-title-wrapper">
                <span class="note-title">{{ meeting.title }}</span>
              </div>
              <div class="meeting-stats">
                <span class="stat-item">
                  <i class="ri-sticky-note-line"></i>
                  {{ meeting.note_count }} 条笔记
                </span>
                <span class="stat-item">
                  <i class="ri-image-line"></i>
                  {{ meeting.screenshot_count }} 张截图
                </span>
              </div>
              <div class="meeting-time">
                <span class="time-item">
                  <i class="ri-time-line"></i>
                  {{ formatMeetingTime(meeting.start_time) }}
                  <span class="duration">{{ formatDuration(meeting.start_time, meeting.end_time) }}</span>
                </span>
              </div>
              <div v-if="meeting.key_words && meeting.key_words.length > 0" class="key-words-row">
                <i class="ri-tag-line"></i>
                <span class="key-words-text">{{ meeting.key_words.slice(0, 3).join(', ') }}</span>
              </div>
              <div class="meeting-actions">
                <button class="share-meeting-btn" @click.stop="openMeetingShareModal(meeting)" title="分享到群组">
                  <i class="ri-share-line"></i>
                  <span>分享</span>
                </button>
              </div>
            </div>
          </div>
          
          <div v-if="meetingTotalPages > 1" class="pagination">
            <button class="page-btn" :disabled="meetingCurrentPage === 1" @click="prevMeetingPage">
              <i class="ri-arrow-left-s-line"></i>
            </button>
            <div class="page-numbers">
              <button
                v-for="page in meetingTotalPages"
                :key="page"
                class="page-num"
                :class="{ active: page === meetingCurrentPage }"
                @click="goToMeetingPage(page)"
              >
                {{ page }}
              </button>
            </div>
            <button class="page-btn" :disabled="meetingCurrentPage === meetingTotalPages" @click="nextMeetingPage">
              <i class="ri-arrow-right-s-line"></i>
            </button>
          </div>
        </div>
      </template>
    </div>

    <Transition name="modal">
      <div v-if="showAddModal" class="modal-overlay" @click="showAddModal = false">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-sticky-note-line"></i> 新建笔记</h3>
            <button class="close-btn" @click="showAddModal = false">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body">
            <div class="input-wrapper">
              <input
                v-model="newNoteTitle"
                class="title-input"
                placeholder="请输入笔记名称"
                @keyup.enter="addNote"
              >
            </div>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="showAddModal = false">取消</button>
            <button class="confirm-btn" @click="addNote">创建</button>
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="modal">
      <div v-if="showDeleteModal" class="modal-overlay" @click="cancelDelete">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-delete-bin-line"></i> 删除笔记</h3>
            <button class="close-btn" @click="cancelDelete">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body delete-modal-body">
            <p>确定要删除这个笔记吗？此操作<span class="danger-text">不可恢复</span>。</p>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="cancelDelete">取消</button>
            <button class="delete-confirm-btn" @click="confirmDelete">删除</button>
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="modal">
      <div v-if="showMoreMenu" class="modal-overlay" @click="closeMoreMenu">
        <div class="more-menu-content" @click.stop>
          <div class="more-menu-header">
            <input
              v-if="editingNote === moreMenuNote?.uuid"
              ref="titleInput"
              v-model="moreMenuNote.title"
              class="more-menu-title-input"
              @blur="saveTitleEdit"
              @keyup.enter="saveTitleEdit"
              @keyup.escape="cancelTitleEdit"
            />
            <span v-else class="more-menu-title">{{ moreMenuNote?.title }}</span>
            <span class="more-menu-date">{{ moreMenuNote ? formatDate(moreMenuNote.updatedAt) : '' }}</span>
          </div>
          <div class="more-menu-actions">
            <button class="more-menu-item" @click="startTitleEdit">
              <i class="ri-edit-line"></i>
              <span>重命名</span>
            </button>
            <button class="more-menu-item" @click="openShareModal">
              <i class="ri-share-line"></i>
              <span>分享到群组</span>
            </button>
            <button class="more-menu-item danger" :class="{ 'confirming': isConfirmingDelete }" @click="handleDeleteClick">
              <i :class="isConfirmingDelete ? 'ri-question-line' : 'ri-delete-bin-line'"></i>
              <span>{{ isConfirmingDelete ? '确认删除' : '删除' }}</span>
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="modal">
      <div v-if="showShareModal" class="modal-overlay" @click="closeShareModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-share-line"></i> 分享到群组</h3>
            <button class="close-btn" @click="closeShareModal">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body">
            <div v-if="shareGroupsLoading" class="loading-state">
              <div class="loading-spinner"></div>
              <p>正在加载群组列表...</p>
            </div>
            <div v-else-if="shareGroups.length === 0" class="empty-state">
              <i class="ri-team-line" style="font-size: 48px; color: var(--text-tertiary);"></i>
              <p class="empty-message">您还没有加入任何群组</p>
              <p class="empty-desc">请先加入群组后再进行分享</p>
            </div>
            <div v-else class="share-groups-list">
              <div
                v-for="group in shareGroups"
                :key="group.uid"
                class="share-group-item"
                :class="{ active: selectedShareGroup === group.uid }"
                @click="selectedShareGroup = group.uid"
              >
                <div class="share-group-info">
                  <span class="share-group-name">{{ group.name }}</span>
                  <span class="share-group-uid">UID: {{ group.uid }}</span>
                </div>
                <i v-if="selectedShareGroup === group.uid" class="ri-checkbox-circle-fill" style="color: var(--color-primary); font-size: 24px;"></i>
                <i v-else class="ri-checkbox-blank-circle-line" style="color: var(--text-tertiary); font-size: 24px;"></i>
              </div>
            </div>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="closeShareModal">取消</button>
            <button 
              class="confirm-btn" 
              @click="confirmShareToGroup"
              :disabled="!selectedShareGroup || shareSubmitting"
            >
              <i v-if="shareSubmitting" class="ri-loader-4-line ri-spin"></i>
              <span v-else>分享</span>
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <Transition name="modal">
      <div v-if="showMeetingShareModal" class="modal-overlay" @click="closeMeetingShareModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3><i class="ri-share-line"></i> 分享课堂记录到群组</h3>
            <button class="close-btn" @click="closeMeetingShareModal">
              <i class="ri-close-line"></i>
            </button>
          </div>
          <div class="modal-body">
            <div v-if="shareGroupsLoading" class="loading-state">
              <div class="loading-spinner"></div>
              <p>正在加载群组列表...</p>
            </div>
            <div v-else-if="shareGroups.length === 0" class="empty-state">
              <i class="ri-team-line" style="font-size: 48px; color: var(--text-tertiary);"></i>
              <p class="empty-message">您还没有加入任何群组</p>
              <p class="empty-desc">请先加入群组后再进行分享</p>
            </div>
            <div v-else class="share-groups-list">
              <div
                v-for="group in shareGroups"
                :key="group.uid"
                class="share-group-item"
                :class="{ active: selectedShareGroup === group.uid }"
                @click="selectedShareGroup = group.uid"
              >
                <div class="share-group-info">
                  <span class="share-group-name">{{ group.name }}</span>
                  <span class="share-group-uid">UID: {{ group.uid }}</span>
                </div>
                <i v-if="selectedShareGroup === group.uid" class="ri-checkbox-circle-fill" style="color: var(--color-primary); font-size: 24px;"></i>
                <i v-else class="ri-checkbox-blank-circle-line" style="color: var(--text-tertiary); font-size: 24px;"></i>
              </div>
            </div>
          </div>
          <div class="modal-footer">
            <button class="cancel-btn" @click="closeMeetingShareModal">取消</button>
            <button 
              class="confirm-btn" 
              @click="confirmShareMeetingToGroup"
              :disabled="!selectedShareGroup || shareSubmitting"
            >
              <i v-if="shareSubmitting" class="ri-loader-4-line ri-spin"></i>
              <span v-else>分享</span>
            </button>
          </div>
        </div>
      </div>
    </Transition>

  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue'
import axios from 'axios'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { Window } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { showToast } from '../components/layout/showToast.js'
import { getBackendUrl } from '../config/backend.js'
import { FileText, AlertCircle } from 'lucide-vue-next'
import { getGroupList, shareNoteToGroup } from '../components/data/group.js'

const timeOut = 10000 // 10 秒超时

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

async function checkMeetingStatus() {
  try {
    const authHeader = await getAuthHeader()
    const response = await axios.get(getBackendUrl() + '/meeting/status', {
      headers: authHeader,
      timeout: 5000
    })
    return response.data.in_meeting === true
  } catch (error) {
    console.error('获取课堂状态失败:', error)
    return false
  }
}

async function sendNoteToBackend(title, content) {
  try {
    const authHeader = await getAuthHeader()
    const response = await axios.post(getBackendUrl() + '/meeting/note/add', {
      title: title,
      content: content
    }, {
      headers: authHeader,
      timeout: 10000
    })
    return response.data
  } catch (error) {
    console.error('发送会议笔记失败:', error)
    throw error
  }
}

async function apiRequest(url, data = {}) {
  const authHeader = await getAuthHeader()
  const response = await axios.post(getBackendUrl() + url, data, {
    headers: { ...authHeader, 'Content-Type': 'application/json' },
    timeout: timeOut
  })
  return response.data
}
const notes = ref([])
const meetings = ref([])
const showAddModal = ref(false)
const newNoteTitle = ref('')
const showDeleteModal = ref(false)
const noteToDelete = ref(null)
const showMoreMenu = ref(false)
const moreMenuNote = ref(null)
const editingNote = ref(null)
const editingCardNote = ref(null)
const titleInput = ref(null)
const cardTitleInput = ref(null)
const showImportExportMenu = ref(false)
const isConfirmingDelete = ref(false)
const refreshBtnSpinning = ref(false)

// 分享相关
const showShareModal = ref(false)
const showMeetingShareModal = ref(false)
const shareGroups = ref([])
const shareGroupsLoading = ref(false)
const selectedShareGroup = ref(null)
const shareSubmitting = ref(false)
const selectedMeetingToShare = ref(null)

// 当前标签页：'notes' 或 'meetings'
const currentTab = ref('meetings')

// 搜索关键词
const searchKeyword = ref('')

// 过滤后的笔记列表（按标题搜索）
const filteredNotes = computed(() => {
  if (!searchKeyword.value.trim()) return notes.value
  const keyword = searchKeyword.value.toLowerCase().trim()
  return notes.value.filter(note =>
    note.title?.toLowerCase().includes(keyword)
  )
})

// 过滤后的课堂记录列表（按标题和关键词搜索）
const filteredMeetings = computed(() => {
  if (!searchKeyword.value.trim()) return meetings.value
  const keyword = searchKeyword.value.toLowerCase().trim()
  return meetings.value.filter(meeting => {
    const titleMatch = meeting.title?.toLowerCase().includes(keyword)
    const keyWordsMatch = meeting.key_words?.some(kw =>
      kw.toLowerCase().includes(keyword)
    )
    return titleMatch || keyWordsMatch
  })
})

const pageSize = 9
const currentPage = ref(1)
const isLoading = ref(false)
const pageLoading = ref(false)
const hasError = ref(false)
const totalPages = ref(1)
const isPageChanging = ref(false)

// 会议记录分页
const meetingPageSize = 9
const meetingCurrentPage = ref(1)
const meetingTotalPages = ref(1)
const isMeetingPageChanging = ref(false)

// 后端已经返回了当前页的数据，直接使用 notes.value 即可
const currentPageNotes = computed(() => {
  return notes.value
})

const loadedNotes = ref({})
let unlistenNoteSaved = null
let unlistenCreateNewNote = null
let unlistenRefreshNotes = null

onMounted(async () => {
  loadMeetings()

  // 监听笔记保存事件，刷新列表 (不显示 toast)
  unlistenNoteSaved = await listen('note-saved', () => {
    loadNotes(false)
  })

  // 监听蓝牙新建笔记命令，调用 createAndOpenNote 方法
  const { listen } = await import('@tauri-apps/api/event')
  unlistenCreateNewNote = await listen('create-new-note', () => {
    createAndOpenNote()
  })

  // 监听刷新笔记列表事件（来自编辑器窗口，不显示 toast）
  unlistenRefreshNotes = await listen('refresh-notes', () => {
    loadNotes(false)
  })
})

// 切换标签页
function switchTab(tab) {
  if (currentTab.value === tab) return
  currentTab.value = tab
  searchKeyword.value = ''
  currentPage.value = 1
  meetingCurrentPage.value = 1
  if (tab === 'meetings') {
    loadMeetings()
  } else {
    loadNotes()
  }
}

function onSearchInput() {}

function clearSearch() {
  searchKeyword.value = ''
}

// 加载会议记录列表
async function loadMeetings(showSuccessToast = false) {
  isLoading.value = true
  hasError.value = false
  try {
    const response = await axios.get(getBackendUrl() + '/meeting/history/query_by_page', {
      params: {
        page: meetingCurrentPage.value,
        page_size: meetingPageSize
      },
      headers: await getAuthHeader(),
      timeout: timeOut
    })

    const data = response.data
    if (data && data.success) {
      meetings.value = data.meetings || []
      meetingTotalPages.value = data.total_page || 1
      
      // 获取每个会议的关键词
      await loadMeetingKeyWords()
    } else {
      meetings.value = []
      meetingTotalPages.value = 1
    }

    if (showSuccessToast) {
      showToast('刷新成功', '#10b981')
    }
  } catch (e) {
    console.error('加载会议记录失败:', e)
    hasError.value = true
    showToast('加载会议记录失败：' + (e.message || '网络错误'), '#ef4444')
    meetings.value = []
  }
  isLoading.value = false
  isMeetingPageChanging.value = true
  setTimeout(() => {
    isMeetingPageChanging.value = false
  }, 500)
}

// 获取课堂关键词
async function loadMeetingKeyWords() {
  try {
    for (const meeting of meetings.value) {
      try {
        const response = await axios.post(
          getBackendUrl() + '/meeting/ai_key_words',
          {
            meeting_uuid: meeting.meeting_uuid
          },
          {
            headers: await getAuthHeader(),
            timeout: 10000
          }
        )
        
        if (response.data && response.data.status === 'success') {
          meeting.key_words = response.data.key_words || []
        } else {
          meeting.key_words = []
        }
      } catch (error) {
        console.error(`获取会议 ${meeting.meeting_uuid} 的关键词失败:`, error)
        meeting.key_words = []
      }
    }
  } catch (e) {
    console.error('获取会议关键词失败:', e)
  }
}

// 会议记录分页
function goToMeetingPage(page) {
  if (page < 1 || page > meetingTotalPages.value) return
  if (isMeetingPageChanging.value) return
  meetingCurrentPage.value = page
  loadMeetings()
}

function prevMeetingPage() {
  goToMeetingPage(meetingCurrentPage.value - 1)
}

function nextMeetingPage() {
  goToMeetingPage(meetingCurrentPage.value + 1)
}

// 格式化课堂时间
function formatMeetingTime(timeStr) {
  if (!timeStr) return ''
  const date = new Date(timeStr)
  if (isNaN(date.getTime())) return ''
  return `${date.getMonth() + 1}月${date.getDate()}日 ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
}

// 格式化会议时长
function formatDuration(startTime, endTime) {
  if (!startTime || !endTime) return ''
  const start = new Date(startTime)
  const end = new Date(endTime)
  if (isNaN(start.getTime()) || isNaN(end.getTime())) return ''
  const durationMs = end - start
  const minutes = Math.floor(durationMs / 60000)
  const hours = Math.floor(minutes / 60)
  const remainingMinutes = minutes % 60
  if (hours > 0) {
    return `${hours}小时${remainingMinutes}分钟`
  }
  return `${minutes}分钟`
}

// 选择会议记录
async function selectMeeting(meeting) {
  try {
    const response = await axios.get(getBackendUrl() + '/meeting/history/query_by_uuid', {
      params: {
        meeting_uuid: meeting.meeting_uuid
      },
      headers: await getAuthHeader(),
      timeout: timeOut
    })

    const data = response.data
    if (data && data.success && data.meeting) {
      openMeetingEditorWindow(data.meeting)
    } else {
      showToast('获取课堂记录详情失败', '#ef4444')
    }
  } catch (e) {
    console.error('获取课堂记录详情失败:', e)
    showToast('获取课堂记录详情失败：' + (e.message || '网络错误'), '#ef4444')
  }
}

// 打开会议记录编辑窗口
async function openMeetingEditorWindow(meeting) {
  const windowLabel = `meeting-editor-${meeting.meeting_uuid}`
  const url = `/meeting-editor?uuid=${meeting.meeting_uuid}&title=${encodeURIComponent(meeting.title)}`

  const webview = new WebviewWindow(windowLabel, {
    url: url,
    title: meeting.title || '课堂记录',
    width: 900,
    height: 600,
    minWidth: 400,
    minHeight: 300,
    center: true,
    decorations: false,
    resizable: true
  })

  webview.once('tauri://created', async () => {
    console.log('会议记录编辑窗口创建成功:', windowLabel)
    await new Promise(resolve => setTimeout(resolve, 300))

    try {
      await webview.emit('load-meeting-content', {
        content: meeting.content || '',
        screenshots: meeting.screenshots || []
      })
    } catch (e) {
      console.error('发送会议记录内容失败:', e)
    }
  })

  webview.once('tauri://error', async (e) => {
    console.error('会议记录编辑窗口创建失败:', e)
    const errorMsg = e?.payload || ''
    if (typeof errorMsg === 'string' && errorMsg.includes('already exists')) {
      // 窗口已存在，获取并置顶
      try {
        const existingWindow = await Window.getByLabel(windowLabel)
        if (existingWindow) {
          await existingWindow.setFocus()
          await existingWindow.setAlwaysOnTop(true)
          setTimeout(async () => {
            await existingWindow.setAlwaysOnTop(false)
          }, 100)
          console.log('课堂记录窗口已置顶')
        }
      } catch (err) {
        console.error('设置窗口置顶失败:', err)
      }
    } else {
      showToast('打开编辑窗口失败', '#ef4444')
    }
  })
}

onUnmounted(() => {
  if (unlistenNoteSaved) {
    unlistenNoteSaved()
  }
  if (unlistenCreateNewNote) {
    unlistenCreateNewNote()
  }
  if (unlistenRefreshNotes) {
    unlistenRefreshNotes()
  }
})

async function loadNotes(showSuccessToast = false) {
  isLoading.value = true
  hasError.value = false
  try {
    const response = await axios.get(getBackendUrl() + '/note/query_by_page', {
      params: {
        page: currentPage.value,
        page_size: pageSize
      },
      headers: await getAuthHeader(),
      timeout: timeOut
    })
    
    const data = response.data
    let notesList = data
    if (data && typeof data === 'object' && !Array.isArray(data)) {
      notesList = data.data || data.notes || data.result || []
    }
    notes.value = Array.isArray(notesList) ? notesList : []
    
    // 更新总页数（后端返回 total_page 字段）
    if (data && typeof data === 'object' && 'total_page' in data) {
      totalPages.value = data.total_page || 1
    }
    
    if (notes.value.length === 0) {
      await createDefaultNote()
    }
    
    // 只在手动刷新时显示成功提示
    if (showSuccessToast) {
      showToast('刷新成功', '#10b981')
    }
  } catch (e) {
    console.error('加载笔记失败:', e)
    hasError.value = true
    showToast('加载笔记失败：' + (e.message || '网络错误'), '#ef4444')
    notes.value = []
  }
  isLoading.value = false
  isPageChanging.value = true
  setTimeout(() => {
    isPageChanging.value = false
  }, 500)
  loadCurrentPageNotes()
}

async function createDefaultNote() {
  const uuid = crypto.randomUUID()
  try {
    await apiRequest('/note/add', { uuid, title: 'Hello' })
    notes.value = [{
      uuid,
      title: 'Hello',
      content: '你好！这是你的第一个笔记。\n\n开始记录你的想法吧！',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString()
    }]
  } catch (e) {
    console.error('创建默认笔记失败:', e)
  }
}

function loadCurrentPageNotes() {
  pageLoading.value = true
  loadedNotes.value = {}
  const pageNotes = currentPageNotes.value
  for (let i = 0; i < pageNotes.length; i++) {
    loadedNotes.value[pageNotes[i].uuid] = true
  }
  pageLoading.value = false
  setTimeout(() => {
    pageLoading.value = false
  }, 50)
}

function goToPage(page) {
  if (page < 1 || page > totalPages.value) return
  if (isPageChanging.value) return
  currentPage.value = page
  loadNotes()
}

function prevPage() {
  goToPage(currentPage.value - 1)
}

function nextPage() {
  goToPage(currentPage.value + 1)
}

// 获取单个笔记内容
async function getNoteContent(uuid) {
  try {
    const data = await apiRequest('/note/query_by_uuid', { uuid })
    let noteData = data
    if (data && typeof data === 'object' && !Array.isArray(data)) {
      noteData = data.data || data.note || data.result || {}
    }
    return noteData
  } catch (e) {
    console.error('获取笔记内容失败:', e)
    showToast('获取笔记内容失败: ' + (e.message || '网络错误'), '#ef4444')
    return null
  }
}

// 打开笔记编辑窗口
async function openNoteEditorWindow(note) {
  const windowLabel = `note-editor-${note.uuid}`
  let url = `/note-editor?uuid=${note.uuid}&title=${encodeURIComponent(note.title)}`
  if (note.isMeetingNote) {
    url += '&isMeetingNote=true'
  }
  
  const webview = new WebviewWindow(windowLabel, {
    url: url,
    title: note.title || '编辑笔记',
    width: 900,
    height: 600,
    minWidth: 400,
    minHeight: 300,
    center: true,
    decorations: false,
    resizable: true
  })
  
  webview.once('tauri://created', async () => {
    console.log('笔记编辑窗口创建成功:', windowLabel)
    await new Promise(resolve => setTimeout(resolve, 300))
    
    try {
      if (note.isMeetingNote) {
        await webview.emit('load-note-content', { 
          content: note.content || '' 
        })
      } else {
        const noteData = await getNoteContent(note.uuid)
        if (noteData) {
          await webview.emit('load-note-content', { 
            content: noteData.content || '' 
          })
        }
      }
    } catch (e) {
      console.error('获取或发送笔记内容失败:', e)
    }
  })

  webview.once('tauri://error', async (e) => {
    console.error('笔记编辑窗口创建失败:', e)
    const errorMsg = e?.payload || ''
    if (typeof errorMsg === 'string' && errorMsg.includes('already exists')) {
      // 窗口已存在，获取并置顶
      try {
        const existingWindow = await Window.getByLabel(windowLabel)
        if (existingWindow) {
          await existingWindow.setFocus()
          await existingWindow.setAlwaysOnTop(true)
          setTimeout(async () => {
            await existingWindow.setAlwaysOnTop(false)
          }, 100)
          console.log('笔记窗口已置顶')
        }
      } catch (err) {
        console.error('设置窗口置顶失败:', err)
      }
    } else {
      showToast('打开编辑窗口失败', '#ef4444')
    }
  })

  const unlistenEditorClosed = await webview.listen('note-editor-closed', async () => {
    console.log('收到笔记编辑窗口关闭事件:', windowLabel)
    await refreshNotes()
    if (unlistenEditorClosed) {
      unlistenEditorClosed()
    }
  })
}

async function createAndOpenNote() {
  const uuid = crypto.randomUUID()
  const now = new Date()
  const timestamp = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}${String(now.getSeconds()).padStart(2, '0')}`
  const defaultTitle = `未命名笔记_${timestamp}`
  
  // 检查会议状态
  const meetingActive = await checkMeetingStatus()
  
  if (meetingActive) {
    // 会议进行中，打开编辑窗口但标记为会议笔记（不添加到笔记列表）
    console.log('会议进行中，打开会议笔记编辑窗口')
    const newNote = {
      uuid,
      title: defaultTitle,
      content: '',
      createdAt: now.toISOString(),
      updatedAt: now.toISOString(),
      isMeetingNote: true
    }
    openNoteEditorWindow(newNote)
    return
  }
  
  // 先在云端创建空白笔记
  try {
    await apiRequest('/note/add', { uuid, title: defaultTitle, content: '' })
  } catch (e) {
    console.error('创建云端笔记失败:', e)
    showToast('创建笔记失败: ' + (e.message || '网络错误'), '#ef4444')
    return
  }
  
  // 本地添加笔记
  const newNote = {
    uuid,
    title: defaultTitle,
    content: '',
    createdAt: now.toISOString(),
    updatedAt: now.toISOString()
  }
  notes.value.unshift(newNote)
  
  // 打开编辑窗口
  openNoteEditorWindow(newNote)
}

async function addNote() {
  if (!newNoteTitle.value.trim()) return

  const uuid = crypto.randomUUID()
  try {
    await apiRequest('/note/add', { uuid, title: newNoteTitle.value })
    const newNote = {
      uuid,
      title: newNoteTitle.value,
      content: '',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString()
    }
    notes.value.unshift(newNote)
    newNoteTitle.value = ''
    showAddModal.value = false
    
    // 创建后直接打开编辑窗口
    openNoteEditorWindow(newNote)
  } catch (e) {
    console.error('添加笔记失败:', e)
    showToast('添加笔记失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

function selectNote(note) {
  // 打开独立编辑窗口
  openNoteEditorWindow(note)
}

function deleteNote(id) {
  noteToDelete.value = id
  showDeleteModal.value = true
}

function confirmDelete() {
  if (noteToDelete.value) {
    deleteNoteApi(noteToDelete.value)
  }
  showDeleteModal.value = false
  noteToDelete.value = null
}

async function deleteNoteApi(uuid) {
  try {
    await apiRequest('/note/delete', { uuid })
    notes.value = notes.value.filter(n => n.uuid !== uuid)
  } catch (e) {
    console.error('删除笔记失败:', e)
    showToast('删除笔记失败: ' + (e.message || '网络错误'), '#ef4444')
  }
}

function cancelDelete() {
  showDeleteModal.value = false
  noteToDelete.value = null
}

function openMoreMenu(note, event) {
  moreMenuNote.value = note
  showMoreMenu.value = true
}

function closeMoreMenu() {
  showMoreMenu.value = false
  moreMenuNote.value = null
  isConfirmingDelete.value = false
}

function startTitleEdit() {
  if (moreMenuNote.value) {
    editingNote.value = moreMenuNote.value.uuid
    setTimeout(() => {
      if (titleInput.value) {
        titleInput.value.focus()
        titleInput.value.select()
      }
    }, 100)
  }
}

function saveTitleEdit() {
  if (moreMenuNote.value && moreMenuNote.value.title.trim()) {
    moreMenuNote.value.title = moreMenuNote.value.title.trim()
    moreMenuNote.value.updatedAt = new Date().toISOString()
    syncNoteToCloud(moreMenuNote.value)
  }
  editingNote.value = null
}

function cancelTitleEdit() {
  editingNote.value = null
}

function startCardTitleEdit(note) {
  editingCardNote.value = note.uuid
  setTimeout(() => {
    if (cardTitleInput.value) {
      cardTitleInput.value.focus()
      cardTitleInput.value.select()
    }
  }, 100)
}

function saveCardTitleEdit(note) {
  if (note && note.title.trim()) {
    note.title = note.title.trim()
    note.updatedAt = new Date().toISOString()
    syncNoteToCloud(note)
  }
  editingCardNote.value = null
}

function cancelCardTitleEdit() {
  editingCardNote.value = null
}

function handleDeleteClick() {
  if (isConfirmingDelete.value) {
    // 确认删除，执行删除操作
    if (moreMenuNote.value) {
      noteToDelete.value = moreMenuNote.value.uuid
      confirmDelete()
      isConfirmingDelete.value = false
      closeMoreMenu()
    }
  } else {
    // 第一次点击，进入确认状态
    isConfirmingDelete.value = true
  }
}



function formatDate(dateStr) {
  if (!dateStr) {
    return ''
  }
  const date = new Date(dateStr)
  if (isNaN(date.getTime())) {
    return ''
  }
  return `${date.getMonth() + 1}月${date.getDate()}日 ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
}

function exportNotes() {
  showImportExportMenu.value = false
  try {
    const data = JSON.stringify(notes.value, null, 2)
    const blob = new Blob([data], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `camfc-notes-${new Date().toISOString().slice(0, 10)}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
    showToast('笔记导出成功', '#10b981')
  } catch (err) {
    console.error('导出失败:', err)
    showToast('导出失败', '#ef4444')
  }
}

async function refreshNotes() {
  refreshBtnSpinning.value = true
  try {
    if (currentTab.value === 'meetings') {
      await loadMeetings(true)
    } else {
      await loadNotes(true)
    }
  } catch (e) {
    console.error('刷新失败:', e)
    showToast('刷新失败: ' + (e.message || '网络错误'), '#ef4444')
  } finally {
    refreshBtnSpinning.value = false
  }
}

function importNotes() {
  showImportExportMenu.value = false
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.json'
  input.onchange = (e) => {
    const file = e.target.files[0]
    if (!file) return
    const reader = new FileReader()
    reader.onload = (event) => {
      try {
        const importedNotes = JSON.parse(event.target.result)
        if (Array.isArray(importedNotes)) {
          notes.value = importedNotes
          currentPage.value = 1
          loadCurrentPageNotes()
          showToast(`已导入 ${importedNotes.length} 条笔记（本地导入，云端未同步）`, '#f59e0b')
        } else {
          showToast('文件格式不正确', '#ef4444')
        }
      } catch (err) {
        console.error('导入失败:', err)
        showToast('导入失败，请检查文件格式', '#ef4444')
      }
    }
    reader.readAsText(file)
  }
  input.click()
}

// 分享相关函数
async function openShareModal() {
  closeMoreMenu()
  showShareModal.value = true
  shareGroupsLoading.value = true
  shareGroups.value = []
  selectedShareGroup.value = null
  
  try {
    const groups = await getGroupList()
    shareGroups.value = groups || []
  } catch (error) {
    console.error('获取群组列表失败:', error)
    showToast('获取群组列表失败', '#ef4444')
  } finally {
    shareGroupsLoading.value = false
  }
}

function closeShareModal() {
  showShareModal.value = false
  shareGroups.value = []
  selectedShareGroup.value = null
  shareSubmitting.value = false
}

async function confirmShareToGroup() {
  if (!selectedShareGroup.value || !moreMenuNote.value) return
  
  shareSubmitting.value = true
  
  try {
    const result = await shareNoteToGroup(
      moreMenuNote.value.uuid,
      selectedShareGroup.value,
      'personal'
    )
    
    if (result && result.success) {
      showToast('分享成功', '#10b981')
      closeShareModal()
    } else {
      showToast('分享失败：' + (result?.message || '未知错误'), '#ef4444')
    }
  } catch (error) {
    console.error('分享失败:', error)
    const errorMsg = error.response?.data?.detail || error.message || '分享失败'
    showToast(errorMsg, '#ef4444')
  } finally {
    shareSubmitting.value = false
  }
}

async function openMeetingShareModal(meeting) {
  showMeetingShareModal.value = true
  selectedMeetingToShare.value = meeting
  shareGroupsLoading.value = true
  shareGroups.value = []
  selectedShareGroup.value = null
  
  try {
    const groups = await getGroupList()
    shareGroups.value = groups || []
  } catch (error) {
    console.error('获取群组列表失败:', error)
    showToast('获取群组列表失败', '#ef4444')
  } finally {
    shareGroupsLoading.value = false
  }
}

function closeMeetingShareModal() {
  showMeetingShareModal.value = false
  selectedMeetingToShare.value = null
  shareGroups.value = []
  selectedShareGroup.value = null
  shareSubmitting.value = false
}

async function confirmShareMeetingToGroup() {
  console.log('开始分享课堂记录')
  console.log('selectedShareGroup:', selectedShareGroup.value)
  console.log('selectedMeetingToShare:', selectedMeetingToShare.value)
  
  if (!selectedShareGroup.value || !selectedMeetingToShare.value) {
    console.error('缺少必要参数')
    return
  }
  
  shareSubmitting.value = true
  
  try {
    console.log('调用 shareNoteToGroup API')
    const result = await shareNoteToGroup(
      selectedMeetingToShare.value.meeting_uuid,
      selectedShareGroup.value,
      'meeting',
      selectedMeetingToShare.value.meeting_uuid
    )
    
    console.log('API 返回结果:', result)
    
    if (result && result.success) {
      showToast('分享成功', '#10b981')
      closeMeetingShareModal()
    } else {
      showToast('分享失败：' + (result?.message || '未知错误'), '#ef4444')
    }
  } catch (error) {
    console.error('分享失败:', error)
    console.error('错误详情:', error.response?.data)
    const errorMsg = error.response?.data?.detail || error.message || '分享失败'
    showToast(errorMsg, '#ef4444')
  } finally {
    shareSubmitting.value = false
  }
}
</script>

<style scoped>
.notes-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 0 30px;
  max-width: 1200px;
  margin: 0 auto;
}

.notes-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 30px 0;
  flex-shrink: 0; /* 防止被压缩 */
  position: sticky;
  top: 0;
  background-color: var(--bg-primary);
  z-index: 10;
  gap: 16px;
  flex-wrap: wrap;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 24px;
}

/* 标签切换样式 */
.tab-switch {
  display: flex;
  align-items: center;
  gap: 4px;
  background-color: var(--bg-secondary);
  padding: 4px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.tab-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tab-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.tab-btn.active {
  background-color: var(--accent-blue);
  color: white;
}

.tab-btn i {
  font-size: 16px;
}

/* 搜索框样式 */
.search-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 14px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  min-width: 220px;
  max-width: 320px;
  transition: all 0.2s ease;
}

.search-wrapper:focus-within {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.15);
}

.search-icon {
  font-size: 16px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  background: none;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-size: 14px;
  min-width: 0;
}

.search-input::placeholder {
  color: var(--text-muted);
}

.search-clear-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px;
  border-radius: 4px;
  transition: all 0.2s;
  flex-shrink: 0;
}

.search-clear-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.search-clear-btn i {
  font-size: 14px;
}

.notes-content {
  flex: 1;
  overflow-y: auto;
  padding-bottom: 30px;
}

.page-title {
  font-size: 28px;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.page-title-icon {
  font-size: 28px;
  color: var(--accent-blue, #3178c6);
}

.add-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.add-btn:hover {
  background-color: var(--accent-blue-bright, #1f6feb);
}

.add-btn i {
  font-size: 18px;
}

.refresh-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.refresh-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
  border-color: var(--accent-blue);
}

.refresh-btn i {
  font-size: 16px;
}

.refresh-btn.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.header-actions {
  display: flex;
  gap: 10px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 16px;
  background-color: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}

.action-btn i {
  font-size: 16px;
}

.dropdown-wrapper {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  min-width: 150px;
  z-index: 100;
  overflow: hidden;
}

.dropdown-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.dropdown-item:hover {
  background-color: var(--hover-bg);
}

.dropdown-item i {
  font-size: 16px;
  color: var(--text-secondary);
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.empty-state {
  text-align: center;
  padding: 80px 20px;
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  border: 1px solid var(--border-color);
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.empty-message {
  font-size: 20px;
  color: var(--text-primary);
  margin-bottom: 10px;
}

.empty-desc {
  color: var(--text-muted);
}

.loading-state {
  text-align: center;
  padding: 80px 20px;
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  border: 1px solid var(--border-color);
}

.loading-state p {
  color: var(--text-secondary);
  margin-top: 16px;
}

/* 骨架屏样式 */
.skeleton-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.skeleton-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  padding: 20px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-title {
  height: 24px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  margin-bottom: 10px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-preview {
  height: 42px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  margin-bottom: 15px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.skeleton-date {
  height: 14px;
  width: 80px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-more {
  height: 20px;
  width: 20px;
  background-color: rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

@keyframes skeleton-pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}

.error-state {
  text-align: center;
  padding: 80px 20px;
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  border: 1px solid var(--border-color);
}

.error-icon {
  font-size: 64px;
  margin-bottom: 20px;
  color: var(--danger-btn-text, #ef4444);
}

.error-message {
  font-size: 20px;
  color: var(--text-primary);
  margin-bottom: 10px;
}

.error-desc {
  color: var(--text-muted);
  margin-bottom: 20px;
}

.retry-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
  margin: 0 auto;
}

.retry-btn:hover {
  background-color: var(--accent-blue-bright, #1f6feb);
}

.retry-btn i {
  font-size: 16px;
}

.loading-overlay {
  text-align: center;
  padding: 40px 20px;
}

.loading-overlay p {
  color: var(--text-secondary);
  margin-top: 12px;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color);
  border-top-color: var(--accent-blue, #3178c6);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.pagination {
  position: fixed;
  bottom: 30px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 12px;
  padding: 20px 0;
  z-index: 50;
}

.page-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.page-btn:hover:not(:disabled) {
  background-color: var(--hover-bg);
  border-color: var(--accent-blue, #3178c6);
}

.page-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.page-btn i {
  font-size: 20px;
}

.page-numbers {
  display: flex;
  gap: 6px;
}

.page-num {
  min-width: 36px;
  height: 36px;
  padding: 0 10px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.page-num:hover {
  background-color: var(--hover-bg);
}

.page-num.active {
  background-color: var(--accent-blue, #3178c6);
  border-color: var(--accent-blue, #3178c6);
  color: white;
}

.notes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.note-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  padding: 20px;
  cursor: pointer;
  transition: all 0.2s;
}

.note-card:hover {
  border-color: var(--accent-blue);
}

.note-card.active {
  border-color: var(--accent-blue);
  box-shadow: 0 4px 12px rgba(var(--accent-blue-rgb), 0.15);
}

/* 课堂记录卡片样式 */
.meeting-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
}

.meeting-card .note-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-height: 1.4;
  word-break: break-word;
}

.meeting-stats {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  color: var(--text-secondary);
}

.stat-item i {
  font-size: 14px;
  color: var(--accent-blue);
}

.meeting-time {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: auto;
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
}

.time-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-muted);
}

.time-item i {
  font-size: 12px;
}

.duration {
  font-size: 12px;
  color: var(--accent-blue);
  background-color: rgba(var(--accent-blue-rgb), 0.1);
  padding: 2px 8px;
  border-radius: 4px;
  margin-left: 8px;
}

.key-words-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border-color);
  font-size: 12px;
  color: var(--accent-green);
}

.key-words-row i {
  font-size: 12px;
  flex-shrink: 0;
}

.key-words-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  min-width: 0;
}

.note-title-wrapper {
  margin-bottom: 10px;
  position: relative;
  min-width: 0;
}

.note-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 10px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-height: 1.4;
  word-break: break-word;
  cursor: pointer;
  transition: all 0.2s;
}

.note-title:hover {
  color: var(--accent-blue);
}

.card-title-edit-input {
  width: 100%;
  font-size: 16px;
  font-weight: 600;
  padding: 6px 10px;
  background-color: var(--bg-primary);
  border: 1px solid var(--accent-blue);
  border-radius: .375rem;
  color: var(--text-primary);
  outline: none;
  box-sizing: border-box;
}

.card-title-edit-input:focus {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.note-preview {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 15px;
  line-height: 1.5;
  min-height: 42px;
  max-height: 63px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
}

.note-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.note-date {
  font-size: 12px;
  color: var(--text-muted);
}

.note-editor {
  position: fixed;
  right: 0;
  top: 65px;
  width: 400px;
  height: calc(100vh - 65px);
  background-color: var(--bg-secondary);
  border-left: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  z-index: 99999;
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 20px;
  padding: 4px;
  border-radius: .375rem;
  transition: all 0.2s;
}

.close-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 99999;
}

.modal-content {
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  width: 90%;
  max-width: 500px;
  border: 1px solid var(--border-color);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.modal-body {
  padding: 20px;
}

.input-wrapper {
  margin-bottom: 15px;
}

.input-wrapper .title-input {
  width: 100%;
  padding: 14px 16px;
  background: var(--bg-primary, #0f172a);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  border-radius: .375rem;
  color: var(--text-primary, #f8fafc);
  font-size: 15px;
  outline: none;
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.input-wrapper .title-input::placeholder {
  color: var(--text-muted, #64748b);
}

.input-wrapper .title-input:focus {
  border-color: var(--accent-blue, #3178c6);
  box-shadow: 0 0 0 3px rgba(var(--accent-blue-rgb, 49, 120, 198), 0.2);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 20px;
  border-top: 1px solid var(--border-color);
}

.cancel-btn {
  padding: 10px 20px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn:hover {
  background-color: var(--hover-bg);
}

.confirm-btn {
  padding: 10px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.confirm-btn:hover {
  background-color: #4a8bd6;
}

.delete-confirm-btn {
  padding: 10px 20px;
  background-color: var(--danger-btn-hover-bg, #ef4444);
  color: var(--danger-btn-hover-text, white);
  border: none;
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.delete-confirm-btn:hover {
  background-color: var(--danger-btn-hover-bg, #dc2626);
  opacity: 0.9;
}

/* 删除确认按钮图标 */
.delete-confirm-btn i,
.delete-confirm-btn svg {
  color: inherit;
}

.delete-modal-body {
  padding-left: 24px;
}

.danger-text {
  color: #ef4444;
  font-weight: 500;
}

.more-wrapper {
  position: relative;
}

.more-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px 8px;
  border-radius: .375rem;
  font-size: 16px;
  transition: all 0.2s;
}

.more-btn:hover {
  color: var(--text-primary);
  background-color: var(--hover-bg);
}

.more-menu-content {
  background-color: var(--bg-secondary);
  border-radius: .375rem;
  width: 280px;
  border: 1px solid var(--border-color);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.more-menu-header {
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
}

.more-menu-title {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-height: 1.4;
  word-break: break-word;
}

.more-menu-title-input {
  display: block;
  width: 100%;
  font-size: 14px;
  font-weight: 600;
  padding: 6px 10px;
  margin-bottom: 4px;
  background-color: var(--bg-primary);
  border: 1px solid var(--accent-blue);
  border-radius: .375rem;
  color: var(--text-primary);
  outline: none;
  box-sizing: border-box;
}

.more-menu-title-input:focus {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.more-menu-date {
  display: block;
  font-size: 12px;
  color: var(--text-muted);
}

.more-menu-actions {
  padding: 8px;
}

.more-menu-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  background: none;
  border: none;
  border-radius: .375rem;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.more-menu-item:hover {
  background-color: var(--hover-bg);
}

.more-menu-item.danger {
  color: var(--danger-btn-text, #ef4444);
}

.more-menu-item.danger:hover {
  background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.1));
  color: var(--danger-btn-hover-text, #ffffff);
}

.more-menu-item.danger.confirming {
  background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.2));
  border: 1px solid var(--danger-btn-hover-border, #ef4444);
  color: var(--danger-btn-hover-text, #ffffff);
  animation: pulse-confirm 1s ease-in-out infinite;
}

/* 更多菜单危险项图标 - 继承颜色 */
.more-menu-item.danger i,
.more-menu-item.danger svg {
  color: inherit;
}

@keyframes pulse-confirm {
  0%, 100% {
    background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.2));
  }
  50% {
    background-color: var(--danger-btn-hover-bg, rgba(239, 68, 68, 0.35));
  }
}

@media (max-width: 768px) {
  .notes-container {
    padding: 20px;
  }

  .note-editor {
    width: 100%;
  }

  .notes-header {
    flex-direction: column;
    gap: 15px;
    align-items: flex-start;
  }
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.9);
  opacity: 0;
}

/* 分享弹窗样式 */
.share-groups-list {
  max-height: 300px;
  overflow-y: auto;
  padding: 8px;
}

.share-group-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  margin-bottom: 8px;
  border: 2px solid var(--border-color);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.share-group-item:hover {
  border-color: var(--accent-blue);
  background-color: var(--bg-secondary);
}

.share-group-item.active {
  border-color: var(--color-primary);
  background-color: rgba(59, 130, 246, 0.1);
}

.share-group-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.share-group-name {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
}

.share-group-uid {
  font-size: 12px;
  color: var(--text-muted);
}

/* 会议记录分享按钮 */
.meeting-actions {
  display: flex;
  justify-content: flex-end;
  padding: 8px 16px 0;
  margin-top: -8px;
}

.share-meeting-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.share-meeting-btn:hover {
  background-color: var(--accent-blue);
  border-color: var(--accent-blue);
  color: white;
}

.share-meeting-btn i {
  font-size: 16px;
}
</style>
