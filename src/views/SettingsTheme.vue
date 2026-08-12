<template>
  <div class="settings-page">
    <main class="settings-content">
      <div class="settings-panel">
        <h3>深色模式</h3>
        <div class="setting-item">
          <span>启用深色模式</span>
          <button 
            class="toggle-btn" 
            :class="{ active: !theme?.isLightMode.value }" 
            @click="theme?.toggleTheme()"
          >
            <span class="toggle-slider"></span>
          </button>
        </div>
        <div class="setting-item">
          <span>跟随系统主题</span>
          <button 
            class="toggle-btn" 
            :class="{ active: storageSettings.followSystemTheme }"
            @click="toggleFollowSystemTheme"
          >
            <span class="toggle-slider"></span>
          </button>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { inject, ref, onMounted } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData, saveAppData } from '../components/data/storage.js'

const theme = inject('theme')

const storageSettings = ref({
  followSystemTheme: false
})

const toggleFollowSystemTheme = async () => {
  storageSettings.value.followSystemTheme = !storageSettings.value.followSystemTheme
  await saveAppData('settings_storage', JSON.stringify(storageSettings.value))
  
  if (storageSettings.value.followSystemTheme) {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    theme?.setTheme(!mediaQuery.matches)
    mediaQuery.addEventListener('change', handleSystemThemeChange)
  } else {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.removeEventListener('change', handleSystemThemeChange)
  }
}

const handleSystemThemeChange = (e) => {
  if (storageSettings.value.followSystemTheme) {
    theme?.setTheme(!e.matches)
  }
}

onMounted(async () => {
  try {
    const saved = await loadAppData('settings_storage')
    if (saved) {
      storageSettings.value = JSON.parse(saved)
      // 如果已开启跟随系统主题，注册监听
      if (storageSettings.value.followSystemTheme) {
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
        mediaQuery.addEventListener('change', handleSystemThemeChange)
      }
    }
  } catch (error) {
    console.error('加载主题设置失败:', error)
  }
})
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

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  margin-bottom: 12px;
  color: var(--text-primary, #24292f);
  font-size: 15px;
}

.toggle-btn {
  position: relative;
  width: 48px;
  height: 26px;
  background-color: var(--border-color, #d0d7de);
  border: none;
  border-radius: 2px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.toggle-btn.active {
  background-color: var(--accent-blue, #0969da);
}

.toggle-slider {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 20px;
  height: 20px;
  background-color: white;
  border-radius: 2px;
  transition: transform 0.3s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.toggle-btn.active .toggle-slider {
  transform: translateX(22px);
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
