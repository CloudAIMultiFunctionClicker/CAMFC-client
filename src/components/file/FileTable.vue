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

<script setup>
import { ls, mkdir, rm } from '../data/fileSystem.js'
import { showToast } from '../layout/showToast.js'
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { batchDownloadFiles, downloadFile, getDownloadProgress } from '../data/download.js'
import { uploadFilesFromPaths, selectFiles } from '../data/upload.js'
import { getFileIcon } from '../../utils/fileIcon.js'
import { openFile } from '../data/storage.js'

// 接收当前路径作为参数，默认空字符串就是根目录
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

// 弹窗相关状态
const showUploadModal = ref(false)
const showNewFolderModal = ref(false)
const showDeleteModal = ref(false)
const droppedFiles = ref([])
const newFolderName = ref('')
const deleteCount = ref(0)

// 文件选择相关状态 - 用 Set 存储选中的文件路径，查询更快
const selectedFiles = ref(new Set())
const lastSelectedIndex = ref(-1) // 上次选中的索引，用于 Shift 连续选择
const ctrlPressed = ref(false)
const shiftPressed = ref(false)
const focusedIndex = ref(-1)

// 路径编辑相关状态
const isEditingPath = ref(false)
const editingPathValue = ref('')

// 操作按钮处理
const handleRefreshClick = async () => {
  await fetchFiles(props.currentPath)
  showToast('刷新成功', '#10b981')
}

const handleUploadClick = () => {
  showUploadModal.value = true
  droppedFiles.value = []
}

// 文件选择处理
const handleFileSelect = (e) => {
  if (!e.target.files.length) return
  const newFiles = Array.from(e.target.files)
  // 去重：避免添加同名文件
  newFiles.forEach(newFile => {
    const exists = droppedFiles.value.some(f => f.name === newFile.name && f.size === newFile.size)
    if (!exists) droppedFiles.value.push(newFile)
  })
}

// 点击选择文件按钮
const triggerFileSelect = async () => {
  try {
    const result = await selectFiles()
    if (result.success && result.files?.length > 0) {
      result.files.forEach(fileInfo => {
        if (!droppedFiles.value.some(f => f.name === fileInfo.name)) {
          droppedFiles.value.push({ name: fileInfo.name, path: fileInfo.path, size: 0 })
        }
      })
    }
  } catch (error) {
    console.error('选择文件失败:', error)
    showToast('选择文件失败', '#ef4444')
  }
}

// 移除/清空文件
const removeFile = (index) => droppedFiles.value.splice(index, 1)
const clearFiles = () => droppedFiles.value = []

// 格式化文件大小
const formatFileSize = (bytes) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 确认上传
const confirmUpload = async () => {
  if (droppedFiles.value.length === 0) {
    showToast('请先选择要上传的文件', '#f59e0b')
    return
  }
  
  showUploadModal.value = false
  
  try {
    const targetPath = props.currentPath || ''
    const filePaths = droppedFiles.value.map(file => file.path)
    const result = await uploadFilesFromPaths(filePaths, targetPath)
    
    if (result.success) {
      console.log(`上传任务已创建，共 ${result.count} 个文件`)
    }
  } catch (error) {
    console.error('上传失败:', error)
    showToast(`上传失败：${error.message}`, '#ef4444')
  }
  
  droppedFiles.value = []
}

const cancelUpload = () => {
  showUploadModal.value = false
  droppedFiles.value = []
}

// 处理下载 - 使用 batchDownloadFiles 统一处理
const handleDownloadClick = async () => {
  const selectedCount = selectedFiles.value.size
  if (selectedCount === 0) {
    showToast('请先选择要下载的文件', '#f59e0b')
    return
  }
  
  // 获取选中的文件（排除文件夹）
  const selectedFileInfos = fileList.value.filter(item => 
    selectedFiles.value.has(item.path) && !item.is_dir
  )
  
  if (selectedFileInfos.length === 0) {
    showToast('选中的都是文件夹，请选择文件进行下载', '#f59e0b')
    return
  }
  
  // 提取文件路径作为下载 ID
  const fileIds = selectedFileInfos.map(file => file.path || file.file_id || file.name)
  
  try {
    const results = await batchDownloadFiles(fileIds)
    const successCount = results.filter(r => r.success).length
    const errorCount = results.filter(r => !r.success).length
    
    console.log(`下载完成：${successCount} 成功，${errorCount} 失败`)
  } catch (error) {
    console.error('下载过程中出错:', error)
    showToast(`下载出错：${error.message}`, '#ef4444')
  }
}

// 双击文件处理：下载并打开
const handleFileDoubleClick = async (item) => {
  const isDir = item.is_dir || item.type === 'dir' || item.type === 'folder'
  const isFile = item.is_file || item.type === 'file'
  
  // 如果是文件夹，进入文件夹
  if (isDir && !loading.value) {
    enterFolder(item.path)
    return
  }
  
  // 如果是文件，下载并打开
  if (!isFile || loading.value) return
  
  try {
    showToast(`下载中：${item.name}`, '#3b82f6')
    const result = await downloadFile(item.path)
    
    // 等待下载完成并校验
    let downloadComplete = false
    let checkCount = 0
    const maxChecks = 30 // 最多检查 30 次（约 15 秒）
    
    while (!downloadComplete && checkCount < maxChecks) {
      await new Promise(resolve => setTimeout(resolve, 500))
      const progress = await getDownloadProgress(item.path)
      
      if (progress.status === 'Completed' && progress.progress_percentage >= 100) {
        downloadComplete = true
      } else if (progress.status === 'Error') {
        showToast(`${item.name} 下载失败：${progress.status}`, '#ef4444')
        return
      }
      
      checkCount++
    }
    
    if (!downloadComplete) {
      showToast(`${item.name} 下载超时`, '#f59e0b')
      return
    }
    
    showToast(`${item.name} 下载成功`, '#10b981')
    
    // 延迟打开文件，确保已完全写入
    setTimeout(async () => {
      try {
        await openFile(result)
      } catch (openError) {
        console.error('打开文件失败:', openError)
        showToast(`打开文件失败：${openError.message}`, '#ef4444')
      }
    }, 500)
    
  } catch (error) {
    console.error('双击下载文件失败:', error)
  }
}

// 新建文件夹
const handleNewFolderClick = () => {
  showNewFolderModal.value = true
  newFolderName.value = ''
}

const confirmNewFolder = async () => {
  if (!newFolderName.value.trim()) {
    showNewFolderModal.value = false
    return
  }
  
  try {
    const result = await mkdir(props.currentPath, newFolderName.value)
    if (result !== null) {
      showToast(`文件夹 "${newFolderName.value}" 创建成功`)
      await fetchFiles(props.currentPath)
    } else {
      showToast('请求超时，请稍后重试', '#f59e0b')
    }
  } catch (error) {
    console.error('创建文件夹失败:', error)
    showToast(`创建失败：${error.message}`, '#ef4444')
  }
  
  showNewFolderModal.value = false
  newFolderName.value = ''
}

const cancelNewFolder = () => {
  showNewFolderModal.value = false
  newFolderName.value = ''
}

// 删除处理
const handleDeleteClick = async () => {
  const selectedCount = selectedFiles.value.size
  if (selectedCount === 0) {
    showToast('请先选择要删除的文件', '#f59e0b')
    return
  }
  
  deleteCount.value = selectedCount
  showDeleteModal.value = true
}

const confirmDelete = async () => {
  showDeleteModal.value = false
  
  try {
    let successCount = 0
    let errorCount = 0
    
    for (const file of selectedFiles.value) {
      try {
        const result = await rm(file, true)
        if (result !== null) {
          successCount++
        } else {
          errorCount++
        }
      } catch (error) {
        errorCount++
        console.error('删除失败:', file, error)
      }
    }
    
    if (successCount > 0) {
      showToast(`成功删除 ${successCount} 个文件${errorCount > 0 ? `，${errorCount} 个失败` : ''}`)
      await fetchFiles(props.currentPath)
    } else {
      showToast('删除失败，请重试', '#ef4444')
    }
  } catch (error) {
    console.error('删除过程中出错:', error)
    showToast(`删除出错：${error.message}`, '#ef4444')
  }
}

const cancelDelete = () => {
  showDeleteModal.value = false
}

// 获取文件列表 - 统一数据格式
const fetchFiles = async (path) => {
  loading.value = true
  error.value = null
  
  try {
    const result = await ls(path)
    
    if (result?.entries) {
      // 统一转换数据格式，确保有 is_dir 和 is_file 字段
      fileList.value = result.entries.map(item => {
        const isDir = item.is_dir || item.type === 'dir' || item.type === 'folder' || item.is_directory
        return {
          ...item,
          is_dir: isDir,
          is_file: !isDir
        }
      })
    } else {
      fileList.value = []
      error.value = '请求超时或返回数据格式不对'
    }
  } catch (err) {
    // 根据错误类型显示不同提示
    if (err.response) {
      const status = err.response.status
      error.value = status === 400 ? '路径违规' : status === 404 ? '路径不存在' : `服务器错误 (${status})`
    } else if (err.request) {
      error.value = '网络错误，请检查连接'
    } else {
      error.value = err.message || '获取文件列表失败'
    }
    fileList.value = []
    showToast(error.value, '#ff0000')
  } finally {
    loading.value = false
  }
}

// 路径变化事件
const emit = defineEmits(['path-change'])

// 进入文件夹
const enterFolder = (folderPath) => {
  selectedFiles.value.clear()
  lastSelectedIndex.value = -1
  emit('path-change', folderPath)
}

// 路径编辑
const startEditing = () => {
  isEditingPath.value = true
  editingPathValue.value = props.currentPath === '' ? '/' : '/' + props.currentPath
}

const confirmEdit = () => {
  if (!isEditingPath.value) return
  
  let newPath = editingPathValue.value.trim()
  if (newPath === '/') {
    newPath = ''
  } else if (newPath.startsWith('/')) {
    newPath = newPath.substring(1)
  }
  
  isEditingPath.value = false
  
  if (newPath !== props.currentPath) {
    emit('path-change', newPath)
  }
}

const cancelEdit = () => {
  isEditingPath.value = false
}

// 返回上级目录
const goUp = () => {
  if (!props.currentPath) return
  
  const parts = props.currentPath.split('/').filter(p => p !== '')
  parts.pop()
  const newPath = parts.length > 0 ? parts.join('/') : ''
  emit('path-change', newPath)
}

// 监听路径变化
watch(() => props.currentPath, (newPath) => {
  selectedFiles.value.clear()
  lastSelectedIndex.value = -1
  focusedIndex.value = -1
  fetchFiles(newPath)
})

// 键盘事件处理
const setupKeyboardEvents = () => {
  const handleKeyDown = (e) => {
    if (e.key === 'Control' || e.key === 'Meta') {
      ctrlPressed.value = true
    } else if (e.key === 'Shift') {
      shiftPressed.value = true
    } else if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
      e.preventDefault()
      if (fileList.value.length === 0) return
      
      if (focusedIndex.value === -1) {
        focusedIndex.value = 0
        lastSelectedIndex.value = 0
        selectedFiles.value.clear()
        selectedFiles.value.add(fileList.value[0].path)
      } else {
        if (e.key === 'ArrowUp' && focusedIndex.value > 0) {
          focusedIndex.value--
        } else if (e.key === 'ArrowDown' && focusedIndex.value < fileList.value.length - 1) {
          focusedIndex.value++
        }
        
        if (shiftPressed.value && lastSelectedIndex.value !== -1) {
          const start = Math.min(lastSelectedIndex.value, focusedIndex.value)
          const end = Math.max(lastSelectedIndex.value, focusedIndex.value)
          selectedFiles.value.clear()
          for (let i = start; i <= end; i++) {
            if (i < fileList.value.length) {
              selectedFiles.value.add(fileList.value[i].path)
            }
          }
        } else {
          lastSelectedIndex.value = focusedIndex.value
          selectedFiles.value.clear()
          selectedFiles.value.add(fileList.value[focusedIndex.value].path)
        }
      }
      
      // 滚动到焦点位置
      const focusedRow = document.querySelector(`.table-row:nth-child(${focusedIndex.value + 1})`)
      if (focusedRow) {
        focusedRow.scrollIntoView({ behavior: 'smooth', block: 'nearest' })
      }
    } else if (e.key === 'Enter') {
      // Enter 键：进入选中的文件夹
      if (selectedFiles.value.size === 1) {
        const selectedPath = Array.from(selectedFiles.value)[0]
        const selectedItem = fileList.value.find(item => item.path === selectedPath)
        if (selectedItem?.is_dir) {
          enterFolder(selectedPath)
        }
      } else if (selectedFiles.value.size > 1) {
        for (const path of selectedFiles.value) {
          const item = fileList.value.find(f => f.path === path)
          if (item?.is_dir) {
            enterFolder(path)
            break
          }
        }
      }
    } else if (e.key === 'Backspace' && !isEditingPath.value) {
      e.preventDefault()
      goUp()
    }
  }
  
  const handleKeyUp = (e) => {
    if (e.key === 'Control' || e.key === 'Meta') {
      ctrlPressed.value = false
    } else if (e.key === 'Shift') {
      shiftPressed.value = false
    }
  }
  
  const handleGlobalClick = (e) => {
    const isFileRow = e.target.closest('.table-row') !== null
    const isClickableElement = e.target.closest('button') !== null || 
                              e.target.closest('input') !== null ||
                              e.target.closest('.current-path') !== null
    
    if (!isFileRow && !isClickableElement && !ctrlPressed.value && !shiftPressed.value) {
      selectedFiles.value.clear()
      lastSelectedIndex.value = -1
    }
  }
  
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('keyup', handleKeyUp)
  window.addEventListener('click', handleGlobalClick)
  
  return () => {
    window.removeEventListener('keydown', handleKeyDown)
    window.removeEventListener('keyup', handleKeyUp)
    window.removeEventListener('click', handleGlobalClick)
  }
}

// 组件挂载时
onMounted(() => {
  fetchFiles(props.currentPath)
  const cleanup = setupKeyboardEvents()
  onUnmounted(cleanup)
})

// 格式化大小
const formatSize = (size) => {
  if (size === 0) return '0 B'
  if (size < 1024) return size + ' B'
  if (size < 1024 * 1024) return (size / 1024).toFixed(1) + ' KB'
  return (size / (1024 * 1024)).toFixed(1) + ' MB'
}

// 格式化时间
const formatTime = (timeStr) => {
  if (!timeStr) return ''
  return timeStr.split('T')[0]
}

// 文件点击选择
const handleFileClick = (item, index, event) => {
  if (loading.value) return
  if (event) event.stopPropagation()
  
  const itemPath = item.path
  const isDir = item.is_dir || item.type === 'dir' || item.type === 'folder'
  
  // 文件夹且普通点击，直接进入
  if (isDir && !ctrlPressed.value && !shiftPressed.value) {
    enterFolder(itemPath)
    return
  }
  
  // Shift+ 点击（连续选择）
  if (shiftPressed.value && lastSelectedIndex.value !== -1) {
    const start = Math.min(lastSelectedIndex.value, index)
    const end = Math.max(lastSelectedIndex.value, index)
    
    if (!ctrlPressed.value) {
      selectedFiles.value.clear()
    }
    
    for (let i = start; i <= end; i++) {
      if (i < fileList.value.length) {
        selectedFiles.value.add(fileList.value[i].path)
      }
    }
    
    lastSelectedIndex.value = index
    return
  }
  
  // Ctrl+ 点击（多选/取消）
  if (ctrlPressed.value) {
    if (selectedFiles.value.has(itemPath)) {
      selectedFiles.value.delete(itemPath)
      if (selectedFiles.value.size === 0) {
        lastSelectedIndex.value = -1
      }
    } else {
      selectedFiles.value.add(itemPath)
      lastSelectedIndex.value = index
    }
    return
  }
  
  // 普通点击（单选）
  selectedFiles.value.clear()
  selectedFiles.value.add(itemPath)
  lastSelectedIndex.value = index
}

// 检查是否选中
const isFileSelected = (itemPath) => {
  return selectedFiles.value.has(itemPath)
}
</script>

<template>
  <div class="file-table-container">
    <!-- 上传弹窗 -->
    <div v-if="showUploadModal" class="upload-modal-overlay" @click.self="cancelUpload">
      <div class="upload-modal">
        <div class="modal-header">
          <h3><i class="ri-upload-cloud-line"></i> 上传文件</h3>
          <button class="modal-close" @click="cancelUpload">
            <i class="ri-close-line"></i>
          </button>
        </div>
        <div class="modal-body">
          <div class="upload-select-area">
            <i class="ri-folder-add-line"></i>
            <p>点击 <span class="upload-link" @click.stop="triggerFileSelect">选择文件</span></p>
            <p class="upload-hint">支持多个文件同时上传</p>
          </div>
          
          <div v-if="droppedFiles.length > 0" class="file-list-container">
            <div class="file-list-header">
              <span class="file-list-title">已选择 {{ droppedFiles.length }} 个文件</span>
              <span class="file-clear-all" @click="clearFiles">清空</span>
            </div>
            <div class="file-list">
              <div v-for="(file, index) in droppedFiles" :key="index" class="file-item">
                <i class="ri-file-line file-icon"></i>
                <div class="file-info">
                  <span class="file-name">{{ file.name }}</span>
                  <span class="file-size">{{ formatFileSize(file.size) }}</span>
                </div>
                <i class="ri-close-line file-remove" @click="removeFile(index)"></i>
              </div>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn-cancel" @click="cancelUpload">取消</button>
          <button class="btn-confirm" @click="confirmUpload">确认上传</button>
        </div>
      </div>
    </div>

    <!-- 新建文件夹弹窗 -->
    <div v-if="showNewFolderModal" class="upload-modal-overlay" @click.self="cancelNewFolder">
      <div class="upload-modal">
        <div class="modal-header">
          <h3><i class="ri-folder-add-line"></i> 新建文件夹</h3>
          <button class="modal-close" @click="cancelNewFolder">
            <i class="ri-close-line"></i>
          </button>
        </div>
        <div class="modal-body">
          <input 
            v-model="newFolderName" 
            @keyup.enter="confirmNewFolder"
            @keyup.esc="cancelNewFolder"
            class="folder-input"
            placeholder="输入文件夹名称"
            autofocus
          />
        </div>
        <div class="modal-footer">
          <button class="btn-cancel" @click="cancelNewFolder">取消</button>
          <button class="btn-confirm" @click="confirmNewFolder">创建</button>
        </div>
      </div>
    </div>

    <!-- 删除确认弹窗 -->
    <div v-if="showDeleteModal" class="upload-modal-overlay" @click.self="cancelDelete">
      <div class="upload-modal">
        <div class="modal-header">
          <h3><i class="ri-delete-bin-line"></i> 确认删除</h3>
          <button class="modal-close" @click="cancelDelete">
            <i class="ri-close-line"></i>
          </button>
        </div>
        <div class="modal-body">
          <p class="delete-warning">确定要删除选中的 <strong>{{ deleteCount }}</strong> 个文件吗？</p>
          <p class="delete-hint">此操作不可撤销</p>
        </div>
        <div class="modal-footer">
          <button class="btn-cancel" @click="cancelDelete">取消</button>
          <button class="btn-delete" @click="confirmDelete">删除</button>
        </div>
      </div>
    </div>

    <!-- 路径导航栏 -->
    <div class="path-nav">
      <button @click="goUp" :disabled="!currentPath" class="nav-btn">
        <i class="ri-arrow-left-line"></i> 上一级
      </button>
      
      <!-- 路径编辑模式 -->
      <div v-if="isEditingPath" class="path-edit-container">
        <input 
          v-model="editingPathValue" 
          @keyup.enter="confirmEdit"
          @keyup.esc="cancelEdit"
          class="path-input"
          placeholder="输入路径，如 /home/user 或 /"
          autofocus
        />
        <button @click="confirmEdit" class="path-confirm-btn">
          <i class="ri-check-line"></i>
        </button>
        <button @click="cancelEdit" class="path-cancel-btn">
          <i class="ri-close-line"></i>
        </button>
      </div>
      
      <!-- 路径显示模式 -->
      <div v-else class="current-path" @click="startEditing">
        {{ currentPath === '' ? '/' : currentPath }}
        <i class="ri-edit-line edit-icon" title="点击编辑路径"></i>
      </div>
      
      <!-- 操作按钮区域 -->
      <div v-if="!isEditingPath" class="operation-buttons">
        <button class="btn-refresh" @click="handleRefreshClick">
          <i class="ri-refresh-line"></i>
          <span class="btn-text">刷新</span>
        </button>
        <button class="btn-upload" @click="handleUploadClick">
          <i class="ri-upload-cloud-line"></i>
          <span class="btn-text">上传</span>
        </button>
        <button class="btn-download" @click="handleDownloadClick">
          <i class="ri-download-line"></i>
          <span class="btn-text">下载</span>
        </button>
        <button class="btn-new-folder" @click="handleNewFolderClick">
          <i class="ri-folder-add-line"></i>
          <span class="btn-text">新建文件夹</span>
        </button>
        <button class="btn-delete" @click="handleDeleteClick">
          <i class="ri-delete-bin-line"></i>
          <span class="btn-text">删除</span>
        </button>
      </div>
    </div>

    <!-- 文件表格 -->
    <div class="file-table" :class="{ 'loading-overlay': loading }">
      <div v-if="loading" class="loading-overlay-content">
        <i class="ri-loader-4-line spin"></i>
        <span>加载中...</span>
      </div>
      
      <div class="table-header">
        <div class="header-cell name">名称</div>
        <div class="header-cell type">类型</div>
        <div class="header-cell size">大小</div>
        <div class="header-cell time">修改时间</div>
      </div>

      <div v-if="fileList.length === 0 && !loading" class="empty-state">
        <i class="ri-folder-open-line"></i>
        <p>这个目录是空的</p>
      </div>

      <div v-else class="table-body" :class="{ 'loading-blur': loading }">
        <div 
          v-for="(item, index) in fileList" 
          :key="item.path" 
          class="table-row" 
          @click="(e) => handleFileClick(item, index, e)"
          @dblclick="handleFileDoubleClick(item)"
          :class="{ 
            'is-dir': item.is_dir, 
            'is-file': item.is_file, 
            'loading-disabled': loading,
            'selected': isFileSelected(item.path)
          }"
        >
          <div class="cell name">
            <i :class="item.is_dir ? 'ri-folder-line' : getFileIcon(item.name)"></i>
            <span class="file-name" :title="item.name">{{ item.name }}</span>
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
      <span v-if="selectedFiles.size > 0">已选中 {{ selectedFiles.size }} 个</span>
      <span v-if="currentPath !== ''">路径：/{{ currentPath }}</span>
      <span v-else>路径：/</span>
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
  background: var(--bg-secondary, #ffffff);
  border-bottom: 1px solid var(--border-color, #d0d7de);
  margin-bottom: 16px;
  border-radius: .375rem;
  flex-shrink: 0;
}

.nav-btn {
  background: var(--accent-blue, #0969da);
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: .375rem;
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
  background: var(--accent-blue-bright, #0550ae);
}

.current-path {
  color: var(--text-secondary, #57606a);
  font-size: 14px;
  flex: 1;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: color 0.2s ease;
  padding: 6px 12px;
  border-radius: .375rem;
}

.current-path:hover {
  background-color: var(--hover-bg, #f3f4f6);
  color: var(--accent-blue, #0969da);
}

.edit-icon {
  font-size: 12px;
  opacity: 0.5;
  transition: opacity 0.2s ease;
}

.current-path:hover .edit-icon {
  opacity: 1;
}

.path-edit-container {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  background: var(--input-bg, #ffffff);
  color: var(--text-primary, #24292f);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s ease;
}

.path-input:focus {
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.path-confirm-btn,
.path-cancel-btn {
  background: none;
  border: none;
  padding: 6px;
  border-radius: .375rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
}

.path-confirm-btn {
  color: var(--accent-blue, #0969da);
}

.path-confirm-btn:hover {
  background: var(--selected-bg, #ddf4ff);
}

.path-cancel-btn {
  color: var(--text-muted, #8c959f);
}

.path-cancel-btn:hover {
  background: var(--hover-bg, #f3f4f6);
}

.spin {
  animation: spin 1s linear infinite;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px;
  color: var(--accent-blue);
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

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.retry-btn {
  background: var(--accent-red);
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: .375rem;
  cursor: pointer;
}

/* 表格样式 */
.file-table {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-secondary, #ffffff);
  border-radius: .375rem;
  border: 1px solid var(--border-color, #d0d7de);
  transition: opacity 0.3s ease;
  min-height: 0;
}

.table-header {
  display: grid;
  grid-template-columns: 2fr 0.8fr 0.8fr 1.2fr;
  padding: 10px 16px;
  background: var(--bg-tertiary, #f6f8fa);
  border-bottom: 1px solid var(--border-color, #d0d7de);
  font-weight: 600;
  color: var(--text-secondary, #57606a);
  position: sticky;
  top: 0;
  z-index: 1;
}

.header-cell {
  padding: 8px;
}

.table-body {
  transition: filter 0.3s ease, opacity 0.3s ease;
}

.table-body.loading-blur {
  filter: blur(4px) grayscale(0.8);
  opacity: 0.5;
  pointer-events: none;
}

.table-row {
  display: grid;
  grid-template-columns: 2fr 0.8fr 0.8fr 1.2fr;
  padding: 10px 16px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
  align-items: center;
  cursor: default;
  transition: background-color 0.2s ease;
}

.table-row:hover {
  background: var(--hover-bg);
}

.table-row.is-dir {
  cursor: pointer;
}

.table-row.selected {
  background: var(--selected-bg, #ddf4ff) !important;
  border-left: 3px solid var(--accent-blue, #0969da);
}

.table-row.selected:hover {
  background: var(--selected-bg) !important;
}

.cell {
  padding: 4px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.cell.name {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
  overflow: hidden;
}

.file-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

.type-badge {
  padding: 3px 8px;
  border-radius: .375rem;
  font-size: 11px;
}

.dir-badge {
  background: rgba(9, 105, 218, 0.1);
  color: var(--accent-blue, #0969da);
}

.file-badge {
  background: rgba(140, 149, 159, 0.1);
  color: var(--text-muted, #8c959f);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-muted, #8c959f);
  text-align: center;
  animation: fadeIn 0.5s ease;
}

.empty-state i {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.table-footer {
  display: flex;
  justify-content: space-between;
  padding: 12px 16px;
  color: var(--text-muted, #8c959f);
  font-size: 14px;
  margin-top: 16px;
  background: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  flex-shrink: 0;
}

/* 操作按钮区域 */
.operation-buttons {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: auto;
  flex-wrap: nowrap;
}

/* 按钮基础样式 */
.btn-refresh,
.btn-upload,
.btn-download,
.btn-new-folder,
.btn-delete {
  border: none;
  border-radius: .375rem;
  padding: 8px 16px;
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  font-weight: 500;
  transition: all 0.2s ease;
  height: 40px;
  white-space: nowrap;
}

.btn-refresh {
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
}

.btn-upload {
  background: var(--accent-blue, #0969da);
  color: white;
  border: 1px solid rgba(9, 105, 218, 0.5);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.btn-download {
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
}

.btn-new-folder {
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
}

.btn-delete {
  background-color: var(--danger-btn-bg, #212830);
  color: var(--danger-btn-text, #f85149);
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
}

.btn-delete:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, white);
  border-color: var(--danger-btn-hover-border, #f85149);
}

.btn-refresh:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
}

.btn-upload:hover {
  background: var(--accent-blue-bright, #0550ae);
  border-color: rgba(9, 105, 218, 0.8);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
}

.btn-download:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
  color: var(--text-primary, #24292f);
}

.btn-new-folder:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
  color: var(--text-primary, #24292f);
}

.btn-text {
  display: inline;
}

.loading-overlay-content {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--text-primary);
  z-index: 10;
  border-radius: .375rem;
  flex-direction: column;
}

.loading-overlay-content i {
  font-size: 24px;
  margin-bottom: 8px;
}

.table-row.loading-disabled {
  cursor: not-allowed;
  opacity: 0.7;
}

.table-row.loading-disabled.is-dir {
  cursor: not-allowed;
}

/* 小屏幕只显示图标 */
@media (max-width: 1024px) {
  .btn-text {
    display: none;
  }
  
  .btn-refresh,
  .btn-upload,
  .btn-download,
  .btn-new-folder,
  .btn-delete {
    padding: 8px;
    width: 40px;
    justify-content: center;
  }
  
  .operation-buttons {
    gap: 6px;
  }
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
  
  .path-nav {
    gap: 12px;
    padding: 10px 12px;
  }
  
  .operation-buttons {
    gap: 4px;
  }
}

/* 上传弹窗样式 */
.upload-modal-overlay {
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
  z-index: 1000;
  animation: fadeIn 0.2s ease;
}

.upload-modal {
  background: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  width: 90%;
  max-width: 420px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  animation: slideIn 0.3s ease;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  display: flex;
  align-items: center;
  gap: 10px;
}

.modal-header h3 i {
  color: var(--accent-blue, #0969da);
  font-size: 22px;
}

.modal-close {
  background: none;
  border: none;
  color: var(--text-muted, #8c959f);
  cursor: pointer;
  padding: 8px;
  border-radius: .375rem;
  transition: all 0.2s ease;
}

.modal-close:hover {
  background: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
}

.modal-close i {
  font-size: 20px;
}

.modal-body {
  padding: 20px;
}

.folder-input {
  width: 100%;
  padding: 14px 16px;
  background: var(--input-bg, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  color: var(--text-primary, #24292f);
  font-size: 15px;
  outline: none;
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.folder-input::placeholder {
  color: var(--text-muted, #8c959f);
}

.folder-input:focus {
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.delete-warning {
  font-size: 15px;
  color: var(--text-primary, #24292f);
  margin: 0 0 8px 0;
}

.delete-warning strong {
  color: var(--accent-red, #cf222e);
}

.delete-hint {
  font-size: 13px;
  color: var(--text-muted, #8c959f);
  margin: 0;
}

.upload-select-area {
  width: 100%;
  padding: 24px 16px;
  border: 2px dashed var(--border-color, #d0d7de);
  border-radius: .375rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
  background: var(--bg-tertiary, #f6f8fa);
  box-sizing: border-box;
  overflow: hidden;
}

.upload-select-area:hover {
  border-color: var(--accent-blue, #0969da);
  background: var(--selected-bg, #ddf4ff);
}

.upload-select-area i {
  font-size: 32px;
  color: var(--accent-blue, #0969da);
  margin-bottom: 12px;
}

.upload-select-area p {
  margin: 0 0 6px 0;
  color: var(--text-primary, #24292f);
  font-size: 14px;
  font-weight: 500;
}

.upload-select-area .upload-link {
  color: var(--accent-blue, #0969da);
  text-decoration: underline;
  font-weight: 600;
  cursor: pointer;
}

.upload-select-area .upload-hint {
  font-size: 12px;
  color: var(--text-muted, #8c959f);
  margin: 0;
}

.file-list-container {
  margin-top: 16px;
  background: var(--bg-tertiary, #f6f8fa);
  border-radius: .375rem;
  overflow: hidden;
  border: 1px solid var(--border-color, #d0d7de);
}

.file-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--selected-bg, #ddf4ff);
  border-bottom: 1px solid var(--border-color, #d0d7de);
}

.file-list-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
}

.file-clear-all {
  font-size: 13px;
  color: var(--accent-blue, #0969da);
  cursor: pointer;
}

.file-clear-all:hover {
  color: var(--accent-blue-bright, #0550ae);
}

.file-list {
  max-height: 200px;
  overflow-y: auto;
}

.file-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  gap: 12px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
}

.file-item:last-child {
  border-bottom: none;
}

.file-icon {
  font-size: 24px;
  color: var(--text-muted, #8c959f);
  flex-shrink: 0;
}

.file-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.file-name {
  font-size: 14px;
  color: var(--text-primary, #24292f);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-size {
  font-size: 12px;
  color: var(--text-muted, #8c959f);
}

.file-remove {
  font-size: 18px;
  color: var(--text-muted, #8c959f);
  cursor: pointer;
}

.file-remove:hover {
  color: var(--accent-red, #cf222e);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px 24px;
}

.btn-cancel,
.btn-confirm {
  padding: 10px 20px;
  border-radius: .375rem;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel {
  background: var(--hover-bg, #f3f4f6);
  border: 1px solid var(--border-color, #d0d7de);
  color: var(--text-secondary, #57606a);
}

.btn-cancel:hover {
  background: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
}

.btn-confirm {
  background: var(--accent-blue, #0969da);
  border: none;
  color: white;
}

.btn-confirm:hover {
  background: var(--accent-blue-bright, #0550ae);
  box-shadow: 0 4px 15px rgba(9, 105, 218, 0.3);
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(-10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}
</style>
