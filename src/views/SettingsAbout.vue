<template>
  <div class="settings-page">
    <main class="settings-content">
      <div class="settings-panel">
        <h3>关于</h3>
        <div class="about-info">
          <Cloud :size="64" class="app-icon" />
          <h4>CAMFC Cloud</h4>
          <p class="version">版本 1.0.0</p>
          <p class="desc">云端多功能点击器客户端</p>
        </div>
        <div class="setting-item">
          <span>检查更新</span>
          <span class="setting-value">已是最新</span>
        </div>
        <button class="action-btn" @click="openChangelog">查看更新日志</button>

        <!-- 开源软件声明 -->
        <div class="setting-card" style="margin-top: 24px;">
          <h4>开源软件声明</h4>
          <p class="opensource-desc">本软件使用了以下开源项目，感谢这些项目的贡献者：</p>
          <div class="opensource-list">
            <div class="opensource-item">
              <span class="lib-name" @click="openUrl('https://github.com/vuejs/vue')">Vue.js</span>
              <span class="lib-license" @click="openUrl('https://github.com/vuejs/vue/blob/main/LICENSE')">MIT License</span>
            </div>
            <div class="opensource-item">
              <span class="lib-name" @click="openUrl('https://github.com/tauri-apps/tauri')">Tauri</span>
              <span class="lib-license" @click="openUrl('https://github.com/tauri-apps/tauri/blob/dev/LICENSE')">MIT License</span>
            </div>
            <div class="opensource-item">
              <span class="lib-name" @click="openUrl('https://github.com/lucide-icons/lucide')">Lucide Icons</span>
              <span class="lib-license" @click="openUrl('https://github.com/lucide-icons/lucide/blob/main/LICENSE')">ISC License</span>
            </div>
            <div class="opensource-item">
              <span class="lib-name" @click="openUrl('https://github.com/Remix-Design/RemixIcon')">Remix Icon</span>
              <span class="lib-license" @click="openUrl('https://github.com/Remix-Design/RemixIcon/blob/master/License')">Apache-2.0</span>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { openUrl } from '@tauri-apps/plugin-opener'
import { Cloud } from 'lucide-vue-next'

const openChangelog = () => {
  openUrl('https://github.com/CloudAIMultiFunctionClicker/CAMFC-client/releases/')
}
</script>

<style scoped>
.settings-page {
  display: flex;
  height: 100%;
  background-color: var(--bg-primary, #ffffff);
  overflow: hidden;
}

.settings-content {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
  background-color: var(--bg-primary, #ffffff);
  height: 100%;
}

.settings-panel {
  width: 100%;
  max-width: 800px;
}

.settings-panel h3 {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 24px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-color, #d0d7de);
}

.about-info {
  text-align: center;
  padding: 32px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  margin-bottom: 24px;
}

.about-info h4 {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 8px 0;
}

.about-info .version {
  color: var(--accent-blue, #0969da);
  font-size: 14px;
  font-weight: 500;
  margin: 0 0 8px 0;
}

.about-info .desc {
  color: var(--text-muted, #8c959f);
  font-size: 14px;
  margin: 0;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  margin-bottom: 12px;
  color: var(--text-primary, #24292f);
  font-size: 15px;
}

.setting-value {
  color: var(--text-muted, #8c959f);
  font-size: 14px;
}

.action-btn {
  margin-top: 16px;
  padding: 10px 20px;
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
}

.setting-card {
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  padding: 20px;
  margin-bottom: 16px;
}

.setting-card h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 16px 0;
}

.opensource-desc {
  font-size: 14px;
  color: var(--text-secondary, #57606a);
  margin: 0 0 16px 0;
}

.opensource-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.opensource-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.opensource-item:hover {
  border-color: var(--accent-blue, #0969da);
  background-color: var(--selected-bg, #ddf4ff);
}

.lib-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--accent-blue, #0969da);
  cursor: pointer;
  transition: color 0.2s ease;
}

.lib-name:hover {
  text-decoration: underline;
}

.lib-license {
  font-size: 12px;
  color: var(--text-muted, #8c959f);
  font-family: monospace;
  cursor: pointer;
  transition: color 0.2s ease;
}

.lib-license:hover {
  color: var(--accent-blue, #0969da);
  text-decoration: underline;
}

@media (max-width: 768px) {
  .settings-page {
    flex-direction: column;
  }

  .settings-content {
    padding: 20px;
  }
}
</style>
