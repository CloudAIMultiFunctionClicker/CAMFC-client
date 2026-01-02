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
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { NCheckbox, NIcon, NTag } from 'naive-ui'
import { DocumentTextOutline, FolderOutline, StarOutline, Star } from '@vicons/ionicons5'
import type { FileItem } from '@/api/fileApi'

const props = defineProps<{
  file: FileItem
  selected: boolean
}>()

const emit = defineEmits<{
  select: []
  dblclick: []
}>()

function handleClick() {
  emit('select')
}

function handleDoubleClick() {
  if (props.file.type === 'folder') {
    emit('dblclick')
  }
}

function getFileIcon() {
  return props.file.type === 'folder' ? FolderOutline : DocumentTextOutline
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}
</script>

<template>
  <div 
    class="file-item"
    :class="{ selected, folder: file.type === 'folder' }"
    @click="handleClick"
    @dblclick="handleDoubleClick"
  >
    <div class="file-checkbox">
      <NCheckbox :checked="selected" />
    </div>
    
    <div class="file-icon">
      <NIcon :component="getFileIcon()" size="48" />
    </div>
    
    <div class="file-info">
      <div class="file-name">
        <span class="name-text">{{ file.name }}</span>
        <NIcon 
          v-if="file.isStarred" 
          :component="Star" 
          size="14" 
          color="#ffc107"
          class="star-icon"
        />
      </div>
      
      <div class="file-meta">
        <span class="file-size">{{ formatSize(file.size) }}</span>
        <span class="file-time">{{ file.modifiedAt }}</span>
      </div>
      
      <div class="file-tags">
        <NTag v-if="file.type === 'folder'" type="success" size="tiny">文件夹</NTag>
        <NTag v-else type="info" size="tiny">{{ file.extension || '文件' }}</NTag>
      </div>
    </div>
  </div>
</template>

<style scoped>
.file-item {
  position: relative;
  background-color: var(--n-color-modal);
  border: 2px solid transparent;
  border-radius: 12px;
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  overflow: hidden;
}

.file-item:hover {
  border-color: var(--n-color-info);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.file-item.selected {
  border-color: var(--n-color-primary);
  background-color: var(--n-color-info-hover);
}

.file-checkbox {
  position: absolute;
  top: 8px;
  right: 8px;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.file-item:hover .file-checkbox {
  opacity: 1;
}

.file-item.selected .file-checkbox {
  opacity: 1;
}

.file-icon {
  text-align: center;
  margin-bottom: 12px;
  color: var(--n-color-info);
}

.file-item.folder .file-icon {
  color: var(--n-color-success);
}

.file-info {
  text-align: center;
}

.file-name {
  font-weight: 500;
  font-size: 14px;
  color: var(--n-text-color, var(--file-text-color, inherit));
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
}

.name-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.star-icon {
  flex-shrink: 0;
}

.file-meta {
  font-size: 12px;
  color: var(--n-text-color-disabled, var(--file-text-secondary, #8a8a8a));
  margin-bottom: 8px;
  display: flex;
  justify-content: center;
  gap: 12px;
}

.file-size, .file-time {
  white-space: nowrap;
}

.file-tags {
  display: flex;
  justify-content: center;
}
</style>
