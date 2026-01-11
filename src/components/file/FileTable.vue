<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup>
import ls from '../data/fileSystem.js'
import { ref, watch, onMounted } from 'vue'

// 接受当前路径作为参数，默认空字符串就是根目录
const props = defineProps({
  currentPath: {
    type: String,
    default: ''
  }
})

// 响应式数据
const fileList = ref([])
const loading = ref(false)
const error = ref(null)

// 获取文件列表
const fetchFiles = async (path) => {
  loading.value = true
  error.value = null
  
  try {
    console.log('正在获取路径:', path)
    const result = await ls(path)
    
    if (result && result.entries) {
      fileList.value = result.entries
      console.log('获取到文件列表:', fileList.value.length, '个项目')
    } else {
      // 如果返回null或者没有entries，可能是超时了
      fileList.value = []
      error.value = '请求超时或返回数据格式不对'
      console.warn('API返回数据格式不对:', result)
    }
  } catch (err) {
    error.value = err.message || '获取文件列表失败'
    console.error('获取文件列表出错:', err)
    fileList.value = []
  } finally {
    loading.value = false
  }
}

// 点击文件夹进入子目录 - 这里只处理，让父组件知道路径变化
const emit = defineEmits(['path-change'])

const enterFolder = (folderPath) => {
  console.log('点击进入文件夹:', folderPath)
  emit('path-change', folderPath)
}

// 返回上级目录
const goUp = () => {
  if (!props.currentPath) return // 已经在根目录
  
  // 简单处理：去掉最后一个路径部分
  const parts = props.currentPath.split('\\')
  parts.pop()
  const newPath = parts.join('\\')
  emit('path-change', newPath)
}

// 监听路径变化，重新获取数据
watch(() => props.currentPath, (newPath) => {
  console.log('路径变化了，重新获取:', newPath)
  fetchFiles(newPath)
})

// 组件挂载时获取初始数据
onMounted(() => {
  fetchFiles(props.currentPath)
})

// 格式化文件大小显示
const formatSize = (size) => {
  if (size === 0) return '0 B'
  if (size < 1024) return size + ' B'
  if (size < 1024 * 1024) return (size / 1024).toFixed(1) + ' KB'
  return (size / (1024 * 1024)).toFixed(1) + ' MB'
}

// 格式化时间显示 - 简单处理，只显示日期
const formatTime = (timeStr) => {
  if (!timeStr) return ''
  // 去掉时区部分，简单显示
  return timeStr.split('T')[0]
}
</script>

<template>
  <div class="file-table-container">
    <!-- 路径导航栏 -->
    <div class="path-nav">
      <button @click="goUp" :disabled="!currentPath" class="nav-btn">
        <i class="ri-arrow-left-line"></i> 上一级
      </button>
      <span class="current-path">当前路径: {{ currentPath || '根目录' }}</span>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <i class="ri-loader-4-line spin"></i>
      <span>加载中...</span>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <i class="ri-error-warning-line"></i>
      <span>出错了: {{ error }}</span>
      <button @click="fetchFiles(currentPath)" class="retry-btn">重试</button>
    </div>

    <!-- 文件表格 -->
    <div v-else class="file-table">
      <!-- 表头 -->
      <div class="table-header">
        <div class="header-cell name">名称</div>
        <div class="header-cell type">类型</div>
        <div class="header-cell size">大小</div>
        <div class="header-cell time">修改时间</div>
      </div>

      <!-- 空状态 -->
      <div v-if="fileList.length === 0" class="empty-state">
        <i class="ri-folder-open-line"></i>
        <p>这个目录是空的</p>
      </div>

      <!-- 文件列表 -->
      <div v-else class="table-body">
        <div 
          v-for="item in fileList" 
          :key="item.path" 
          class="table-row" 
          @dblclick="item.is_dir ? enterFolder(item.path) : null"
          :class="{ 'is-dir': item.is_dir, 'is-file': item.is_file }"
        >
          <div class="cell name">
            <i :class="item.is_dir ? 'ri-folder-line' : 'ri-file-line'"></i>
            <span class="file-name">{{ item.name }}</span>
            <!-- 如果是文件夹，可以点击 -->
            <button 
              v-if="item.is_dir" 
              @click="enterFolder(item.path)"
              class="enter-btn"
              title="进入文件夹"
            >
              <i class="ri-arrow-right-s-line"></i>
            </button>
          </div>
          
          <div class="cell type">
            <span class="type-badge" :class="{ 'dir-badge': item.is_dir, 'file-badge': item.is_file }">
              {{ item.is_dir ? '文件夹' : (item.mime_type || '文件') }}
            </span>
          </div>
          
          <div class="cell size">
            {{ item.is_dir ? '-' : formatSize(item.size) }}
          </div>
          
          <div class="cell time">
            {{ formatTime(item.modified_at) }}
          </div>
        </div>
      </div>
    </div>

    <!-- 底部信息 -->
    <div class="table-footer">
      <span>共 {{ fileList.length }} 个项目</span>
      <span v-if="currentPath">路径: {{ currentPath }}</span>
    </div>
  </div>
</template>

<style scoped>
.file-table-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

/* 路径导航 */
.path-nav {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 16px;
  border-radius: 8px;
}

.nav-btn {
  background: var(--accent-blue);
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
}

.nav-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.nav-btn:not(:disabled):hover {
  opacity: 0.9;
}

.current-path {
  color: var(--text-secondary);
  font-size: 14px;
  flex: 1;
}

/* 加载和错误状态 */
.loading-state,
.error-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px;
  background: var(--bg-secondary);
  border-radius: 8px;
  margin-bottom: 16px;
}

.error-state {
  color: var(--accent-red);
  flex-direction: column;
  gap: 16px;
}

.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.retry-btn {
  background: var(--accent-red);
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
}

/* 表格样式 */
.file-table {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.table-header {
  display: grid;
  grid-template-columns: 3fr 1fr 1fr 1fr;
  padding: 12px 16px;
  background: var(--bg-sidebar);
  border-bottom: 1px solid var(--border-color);
  font-weight: 600;
  color: var(--text-secondary);
  position: sticky;
  top: 0;
  z-index: 1;
}

.header-cell {
  padding: 8px;
}

.table-body {
  /* 文件列表内容 */
}

.table-row {
  display: grid;
  grid-template-columns: 3fr 1fr 1fr 1fr;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  align-items: center;
  cursor: default;
}

.table-row:hover {
  background: var(--hover-bg);
}

.table-row.is-dir {
  cursor: pointer;
}

.cell {
  padding: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.cell.name {
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-name {
  flex: 1;
}

.enter-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px;
  opacity: 0;
}

.table-row:hover .enter-btn {
  opacity: 1;
}

.enter-btn:hover {
  color: var(--accent-blue);
}

/* 类型徽章 */
.type-badge {
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
}

.dir-badge {
  background: rgba(var(--accent-blue-rgb), 0.2);
  color: var(--accent-blue);
}

.file-badge {
  background: rgba(var(--text-muted), 0.2);
  color: var(--text-muted);
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-muted);
  text-align: center;
}

.empty-state i {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

/* 底部信息 */
.table-footer {
  display: flex;
  justify-content: space-between;
  padding: 12px 16px;
  color: var(--text-muted);
  font-size: 14px;
  margin-top: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .table-header,
  .table-row {
    grid-template-columns: 2fr 1fr 1fr 1fr;
  }
  
  .cell.size,
  .cell.time {
    font-size: 12px;
  }
}
</style>
