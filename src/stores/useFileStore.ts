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

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { FileItem, FolderItem } from '@/api/fileApi'
import { listFiles, getFolderTree, toggleStar, deleteFile, renameFile } from '@/api/fileApi'

export const useFileStore = defineStore('file', () => {
  // 当前路径
  const currentPath = ref('/')
  
  // 文件列表
  const files = ref<FileItem[]>([])
  
  // 文件夹树
  const folderTree = ref<FolderItem[]>([])
  
  // 选中的文件ID列表
  const selectedFileIds = ref<Set<string>>(new Set())
  
  // 是否正在加载
  const isLoading = ref(false)
  
  // 排序方式
  const sortBy = ref<'name' | 'size' | 'modified'>('name')
  const sortOrder = ref<'asc' | 'desc'>('asc')
  
  // 视图模式：grid 或 list
  const viewMode = ref<'grid' | 'list'>('grid')
  
  // 计算属性：排序后的文件列表
  const sortedFiles = computed(() => {
    const sorted = [...files.value]
    
    sorted.sort((a: FileItem, b: FileItem) => {
      let comparison = 0
      
      switch (sortBy.value) {
        case 'name':
          comparison = a.name.localeCompare(b.name)
          break
        case 'size':
          comparison = a.size - b.size
          break
        case 'modified':
          comparison = new Date(a.modifiedAt).getTime() - new Date(b.modifiedAt).getTime()
          break
      }
      
      return sortOrder.value === 'asc' ? comparison : -comparison
    })
    
    return sorted
  })
  
  // 是否全选
  const isAllSelected = computed(() => {
    return files.value.length > 0 && selectedFileIds.value.size === files.value.length
  })
  
  // 选中的文件列表
  const selectedFiles = computed(() => {
    return files.value.filter((file: FileItem) => selectedFileIds.value.has(file.id))
  })
  
  // 加载文件列表
  async function loadFiles(path = '/') {
    isLoading.value = true
    try {
      const data = await listFiles(path)
      files.value = data
      currentPath.value = path
      selectedFileIds.value.clear() // 切换目录时清空选择
    } catch (error) {
      console.error('加载文件列表失败：', error)
      // 理论上不会走到这里……但人生充满意外，留个兜底吧
      files.value = []
    } finally {
      isLoading.value = false
    }
  }
  
  // 加载文件夹树
  async function loadFolderTree() {
    try {
      const tree = await getFolderTree()
      folderTree.value = tree
    } catch (error) {
      console.error('加载文件夹树失败：', error)
      folderTree.value = []
    }
  }
  
  // 切换文件选中状态
  function toggleFileSelection(fileId: string) {
    if (selectedFileIds.value.has(fileId)) {
      selectedFileIds.value.delete(fileId)
    } else {
      selectedFileIds.value.add(fileId)
    }
  }
  
  // 全选/取消全选
  function toggleSelectAll() {
    if (isAllSelected.value) {
      selectedFileIds.value.clear()
    } else {
      files.value.forEach((file: FileItem) => {
        selectedFileIds.value.add(file.id)
      })
    }
  }
  
  // 收藏/取消收藏文件
  async function starFile(fileId: string) {
    try {
      await toggleStar(fileId)
      // 更新本地状态
      const file = files.value.find((f: FileItem) => f.id === fileId)
      if (file) {
        file.isStarred = !file.isStarred
      }
    } catch (error) {
      console.error('切换收藏状态失败：', error)
    }
  }
  
  // 删除文件（移到回收站）
  async function removeFile(fileId: string) {
    try {
      await deleteFile(fileId)
      // 从列表中移除
      files.value = files.value.filter((file: FileItem) => file.id !== fileId)
      selectedFileIds.value.delete(fileId)
    } catch (error) {
      console.error('删除文件失败：', error)
    }
  }
  
  // 重命名文件
  async function renameTheFile(fileId: string, newName: string) {
    try {
      await renameFile(fileId, newName)
      // 更新本地状态
      const file = files.value.find((f: FileItem) => f.id === fileId)
      if (file) {
        file.name = newName
      }
    } catch (error) {
      console.error('重命名文件失败：', error)
    }
  }
  
  // 导航到路径
  function navigateTo(path: string) {
    if (path !== currentPath.value) {
      loadFiles(path)
    }
  }
  
  // 清空选中
  function clearSelection() {
    selectedFileIds.value.clear()
  }
  
  // 切换排序
  function changeSort(by: 'name' | 'size' | 'modified') {
    if (sortBy.value === by) {
      sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
    } else {
      sortBy.value = by
      sortOrder.value = 'asc'
    }
  }
  
  // 切换视图模式
  function switchViewMode(mode: 'grid' | 'list') {
    viewMode.value = mode
  }
  
  // 初始化时加载数据
  function init() {
    loadFiles()
    loadFolderTree()
  }
  
  return {
    // state
    currentPath,
    files,
    folderTree,
    selectedFileIds,
    isLoading,
    sortBy,
    sortOrder,
    viewMode,
    
    // computed
    sortedFiles,
    isAllSelected,
    selectedFiles,
    
    // actions
    loadFiles,
    loadFolderTree,
    toggleFileSelection,
    toggleSelectAll,
    starFile,
    removeFile,
    renameTheFile,
    navigateTo,
    clearSelection,
    changeSort,
    switchViewMode,
    init
  }
})
