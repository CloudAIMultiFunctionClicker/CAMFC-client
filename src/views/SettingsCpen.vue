<template>
  <div class="settings-page">
    <main class="settings-content">
      <div class="settings-panel">
        <h3>Cpen 设置</h3>
        <div class="setting-item">
          <span>设备名称</span>
          <span class="setting-value">{{ deviceName || '未连接' }}</span>
        </div>
        <div class="setting-item">
          <span>设备 ID</span>
          <span class="setting-value">{{ deviceId || '未连接' }}</span>
        </div>
        <button class="action-btn danger" @click="disconnectDevice">断开设备</button>
      </div>
    </main>
  </div>
</template>

<script setup>
import { inject, ref, onMounted } from 'vue'
import { showToast } from '../components/layout/showToast.js'
import { disconnect, getDeviceId } from '../components/data/bluetooth.js'
import { ls } from '../components/data/fileSystem.js'
import { loadAppData } from '../components/data/storage.js'

const theme = inject('theme')

const deviceId = ref(null)
const deviceName = ref(null)

const cpenSettings = ref({
  autoConnect: false,
  lastDeviceAddress: ''
})

const checkFilesystemLogin = async () => {
  try {
    let id = null
    let cloudAccessible = false
    let name = null

    try {
      id = await getDeviceId()
      deviceId.value = id

      const { invoke } = await import('@tauri-apps/api/core')
      const status = await invoke('get_connection_status')
      console.log('连接状态原始值:', status)
      if (status && status.startsWith('已连接')) {
        const colonIndex = status.indexOf(':') !== -1 ? status.indexOf(':') : status.indexOf('：')
        if (colonIndex !== -1 && colonIndex < status.length - 1) {
          name = status.substring(colonIndex + 1).trim()
        } else {
          name = status
        }
      } else if (status && status !== '未连接') {
        name = status
      }
      console.log('提取的设备名:', name)
      deviceName.value = name
    } catch (idError) {
      console.warn('获取设备信息失败:', idError)
    }

    if (id) {
      try {
        const result = await ls('')
        cloudAccessible = result !== null
      } catch (lsError) {
        console.warn('访问云盘失败:', lsError)
        cloudAccessible = false
      }
    }
  } catch (error) {
    console.warn('检查登录状态失败:', error)
    deviceId.value = null
    deviceName.value = null
  }
}

const disconnectDevice = async () => {
  showToast('正在断开设备...', '#f59e0b')
  await disconnect()
  deviceName.value = null
  deviceId.value = null
  showToast('已断开设备连接', '#10b981')
}

const loadSettings = async () => {
  try {
    const savedCpen = await loadAppData('settings_cpen')
    if (savedCpen) {
      cpenSettings.value = JSON.parse(savedCpen)
    }
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

onMounted(() => {
  loadSettings()
  checkFilesystemLogin()
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
</style>
