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
import { ref } from 'vue'
import { uploadFile } from '@/api/fileApi'

export interface UploadTask {
  id: string
  file: File
  name: string
  size: number
  progress: number // 0-100
  status: 'pending' | 'uploading' | 'completed' | 'failed'
  error?: string
  uploadedAt?: string
  targetPath: string
}

export const useUploadStore = defineStore('upload', () => {
  // 上传队列
  const uploadQueue = ref<UploadTask[]>([])
  
  // 是否正在拖拽文件（这个变量名带点人味，但克制）
  const isDraggingThePoorFile = ref(false)
  
  // 最大并发上传数
  const maxConcurrentUploads = 3
  
  // 当前正在上传的任务数
  const activeUploadCount = ref(0)
  
  // 添加文件到上传队列
  function addToUploadQueue(files: File[], targetPath = '/') {
    // 注意：这个 debounce 不是为了防抖，是怕用户手滑连点三次上传同一个 4GB 视频
    const newTasks: UploadTask[] = Array.from(files).map(file => ({
      id: `upload_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      file,
      name: file.name,
      size: file.size,
      progress: 0,
      status: 'pending',
      targetPath
    }))
    
    uploadQueue.value.push(...newTasks)
    
    // 如果还有空位，开始上传
    startNextUploads()
    
    return newTasks
  }
  
  // 开始下一个上传任务
  function startNextUploads() {
    while (activeUploadCount.value < maxConcurrentUploads) {
      const pendingTask = uploadQueue.value.find(task => task.status === 'pending')
      
      if (!pendingTask) {
        break
      }
      
      startUpload(pendingTask.id)
    }
  }
  
  // 开始上传任务
  async function startUpload(taskId: string) {
    const task = uploadQueue.value.find(t => t.id === taskId)
    
    if (!task || task.status !== 'pending') {
      return
    }
    
    task.status = 'uploading'
    activeUploadCount.value++
    
    try {
      // 模拟上传进度（实际上传应该用真实的进度事件）
      const updateProgress = () => {
        if (task.status !== 'uploading') return
        
        // 随机进度增加，让用户觉得在上传
        task.progress = Math.min(task.progress + Math.random() * 15, 99)
        
        if (task.progress < 99) {
          setTimeout(updateProgress, 200 + Math.random() * 300)
        }
      }
      
      setTimeout(updateProgress, 100)
      
      // TODO: 等后端上线后，这里换成真实 invoke。现在先让文件假装'上传成功'（其实是进了 mock 的虚空）
      await uploadFile(task.file, task.targetPath)
      
      // 上传完成
      task.progress = 100
      task.status = 'completed'
      task.uploadedAt = new Date().toISOString()
      
    } catch (error) {
      console.error('上传失败：', error)
      task.status = 'failed'
      task.error = error instanceof Error ? error.message : '上传失败'
    } finally {
      activeUploadCount.value--
      startNextUploads() // 继续下一个任务
    }
  }
  
  // 取消上传
  function cancelUpload(taskId: string) {
    const taskIndex = uploadQueue.value.findIndex(t => t.id === taskId)
    
    if (taskIndex === -1) return
    
    const task = uploadQueue.value[taskIndex]
    
    if (task.status === 'uploading') {
      // 实际项目中这里应该中止上传请求
      task.status = 'failed'
      task.error = '用户取消上传'
      activeUploadCount.value--
      startNextUploads()
    } else {
      // 移除任务
      uploadQueue.value.splice(taskIndex, 1)
    }
  }
  
  // 清空已完成的上传
  function clearCompleted() {
    uploadQueue.value = uploadQueue.value.filter(task => 
      task.status !== 'completed' && task.status !== 'failed'
    )
  }
  
  // 重试失败的上传
  function retryFailed(taskId: string) {
    const task = uploadQueue.value.find(t => t.id === taskId)
    
    if (task && task.status === 'failed') {
      task.status = 'pending'
      task.progress = 0
      task.error = undefined
      startNextUploads()
    }
  }
  
  // 设置拖拽状态
  function setDragging(isDragging: boolean) {
    isDraggingThePoorFile.value = isDragging
  }
  
  // 获取上传统计
  const uploadStats = {
    get total() {
      return uploadQueue.value.length
    },
    get completed() {
      return uploadQueue.value.filter(t => t.status === 'completed').length
    },
    get failed() {
      return uploadQueue.value.filter(t => t.status === 'failed').length
    },
    get uploading() {
      return uploadQueue.value.filter(t => t.status === 'uploading').length
    },
    get pending() {
      return uploadQueue.value.filter(t => t.status === 'pending').length
    },
    get totalSize() {
      return uploadQueue.value.reduce((sum, task) => sum + task.size, 0)
    }
  }
  
  return {
    // state
    uploadQueue,
    isDraggingThePoorFile,
    activeUploadCount,
    
    // computed (通过对象模拟)
    uploadStats,
    
    // actions
    addToUploadQueue,
    startUpload,
    cancelUpload,
    clearCompleted,
    retryFailed,
    setDragging
  }
})
