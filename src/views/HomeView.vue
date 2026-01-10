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
  // 导入应用头部组件
  import AppHeader from '../components/layout/AppHeader.vue'
  // 导入侧边栏组件
  import Sidebar from '../components/layout/Sidebar.vue'
  // 这个主页视图很简单，就是显示头部、侧边栏和一个内容区域
  // TODO: 未来可能需要在这里添加更多内容，比如文件列表等
</script>

<template>
  <!-- 主页布局 -->
  <AppHeader/>
  
  <!-- 主内容区域容器 -->
  <div class="main-container">
    <!-- 左侧边栏 -->
    <Sidebar/>
    
    <!-- 右侧主要内容区域 - 目前是空的，只是占位 -->
    <div class="content-area">
      <!-- 内容区域占位文本 -->
      <div class="placeholder">
        <i class="ri-file-cloud-line"></i>
        <h3>文件内容区域</h3>
        <p>这里将来会显示文件列表、预览等内容</p>
        <!-- TODO: 添加实际的文件管理界面 -->
        <!-- FIXME: 现在只是个占位，需要实现真正的文件浏览功能 -->
      </div>
    </div>
  </div>
</template>

<style scoped>
  /* 主容器样式 - 使用flex布局 */
  .main-container {
    display: flex;
    width: 100%;
    height: calc(100vh - 64px); /* 减去头部高度 */
    overflow: hidden; /* 防止滚动条出现在容器上 */
  }

  /* 内容区域样式 - 使用CSS变量支持主题切换 */
  .content-area {
    flex: 1; /* 占据剩余空间 */
    background: var(--bg-primary, #0f172a); /* 使用主题主背景色 */
    padding: 24px;
    box-sizing: border-box;
    overflow-y: auto; /* 内容区域可滚动 */
    transition: background 0.3s ease; /* 主题切换时的过渡效果 */
  }

  /* 占位内容样式 - 也使用CSS变量 */
  .placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted, #64748b); /* 使用主题次要文字色 */
    text-align: center;
  }

  .placeholder i {
    font-size: 4rem;
    margin-bottom: 20px;
    color: var(--text-muted, #334155); /* 使用主题次要文字色 */
    opacity: 0.5;
  }

  .placeholder h3 {
    margin: 0 0 12px 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-secondary, #cbd5e1); /* 使用主题次要文字色 */
  }

  .placeholder p {
    margin: 0;
    font-size: 1rem;
    max-width: 400px;
    line-height: 1.6;
  }

  /* 响应式设计 - 小屏幕调整 */
  /* TODO: 在手机上可能需要调整侧边栏和内容的布局 */
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
