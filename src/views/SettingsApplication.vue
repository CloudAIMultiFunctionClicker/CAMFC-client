<template>
  <div class="settings-page">
    <main class="settings-content">
      <div class="settings-panel">
        <h3>应用设置</h3>
        <div class="setting-card">
          <h4>窗口关闭行为</h4>
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">点击关闭按钮时</span>
              <span class="label-desc">选择点击窗口右上角关闭按钮时的行为</span>
            </div>
            <div class="close-behavior-options">
              <button
                class="behavior-option"
                :class="{ active: closeBehavior === 'minimize' }"
                @click="setCloseBehavior('minimize')"
              >
                <i class="ri-window-line"></i>
                <span>隐藏到托盘</span>
              </button>
              <button
                class="behavior-option"
                :class="{ active: closeBehavior === 'exit' }"
                @click="setCloseBehavior('exit')"
              >
                <i class="ri-close-circle-line"></i>
                <span>完全关闭</span>
              </button>
              <button
                class="behavior-option"
                :class="{ active: closeBehavior === 'ask' }"
                @click="setCloseBehavior('ask')"
              >
                <i class="ri-question-line"></i>
                <span>每次询问</span>
              </button>
            </div>
          </div>
        </div>

        <div class="setting-card">
          <h4>悬浮窗功能</h4>
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">启用悬浮窗</span>
              <span class="label-desc">显示可拖动的悬浮窗，提供快速访问功能</span>
            </div>
            <div class="setting-control">
              <button
                class="toggle-btn"
                :class="{ active: floatWindowEnabled }"
                @click="toggleFloatWindow"
              >
                <span class="toggle-slider"></span>
              </button>
            </div>
          </div>
        </div>

        <div class="setting-card">
          <h4>截图功能</h4>
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">截图时隐藏主窗口</span>
              <span class="label-desc">截图时自动隐藏主窗口，避免截图中包含主窗口内容</span>
            </div>
            <div class="setting-control">
              <button
                class="toggle-btn"
                :class="{ active: screenshotHideWindow }"
                @click="toggleScreenshotHideWindow"
              >
                <span class="toggle-slider"></span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData, saveAppData } from '../components/data/storage.js'

const closeBehavior = ref('ask')
const floatWindowEnabled = ref(true)
const screenshotHideWindow = ref(true)

const loadSettings = async () => {
  try {

    try {
      const savedCloseBehavior = await loadAppData('close_preference')
      if (savedCloseBehavior) {
        const pref = JSON.parse(savedCloseBehavior)
        closeBehavior.value = pref.preference || 'ask'
      }
    } catch (e) {
      console.warn('加载关闭偏好失败:', e)
      closeBehavior.value = 'ask'
    }

    try {
      const { getFloatWindowEnabled } = await import('../components/data/storage.js')
      floatWindowEnabled.value = await getFloatWindowEnabled()
    } catch (e) {
      console.warn('加载悬浮窗状态失败:', e)
      floatWindowEnabled.value = true
    }

    try {
      const savedScreenshotHideWindow = await loadAppData('screenshot_hide_window')
      if (savedScreenshotHideWindow) {
        screenshotHideWindow.value = JSON.parse(savedScreenshotHideWindow)
      }
    } catch (e) {
      console.warn('加载截图隐藏窗口设置失败:', e)
      screenshotHideWindow.value = true
    }
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

const toggleFloatWindow = async () => {
  floatWindowEnabled.value = !floatWindowEnabled.value
  try {
    const { setFloatWindowEnabled } = await import('../components/data/storage.js')
    await setFloatWindowEnabled(floatWindowEnabled.value)
    const status = floatWindowEnabled.value ? '已启用' : '已禁用'
    showToast(`悬浮窗功能：${status}`, '#3b82f6')
    console.log('[应用设置] 悬浮窗状态已更新:', floatWindowEnabled.value)

    try {
      const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
      console.log('[应用设置] 尝试获取悬浮窗...')
      const floatWindow = await WebviewWindow.getByLabel('float-normal-empty')
      console.log('[应用设置] 悬浮窗获取成功:', floatWindow)
      if (floatWindow) {
        console.log('[应用设置] 发送悬浮窗状态变化事件...')
        await floatWindow.emit('float-window-toggled', floatWindowEnabled.value)
        console.log('[应用设置] 事件发送成功')

        if (floatWindowEnabled.value) {
          const isVisible = await floatWindow.isVisible()
          if (!isVisible) {
            console.log('[应用设置] 悬浮窗被隐藏，正在显示...')
            await floatWindow.show()
            await floatWindow.center()
          }
        }
      } else {
        console.log('[应用设置] 悬浮窗不存在')
        if (floatWindowEnabled.value) {
          console.log('[应用设置] 悬浮窗不存在，尝试创建...')
          try {
            const newFloatWindow = new WebviewWindow('float-normal-empty', {
              url: '/float',
              width: 450,
              height: 60,
              x: 100,
              y: 100,
              alwaysOnTop: true,
              decorations: false,
              transparent: true,
              skipTaskbar: true
            })
            console.log('[应用设置] 悬浮窗创建成功')
          } catch (createError) {
            console.error('[应用设置] 悬浮窗创建失败:', createError)
          }
        }
      }
    } catch (e) {
      console.error('[应用设置] 广播悬浮窗状态失败:', e)
    }
  } catch (e) {
    console.error('切换悬浮窗状态失败:', e)
    showToast('保存设置失败', '#ef4444')
  }
}

const toggleScreenshotHideWindow = async () => {
  screenshotHideWindow.value = !screenshotHideWindow.value
  try {
    await saveAppData('screenshot_hide_window', JSON.stringify(screenshotHideWindow.value))
    const status = screenshotHideWindow.value ? '已启用' : '已禁用'
    showToast(`截图时隐藏主窗口：${status}`, '#3b82f6')
    console.log('[应用设置] 截图隐藏窗口设置已更新:', screenshotHideWindow.value)
  } catch (e) {
    console.error('保存截图隐藏窗口设置失败:', e)
    showToast('保存设置失败', '#ef4444')
  }
}

const setCloseBehavior = async (behavior) => {
  try {
    closeBehavior.value = behavior
    await saveAppData('close_preference', JSON.stringify({ preference: behavior }))
    console.log('[应用设置] 保存关闭偏好:', behavior)
    const text = behavior === 'minimize' ? '隐藏到托盘' : behavior === 'exit' ? '完全关闭' : '每次询问'
    showToast(`关闭行为已设置为：${text}`, '#10b981')
  } catch (e) {
    console.error('[应用设置] 保存关闭偏好失败:', e)
    showToast('保存失败', '#ef4444')
  }
}

onMounted(() => {
  loadSettings()
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

.setting-card {
  background-color: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  padding: 20px;
  margin-bottom: 16px;
}

.setting-card h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
  margin: 0 0 16px 0;
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

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.label-text {
  font-size: 15px;
  font-weight: 500;
}

.label-desc {
  font-size: 13px;
  color: var(--text-muted, #8c959f);
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 8px;
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

.behavior-option {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background-color: var(--bg-tertiary, #f6f8fa);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--text-primary, #24292f);
  font-size: 14px;
}

.behavior-option i {
  font-size: 24px;
  color: var(--text-muted, #8c959f);
  transition: color 0.2s ease;
}

.behavior-option:hover {
  border-color: var(--accent-blue, #0969da);
  background-color: var(--selected-bg, #ddf4ff);
}

.behavior-option:hover i {
  color: var(--accent-blue, #0969da);
}

.behavior-option.active {
  border-color: var(--accent-blue, #0969da);
  background-color: var(--selected-bg, #ddf4ff);
}

.behavior-option.active i {
  color: var(--accent-blue, #0969da);
}

.close-behavior-options {
  display: flex;
  gap: 12px;
  margin-top: 12px;
}
</style>
