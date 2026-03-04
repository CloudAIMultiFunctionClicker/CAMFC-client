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

<template>
  <div class="dashboard-container">
    <!-- 主标题 -->
    <h1 class="dashboard-title">CAMFC</h1>
    
    <!-- 导航按钮网格 -->
    <div class="nav-grid">
      <!-- 云盘按钮 -->
      <button class="nav-card file-manager" @click="goToFileView">
        <div class="card-icon">☁️</div>
        <h3 class="card-title">云盘</h3>
        <p class="card-desc">浏览和管理云盘文件</p>
        <div class="card-hint">点击进入</div>
      </button>
      
      <!-- 笔记按钮 -->
      <button class="nav-card notes" @click="goToNotes">
        <div class="card-icon">📝</div>
        <h3 class="card-title">笔记</h3>
        <p class="card-desc">创建和管理笔记</p>
        <div class="card-hint">占位功能</div>
      </button>
      
      <!-- 设置按钮 -->
      <button class="nav-card settings" @click="goToSettings">
        <div class="card-icon">⚙️</div>
        <h3 class="card-title">设置</h3>
        <p class="card-desc">硬件设置、软件设置和更多信息</p>
        <div class="card-hint">点击进入</div>
      </button>
    </div>
    
    <!-- 底部说明 -->
    <p class="dashboard-footer">
      提示：文件管理功能已实现，其他功能暂为占位，后续会逐步完善
      <br>
      <!-- TODO: 这里可以加个版本号或者状态提示 -->
    </p>
  </div>
</template>

<script setup>
// 仪表板主组件 - 显示四个导航卡片
// 设计思路：简单网格布局，居中显示，使用现有项目的CSS变量
// 后续可以加动画效果，但现在先保证基本功能能用

import { useRouter } from 'vue-router'

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
  min-height: calc(100vh - 100px); /* 减去header和一些padding */
  padding: 40px 20px;
  text-align: center;
}

.dashboard-title {
  font-size: 28px;
  margin-bottom: 40px;
  color: var(--text-primary);
  /* 加个渐变效果？先简单点，用纯色 */
}

/* 导航网格布局 */
.nav-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 25px;
  max-width: 1000px;
  width: 100%;
  margin-bottom: 40px;
  
  /* 思考：这里用auto-fit还是固定2x2？auto-fit更响应式
     但用户要求四个居中，可能希望固定2x2？先这样吧，看起来还行 */
}

/* 导航卡片样式 */
.nav-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 25px 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: center;
  
  /* 去掉button默认样式 */
  outline: none;
  font-family: inherit;
  
  /* 悬停效果 */
  &:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
    border-color: var(--accent-blue);
  }
  
  &:active {
    transform: translateY(-2px);
  }
}

/* 文件管理卡片特殊样式 */
.file-manager {
  /* 文件管理是主要功能，给个特殊颜色提示 */
  border-color: var(--accent-blue);
  
  &:hover {
    box-shadow: 0 8px 20px rgba(var(--accent-blue-rgb), 0.2);
  }
}

/* 设置卡片特殊样式 */
.settings {
  border-color: var(--accent-green);
  
  &:hover {
    box-shadow: 0 8px 20px rgba(var(--accent-green-rgb), 0.2);
  }
}

/* 笔记卡片特殊样式 */
.notes {
  border-color: var(--accent-purple);
  
  &:hover {
    box-shadow: 0 8px 20px rgba(var(--accent-purple-rgb), 0.2);
  }
}

/* 卡片图标 */
.card-icon {
  font-size: 48px;
  margin-bottom: 15px;
  /* 图标颜色不用特别设置，用系统默认 */
}

/* 卡片标题 */
.card-title {
  font-size: 20px;
  margin-bottom: 10px;
  color: var(--text-primary);
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