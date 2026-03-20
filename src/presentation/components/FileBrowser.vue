<template>
  <div class="file-browser">
    <div class="path-bar">
      <span 
        v-for="segment in pathSegments" 
        :key="segment.path"
        class="path-segment"
        @click="navigateTo(segment.path)"
      >
        {{ segment.name }}
        <span v-if="segment.path !== currentPath"> / </span>
      </span>
    </div>
    
    <div class="actions">
      <button @click="handleCreateDirectory" class="btn btn-primary">
        创建目录
      </button>
      <input 
        type="file" 
        @change="handleFileUpload" 
        style="display: none" 
        ref="fileInput"
      />
      <button @click="triggerFileUpload" class="btn btn-success">
        上传文件
      </button>
    </div>
    
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
    
    <div class="file-list" v-if="!isLoading">
      <!-- 目录 -->
      <div class="section" v-if="directories.length > 0">
        <h4>目录</h4>
        <ul>
          <li v-for="dir in directories" :key="dir.path" @click="navigateTo(dir.path)">
            <span class="file-icon">📁</span>
            <span class="file-name">{{ dir.name }}</span>
            <div class="file-actions">
              <button @click.stop="handleRename(dir)" class="btn btn-sm btn-secondary">
                重命名
              </button>
              <button @click.stop="handleDelete(dir.path)" class="btn btn-sm btn-danger">
                删除
              </button>
            </div>
          </li>
        </ul>
      </div>
      
      <!-- 文件 -->
      <div class="section" v-if="regularFiles.length > 0">
        <h4>文件</h4>
        <ul>
          <li v-for="file in regularFiles" :key="file.path">
            <span class="file-icon">📄</span>
            <span class="file-name">{{ file.name }}</span>
            <span class="file-size">{{ formatFileSize(file.size) }}</span>
            <div class="file-actions">
              <button @click="handleDownload(file.path)" class="btn btn-sm btn-info">
                下载
              </button>
              <button @click="handleRename(file)" class="btn btn-sm btn-secondary">
                重命名
              </button>
              <button @click="handleDelete(file.path)" class="btn btn-sm btn-danger">
                删除
              </button>
            </div>
          </li>
        </ul>
      </div>
      
      <div v-if="files.length === 0" class="empty">
        目录为空
      </div>
    </div>
    
    <div v-else class="loading">
      加载中...
    </div>
    
    <!-- 进度条 -->
    <div v-if="uploadProgress > 0" class="progress-container">
      <div class="progress">
        <div 
          class="progress-bar" 
          :style="{ width: uploadProgress + '%' }"
        >
          {{ uploadProgress }}%
        </div>
      </div>
      <p>上传中...</p>
    </div>
    
    <div v-if="downloadProgress > 0" class="progress-container">
      <div class="progress">
        <div 
          class="progress-bar progress-bar-info" 
          :style="{ width: downloadProgress + '%' }"
        >
          {{ downloadProgress }}%
        </div>
      </div>
      <p>下载中...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useFileStore } from '../../presentation/stores';
import { formatFileSize } from '../../core';

const fileStore = useFileStore();
const fileInput = ref<HTMLInputElement | null>(null);

const {
  files,
  currentPath,
  isLoading,
  error,
  uploadProgress,
  downloadProgress,
  directories,
  regularFiles,
  pathSegments,
  loadFiles,
  createDirectory,
  deleteFile,
  renameFile,
  uploadFile,
  downloadFile
} = fileStore;

const navigateTo = (path: string) => {
  loadFiles(path);
};

const handleCreateDirectory = async () => {
  const name = prompt('请输入目录名称:');
  if (name && name.trim()) {
    try {
      await createDirectory(name.trim());
    } catch (error) {
      console.error('Create directory error:', error);
    }
  }
};

const triggerFileUpload = () => {
  fileInput.value?.click();
};

const handleFileUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    try {
      const filePath = currentPath === '/' ? `/${file.name}` : `${currentPath}/${file.name}`;
      await uploadFile(filePath, file);
    } catch (error) {
      console.error('Upload file error:', error);
    } finally {
      // 重置文件输入
      if (target) {
        target.value = '';
      }
    }
  }
};

const handleDownload = async (path: string) => {
  try {
    await downloadFile(path);
  } catch (error) {
    console.error('Download file error:', error);
  }
};

const handleRename = async (file: any) => {
  const newName = prompt('请输入新名称:', file.name);
  if (newName && newName.trim() && newName !== file.name) {
    try {
      await renameFile(file.path, newName.trim());
    } catch (error) {
      console.error('Rename file error:', error);
    }
  }
};

const handleDelete = async (path: string) => {
  if (confirm('确定要删除吗？')) {
    try {
      await deleteFile(path);
    } catch (error) {
      console.error('Delete file error:', error);
    }
  }
};

// 初始化加载
loadFiles();
</script>

<style scoped>
.file-browser {
  padding: 20px;
  background: #f5f5f5;
  border-radius: 8px;
  margin-bottom: 20px;
}

.path-bar {
  margin-bottom: 20px;
  padding: 10px;
  background: #fff;
  border-radius: 4px;
  font-size: 14px;
}

.path-segment {
  cursor: pointer;
  color: #007bff;
}

.path-segment:hover {
  text-decoration: underline;
}

.actions {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.error-message {
  color: #dc3545;
  margin: 10px 0;
  padding: 10px;
  background: #f8d7da;
  border-radius: 4px;
}

.file-list {
  background: #fff;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.section {
  margin-bottom: 20px;
}

.section h4 {
  margin-bottom: 10px;
  color: #333;
  border-bottom: 1px solid #ddd;
  padding-bottom: 5px;
}

ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

li {
  display: flex;
  align-items: center;
  padding: 10px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
}

li:hover {
  background: #f8f9fa;
}

.file-icon {
  margin-right: 10px;
  font-size: 18px;
}

.file-name {
  flex: 1;
  font-size: 14px;
}

.file-size {
  margin-right: 15px;
  font-size: 12px;
  color: #666;
  min-width: 80px;
  text-align: right;
}

.file-actions {
  display: flex;
  gap: 5px;
  opacity: 0;
  transition: opacity 0.2s;
}

li:hover .file-actions {
  opacity: 1;
}

.empty {
  text-align: center;
  color: #666;
  padding: 40px;
  font-style: italic;
}

.loading {
  text-align: center;
  color: #666;
  padding: 40px;
  font-style: italic;
}

.progress-container {
  margin-top: 20px;
}

.progress {
  width: 100%;
  height: 20px;
  background: #e9ecef;
  border-radius: 10px;
  overflow: hidden;
  margin-bottom: 5px;
}

.progress-bar {
  height: 100%;
  background: #28a745;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 12px;
  font-weight: bold;
}

.progress-bar-info {
  background: #17a2b8;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-success {
  background: #28a745;
  color: white;
}

.btn-info {
  background: #17a2b8;
  color: white;
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-danger {
  background: #dc3545;
  color: white;
}

.btn-sm {
  padding: 4px 8px;
  font-size: 12px;
}

.btn:hover {
  opacity: 0.8;
}
</style>