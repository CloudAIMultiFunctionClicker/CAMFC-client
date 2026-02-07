<template>
  <!-- 主页布局 -->
  
  <!-- 主内容区域容器 -->
  <div class="main-container">
    <!-- 左侧边栏 -->
    <!-- 监听collapse-change事件来同步状态 -->
    <Sidebar @collapse-change="handleCollapseChange"/>
    
    <!-- 右侧主要内容区域 -->
    <div class="content-area" :class="{ 'expanded': isSidebarCollapsed }">
      <div class="placeholder-container">
        <h1 class="placeholder-title">传输</h1>
        
        <div class="placeholder-content">
          <!-- 占位图标 -->
          <div class="placeholder-icon">📤</div>
          
          <p class="placeholder-message">
            传输功能正在开发中...
          </p>
          
          <p class="placeholder-desc">
            这里将显示文件上传和下载的进度、历史记录等信息。
            <br>
            目前该页面仅为占位，后续会逐步完善。
          </p>
          
          <!-- 返回按钮 -->
          <button class="back-btn" @click="goBack">
            返回文件页面
          </button>
        </div>
        
        <!-- TODO: 这里可以加个进度条或者预计完成时间 -->
        <p class="placeholder-footer">
          TODO: 传输页面的具体功能还需要讨论确定
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
// 传输占位页面
// 就是一个简单的占位页面，显示"功能开发中"
// 设计思路：和HardwareSettings.vue保持一致，简单明了

import { useRouter } from 'vue-router'
import { ref } from 'vue'
// 导入侧边栏组件
import Sidebar from '../components/layout/Sidebar.vue'

const router = useRouter()

// 创建一个响应式的折叠状态，用于控制内容区域的扩展
// 默认是展开的（侧边栏没折叠）
const isSidebarCollapsed = ref(false)

// 处理侧边栏折叠状态变化的函数
// 当Sidebar触发collapse-change事件时调用
const handleCollapseChange = (collapsed) => {
  isSidebarCollapsed.value = collapsed
}

// 返回文件页面
function goBack() {
  console.log('返回文件页面')
  router.push('/fileView')
}

// 注：这个页面没有复杂的逻辑，就是个展示页
// 如果以后要加传输列表、进度条等，再重构
</script>

<style scoped>
/* 主容器样式 - 使用flex布局 */
.main-container {
  display: flex;
  width: 100%;
  height: calc(100vh - 65px); /* 减去头部高度 */
  overflow: hidden; /* 防止滚动条出现在容器上 */
}

/* 内容区域样式 - 使用CSS变量支持主题切换 */
.content-area {
  flex: 1; /* 占据剩余空间 */
  background: var(--bg-primary, #0f172a); /* 使用主题主背景色 */
  padding: 24px;
  margin-left: 0; /* 默认没有左边距 */
  box-sizing: border-box;
  overflow-y: auto; /* 内容区域可滚动 */
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); /* 添加平滑过渡效果，与侧边栏动画保持一致 */
}

/* 当侧边栏收起时，内容区域向左扩展填充空间 */
/* 通过添加负的margin-left来实现平滑的左移效果 */
.content-area.expanded {
  margin-left: -240px; /* 向左移动240px，填充侧边栏的空间 */
  /* 注意：这里用负的margin-left，实际上内容区域会向左移动 */
  /* 配合侧边栏的transform: translateX(-100%)，实现同步的滑动效果 */
}

.placeholder-container {
  padding: 40px 20px;
  max-width: 800px;
  margin: 0 auto;
  text-align: center;
}

.placeholder-title {
  font-size: 28px;
  margin-bottom: 30px;
  color: var(--text-primary);
}

.placeholder-content {
  background-color: var(--bg-secondary);
  border-radius: 12px;
  padding: 40px 30px;
  border: 1px solid var(--border-color);
  margin-bottom: 30px;
}

.placeholder-icon {
  font-size: 64px;
  margin-bottom: 20px;
  /* 图标稍微有点大，但占位页面醒目点也行 */
}

.placeholder-message {
  font-size: 20px;
  color: var(--text-primary);
  margin-bottom: 15px;
  font-weight: 500;
}

.placeholder-desc {
  font-size: 16px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin-bottom: 25px;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.back-btn {
  padding: 12px 30px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  
  &:hover {
    background-color: #4a8bd6;
  }
  
  &:active {
    transform: translateY(1px);
  }
}

.placeholder-footer {
  font-size: 14px;
  color: var(--text-muted);
  font-style: italic;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px dashed var(--border-color);
}

/* 响应式调整 */
@media (max-width: 768px) {
  .main-container {
    height: calc(100vh - 64px);
  }
  
  .content-area {
    padding: 16px;
  }
  
  .placeholder-container {
    padding: 20px 15px;
  }
  
  .placeholder-content {
    padding: 30px 20px;
  }
  
  .placeholder-icon {
    font-size: 48px;
  }
}
</style>
