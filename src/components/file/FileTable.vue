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
import { ls,mkdir,rm } from '../data/fileSystem.js'
import { showToast} from '../layout/showToast.js'
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { batchDownloadFiles, extractFileId } from '../data/download.js'
import { selectAndUploadFile } from '../data/upload.js'


// TODO: 这里要不要把路径编辑功能抽成单独组件？先放一起看看，如果代码太多再考虑

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

// 文件选择相关状态
const selectedFiles = ref(new Set()) // 用Set存储选中的文件路径，因为Set查询更快
const lastSelectedIndex = ref(-1) // 记录上一次选中的文件索引，用于Shift连续选择
const ctrlPressed = ref(false) // 是否按下了Ctrl键
const shiftPressed = ref(false) // 是否按下了Shift键

// 路径编辑相关状态 - 支持点击当前路径手动输入
const isEditingPath = ref(false)
const editingPathValue = ref('')

// 操作按钮点击处理 - 暂时先定义空函数，功能后面再加
const handleListClick = () => {
  console.log('列表视图点击，还没想好要干嘛')
  // 列表视图不就是当前视图吗？有点重复，先留着吧
}

const handleUploadClick = async () => {
  console.log('上传点击，调用上传功能')
  
  try {
    // 调用上传模块的选择文件并上传功能
    const result = await selectAndUploadFile()
    console.log('上传结果:', result)
    
    // 上传成功后刷新文件列表
    if (result && result.success) {
      await fetchFiles(props.currentPath)
    }
  } catch (error) {
    console.error('上传失败:', error)
    // 错误处理已经在selectAndUploadFile内部完成了
  }
}

const handleDownloadClick = async () => {
  console.log('下载点击，处理选中的文件')
  
  const selectedCount = selectedFiles.value.size
  if (selectedCount === 0) {
    showToast('请先选择要下载的文件', '#f59e0b')
    return
  }
  
  // 获取选中的文件信息
  const selectedFileInfos = fileList.value.filter(item => 
    selectedFiles.value.has(item.path)
  )
  
  console.log('选中的文件信息:', selectedFileInfos)
  
  // 提取文件ID - 使用完整的文件路径，因为文件可能在子目录中
  const fileIds = selectedFileInfos.map(file => {
    // 如果是文件夹，不能下载
    if (file.is_dir) {
      console.warn(`跳过文件夹下载: ${file.name}`)
      return null
    }
    
    // 使用完整的文件路径，而不仅仅是文件名
    // 文件路径相对于用户存储目录，例如 "ds/下载.png"
    // 注意：path字段已经包含完整路径，不需要再拼接当前路径
    if (file.path) {
      console.log(`下载文件完整路径: ${file.path}`)
      return file.path
    }
    
    // 如果没有path字段，尝试使用file_id或name
    // 但这种情况应该很少见，因为API应该总是返回path
    console.warn(`文件缺少path字段，使用name作为备用: ${file.name}`)
    return file.file_id || file.name
  }).filter(id => id !== null)
  
  if (fileIds.length === 0) {
    showToast('选中的都是文件夹，请选择文件进行下载', '#f59e0b')
    return
  }
  
  console.log('要下载的文件ID:', fileIds)
  
  // 确认下载
  if (confirm(`确定要下载选中的 ${fileIds.length} 个文件吗？`)) {
    try {
      // 调用批量下载函数
      const results = await batchDownloadFiles(fileIds)
      
      // 显示下载结果
      const successCount = results.filter(r => r.success).length
      const errorCount = results.filter(r => !r.success).length
      
      console.log(`下载完成：${successCount} 成功，${errorCount} 失败`)
      
      if (errorCount > 0) {
        // 如果有失败的文件，显示详细信息
        const errorFiles = results.filter(r => !r.success)
        console.error('下载失败的文件:', errorFiles)
      }
      
    } catch (error) {
      console.error('下载过程中出错:', error)
      showToast(`下载出错: ${error.message}`, '#ef4444')
    }
  }
}

const handleNewFolderClick = async () => {
  console.log('新建文件夹点击')
  // 用prompt输入框，虽然不太好看但简单
  const folderName = prompt('请输入新文件夹名称', '新建文件夹')
  if (folderName && folderName.trim()) {
    console.log('创建文件夹:', folderName, '在当前路径:', props.currentPath)
    
    try {
      // 调用API创建文件夹，创建成功后刷新列表
      // 注意：props.currentPath是字符串，不是ref
      const result = await mkdir(props.currentPath, folderName)
      if (result !== null) {
        showToast(`文件夹 "${folderName}" 创建成功`)
        // 创建成功后刷新文件列表
        await fetchFiles(props.currentPath)
      } else {
        // mkdir返回null表示超时
        showToast('请求超时，请稍后重试', '#f59e0b')
      }
    } catch (error) {
      console.error('创建文件夹失败:', error)
      // TODO: 这里可以根据错误类型显示不同提示
      showToast(`创建失败: ${error.message}`, '#ef4444')
    }
  } else {
    // 用户取消了输入或输入为空
    console.log('用户取消了创建文件夹')
  }
}

const handleDeleteClick = async () => {
  console.log('删除点击')
  const selectedCount = selectedFiles.value.size
  if (selectedCount === 0) {
    // 这里改成showToast，比alert好看点
    showToast('请先选择要删除的文件', '#f59e0b')
    return
  }
  
  // 确认对话框还是用原生的吧，简单
  if (confirm(`确定要删除选中的 ${selectedCount} 个文件吗？`)) {
    console.log('删除选中的文件:', Array.from(selectedFiles.value))
    
    try {
      // 依次删除选中的文件
      let successCount = 0
      let errorCount = 0
      
      for (const file of selectedFiles.value) {
        try {
          const result = await rm(file, true)
          if (result !== null) {
            successCount++
            console.log('删除成功:', file)
          } else {
            // 超时
            errorCount++
            console.warn('删除超时:', file)
          }
        } catch (error) {
          errorCount++
          console.error('删除失败:', file, error)
        }
      }
      
      // 显示结果
      if (successCount > 0) {
        showToast(`成功删除 ${successCount} 个文件${errorCount > 0 ? `，${errorCount} 个失败` : ''}`)
        // 删除成功后刷新文件列表，保持当前路径
        await fetchFiles(props.currentPath)
      } else {
        showToast('删除失败，请重试', '#ef4444')
      }
      
    } catch (error) {
      console.error('删除过程中出错:', error)
      showToast(`删除出错: ${error.message}`, '#ef4444')
    }
  }
}

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
    // 处理错误信息，根据状态码显示不同的提示
    if (err.response) {
      // 服务器返回了错误状态码
      const status = err.response.status
      if (status === 400) {
        error.value = '路径违规'
      } else if (status === 404) {
        error.value = '路径不存在'
      } else {
        error.value = `服务器错误 (${status})`
      }
      console.error('获取文件列表出错 - 状态码:', status, err)
      
    } else if (err.request) {
      // 请求已发出但没有收到响应
      error.value = '网络错误，请检查连接'
      console.error('网络错误:', err)
    } else {
      // 其他错误
      error.value = err.message || '获取文件列表失败'
      console.error('其他错误:', err)
    }
    fileList.value = []
    showToast(error.value,'#ff0000')
  } finally {
    loading.value = false
  }
}



// 点击文件夹进入子目录 - 这里只处理，让父组件知道路径变化
const emit = defineEmits(['path-change'])

const enterFolder = (folderPath) => {
  console.log('点击进入文件夹:', folderPath)
  // 进入新文件夹时清空选择状态，不然上个目录选中的文件在新目录里可能不存在
  selectedFiles.value.clear()
  lastSelectedIndex.value = -1
  emit('path-change', folderPath)
}

// 开始编辑路径 - 点击当前路径时触发
const startEditing = () => {
  isEditingPath.value = true
  // 显示路径时要以'/'开头，空路径显示为'/'，非空路径也加上'/'前缀
  // 这样用户编辑时看到的就是/test1这样的格式
  editingPathValue.value = props.currentPath === '' ? '/' : '/' + props.currentPath
  console.log('开始编辑路径，当前值:', editingPathValue.value)
}

// 确认路径编辑 - 回车或点击确认按钮
const confirmEdit = () => {
  if (!isEditingPath.value) return
  
  let newPath = editingPathValue.value.trim()
  console.log('确认编辑路径，输入值:', newPath)
  
  // 处理输入值：如果输入的是'/'，转为空字符串（根目录）
  if (newPath === '/') {
    newPath = ''
  } else if (newPath.startsWith('/')) {
    // 去掉开头的斜杠，因为API不需要开头的斜杠
    // 用户输入/test1，传到后端应该是test1
    newPath = newPath.substring(1)
  }
  
  // 结束编辑模式
  isEditingPath.value = false
  
  // 如果路径没变，就不发请求了
  if (newPath !== props.currentPath) {
    console.log('路径变化，跳转到:', newPath)
    emit('path-change', newPath)
  }
}

// 取消路径编辑
const cancelEdit = () => {
  isEditingPath.value = false
  console.log('取消路径编辑')
}

// 返回上级目录
const goUp = () => {
  if (!props.currentPath) return // 已经在根目录
  
  // 改用正斜杠作为路径分隔符 - 之前用的反斜杠是Windows风格，不通用
  // 注意：这里要过滤掉空字符串，因为split('/')会在路径开头产生空元素
  const parts = props.currentPath.split('/').filter(p => p !== '')
  parts.pop() // 去掉最后一级
  
  // 重新拼接路径，如果parts空了就返回根目录''
  const newPath = parts.length > 0 ? parts.join('/') : ''
  emit('path-change', newPath)
}

// 监听路径变化，重新获取数据
watch(() => props.currentPath, (newPath) => {
  console.log('路径变化了，重新获取:', newPath)
  // 路径变化时清空选择状态
  selectedFiles.value.clear()
  lastSelectedIndex.value = -1
  fetchFiles(newPath)
})

// 组件挂载时获取初始数据
onMounted(() => {
  fetchFiles(props.currentPath)
  
  // 添加键盘事件监听，用于检测Ctrl和Shift键
  const handleKeyDown = (e) => {
    if (e.key === 'Control' || e.key === 'Meta') { // Meta是Mac的Command键
      ctrlPressed.value = true
    } else if (e.key === 'Shift') {
      shiftPressed.value = true
    }
  }
  
  const handleKeyUp = (e) => {
    if (e.key === 'Control' || e.key === 'Meta') {
      ctrlPressed.value = false
    } else if (e.key === 'Shift') {
      shiftPressed.value = false
    }
  }
  
  // 全局点击事件，点击空白处清空选择（除了Ctrl和Shift操作时）
  // 这里有个问题：如果点击的是按钮或其他可交互元素，不应该清空选择
  // 先简单实现，后面再优化
  const handleGlobalClick = (e) => {
    // 检查点击的是不是文件行，如果是的话就不在这里处理（文件行有自己的点击事件）
    // 主要处理点击表格空白区域的情况
    const isFileRow = e.target.closest('.table-row') !== null
    const isClickableElement = e.target.closest('button') !== null || 
                              e.target.closest('input') !== null ||
                              e.target.closest('.current-path') !== null
    
    if (!isFileRow && !isClickableElement && !ctrlPressed.value && !shiftPressed.value) {
      // 点击空白处且没有按Ctrl/Shift，清空选择
      selectedFiles.value.clear()
      lastSelectedIndex.value = -1
    }
  }
  
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('keyup', handleKeyUp)
  window.addEventListener('click', handleGlobalClick)
  
  // 组件卸载时清理事件监听
  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown)
    window.removeEventListener('keyup', handleKeyUp)
    window.removeEventListener('click', handleGlobalClick)
  })
})

// 点击其他地方取消编辑 - 简单处理，先不弄，用ESC键取消就行
// TODO: 可以加个点击外部关闭编辑的功能，但需要处理事件冒泡，有点麻烦

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

// 文件选择逻辑 - 处理三种情况：普通点击、Ctrl+点击、Shift+点击
const handleFileClick = (item, index, event) => {
  if (loading.value) return // 加载时不能点击
  if (event) event.stopPropagation() // 阻止事件冒泡
  
  const itemPath = item.path
  /*
  
  // 处理Shift+点击（连续选择）
  if (shiftPressed.value && lastSelectedIndex.value !== -1) {
    // 找到开始和结束索引（从小到大）
    const start = Math.min(lastSelectedIndex.value, index)
    const end = Math.max(lastSelectedIndex.value, index)
    
    // 清除选择（除非按着Ctrl，但Shift+Ctrl组合比较复杂，先不处理）
    if (!ctrlPressed.value) {
      selectedFiles.value.clear()
    }
    
    // 选择这个范围内的所有文件
    for (let i = start; i <= end; i++) {
      if (i < fileList.value.length) {
        selectedFiles.value.add(fileList.value[i].path)
      }
    }
    
    // 更新最后一次选中的索引
    lastSelectedIndex.value = index
    return
  }
  
  // 处理Ctrl+点击（多选/取消选择）
  if (ctrlPressed.value) {
    if (selectedFiles.value.has(itemPath)) {
      // 已经选中了，就取消选择
      selectedFiles.value.delete(itemPath)
      // 这里有个问题：如果取消了最后一个选中的，lastSelectedIndex不好处理
      // 先简单设为-1，后面可能会出bug
      if (selectedFiles.value.size === 0) {
        lastSelectedIndex.value = -1
      }
    } else {
      // 没选中，就添加选择
      selectedFiles.value.add(itemPath)
      lastSelectedIndex.value = index
    }
    return
  }
  */
  // 普通点击（单选）
  selectedFiles.value.clear()
  selectedFiles.value.add(itemPath)
  lastSelectedIndex.value = index
}

// 检查文件是否被选中
const isFileSelected = (itemPath) => {
  return selectedFiles.value.has(itemPath)
}
</script>

<template>
  <div class="file-table-container">
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
          ref="pathInputRef"
          autofocus
        />
        <button @click="confirmEdit" class="path-confirm-btn">
          <i class="ri-check-line"></i>
        </button>
        <button @click="cancelEdit" class="path-cancel-btn">
          <i class="ri-close-line"></i>
        </button>
      </div>
      
      <!-- 路径显示模式（可点击） -->
      <div v-else class="current-path" @click="startEditing">
        {{ currentPath === '' ? '/' : currentPath }}
        <i class="ri-edit-line edit-icon" title="点击编辑路径"></i>
      </div>
      
      <!-- 操作按钮区域 - 路径编辑模式下隐藏 -->
      <div v-if="!isEditingPath" class="operation-buttons">
        <button class="btn-dropdown" @click="handleListClick">
          <i class="ri-list-view"></i>
          <span class="btn-text">列表视图</span>
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



    <!-- 文件表格 - 始终显示，加载时加上遮罩 -->
    <div class="file-table" :class="{ 'loading-overlay': loading }">
      <!-- 加载遮罩 -->
      <div v-if="loading" class="loading-overlay-content">
        <i class="ri-loader-4-line spin"></i>
        <span>加载中...</span>
      </div>
      
      <!-- 表头 -->
      <div class="table-header">
        <div class="header-cell name">名称</div>
        <div class="header-cell type">类型</div>
        <div class="header-cell size">大小</div>
        <div class="header-cell time">修改时间</div>
      </div>

      <!-- 空状态 -->
      <div v-if="fileList.length === 0 && !loading" class="empty-state">
        <i class="ri-folder-open-line"></i>
        <p>这个目录是空的</p>
      </div>

      <!-- 文件列表 -->
      <div v-else class="table-body">
        <div 
          v-for="(item, index) in fileList" 
          :key="item.path" 
          class="table-row" 
          @click="(e) => handleFileClick(item, index, e)"
          @dblclick="item.is_dir && !loading ? enterFolder(item.path) : null"
          :class="{ 
            'is-dir': item.is_dir, 
            'is-file': item.is_file, 
            'loading-disabled': loading,
            'selected': isFileSelected(item.path)
          }"
        >
          <div class="cell name">
            <i :class="item.is_dir ? 'ri-folder-line' : 'ri-file-line'"></i>
            <span class="file-name">{{ item.name }}</span>
            <!-- 如果是文件夹，可以点击 -->
            <button 
              v-if="item.is_dir" 
              @click="!loading && enterFolder(item.path)"
              class="enter-btn"
              title="进入文件夹"
              :disabled="loading"
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
      <span v-if="selectedFiles.size > 0">已选中 {{ selectedFiles.size }} 个</span>
      <span v-if="currentPath !== ''">路径: /{{ currentPath }}</span>
      <span v-else>路径: /</span>
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
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  /* 添加悬停效果，让用户知道可以点击 */
  transition: color 0.2s ease;
}

.current-path:hover {
  color: var(--accent-blue);
}

.edit-icon {
  font-size: 12px;
  opacity: 0.5;
  transition: opacity 0.2s ease;
}

.current-path:hover .edit-icon {
  opacity: 1;
}

/* 路径编辑容器 */
.path-edit-container {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 6px 12px;
  border: 1px solid var(--accent-blue);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s ease;
}

.path-input:focus {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 2px rgba(var(--accent-blue-rgb), 0.2);
}

.path-confirm-btn,
.path-cancel-btn {
  background: none;
  border: none;
  padding: 6px;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
}

.path-confirm-btn {
  color: var(--accent-blue);
}

.path-confirm-btn:hover {
  background: rgba(var(--accent-blue-rgb), 0.1);
}

.path-cancel-btn {
  color: var(--text-muted);
}

.path-cancel-btn:hover {
  background: rgba(var(--text-muted), 0.1);
}

.spin {
  animation: spin 1s linear infinite;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px;
  background: var(--bg-secondary);
  border-radius: 8px;
  margin-bottom: 16px;
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

/* 淡入动画，给空状态用 */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.retry-btn {
  background: var(--accent-red);
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
}

/* 表格样式 - 添加渐变效果解决闪烁问题 */
.file-table {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-secondary);
  border-radius: 8px;
  /* 给整个表格添加淡入淡出效果，解决内容切换时的闪烁 */
  transition: opacity 0.3s ease;
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
  /* 给行添加背景色变化的渐变效果，让悬停更平滑 */
  transition: background-color 0.2s ease;
}

.table-row:hover {
  background: var(--hover-bg);
}

.table-row.is-dir {
  cursor: pointer;
}

/* 选中状态 - 给选中的行添加明显的背景色 */
.table-row.selected {
  background: rgba(var(--accent-blue-rgb), 0.15) !important;
  border-left: 3px solid var(--accent-blue);
}

.table-row.selected:hover {
  background: rgba(var(--accent-blue-rgb), 0.25) !important;
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

/* 空状态 - 添加淡入动画 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-muted);
  text-align: center;
  /* 空状态出现时的淡入效果 */
  animation: fadeIn 0.5s ease;
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

/* 操作按钮区域 */
.operation-buttons {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: auto; /* 靠右对齐 */
  flex-wrap: nowrap;
}

/* 按钮基础样式 - 参考AppHeader的样式 */
.btn-dropdown,
.btn-upload,
.btn-download,
.btn-new-folder,
.btn-delete {
  border: none;
  border-radius: 8px;
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
  white-space: nowrap; /* 防止文字换行 */
}

/* 下拉按钮 - 中性色 */
.btn-dropdown {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.08));
  color: var(--text-secondary, #cbd5e1);
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

/* 上传按钮 - 主操作按钮 */
.btn-upload {
  background: linear-gradient(135deg, var(--accent-blue, #3b82f6) 0%, #1d4ed8 100%);
  color: white;
  border: none;
  box-shadow: 0 2px 10px rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
}

/* 下载按钮 - 深蓝色 */
.btn-download {
  background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.2);
  color: var(--text-secondary);
  border: 1px solid rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
}

/* 新建文件夹按钮 - 绿色 */
.btn-new-folder {
  background-color: rgba(var(--accent-green-rgb, 40, 167, 69), 0.2);
  color: var(--text-secondary);
  border: 1px solid rgba(var(--accent-green-rgb, 40, 167, 69), 0.3);
}

/* 删除按钮 - 红色 */
.btn-delete {
  background-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.8);
  color: white;
  border: 1px solid rgba(var(--accent-red-rgb, 220, 53, 69), 0.3);
}

/* 按钮hover效果 */
.btn-dropdown:hover {
  background-color: var(--accent-blue, #3b82f6);
  color: white;
  border-color: var(--accent-blue, #3b82f6);
}

.btn-upload:hover {
  background: linear-gradient(135deg, #4a94ff 0%, #2563eb 100%);
  box-shadow: 0 4px 15px rgba(var(--accent-blue-rgb, 59, 130, 246), 0.4);
}

.btn-download:hover {
  background-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.3);
  border-color: rgba(var(--accent-blue-rgb, 59, 130, 246), 0.5);
  color:white
}

.btn-new-folder:hover {
  background-color: rgba(var(--accent-green-rgb, 40, 167, 69), 0.3);
  border-color: rgba(var(--accent-green-rgb, 40, 167, 69), 0.5);
  color:white
}

.btn-delete:hover {
  background-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.95);
  border-color: rgba(var(--accent-red-rgb, 220, 53, 69), 0.5);
}

/* 按钮文字 - 响应式隐藏 */
.btn-text {
  display: inline;
}

/* 加载遮罩效果 */
.file-table.loading-overlay {
  position: relative;
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
  border-radius: 8px;
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
  
  .btn-dropdown,
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
  
  .loading-overlay-content {
    padding: 20px;
  }
  
  .loading-overlay-content i {
    font-size: 20px;
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
  
  .loading-overlay-content {
    padding: 15px;
    font-size: 14px;
  }
  
  .loading-overlay-content i {
    font-size: 18px;
  }
}
</style>
