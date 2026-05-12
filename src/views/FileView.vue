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
  // 导入应用头部组件
  import AppHeader from '../components/layout/AppHeader.vue'
  
  // 导入 Vue 响应式功能
  import { ref } from 'vue'

  // 导入文件表格组件
  import FileTable from '../components/file/FileTable.vue'
  // 原来的 FileTree 暂时留着，后面可能有用
  import FileTree from '../components/file/FileTree.vue'
  
  // 当前路径 - 默认为空字符串表示根目录
  const currentPath = ref('')
  
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
    <!-- 右侧主要内容区域 -->
    <div class="content-area">
      <!-- 文件表格组件 -->
      <FileTable 
        :currentPath="currentPath"
        @path-change="handlePathChange"
      />
    </div>
  </div>
</template>

<style scoped>
  /* 主容器样式 - 使用 flex 布局 */
  .main-container {
    display: flex;
    width: 100%;
    height: calc(100vh - 48px); /* 减去标题栏高度 */
    overflow: hidden;
  }

  /* 内容区域样式 */
  .content-area {
    flex: 1;
    background: var(--bg-primary, #0f172a);
    padding: 24px;
    padding-left: 24px;
    box-sizing: border-box;
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
  }
  
  /* 提示文字样式 */
  .hint {
    margin-top: 10px;
    font-size: 0.9rem;
    color: var(--text-muted, #64748b);
    font-style: italic;
  }

  /* 响应式设计 - 小屏幕调整 */
  @media (max-width: 768px) {
    .main-container {
      height: calc(100vh - 64px);
    }
    
    .content-area {
      padding: 16px;
    }
  }
</style>
