

<template>
  <div class="dashboard-container">

    <h1 class="dashboard-title">CAMFC</h1>

    <div class="bluetooth-status" :class="statusClass">
      <i :class="statusIcon"></i>
      <span class="status-text">{{ statusText }}</span>
      <button
        v-if="showRescanButton"
        class="rescan-btn"
        @click="rescanDevices"
        :disabled="isScanning"
      >
        <i :class="isScanning ? 'ri-loader-4-line spinning' : 'ri-refresh-line'"></i>
        <span>{{ isScanning ? '扫描中...' : '重新扫描' }}</span>
      </button>
    </div>

    <div class="nav-grid">

      <button class="nav-card file-manager" @click="goToFileView">
        <Cloud :size="48" class="card-icon" />
        <h3 class="card-title">云盘</h3>
      </button>

      <button class="nav-card notes" @click="goToNotes">
        <FileText :size="48" class="card-icon" />
        <h3 class="card-title">笔记</h3>
      </button>

      <button class="nav-card group-manager" @click="goToGroupManager">
        <Users :size="48" class="card-icon" />
        <h3 class="card-title">班级管理</h3>
      </button>

      <button class="nav-card recent-activities" @click="goToRecentActivities">
        <History :size="48" class="card-icon" />
        <h3 class="card-title">最近活动</h3>
      </button>

      <button class="nav-card settings" @click="goToSettings">
        <Settings :size="48" class="card-icon" />
        <h3 class="card-title">设置</h3>
      </button>
    </div>

    <button class="agent-btn" @click="openAgentWindow">
      <Bot :size="24" class="agent-icon" />
      <span>自动执行</span>
    </button>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { Cloud, FileText, Settings, History, Users, Bot } from 'lucide-vue-next'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useBluetoothStore } from '../stores/bluetooth.js'
import { scanCpenDevices } from '../components/data/bluetooth.js'
import { showToast } from '../components/layout/showToast.js'

const router = useRouter()
const bluetoothStore = useBluetoothStore()

const isScanning = ref(false)

const isConnected = computed(() => bluetoothStore.isConnected())
const bluetoothError = computed(() => bluetoothStore.error)

const statusClass = computed(() => {
  if (isConnected.value) return 'connected'
  if (isScanning.value) return 'scanning'
  if (bluetoothError.value) return 'error'
  return 'disconnected'
})

const statusText = computed(() => {
  if (isConnected.value) return '已连接设备'
  if (isScanning.value) return '扫描中...'
  if (bluetoothError.value) return '连接失败'
  return '未连接'
})

const statusIcon = computed(() => {
  if (isConnected.value) return 'ri-bluetooth-fill'
  if (isScanning.value) return 'ri-loader-4-line spinning'
  if (bluetoothError.value) return 'ri-error-warning-line'
  return 'ri-bluetooth-off-line'
})

const showRescanButton = computed(() => {
  return !isConnected.value || bluetoothError.value
})

async function rescanDevices() {
  isScanning.value = true
  try {
    const devices = await scanCpenDevices()
    if (devices.length === 0) {
      showToast('未发现 Cpen 设备')
      router.push('/')
    } else {
      router.push('/')
    }
  } catch (error) {
    console.error('扫描失败:', error)
    showToast('扫描失败: ' + error.message)
    router.push('/')
  } finally {
    isScanning.value = false
  }
}

function goToFileView() {
  console.log('跳转到文件管理页面')
  router.push('/fileView')
}

function goToSettings() {
  console.log('跳转到设置页面')
  router.push('/settings')
}

function goToNotes() {
  console.log('跳转到笔记页面（占位）')
  router.push('/notes')
}

function goToRecentActivities() {
  console.log('跳转到最近活动页面')
  router.push('/recent-activities')
}

function goToGroupManager() {
  console.log('跳转到班级管理页面')
  router.push('/group-manager')
}

async function openAgentWindow() {
  console.log('打开 agent 自动化窗口')

  const agentWindow = new WebviewWindow('agent-window', {
    url: '/agent-window',
    title: '自动执行 - CAMFC',
    width: 600,
    height: 700,
    resizable: true,
    center: true,
    decorations: true,
    maximizable: false,
    fullscreen: false,
  })

  agentWindow.once('tauri://created', () => {
    console.log('agent 窗口已创建')
  })

  agentWindow.once('tauri://error', (e) => {
    console.error('创建 agent 窗口失败:', e)

    const existingWindow = WebviewWindow.getByLabel('agent-window')
    if (existingWindow) {
      existingWindow.show()
      existingWindow.setFocus()
    }
  })
}

</script>

<style scoped>
.dashboard-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: calc(100vh - 48px);
  width: 100%;
  padding: 40px 20px;
  text-align: center;
  box-sizing: border-box;
}

.dashboard-title {
  font-size: 28px;
  margin-bottom: 20px;
  color: var(--text-primary, #f0f6fc);
}

.bluetooth-status {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  margin-bottom: 30px;
  border-radius: 2px;
  border: 1px solid var(--border-color, #30363d);
  background-color: var(--bg-secondary, #161b22);
  transition: all 0.3s ease;
}

.bluetooth-status.connected {
  border-color: var(--accent-green, #2da44e);
  background-color: rgba(45, 164, 78, 0.1);
}

.bluetooth-status.scanning {
  border-color: var(--accent-blue, #0969da);
  background-color: rgba(9, 105, 218, 0.1);
}

.bluetooth-status.error {
  border-color: var(--accent-red, #cf222e);
  background-color: rgba(207, 34, 46, 0.1);
}

.bluetooth-status.disconnected {
  border-color: var(--text-muted, #8c959f);
  background-color: var(--bg-secondary, #161b22);
}

.bluetooth-status i {
  font-size: 20px;
  color: var(--text-primary, #f0f6fc);
}

.bluetooth-status.connected i {
  color: var(--accent-green, #2da44e);
}

.bluetooth-status.scanning i {
  color: var(--accent-blue, #0969da);
}

.bluetooth-status.error i {
  color: var(--accent-red, #cf222e);
}

.status-text {
  font-size: 14px;
  color: var(--text-secondary, #8b949e);
  font-weight: 500;
}

.bluetooth-status.connected .status-text {
  color: var(--accent-green, #2da44e);
}

.bluetooth-status.scanning .status-text {
  color: var(--accent-blue, #0969da);
}

.bluetooth-status.error .status-text {
  color: var(--accent-red, #cf222e);
}

.rescan-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--bg-tertiary, #21262d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 2px;
  color: var(--text-primary, #f0f6fc);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  outline: none;
}

.rescan-btn:hover:not(:disabled) {
  background-color: var(--bg-secondary, #161b22);
  border-color: var(--accent-blue, #0969da);
  color: var(--accent-blue, #0969da);
}

.rescan-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.rescan-btn i {
  font-size: 14px;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.nav-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  max-width: 700px;
  width: 100%;
  margin: 0 auto 40px auto;
  justify-content: center;
}

.nav-card {
  flex: 0 0 calc(33.333% - 14px);
  max-width: 180px;
}

.nav-card {
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 2px;
  padding: 25px 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: center;

  outline: none;
  font-family: inherit;

  &:hover {
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
    border-color: var(--accent-blue, #3178c6);
  }

  &:active {
  }
}

.file-manager:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-blue-rgb, 49, 120, 198), 0.2);
}

.settings:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-green-rgb, 63, 185, 80), 0.2);
}

.notes:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-purple-rgb, 188, 140, 255), 0.2);
}

.group-manager:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-blue-rgb, 31, 119, 198), 0.2);
}

.recent-activities:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-orange-rgb, 245, 158, 11), 0.2);
}

.card-icon {
  font-size: 48px;
  margin-bottom: 15px;
  color: var(--text-primary, #24292f) !important;
}

.card-title {
  font-size: 20px;
  margin-bottom: 10px;
  color: var(--text-primary, #f0f6fc);
}

.card-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 15px;
  line-height: 1.4;
}

.agent-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 28px;
  background-color: var(--accent-purple, #bc8cff);
  color: white;
  border: none;
  border-radius: 2px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(188, 140, 255, 0.2);
}

.agent-btn:hover {
  background-color: #a576e6;
  box-shadow: 0 6px 16px rgba(188, 140, 255, 0.3);
  transform: translateY(-2px);
}

.agent-btn:active {
  transform: translateY(0);
}

.agent-icon {
  color: white !important;
}
.card-hint {
  font-size: 12px;
  color: var(--text-muted);
  font-style: italic;
  margin-top: 10px;
}

.dashboard-footer {
  margin-top: 30px;
  color: var(--text-muted);
  font-size: 14px;
  max-width: 600px;
  line-height: 1.5;
}

@media (max-width: 768px) {
  .nav-grid {
    grid-template-columns: repeat(2, 1fr);
    max-width: 400px;
  }

  .dashboard-title {
    font-size: 24px;
  }
}

</style>