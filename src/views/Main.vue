<!--
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
-->

<template>
  <div class="dashboard-container">
    <!-- 主标题 -->
    <h1 class="dashboard-title">CAMFC</h1>
    
    <!-- 蓝牙状态提示 -->
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
    
    <!-- 导航按钮网格 -->
    <div class="nav-grid">
      <!-- 云盘按钮 -->
      <button class="nav-card file-manager" @click="goToFileView">
        <Cloud :size="48" class="card-icon" />
        <h3 class="card-title">云盘</h3>
      </button>
      
      <!-- 笔记按钮 -->
      <button class="nav-card notes" @click="goToNotes">
        <FileText :size="48" class="card-icon" />
        <h3 class="card-title">笔记</h3>
      </button>
      
      <!-- 班级管理按钮 -->
      <button class="nav-card group-manager" @click="goToGroupManager">
        <Users :size="48" class="card-icon" />
        <h3 class="card-title">班级管理</h3>
      </button>
      
      <!-- 最近活动按钮 -->
      <button class="nav-card recent-activities" @click="goToRecentActivities">
        <History :size="48" class="card-icon" />
        <h3 class="card-title">最近活动</h3>
      </button>
      
      <!-- 设置按钮 -->
      <button class="nav-card settings" @click="goToSettings">
        <Settings :size="48" class="card-icon" />
        <h3 class="card-title">设置</h3>
      </button>
    </div>
    
    <!-- 自动执行按钮 -->
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

// 跳转到文件管理页面（已有功能）
function goToFileView() {
  console.log('跳转到文件管理页面')
  router.push('/fileView')
}

// 跳转到设置页面
function goToSettings() {
  console.log('跳转到设置页面')
  router.push('/settings')
}

// 跳转到笔记页面（占位页面）
function goToNotes() {
  console.log('跳转到笔记页面（占位）')
  router.push('/notes')
}

// 跳转到最近活动页面
function goToRecentActivities() {
  console.log('跳转到最近活动页面')
  router.push('/recent-activities')
}

// 跳转到班级管理页面
function goToGroupManager() {
  console.log('跳转到班级管理页面')
  router.push('/group-manager')
}

// 打开 agent 自动化窗口
async function openAgentWindow() {
  console.log('打开 agent 自动化窗口')
  
  const agentWindow = new WebviewWindow('agent-window', {
    url: '/agent-window',
    title: '自动执行 - CAMFC',
    width: 600,
    height: 700,
    resizable: true,
    center: true,
    decorations: true,   // 启用系统标题栏
    maximizable: false,  // 禁用最大化按钮
    fullscreen: false,   // 禁止全屏
  })
  
  agentWindow.once('tauri://created', () => {
    console.log('agent 窗口已创建')
  })
  
  agentWindow.once('tauri://error', (e) => {
    console.error('创建 agent 窗口失败:', e)
    // 如果窗口已存在，则获取并显示它
    const existingWindow = WebviewWindow.getByLabel('agent-window')
    if (existingWindow) {
      existingWindow.show()
      existingWindow.setFocus()
    }
  })
}

// 注：这里没有 onMounted 之类的生命周期，因为就是个静态导航页
// 如果以后要加数据加载，可以再加
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

/* 蓝牙状态提示 */
.bluetooth-status {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  margin-bottom: 30px;
  border-radius: .375rem;
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

/* 重新扫描按钮 */
.rescan-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background-color: var(--bg-tertiary, #21262d);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
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

/* 导航网格布局 - 5 个按钮：第一行 3 个，第二行 2 个居中 */
.nav-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  max-width: 700px;
  width: 100%;
  margin: 0 auto 40px auto;
  justify-content: center;
}

/* 每个按钮的宽度相同 */
.nav-card {
  flex: 0 0 calc(33.333% - 14px);
  max-width: 180px;
}

/* 第二行的按钮自动居中 */

/* 导航卡片样式 */
.nav-card {
  background-color: var(--bg-secondary, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: .375rem;
  padding: 25px 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: center;
  
  /* 去掉 button 默认样式 */
  outline: none;
  font-family: inherit;
  
  /* 悬停效果 */
  &:hover {
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
    border-color: var(--accent-blue, #3178c6);
  }
  
  &:active {
  }
}

/* 文件管理卡片特殊样式 */
.file-manager:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-blue-rgb, 49, 120, 198), 0.2);
}

/* 设置卡片特殊样式 */
.settings:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-green-rgb, 63, 185, 80), 0.2);
}

/* 笔记卡片特殊样式 */
.notes:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-purple-rgb, 188, 140, 255), 0.2);
}

/* 班级管理卡片特殊样式 */
.group-manager:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-blue-rgb, 31, 119, 198), 0.2);
}

/* 最近活动卡片特殊样式 */
.recent-activities:hover {
  box-shadow: 0 8px 20px rgba(var(--accent-orange-rgb, 245, 158, 11), 0.2);
}

/* 卡片图标 */
.card-icon {
  font-size: 48px;
  margin-bottom: 15px;
  color: var(--text-primary, #24292f) !important;
}

/* 卡片标题 */
.card-title {
  font-size: 20px;
  margin-bottom: 10px;
  color: var(--text-primary, #f0f6fc);
}

/* 卡片描述 */
.card-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 15px;
  line-height: 1.4;
}

/* 卡片提示 */

/* 自动执行按钮 */
.agent-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 28px;
  background-color: var(--accent-purple, #bc8cff);
  color: white;
  border: none;
  border-radius: 8px;
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

/* 底部说明 */
.dashboard-footer {
  margin-top: 30px;
  color: var(--text-muted);
  font-size: 14px;
  max-width: 600px;
  line-height: 1.5;
}

/* 响应式调整 - 小屏幕时改为 2 列 */
@media (max-width: 768px) {
  .nav-grid {
    grid-template-columns: repeat(2, 1fr);
    max-width: 400px;
  }
  
  .dashboard-title {
    font-size: 24px;
  }
}

/* TODO: 可以加个加载动画或者状态指示，但用户说简单实现，先不加 */
</style>