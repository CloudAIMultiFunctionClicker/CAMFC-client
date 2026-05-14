<template>
  <div class="settings-page">
    <main class="settings-content">
      <div class="settings-panel">
        <h3>下载设置</h3>
        <div class="setting-card">
          <div class="setting-item">
            <div class="setting-label">
              <span class="label-text">自定义下载路径</span>
              <span class="label-desc">文件将下载到指定目录，留空使用系统默认下载目录</span>
            </div>
            <div class="path-control">
              <input
                type="text"
                v-model="downloadPath"
                class="path-input"
                placeholder="点击右侧按钮选择目录"
                readonly
              />
              <button class="action-btn small" @click="selectDownloadPath">选择</button>
              <button
                v-if="downloadPath"
                class="action-btn small danger"
                @click="clearDownloadPath"
              >
                清除
              </button>
            </div>
          </div>
          <div class="path-actions">
            <button class="action-btn secondary" @click="openDownloadFolder">
              <i class="ri-folder-open-line"></i>
              打开下载目录
            </button>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData } from '../components/data/storage.js'
import { open } from '@tauri-apps/plugin-dialog'

const downloadPath = ref('')

const loadDownloadSettings = async () => {
  const { invoke } = await import('@tauri-apps/api/core')
  try {
    const customPath = await invoke('get_custom_download_path')
    downloadPath.value = customPath || ''
  } catch (e) {
    console.warn('获取自定义下载路径失败:', e)
    downloadPath.value = ''
  }
}

const selectDownloadPath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择下载目录'
    })

    if (selected) {
      downloadPath.value = selected
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('set_custom_download_path', { path: selected })
      showToast('下载路径已设置为: ' + selected, '#10b981')
    }
  } catch (e) {
    console.error('选择下载目录失败:', e)
    showToast('选择下载目录失败', '#ef4444')
  }
}

const clearDownloadPath = async () => {
  try {
    downloadPath.value = ''
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('set_custom_download_path', { path: '' })
    showToast('已恢复使用系统默认下载目录', '#10b981')
  } catch (e) {
    console.error('清除下载路径失败:', e)
    showToast('清除下载路径失败', '#ef4444')
  }
}

const openDownloadFolder = async () => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    let targetPath = downloadPath.value

    if (!targetPath) {
      try {
        targetPath = await invoke('get_default_download_path')
      } catch (e) {
        console.error('获取默认下载路径失败:', e)
        showToast('无法确定下载目录', '#f59e0b')
        return
      }
    }

    if (targetPath) {
      await invoke('open_folder', { folderPath: targetPath })
      showToast('已打开下载目录', '#10b981')
    } else {
      showToast('无法确定下载目录', '#f59e0b')
    }
  } catch (e) {
    console.error('打开下载目录失败:', e)
    showToast('打开失败: ' + e.message, '#ef4444')
  }
}

onMounted(() => {
  loadDownloadSettings()
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

.path-control {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 12px;
}

.path-input {
  flex: 1;
  padding: 10px 14px;
  background-color: var(--input-bg, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  color: var(--text-primary, #24292f);
  font-size: 14px;
}

.path-input:focus {
  outline: none;
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.1);
}

.path-input::placeholder {
  color: var(--text-muted, #8c959f);
}

.path-actions {
  margin-top: 12px;
  display: flex;
  gap: 8px;
}

.action-btn {
  padding: 10px 20px;
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: 2px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background-color: var(--hover-bg, #f3f4f6);
  border-color: var(--text-muted, #8c959f);
}

.action-btn.secondary {
  background-color: var(--bg-secondary, #f6f8fa);
  color: var(--text-primary, #24292f);
  border: 1px solid var(--border-color, #d0d7de);
}

.action-btn.danger {
  background-color: var(--danger-btn-bg, #212830);
  color: var(--danger-btn-text, #f85149);
  border: 1px solid var(--danger-btn-border, rgba(248, 81, 73, 0.4));
}

.action-btn.danger:hover {
  background-color: var(--danger-btn-hover-bg, #f85149);
  color: var(--danger-btn-hover-text, white);
  border-color: var(--danger-btn-hover-border, #f85149);
}

.action-btn.danger i,
.action-btn.danger svg {
  color: inherit;
}

.action-btn.small {
  margin-top: 0;
  padding: 6px 14px;
  font-size: 13px;
  border-radius: 2px;
}
</style>
