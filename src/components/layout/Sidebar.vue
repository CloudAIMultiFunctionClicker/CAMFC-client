<!--
左侧边栏组件 - 主要导航
功能：折叠/展开，显示文件管理和传输入口
注意：悬浮按钮只在收起时显示
-->

<script setup>
import { ref } from 'vue'

// 折叠状态 - 默认展开
const isCollapsed = ref(false)

const emit = defineEmits(['collapse-change'])

// 切换折叠
const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
  emit('collapse-change', isCollapsed.value)
}
</script>

<template>
  <!-- 悬浮按钮 - 只在收起时显示 -->
  <button
    v-if="isCollapsed"
    class="float-collapse-btn"
    @click="toggleCollapse"
    title="展开侧边栏"
  >
    <i class="ri-side-bar-line"></i>
  </button>

  <!-- 侧边栏容器 -->
  <aside class="sidebar" :class="{ collapsed: isCollapsed }">
    <!-- Logo 区域 -->
    <div class="logo-area">
      <h2>
        <i class="ri-folder-line"></i>
        <span>云盘</span>
      </h2>
      <!-- 用量进度条 -->
      <div class="storage-usage">
        <div class="usage-label">
          <span>云空间用量</span>
          <span class="usage-text">不限容量</span>
        </div>
        <div class="usage-bar">
          <div class="usage-progress unlimited"></div>
        </div>
      </div>

      <!-- 折叠按钮 -->
      <button class="collapse-btn" @click="toggleCollapse" title="收起侧边栏">
        <i class="ri-arrow-left-s-line"></i>
      </button>
    </div>

    <!-- 主菜单 -->
    <nav class="main-menu">
      <h3 class="menu-title">
        <i class="ri-cloud-line"></i>
        云盘
      </h3>

      <ul class="menu-list">
        <li class="menu-item">
          <router-link to="/fileView" class="menu-link">
            <i class="ri-folder-line"></i>
            <span>文件</span>
          </router-link>
        </li>
        
        <li class="menu-item">
          <router-link to="/transfer" class="menu-link">
            <i class="ri-exchange-line"></i>
            <span>传输</span>
          </router-link>
        </li>
      </ul>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 240px;
  height: calc(100vh - 65px);
  background: var(--bg-sidebar, #161b22);
  border-right: 1px solid var(--border-color, #30363d);
  display: flex;
  flex-direction: column;
  padding: 20px 0;
  box-sizing: border-box;
  position: relative;
  z-index: 900;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow-y: auto;
}

/* 自定义滚动条 */
.sidebar::-webkit-scrollbar {
  width: 6px;
}

.sidebar::-webkit-scrollbar-track {
  background: transparent;
}

.sidebar::-webkit-scrollbar-thumb {
  background: var(--border-color, #30363d);
  border-radius: .375rem;
}

.sidebar::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted, #8b949e);
}

/* 折叠状态 - 滑出屏幕左边 */
.sidebar.collapsed {
  width: 240px;
  opacity: 0;
  transform: translateX(-100%);
  overflow: hidden;
  padding: 0;
  border-right: none;
  pointer-events: none;
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 悬浮按钮 */
.float-collapse-btn {
  position: fixed;
  left: 16px;
  top: 80px;
  background: var(--bg-sidebar, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  width: 36px;
  height: 36px;
  border-radius: .375rem;
  color: var(--text-secondary, #57606a);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  opacity: 0.9;
}

.float-collapse-btn:hover {
  background: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  opacity: 1;
}

.float-collapse-btn i {
  font-size: 1.3rem;
  transition: transform 0.3s ease;
}

.float-collapse-btn {
  animation: floatIn 0.3s ease-out;
}

@keyframes floatIn {
  from {
    opacity: 0;
    transform: translateX(-10px);
  }
  to {
    opacity: 0.9;
    transform: translateX(0);
  }
}

/* Logo 区域 */
.logo-area {
  padding: 0 20px 20px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
  margin-bottom: 20px;
  position: relative;
}

.collapse-btn {
  position: absolute;
  right: 12px;
  top: 12px;
  background: var(--hover-bg, #f3f4f6);
  border: none;
  width: 28px;
  height: 28px;
  border-radius: .375rem;
  color: var(--text-secondary, #57606a);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.collapse-btn:hover {
  background: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
  transform: rotate(15deg);
}

.collapse-btn i {
  font-size: 1.2rem;
  transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 折叠状态下按钮旋转 */
.sidebar:not(.collapsed) .collapse-btn i {
  transform: rotate(0deg);
}

.sidebar.collapsed .collapse-btn i {
  transform: rotate(180deg);
}

.logo-area h2 {
  margin: 0;
  color: var(--text-primary, #24292f);
  font-size: 1.25rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
}

.logo-area h2 i {
  font-size: 1.5rem;
  color: var(--accent-blue, #0969da);
}

/* 用量进度条 */
.storage-usage {
  padding: 8px 0;
}

.usage-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
  font-size: 12px;
  color: var(--text-muted, #8c959f);
}

.usage-text {
  color: var(--accent-blue, #0969da);
  font-weight: 500;
}

.usage-bar {
  height: 6px;
  background: var(--bg-tertiary, #f6f8fa);
  border-radius: .375rem;
  overflow: hidden;
}

.usage-progress {
  height: 100%;
  border-radius: .375rem;
  transition: width 0.3s ease;
}

.usage-progress.unlimited {
  width: 100%;
  background: var(--accent-green, #2da44e);
}

/* 菜单样式 */
.main-menu {
  padding: 0 20px;
  margin-bottom: 24px;
}

.menu-title {
  margin: 0 0 12px 0;
  color: var(--text-secondary, #57606a);
  font-size: 0.875rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  display: flex;
  align-items: center;
  gap: 8px;
}

.menu-title i {
  font-size: 1rem;
  opacity: 0.7;
}

.menu-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.menu-item {
  margin-bottom: 4px;
}

.menu-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  color: var(--text-secondary, #57606a);
  text-decoration: none;
  border-radius: .375rem;
  transition: all 0.2s ease;
  font-size: 0.9375rem;
}

.menu-link i {
  font-size: 1.125rem;
  width: 24px;
  display: flex;
  justify-content: center;
}

.menu-link:hover {
  background-color: var(--hover-bg, #f3f4f6);
  color: var(--text-primary, #24292f);
}

.menu-link.router-link-active {
  background-color: var(--selected-bg, #ddf4ff);
  color: var(--accent-blue, #0969da);
  font-weight: 500;
}

.menu-link.router-link-active i {
  color: var(--accent-blue, #0969da);
}

/* 响应式 */
@media (max-width: 1024px) {
  .sidebar {
    width: 200px;
  }
}

@media (max-width: 768px) {
  .sidebar {
    width: 200px;
  }

  .float-collapse-btn {
    top: 70px;
    left: 8px;
    width: 36px;
    height: 36px;
  }
}
</style>