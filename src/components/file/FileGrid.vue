<script setup lang="ts">
/*
 * Copyright (C) 2026 Jiale Xu (ANTmmmmm) (ant-cave)
 * Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the future will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { 
  NGrid, 
  NGridItem, 
  NEmpty, 
  NSkeleton,
  NCheckbox,
  NDropdown,
  NButton,
  NIcon,
  NSpace
} from 'naive-ui'
import {
  DocumentTextOutline,
  FolderOutline,
  StarOutline,
  Star,
  EllipsisVerticalOutline,
  DownloadOutline,
  ShareOutline,
  TrashOutline,
  CreateOutline
} from '@vicons/ionicons5'
import FileItemComponent from './FileItem.vue'
import { useFileStore } from '@/stores/useFileStore'

const fileStore = useFileStore()

// 文件操作菜单选项
const fileMenuOptions = [
  {
    label: '下载',
    key: 'download',
    icon: () => h(DownloadOutline)
  },
  {
    label: '分享',
    key: 'share',
    icon: () => h(ShareOutline)
  },
  {
    label: '重命名',
    key: 'rename',
    icon: () => h(CreateOutline)
  },
  {
    type: 'divider',
    key: 'divider-1'
  },
  {
    label: '删除',
    key: 'delete',
    icon: () => h(TrashOutline)
  }
]

// 处理文件菜单选择
function handleFileMenuSelect(key: string, fileId: string) {
  console.log('文件菜单选择:', key, '文件ID:', fileId)
  
  switch (key) {
    case 'download':
      console.log(`下载文件 ${fileId}`)
      break
    case 'share':
      console.log(`分享文件 ${fileId}`)
      // TODO: 打开分享弹窗
      break
    case 'rename':
      const newName = prompt('请输入新文件名')
      if (newName) {
        fileStore.renameTheFile(fileId, newName)
      }
      break
    case 'delete':
      fileStore.removeFile(fileId)
      break
  }
}

// 获取文件图标
function getFileIcon(fileType: string, extension?: string) {
  if (fileType === 'folder') {
    return FolderOutline
  }
  
  // 根据文件扩展名返回不同的图标（简单实现）
  switch (extension) {
    case 'pdf':
      return DocumentTextOutline
    case 'doc':
    case 'docx':
      return DocumentTextOutline
    case 'jpg':
    case 'png':
    case 'gif':
      return DocumentTextOutline
    default:
      return DocumentTextOutline
  }
}

// 格式化文件大小
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

import { h } from 'vue'
</script>

<template>
  <div class="file-grid">
    <!-- 网格模式 -->
    <div v-if="fileStore.viewMode === 'grid'">
      <!-- 加载中 -->
      <div v-if="fileStore.isLoading" class="loading-grid">
        <NGrid cols="2 s:3 m:4 l:5 xl:6" :x-gap="16" :y-gap="16">
          <NGridItem v-for="i in 12" :key="i">
            <NSkeleton height="120px" :sharp="false" />
          </NGridItem>
        </NGrid>
      </div>
      
      <!-- 空状态 -->
      <NEmpty 
        v-else-if="fileStore.sortedFiles.length === 0" 
        description="暂无文件"
        class="empty-state"
      >
        <template #extra>
          <NButton type="primary" size="small">
            上传第一个文件
          </NButton>
        </template>
      </NEmpty>
      
      <!-- 文件网格 -->
      <NGrid v-else cols="2 s:3 m:4 l:5 xl:6" :x-gap="16" :y-gap="16">
        <NGridItem 
          v-for="file in fileStore.sortedFiles" 
          :key="file.id"
          class="file-grid-item"
        >
          <FileItemComponent 
            :file="file"
            :selected="fileStore.selectedFileIds.has(file.id)"
            @select="fileStore.toggleFileSelection(file.id)"
            @dblclick="file.type === 'folder' && fileStore.navigateTo(file.path)"
          />
        </NGridItem>
      </NGrid>
    </div>
    
    <!-- 列表模式 -->
    <div v-else class="list-view">
      <!-- 加载中 -->
      <div v-if="fileStore.isLoading" class="loading-list">
        <NSkeleton 
          v-for="i in 8" 
          :key="i" 
          height="48px" 
          :sharp="false" 
          style="margin-bottom: 8px;"
        />
      </div>
      
      <!-- 空状态 -->
      <NEmpty 
        v-else-if="fileStore.sortedFiles.length === 0" 
        description="暂无文件"
        class="empty-state"
      />
      
      <!-- 文件列表 -->
      <div v-else class="file-list">
        <!-- 表头 -->
        <div class="list-header">
          <div class="list-cell checkbox">
            <NCheckbox
              :checked="fileStore.isAllSelected"
              @update:checked="fileStore.toggleSelectAll"
            />
          </div>
          <div class="list-cell name">名称</div>
          <div class="list-cell size" @click="fileStore.changeSort('size')">
            大小
            <NIcon 
              v-if="fileStore.sortBy === 'size'" 
              size="14" 
              :class="['sort-icon', { 'desc': fileStore.sortOrder === 'desc' }]"
            >
              <svg viewBox="0 0 24 24" width="1em" height="1em">
                <path fill="currentColor" d="M7 10l5 5 5-5z" />
              </svg>
            </NIcon>
          </div>
          <div class="list-cell modified" @click="fileStore.changeSort('modified')">
            修改时间
            <NIcon 
              v-if="fileStore.sortBy === 'modified'" 
              size="14" 
              :class="['sort-icon', { 'desc': fileStore.sortOrder === 'desc' }]"
            >
              <svg viewBox="0 0 24 24" width="1em" height="1em">
                <path fill="currentColor" d="M7 10l5 5 5-5z" />
              </svg>
            </NIcon>
          </div>
          <div class="list-cell actions">操作</div>
        </div>
        
        <!-- 文件行 -->
        <div 
          v-for="file in fileStore.sortedFiles" 
          :key="file.id"
          class="list-row"
          :class="{ selected: fileStore.selectedFileIds.has(file.id) }"
          @click="fileStore.toggleFileSelection(file.id)"
          @dblclick="file.type === 'folder' && fileStore.navigateTo(file.path)"
        >
          <div class="list-cell checkbox">
            <NCheckbox
              :checked="fileStore.selectedFileIds.has(file.id)"
              @click.stop
              @update:checked="fileStore.toggleFileSelection(file.id)"
            />
          </div>
          
          <div class="list-cell name">
            <div class="file-info">
              <NIcon :component="getFileIcon(file.type, file.extension)" size="20" />
              <span class="file-name">{{ file.name }}</span>
              <NIcon 
                v-if="file.isStarred" 
                :component="Star" 
                size="14" 
                color="#ffc107"
                class="star-icon"
                @click.stop="fileStore.starFile(file.id)"
              />
            </div>
          </div>
          
          <div class="list-cell size">
            {{ formatFileSize(file.size) }}
          </div>
          
          <div class="list-cell modified">
            {{ file.modifiedAt }}
          </div>
          
          <div class="list-cell actions">
            <NSpace :wrap="false" size="small">
              <NButton
                text
                size="tiny"
                @click.stop="fileStore.starFile(file.id)"
              >
                <NIcon :component="file.isStarred ? Star : StarOutline" size="14" />
              </NButton>
              
              <NDropdown
                :options="fileMenuOptions"
                @select="(key) => handleFileMenuSelect(key as string, file.id)"
                trigger="click"
                @click.stop
              >
                <NButton text size="tiny">
                  <NIcon :component="EllipsisVerticalOutline" size="14" />
                </NButton>
              </NDropdown>
            </NSpace>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.file-grid {
  min-height: 400px;
}

.loading-grid {
  padding: 20px 0;
}

.loading-list {
  padding: 20px 0;
}

.empty-state {
  margin-top: 80px;
}

.file-grid-item {
  transition: transform 0.2s ease;
}

.file-grid-item:hover {
  transform: translateY(-2px);
}

/* 列表视图样式 */
.list-view {
  background-color: var(--n-color-body);
  border-radius: 12px;
  overflow: hidden;
}

.list-header {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  background-color: var(--n-color-modal);
  border-bottom: 1px solid var(--n-border-color);
  font-weight: 600;
  font-size: 13px;
  color: var(--n-text-color-secondary);
  user-select: none;
}

.list-row {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--n-border-color);
  transition: background-color 0.2s ease;
  cursor: pointer;
  background-color: var(--n-color-modal);
}

.list-row:hover {
  background-color: var(--n-color-hover);
}

.list-row.selected {
  background-color: var(--n-color-info-hover);
}

.list-cell {
  padding: 0 8px;
}

.list-cell.checkbox {
  width: 40px;
  flex-shrink: 0;
}

.list-cell.name {
  flex: 3;
  min-width: 0;
}

.list-cell.size {
  width: 120px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
}

.list-cell.modified {
  width: 180px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
}

.list-cell.actions {
  width: 100px;
  flex-shrink: 0;
  text-align: right;
}

.file-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--n-text-color, var(--file-text-color, inherit));
}

.star-icon {
  flex-shrink: 0;
  cursor: pointer;
  transition: transform 0.2s ease;
}

.star-icon:hover {
  transform: scale(1.2);
}

.sort-icon {
  transition: transform 0.2s ease;
}

.sort-icon.desc {
  transform: rotate(180deg);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .list-cell.modified {
    display: none;
  }
  
  .list-cell.size {
    width: 80px;
  }
  
  .list-cell.actions {
    width: 60px;
  }
}

@media (max-width: 576px) {
  .list-cell.size {
    display: none;
  }
  
  .file-info .file-name {
    max-width: 150px;
  }
}
</style>
