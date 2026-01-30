<template>
  <div class="settings-page">
    <aside class="settings-sidebar">
      <h2 class="sidebar-title">设置</h2>
      <nav class="settings-nav">
        <button
          v-for="item in navItems"
          :key="item.id"
          class="nav-item"
          :class="{ active: activeNav === item.id }"
          @click="activeNav = item.id"
        >
          <i :class="item.icon"></i>
          <span>{{ item.label }}</span>
        </button>
      </nav>
    </aside>

    <main class="settings-content">
      <div v-if="activeNav === 'cpen'" class="settings-panel">
        <h3>Cpen 设置</h3>
        <p class="placeholder-text">Cpen 相关设置项占位...</p>
      </div>

      <div v-else-if="activeNav === 'account'" class="settings-panel">
        <h3>账户</h3>
        <p class="placeholder-text">账户管理相关设置占位...</p>
      </div>

      <div v-else-if="activeNav === 'ui'" class="settings-panel">
        <h3>界面缩放和布局</h3>
        <p class="placeholder-text">界面缩放、布局调整设置占位...</p>
      </div>

      <div v-else-if="activeNav === 'theme'" class="settings-panel">
        <h3>深色模式</h3>
        <div class="setting-item">
          <span>启用深色模式</span>
          <button class="toggle-btn" :class="{ active: !theme?.isLightMode.value }" @click="theme?.toggleTheme()">
            <span class="toggle-slider"></span>
          </button>
        </div>
      </div>

      <div v-else-if="activeNav === 'storage'" class="settings-panel">
        <h3>储存空间管理</h3>
        <p class="placeholder-text">储存空间管理设置占位...</p>
      </div>

      <div v-else-if="activeNav === 'help'" class="settings-panel help-panel">
        <h3>帮助与反馈</h3>
        <div class="iframe-container">
          <iframe src="https://cn.bing.com/" title="帮助与反馈"></iframe>
        </div>
      </div>

      <div v-else-if="activeNav === 'about'" class="settings-panel">
        <h3>关于</h3>
        <p class="placeholder-text">关于页面内容占位...</p>
      </div>
    </main>
  </div>
</template>

<script setup>
import { inject, ref } from 'vue'

const theme = inject('theme')
const activeNav = ref('cpen')

const navItems = [
  { id: 'cpen', label: 'Cpen 设置', icon: 'ri-settings-3-line' },
  { id: 'account', label: '账户', icon: 'ri-user-line' },
  { id: 'ui', label: '界面缩放和布局', icon: 'ri-layout-grid-line' },
  { id: 'theme', label: '深色模式', icon: 'ri-moon-line' },
  { id: 'storage', label: '储存空间管理', icon: 'ri-hard-drive-line' },
  { id: 'help', label: '帮助与反馈', icon: 'ri-question-line' },
  { id: 'about', label: '关于', icon: 'ri-information-line' }
]
</script>

<style scoped>
.settings-page {
  display: flex;
  min-height: 100vh;
  background-color: var(--bg-primary, #0f172a);
}

.settings-sidebar {
  width: 260px;
  background-color: var(--bg-secondary, #1e293b);
  border-right: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  padding: 24px 16px;
  flex-shrink: 0;
}

.sidebar-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #f1f5f9);
  margin: 0 0 24px 8px;
  padding: 0 8px;
}

.settings-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 12px 16px;
  background: none;
  border: none;
  border-radius: 8px;
  color: var(--text-secondary, #94a3b8);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
}

.nav-item:hover {
  background-color: var(--hover-bg, rgba(255, 255, 255, 0.05));
  color: var(--text-primary, #f1f5f9);
}

.nav-item.active {
  background-color: var(--accent-blue, #3b82f6);
  color: white;
}

.nav-item i {
  font-size: 18px;
  width: 20px;
  text-align: center;
}

.settings-content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
}

.settings-panel {
  max-width: 600px;
}

.settings-panel h3 {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #f1f5f9);
  margin: 0 0 24px 0;
}

.placeholder-text {
  color: var(--text-muted, #64748b);
  font-size: 15px;
  line-height: 1.6;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background-color: var(--bg-secondary, #1e293b);
  border-radius: 12px;
  margin-bottom: 12px;
  color: var(--text-primary, #f1f5f9);
  font-size: 15px;
}

.toggle-btn {
  position: relative;
  width: 48px;
  height: 26px;
  background-color: var(--border-color, rgba(255, 255, 255, 0.2));
  border: none;
  border-radius: 13px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.toggle-btn.active {
  background-color: var(--accent-blue, #3b82f6);
}

.toggle-slider {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 20px;
  height: 20px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.3s ease;
}

.toggle-btn.active .toggle-slider {
  transform: translateX(22px);
}

.help-panel {
  max-width: 100%;
  height: calc(100vh - 150px);
}

.help-panel h3 {
  margin-bottom: 16px;
}

.iframe-container {
  width: 100%;
  height: calc(100% - 40px);
  border-radius: 12px;
  overflow: hidden;
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
}

.iframe-container iframe {
  width: 100%;
  height: 100%;
  border: none;
  background-color: white;
}

@media (max-width: 768px) {
  .settings-page {
    flex-direction: column;
  }

  .settings-sidebar {
    width: 100%;
    padding: 16px;
    border-right: none;
    border-bottom: 1px solid var(--border-color, rgba(255, 255, 255, 0.1));
  }

  .settings-nav {
    flex-direction: row;
    flex-wrap: wrap;
    gap: 8px;
  }

  .nav-item {
    padding: 8px 12px;
    font-size: 13px;
  }

  .nav-item span {
    display: none;
  }

  .settings-content {
    padding: 20px;
  }

  .help-panel {
    height: calc(100vh - 250px);
  }
}
</style>
