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
  min-height: calc(100vh - 100px);
  padding: 40px 20px;
  text-align: center;
}

.dashboard-title {
  font-size: 2.5rem;
  margin-bottom: 40px;
  color: #333;
}

.nav-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 25px;
  max-width: 800px;
  width: 100%;
  margin-bottom: 40px;
}

.nav-card {
  background-color: #fff;
  border: 1px solid #eee;
  border-radius: 4px;
  padding: 24px 20px;
  cursor: pointer;
  text-align: center;
  outline: none;
  font-family: inherit;
}

.nav-card:hover {
  border-color: #333;
}

.file-manager {
  border-color: #333;
}

.settings {
  border-color: #333;
}

.notes {
  border-color: #333;
}

.card-icon {
  font-size: 48px;
  margin-bottom: 15px;
}

.card-title {
  font-size: 1.5rem;
  margin-bottom: 10px;
  color: #333;
}

.card-desc {
  font-size: 14px;
  color: #666;
  margin-bottom: 15px;
  line-height: 1.4;
}

.card-hint {
  font-size: 12px;
  color: #999;
  font-style: italic;
  margin-top: 10px;
}

.dashboard-footer {
  margin-top: 30px;
  color: #999;
  font-size: 14px;
  max-width: 600px;
  line-height: 1.5;
}

@media (max-width: 768px) {
  .nav-grid {
    grid-template-columns: 1fr;
    max-width: 400px;
  }

  .dashboard-title {
    font-size: 1.5rem;
  }
}
</style>