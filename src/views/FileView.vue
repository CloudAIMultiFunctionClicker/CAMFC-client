<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top

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
  // 导入应用头部组件
  import AppHeader from '../components/layout/AppHeader.vue'
  // 导入侧边栏组件
  import Sidebar from '../components/layout/Sidebar.vue'
  
  // 导入Vue响应式功能
  import { ref } from 'vue'

  // 导入文件表格组件
  import FileTable from '../components/file/FileTable.vue'
  // 原来的FileTree暂时留着，后面可能有用
  import FileTree from '../components/file/FileTree.vue'
  
  // 创建一个响应式的折叠状态，用于控制内容区域的扩展
  // 默认是展开的（侧边栏没折叠）
  const isSidebarCollapsed = ref(false)
  
  // 当前路径 - 默认为空字符串表示根目录
  const currentPath = ref('')
  
  // 处理侧边栏折叠状态变化的函数
  // 当Sidebar触发collapse-change事件时调用
  const handleCollapseChange = (collapsed) => {
    isSidebarCollapsed.value = collapsed
  }
  
  // 处理路径变化
  const handlePathChange = (newPath) => {
    console.log('路径变化:', newPath)
    currentPath.value = newPath
  }
</script>

<template>
  <!-- 主页布局 -->
  
  <!-- 主内容区域容器 -->
  <div class="main-container">
    <!-- 左侧边栏 -->
    <!-- 监听collapse-change事件来同步状态 -->
    <Sidebar @collapse-change="handleCollapseChange"/>
    
    <!-- 右侧主要内容区域 - 目前是空的，只是占位 -->
    <!-- 根据侧边栏折叠状态添加类名 -->
    <div class="content-area" :class="{ 'expanded': isSidebarCollapsed }">
      <!-- 文件表格组件 -->
      <FileTable 
        :currentPath="currentPath"
        @path-change="handlePathChange"
      />
      
      <!-- 原来的占位内容先注释掉，测试用 -->
      <!-- 
      <div class="placeholder">
        <i class="ri-file-cloud-line"></i>
        <h3>文件内容区域</h3>
        <p>这里将来会显示文件列表、预览等内容</p>
        <p class="hint">侧边栏状态：{{ isSidebarCollapsed ? '已收起' : '展开中' }}</p>
        <p class="hint">当前路径：{{ currentPath || '根目录' }}</p>
        <FileTree/>
      </div>
      -->
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
    background: #f5f5f5;
    padding: 24px;
    margin-left: 0;
    box-sizing: border-box;
    overflow-y: auto;
  }

  .content-area.expanded {
    margin-left: -240px;
  }

  .hint {
    margin-top: 10px;
    font-size: 0.9rem;
    color: #999;
    font-style: italic;
  }

  .placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #999;
    text-align: center;
  }

  .placeholder i {
    font-size: 4rem;
    margin-bottom: 20px;
    color: #999;
    opacity: 0.5;
  }

  .placeholder h3 {
    margin: 0 0 12px 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #666;
  }

  .placeholder p {
    margin: 0;
    font-size: 1rem;
    max-width: 400px;
    line-height: 1.6;
  }

  @media (max-width: 768px) {
    .main-container {
      height: calc(100vh - 64px);
    }

    .content-area {
      padding: 16px;
    }

    .placeholder i {
      font-size: 3rem;
    }

    .placeholder h3 {
      font-size: 1.25rem;
    }
  }
</style>
