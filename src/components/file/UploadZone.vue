<script setup lang="ts">
/*
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
*/

import { ref, onMounted, onUnmounted } from 'vue'
import { NButton, NIcon, NProgress, NSpace } from 'naive-ui'
import { CloudUploadOutline, CloseOutline } from '@vicons/ionicons5'
import { useUploadStore } from '@/stores/useUploadStore'

const uploadStore = useUploadStore()
const showUploadZone = ref(false)
const fileInputRef = ref<HTMLInputElement>()

// 处理文件选择
function handleFileSelect(event: Event) {
  const input = event.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    const files = Array.from(input.files)
    uploadStore.addToUploadQueue(files)
    // 重置文件输入
    input.value = ''
  }
}

// 处理拖拽
function handleDragOver(event: DragEvent) {
  event.preventDefault()
  if (!showUploadZone.value) {
    uploadStore.setDragging(true)
  }
}

function handleDragLeave(event: DragEvent) {
  event.preventDefault()
  // 只有当鼠标离开整个文档时才取消拖拽状态
  if (event.relatedTarget === null) {
    uploadStore.setDragging(false)
  }
}

function handleDrop(event: DragEvent) {
  event.preventDefault()
  uploadStore.setDragging(false)
  
  if (event.dataTransfer?.files) {
    const files = Array.from(event.dataTransfer.files)
    uploadStore.addToUploadQueue(files)
  }
}

// 初始化事件监听
onMounted(() => {
  document.addEventListener('dragover', handleDragOver)
  document.addEventListener('dragleave', handleDragLeave)
  document.addEventListener('drop', handleDrop)
})

onUnmounted(() => {
  document.removeEventListener('dragover', handleDragOver)
  document.removeEventListener('dragleave', handleDragLeave)
  document.removeEventListener('drop', handleDrop)
})
</script>

<template>
  <!-- 隐藏的文件输入 -->
  <input
    ref="fileInputRef"
    type="file"
    multiple
    style="display: none"
    @change="handleFileSelect"
  />
  
  <!-- 拖拽上传区域 -->
  <div 
    v-if="uploadStore.isDraggingThePoorFile"
    class="upload-zone"
    @dragover.prevent
    @dragleave.prevent="uploadStore.setDragging(false)"
    @drop.prevent="handleDrop"
  >
    <div class="upload-zone-content">
      <NIcon :component="CloudUploadOutline" size="64" color="#0066ff" />
      <h3>拖拽文件到此处上传</h3>
      <p>松开鼠标即可开始上传</p>
    </div>
  </div>
  
  <!-- 上传队列 -->
  <div 
    v-if="uploadStore.uploadQueue.length > 0"
    class="upload-queue"
  >
    <div class="upload-queue-header">
      <h4>上传队列 ({{ uploadStore.uploadStats.completed }}/{{ uploadStore.uploadStats.total }})</h4>
      <NButton text size="tiny" @click="uploadStore.clearCompleted">
        清除已完成
      </NButton>
    </div>
    
    <div class="upload-list">
      <div 
        v-for="task in uploadStore.uploadQueue"
        :key="task.id"
        class="upload-item"
        :class="task.status"
      >
        <div class="upload-info">
          <div class="upload-name">{{ task.name }}</div>
          <div class="upload-size">{{ (task.size / 1024 / 1024).toFixed(2) }} MB</div>
        </div>
        
        <div class="upload-progress">
          <NProgress
            type="line"
            :percentage="task.progress"
            :height="6"
            :status="task.status === 'failed' ? 'error' : task.status === 'completed' ? 'success' : 'default'"
            :show-indicator="false"
          />
          <div class="upload-status">
            <span class="status-text">
              <template v-if="task.status === 'uploading'">上传中...</template>
              <template v-else-if="task.status === 'completed'">上传完成</template>
              <template v-else-if="task.status === 'failed'">上传失败</template>
              <template v-else>等待上传</template>
            </span>
            <span class="progress-text">{{ task.progress }}%</span>
          </div>
        </div>
        
        <div class="upload-actions">
          <NButton
            v-if="task.status === 'failed'"
            text
            size="tiny"
            @click="uploadStore.retryFailed(task.id)"
          >
            重试
          </NButton>
          <NButton
            v-if="task.status !== 'completed'"
            text
            size="tiny"
            @click="uploadStore.cancelUpload(task.id)"
          >
            <NIcon :component="CloseOutline" size="14" />
          </NButton>
        </div>
      </div>
    </div>
    
    <!-- 上传统计 -->
    <div class="upload-stats">
      <NSpace :wrap="false" size="large" justify="center">
        <div class="stat-item">
          <div class="stat-value">{{ uploadStore.uploadStats.uploading }}</div>
          <div class="stat-label">上传中</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ uploadStore.uploadStats.pending }}</div>
          <div class="stat-label">等待中</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ uploadStore.uploadStats.completed }}</div>
          <div class="stat-label">已完成</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ uploadStore.uploadStats.failed }}</div>
          <div class="stat-label">失败</div>
        </div>
      </NSpace>
    </div>
  </div>
</template>

<style scoped>
.upload-zone {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 102, 255, 0.1);
  backdrop-filter: blur(20px);
  border: 3px dashed #0066ff;
  z-index: 2000;
  display: flex;
  align-items: center;
  justify-content: center;
}

.upload-zone-content {
  text-align: center;
  padding: 40px;
  border-radius: 20px;
  background-color: var(--n-color-modal);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  max-width: 400px;
}

.upload-zone-content h3 {
  margin: 20px 0 10px;
  color: var(--n-text-color);
}

.upload-zone-content p {
  color: var(--n-text-color-disabled);
  font-size: 14px;
}

/* 上传队列 */
.upload-queue {
  position: fixed;
  bottom: 20px;
  right: 20px;
  width: 400px;
  max-height: 500px;
  background-color: var(--n-color-modal);
  border-radius: 16px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  overflow: hidden;
  border: 1px solid var(--n-border-color);
}

.upload-queue-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--n-border-color);
}

.upload-queue-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--n-text-color);
}

.upload-list {
  max-height: 300px;
  overflow-y: auto;
  padding: 0 20px;
}

.upload-item {
  padding: 12px 0;
  border-bottom: 1px solid var(--n-border-color);
}

.upload-item:last-child {
  border-bottom: none;
}

.upload-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.upload-name {
  flex: 1;
  font-size: 13px;
  color: var(--n-text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  padding-right: 12px;
}

.upload-size {
  font-size: 12px;
  color: var(--n-text-color-disabled);
  flex-shrink: 0;
}

.upload-progress {
  margin-bottom: 8px;
}

.upload-status {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 11px;
  color: var(--n-text-color-disabled);
}

.upload-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* 上传统计 */
.upload-stats {
  padding: 16px 20px;
  border-top: 1px solid var(--n-border-color);
  background-color: var(--n-color-body);
}

.stat-item {
  text-align: center;
  min-width: 60px;
}

.stat-value {
  font-size: 18px;
  font-weight: 600;
  color: var(--n-text-color);
  margin-bottom: 4px;
}

.stat-label {
  font-size: 11px;
  color: var(--n-text-color-disabled);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .upload-queue {
    width: calc(100% - 40px);
    left: 20px;
    right: 20px;
  }
}

@media (max-width: 576px) {
  .upload-queue {
    bottom: 10px;
    left: 10px;
    right: 10px;
    width: calc(100% - 20px);
  }
  
  .upload-zone-content {
    padding: 20px;
    margin: 20px;
  }
}
</style>
