<script setup>
import { ref } from 'vue'
import Sidebar from '../components/layout/Sidebar.vue'
import FileTable from '../components/file/FileTable.vue'

// 侧边栏折叠状态
const isSidebarCollapsed = ref(false)
// 当前文件路径
const currentPath = ref('')

// 处理侧边栏折叠状态变化
const handleCollapseChange = (collapsed) => {
  isSidebarCollapsed.value = collapsed
}

// 处理路径变化
const handlePathChange = (newPath) => {
  currentPath.value = newPath
}
</script>

<template>
  <div class="main-container">
    <Sidebar @collapse-change="handleCollapseChange"/>
    
    <div class="content-area" :class="{ 'expanded': isSidebarCollapsed }">
      <FileTable 
        :currentPath="currentPath"
        @path-change="handlePathChange"
      />
    </div>
  </div>
</template>

<style scoped>
.main-container {
  display: flex;
  width: 100%;
  height: calc(100vh - 65px);
  overflow: hidden;
}

.content-area {
  flex: 1;
  background: var(--bg-primary, #0f172a);
  padding: 24px;
  margin-left: 0;
  box-sizing: border-box;
  overflow-y: auto;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.content-area.expanded {
  margin-left: -240px;
}

@media (max-width: 768px) {
  .main-container {
    height: calc(100vh - 64px);
  }
  
  .content-area {
    padding: 16px;
  }
}
</style>
