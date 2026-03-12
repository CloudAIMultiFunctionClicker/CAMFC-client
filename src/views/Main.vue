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

<template>
  <div class="dashboard-container">
    <!-- 主标题 -->
    <h1 class="dashboard-title">CAMFC</h1>
    
    <!-- 导航按钮网格 -->
    <div class="nav-grid">
      <!-- 云盘按钮 -->
      <button class="nav-card file-manager" @click="goToFileView">
        <Cloud :size="48" class="card-icon" />
        <h3 class="card-title">云盘</h3>
      </button>
      
      <!-- 笔记按钮 -->
      <button class="nav-card notes" @click="goToNotes">
        <FileText :size="48" class="card-icon" />
        <h3 class="card-title">笔记</h3>
      </button>
      
      <!-- 设置按钮 -->
      <button class="nav-card settings" @click="goToSettings">
        <Settings :size="48" class="card-icon" />
        <h3 class="card-title">设置</h3>
      </button>
    </div>
  </div>
</template>

<script setup>
// 仪表板主组件 - 显示四个导航卡片
// 设计思路：简单网格布局，居中显示，使用现有项目的CSS变量
// 后续可以加动画效果，但现在先保证基本功能能用

import { useRouter } from 'vue-router'
import { Cloud, FileText, Settings } from 'lucide-vue-next'

const router = useRouter()

// 跳转到文件管理页面（已有功能）
function goToFileView() {
  console.log('跳转到文件管理页面')
  router.push('/fileView')
}

// 跳转到设置页面
function goToSettings() {
  console.log('跳转到设置页面')
  router.push('/settings')
}

// 跳转到笔记页面（占位页面）
function goToNotes() {
  console.log('跳转到笔记页面（占位）')
  router.push('/notes')
}

// 注：这里没有onMounted之类的生命周期，因为就是个静态导航页
// 如果以后要加数据加载，可以再加
</script>

<style scoped>
.dashboard-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 100px);
  padding: 40px 20px;
  text-align: center;
}

.dashboard-title {
  font-size: 28px;
  margin-bottom: 40px;
  color: var(--text-primary, #f0f6fc);
}

/* 导航网格布局 */
.nav-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 25px;
  max-width: 1000px;
  width: 100%;
  margin-bottom: 40px;
}

/* 导航卡片样式 */
.nav-card {
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  padding: 25px 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: center;
  
  /* 去掉 button 默认样式 */
  outline: none;
  font-family: inherit;
  
  /* 悬停效果 */
  &:hover {
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
    border-color: var(--accent-blue, #3178c6);
  }
  
  &:active {
  }
}

/* 文件管理卡片特殊样式 */
.file-manager:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-blue-rgb, 49, 120, 198), 0.2);
}

/* 设置卡片特殊样式 */
.settings:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-green-rgb, 63, 185, 80), 0.2);
}

/* 笔记卡片特殊样式 */
.notes:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-purple-rgb, 188, 140, 255), 0.2);
}

/* 卡片图标 */
.card-icon {
  font-size: 48px;
  margin-bottom: 15px;
  color: #f0f6fc !important;
}

/* 卡片标题 */
.card-title {
  font-size: 20px;
  margin-bottom: 10px;
  color: var(--text-primary, #f0f6fc);
}

/* 卡片描述 */
.card-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 15px;
  line-height: 1.4;
}

/* 卡片提示 */
.card-hint {
  font-size: 12px;
  color: var(--text-muted);
  font-style: italic;
  margin-top: 10px;
}

/* 底部说明 */
.dashboard-footer {
  margin-top: 30px;
  color: var(--text-muted);
  font-size: 14px;
  max-width: 600px;
  line-height: 1.5;
}

/* 响应式调整 - 小屏幕时改为单列 */
@media (max-width: 768px) {
  .nav-grid {
    grid-template-columns: 1fr;
    max-width: 400px;
  }
  
  .dashboard-title {
    font-size: 24px;
  }
}

/* TODO: 可以加个加载动画或者状态指示，但用户说简单实现，先不加 */
</style>