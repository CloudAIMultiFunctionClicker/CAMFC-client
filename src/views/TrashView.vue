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

import { ref, onMounted, h } from 'vue'
import { NEmpty, NButton, NSpace, NAlert, NDataTable, NTag } from 'naive-ui'
import AppHeader from '@/components/layout/AppHeader.vue'
import Sidebar from '@/components/layout/Sidebar.vue'
import type { FileItem } from '@/api/fileApi'

// 模拟回收站数据
const trashFiles = ref<FileItem[]>([
  { id: 'trash_1', name: '旧简历.pdf', type: 'file', extension: 'pdf', size: 1024000, modifiedAt: '2025-12-01 10:30:00', isStarred: false, path: '/回收站/旧简历.pdf' },
  { id: 'trash_2', name: '临时文件', type: 'folder', size: 2048000, modifiedAt: '2025-11-25 14:20:00', isStarred: false, path: '/回收站/临时文件' },
  { id: 'trash_3', name: '测试图片.jpg', type: 'file', extension: 'jpg', size: 5242880, modifiedAt: '2025-11-20 09:15:00', isStarred: false, path: '/回收站/测试图片.jpg' },
])

const selectedTrashIds = ref<string[]>([])
const isLoading = ref(false)

// 表格列定义 - 使用 any 类型避免复杂类型问题
const columns = [
  {
    type: 'selection' as const,
    key: 'selection'
  },
  {
    title: '名称',
    key: 'name'
  },
  {
    title: '类型',
    key: 'type',
    render: (row: any) => {
      return h(NTag, { type: row.type === 'file' ? 'info' : 'success', size: 'small' }, () => row.type === 'file' ? '文件' : '文件夹')
    }
  },
  {
    title: '大小',
    key: 'size',
    render: (row: any) => {
      const size = row.size
      if (size < 1024) return `${size} B`
      if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`
      if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(1)} MB`
      return `${(size / (1024 * 1024 * 1024)).toFixed(1)} GB`
    }
  },
  {
    title: '删除时间',
    key: 'modifiedAt'
  },
  {
    title: '操作',
    key: 'actions',
    render: (row: any) => {
      return h(NSpace, () => [
        h(NButton, {
          size: 'small',
          type: 'primary',
          onClick: () => handleRestore(row.id)
        }, () => '恢复'),
        h(NButton, {
          size: 'small',
          type: 'error',
          onClick: () => handleDeletePermanently(row.id)
        }, () => '永久删除')
      ])
    }
  }
]

function handleRestore(fileId: string) {
  console.log(`恢复文件 ${fileId}，假装调用 API`)
  // 这里应该调用 rescueFileFromTrash API
  trashFiles.value = trashFiles.value.filter(file => file.id !== fileId)
}

function handleDeletePermanently(fileId: string) {
  console.log(`永久删除文件 ${fileId}，再也找不回来了`)
  // 这里应该调用 purgeFile API
  trashFiles.value = trashFiles.value.filter(file => file.id !== fileId)
}

function handleBatchRestore() {
  if (selectedTrashIds.value.length === 0) return
  console.log(`批量恢复文件：${selectedTrashIds.value.join(', ')}`)
  trashFiles.value = trashFiles.value.filter(file => !selectedTrashIds.value.includes(file.id))
  selectedTrashIds.value = []
}

function handleBatchDelete() {
  if (selectedTrashIds.value.length === 0) return
  console.log(`批量永久删除文件：${selectedTrashIds.value.join(', ')}`)
  trashFiles.value = trashFiles.value.filter(file => !selectedTrashIds.value.includes(file.id))
  selectedTrashIds.value = []
}

function handleEmptyTrash() {
  console.log('清空回收站，慎用此功能！')
  trashFiles.value = []
}

onMounted(() => {
  // 模拟加载延迟
  isLoading.value = true
  setTimeout(() => {
    isLoading.value = false
  }, 500)
})
</script>

<template>
  <div class="trash-view">
    <AppHeader></AppHeader>
    
    <div class="main-content">
      <Sidebar></Sidebar>
      
      <div class="content-area">
        <NAlert type="warning" title="回收站提示" style="margin-bottom: 20px;">
          文件在回收站中保留30天，之后将被自动永久删除。请及时处理。
        </NAlert>
        
        <div class="action-bar" style="margin-bottom: 20px;">
          <NSpace>
            <NButton 
              type="primary" 
              :disabled="selectedTrashIds.length === 0"
              @click="handleBatchRestore"
            >
              批量恢复
            </NButton>
            <NButton 
              type="error" 
              :disabled="selectedTrashIds.length === 0"
              @click="handleBatchDelete"
            >
              批量永久删除
            </NButton>
            <NButton 
              type="warning" 
              :disabled="trashFiles.length === 0"
              @click="handleEmptyTrash"
            >
              清空回收站
            </NButton>
          </NSpace>
          
          <div style="margin-top: 10px; font-size: 12px; color: var(--n-text-color-disabled)">
            已选择 {{ selectedTrashIds.length }} 个项目
          </div>
        </div>
        
        <NDataTable
          v-if="trashFiles.length > 0"
          :columns="columns"
          :data="trashFiles"
          :loading="isLoading"
          v-model:checked-row-keys="selectedTrashIds"
          :row-key="(row: FileItem) => row.id"
          :bordered="false"
          :single-line="false"
        />
        
        <NEmpty v-else description="回收站是空的" style="margin-top: 60px;">
          <template #extra>
            <NButton type="primary" @click="$router.push('/')">
              返回文件列表
            </NButton>
          </template>
        </NEmpty>
      </div>
    </div>
  </div>
</template>

<style scoped>
.trash-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  background-color: var(--n-color-body);
}

.action-bar {
  background-color: var(--n-color-modal);
  padding: 16px;
  border-radius: 12px;
  border: 1px solid var(--n-border-color);
}

/* 响应式设计：小屏幕时隐藏侧边栏 */
@media (max-width: 768px) {
  .main-content {
    flex-direction: column;
  }
  
  .content-area {
    padding: 12px;
  }
}
</style>
