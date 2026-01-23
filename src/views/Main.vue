<template>
  <div class="dashboard-container">
    <!-- 主标题 -->
    <h1 class="dashboard-title">CAMFC 主仪表板</h1>
    
    <!-- 导航按钮网格 -->
    <div class="nav-grid">
      <!-- 文件管理按钮 -->
      <button class="nav-card file-manager" @click="goToFileView">
        <div class="card-icon">📁</div>
        <h3 class="card-title">文件管理</h3>
        <p class="card-desc">浏览和管理Cpen设备文件</p>
        <div class="card-hint">点击进入</div>
      </button>
      
      <!-- 硬件设置按钮 -->
      <button class="nav-card hardware-settings" @click="goToHardwareSettings">
        <div class="card-icon">⚙️</div>
        <h3 class="card-title">硬件设置</h3>
        <p class="card-desc">配置设备硬件参数</p>
        <div class="card-hint">占位功能</div>
      </button>
      
      <!-- 软件设置按钮 -->
      <button class="nav-card software-settings" @click="goToSoftwareSettings">
        <div class="card-icon">🖥️</div>
        <h3 class="card-title">软件设置</h3>
        <p class="card-desc">调整软件偏好和选项</p>
        <div class="card-hint">占位功能</div>
      </button>
      
      <!-- 更多信息按钮 -->
      <button class="nav-card more-info" @click="goToMoreInfo">
        <div class="card-icon">ℹ️</div>
        <h3 class="card-title">更多信息</h3>
        <p class="card-desc">查看关于和帮助信息</p>
        <div class="card-hint">占位功能</div>
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

// 跳转到硬件设置（占位页面）
// TODO: 硬件设置页面还没做，先跳转到占位页
// 思考：要不要直接跳转到现有的设置页面？还是新建专门的硬件设置？
// 先新建占位页吧，保持功能分离
function goToHardwareSettings() {
  console.log('跳转到硬件设置（占位）')
  router.push('/hardware-settings')
}

// 跳转到软件设置（占位页面）
function goToSoftwareSettings() {
  console.log('跳转到软件设置（占位）')
  router.push('/software-settings')
}

// 跳转到更多信息（占位页面）
function goToMoreInfo() {
  console.log('跳转到更多信息（占位）')
  router.push('/more-info')
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