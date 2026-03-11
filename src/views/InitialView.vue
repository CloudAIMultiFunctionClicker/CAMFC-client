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
  <div class="initial-container">
    <!-- 左半部分 - 保留区域 -->
    <div class="left-panel">
      <div class="left-content">
        <h1 class="app-title">CAMFC 客户端</h1>
        <p class="app-subtitle">智能蓝牙连接</p>
      </div>
    </div>
    
    <!-- 右半部分 - 蓝牙设备列表 -->
    <div class="right-panel">
      <div class="right-header">
        <h2 class="panel-title">
          <i class="ri-bluetooth-line"></i>
          蓝牙设备
        </h2>
        <button 
          class="refresh-btn" 
          @click="rescanDevices" 
          :disabled="isScanning"
          :class="{ 'spinning': isScanning }"
        >
          <i class="ri-refresh-line"></i>
        </button>
      </div>
      
      <!-- Cpen 设备列表 -->
      <div class="device-section" v-if="cpenDevices.length > 0">
        <div class="device-list">
          <div 
            v-for="device in cpenDevices" 
            :key="device.address"
            class="device-card cpen-device"
            :class="{ 'connecting': isConnectingDevice && selectedDevice?.address === device.address }"
            @click="selectDevice(device)"
          >
            <div class="device-info">
              <span class="device-name">{{ device.name }}</span>
              <span class="device-address">{{ formatAddress(device.address) }}</span>
            </div>
            <div class="device-action">
              <i v-if="isConnectingDevice && selectedDevice?.address === device.address" class="ri-loader-4-line spinning"></i>
              <i v-else class="ri-arrow-right-s-line"></i>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 空状态 -->
      <div v-if="!isScanning && cpenDevices.length === 0" class="empty-state">
        <i class="ri-bluetooth-off-line"></i>
        <p>未发现蓝牙设备</p>
        <button class="retry-btn" @click="rescanDevices">
          重新扫描
        </button>
      </div>
    </div>
    
    <!-- 倒计时弹窗 -->
    <div v-if="showCountdown" class="countdown-overlay">
      <div class="countdown-modal">
        <div class="success-icon">
          <i class="ri-check-line"></i>
        </div>
        <h3>连接成功！</h3>
        <p>{{ countdownSeconds }} 秒后自动跳转</p>
        <div class="countdown-bar">
          <div class="countdown-progress" :style="{ width: countdownProgress + '%' }"></div>
        </div>
        <button class="skip-countdown-btn" @click="skipCountdown">立即进入</button>
      </div>
    </div>

    <!-- 左下角问号帮助按钮 -->
    <div class="help-corner">
      <span class="help-text">连接失败</span>
      <div class="help-icon">?</div>
      <!-- Tooltip -->
      <div class="help-tooltip">
        <div class="tooltip-content">
          <p>① 确定设备处于配对状态</p>
          <p>② 确定电脑的蓝牙版本需要5.0及以上</p>
          <p>③ 其他问题请通过 GitHub Issue 提交</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useBluetoothStore } from '../stores/bluetooth.js'
import { 
  scanCpenDevices,
  connectCpenDevice
} from '../components/data/bluetooth.js'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData, saveAppData } from '../components/data/storage.js'

console.info('InitialView - 蓝牙连接界面（左右分栏版）')

const router = useRouter()
const bluetoothStore = useBluetoothStore()

// 设备列表
const cpenDevices = ref([])

// 扫描状态
const isScanning = ref(false)
const isConnectingDevice = ref(false)
const selectedDevice = ref(null)

// 倒计时相关
const showCountdown = ref(false)
const countdownSeconds = ref(5)
const countdownProgress = ref(100)
let countdownTimer = null

// 状态计算
const isConnected = computed(() => bluetoothStore.isConnected())
const connectedDeviceName = computed(() => bluetoothStore.deviceInfo?.name || 'Cpen设备')
const error = computed(() => bluetoothStore.error)

// 状态样式
const statusClass = computed(() => {
  if (isConnected.value) return 'connected'
  if (isScanning.value) return 'scanning'
  if (error.value) return 'error'
  return 'disconnected'
})

const statusText = computed(() => {
  if (isConnected.value) return '已连接'
  if (isScanning.value) return '扫描中...'
  if (error.value) return '连接失败'
  return '未连接'
})

// 监听连接成功（已改为在 selectDevice 中直接跳转）
watch(isConnected, (newVal) => {
  if (newVal) {
    console.log('设备已连接')
  }
})

/**
 * 格式化蓝牙地址
 */
function formatAddress(address) {
  if (!address) return ''
  // 只显示前8位和后4位，中间用...代替
  if (address.length > 12) {
    return address.substring(0, 8) + '...' + address.substring(address.length - 4)
  }
  return address
}

/**
 * 扫描所有蓝牙设备
 */
async function scanDevices() {
  try {
    isScanning.value = true
    console.log('开始扫描蓝牙设备...')
    
    // 扫描 Cpen 设备
    const cpenList = await scanCpenDevices()
    cpenDevices.value = cpenList
    console.log(`找到 ${cpenList.length} 个 Cpen 设备`)
    
    if (cpenList.length === 0) {
      showToast('未发现 Cpen 设备')
    } else {
      showToast(`发现 ${cpenList.length} 个 Cpen 设备`)
    }
  } catch (error) {
    console.error('扫描设备失败:', error)
    showToast('扫描设备失败: ' + error.message)
  } finally {
    isScanning.value = false
  }
}

/**
 * 重新扫描
 */
async function rescanDevices() {
  await scanDevices()
}

/**
 * 选择设备并连接
 */
async function selectDevice(device) {
  if (isConnectingDevice.value) return
  
  selectedDevice.value = device
  isConnectingDevice.value = true
  
  try {
    console.log(`选择设备: ${device.name} (${device.address})`)
    bluetoothStore.setStatus('connecting')
    
    // 设置连接超时
    const connectPromise = connectCpenDevice(device.address)
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => reject(new Error('连接超时')), 10000) // 10秒超时
    })
    
    await Promise.race([connectPromise, timeoutPromise])
    
    bluetoothStore.setStatus('connected')
    showToast('设备连接成功！')
    
    // 保存设备地址
    try {
      const savedCpen = await loadAppData('settings_cpen')
      const settings = savedCpen ? JSON.parse(savedCpen) : { autoConnect: false, lastDeviceAddress: '' }
      settings.lastDeviceAddress = device.address
      await saveAppData('settings_cpen', JSON.stringify(settings))
    } catch (e) {
      console.warn('保存设备地址失败:', e)
    }
    
    // 连接成功直接跳转
    jumpToMain()
  } catch (error) {
    console.error('连接设备失败:', error)
    bluetoothStore.setError('连接失败')
    showToast('连接失败：' + error.message)
  } finally {
    isConnectingDevice.value = false
  }
}

/**
 * 开始倒计时
 */
function startCountdown() {
  showCountdown.value = true
  countdownSeconds.value = 5
  countdownProgress.value = 100
  
  if (countdownTimer) {
    clearInterval(countdownTimer)
  }
  
  countdownTimer = setInterval(() => {
    countdownSeconds.value--
    countdownProgress.value = countdownSeconds.value * 20
    
    if (countdownSeconds.value <= 0) {
      clearInterval(countdownTimer)
      jumpToMain()
    }
  }, 1000)
}

/**
 * 跳过倒计时
 */
function skipCountdown() {
  if (countdownTimer) {
    clearInterval(countdownTimer)
  }
  jumpToMain()
}

/**
 * 跳转到主页面
 */
function jumpToMain() {
  showCountdown.value = false
  router.push('/main')
}

/**
 * 跳过蓝牙连接
 */
function skipToMain() {
  console.log('用户跳过蓝牙连接')
  router.push('/main')
}

/**
 * 显示连接失败帮助信息
 */
function showConnectionHelp() {
  showToast('连接失败帮助：\n1. 确保设备已开启\n2. 确保设备在附近\n3. 尝试重新扫描\n4. 重启应用后重试')
}

// 组件挂载时自动扫描
onMounted(async () => {
  console.log('InitialView mounted，开始自动扫描')
  
  // 重置状态
  bluetoothStore.reset()
  
  // 立即开始扫描
  await scanDevices()
})
</script>

<style scoped>
.initial-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  background: var(--bg-primary);
  overflow: hidden;
}

/* 左半部分 */
.left-panel {
  flex: 0 0 40%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
}

.left-content {
  text-align: center;
  color: var(--text-primary);
}

.app-title {
  font-size: 42px;
  font-weight: 700;
  margin-bottom: 10px;
  color: var(--text-primary);
}

.app-subtitle {
  font-size: 18px;
  color: var(--text-secondary);
  margin-bottom: 40px;
}

.connection-status {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 30px;
  padding: 12px 24px;
  background: var(--bg-primary);
  border-radius: 25px;
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  animation: pulse 2s infinite;
}

.status-indicator.connected {
  background: #4ade80;
}

.status-indicator.scanning {
  background: #fbbf24;
}

.status-indicator.error {
  background: #f87171;
}

.status-indicator.disconnected {
  background: #9ca3af;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(1.1); }
}

.status-text {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-secondary);
}

.connected-device {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 15px 25px;
  background: rgba(16, 185, 129, 0.1);
  border-radius: 12px;
  margin-bottom: 30px;
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.connected-device i {
  font-size: 24px;
  color: #10b981;
}

.connected-device span {
  color: #10b981;
  font-weight: 500;
}

.skip-btn {
  padding: 12px 30px;
  background: var(--bg-primary);
  border: 2px solid var(--border-color);
  color: var(--text-secondary);
  border-radius: 25px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.skip-btn:hover {
  background: var(--hover-bg);
  border-color: var(--text-muted);
}

/* 右半部分 */
.right-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 30px;
  background: var(--bg-primary);
  overflow-y: auto;
}

.right-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 2px solid var(--border-color);
}

.panel-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 22px;
  font-weight: 600;
  color: var(--text-primary);
}

.panel-title i {
  color: var(--accent-blue);
  font-size: 26px;
}

.refresh-btn {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: none;
  background: var(--bg-secondary);
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.refresh-btn:hover:not(:disabled) {
  background: var(--hover-bg);
  color: var(--accent-blue);
}

.refresh-btn.spinning i {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 设备分区 */
.device-section {
  margin-bottom: 25px;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-muted);
  margin-bottom: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.section-title i {
  color: var(--accent-blue);
}

/* 设备列表 */
.device-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.device-card {
  display: flex;
  align-items: center;
  gap: 15px;
  padding: 16px 20px;
  background: var(--bg-secondary);
  border: 2px solid var(--border-color);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.device-card:hover {
  border-color: var(--accent-blue);
  box-shadow: 0 4px 12px rgba(var(--accent-blue-rgb), 0.15);
  transform: translateY(-2px);
}

.device-card.connecting {
  opacity: 0.7;
  pointer-events: none;
}

.device-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.device-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.device-address {
  font-size: 12px;
  color: var(--text-muted);
}

.device-action {
  font-size: 20px;
  color: var(--text-muted);
}

.device-card:hover .device-action {
  color: var(--accent-blue);
}

.spinning {
  animation: spin 1s linear infinite;
  display: inline-block;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: var(--text-muted);
}

.empty-state i {
  font-size: 64px;
  margin-bottom: 20px;
  opacity: 0.5;
}

.empty-state p {
  font-size: 16px;
  margin-bottom: 20px;
}

.retry-btn {
  padding: 10px 24px;
  background: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: background 0.3s ease;
}

.retry-btn:hover {
  background: var(--accent-blue);
  filter: brightness(1.1);
}

/* 倒计时弹窗 */
.countdown-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.countdown-modal {
  background: var(--bg-secondary);
  padding: 40px;
  border-radius: 20px;
  text-align: center;
  min-width: 300px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-color);
}

.success-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: #4ade80;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 20px;
}

.success-icon i {
  font-size: 40px;
  color: white;
}

.countdown-modal h3 {
  font-size: 24px;
  color: var(--text-primary);
  margin-bottom: 10px;
}

.countdown-modal p {
  color: var(--text-secondary);
  margin-bottom: 20px;
}

.countdown-bar {
  width: 100%;
  height: 6px;
  background: var(--bg-primary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 20px;
}

.countdown-progress {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-blue), #8b5cf6);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.skip-countdown-btn {
  padding: 12px 30px;
  background: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 25px;
  font-size: 14px;
  cursor: pointer;
  transition: transform 0.3s ease;
}

.skip-countdown-btn:hover {
  transform: scale(1.05);
  filter: brightness(1.1);
}

/* 左下角帮助按钮 */
.help-corner {
  position: fixed;
  bottom: 20px;
  left: 20px;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 25px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: all 0.3s ease;
  z-index: 100;
}

.help-corner:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.help-icon {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: var(--bg-primary);
  color: var(--text-muted);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
}

.help-text {
  font-size: 14px;
  color: var(--text-muted);
  font-weight: 500;
}

/* Tooltip 样式 */
.help-tooltip {
  position: absolute;
  bottom: 100%;
  left: 0;
  margin-bottom: 10px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border-radius: 8px;
  font-size: 13px;
  line-height: 1.6;
  white-space: nowrap;
  opacity: 0;
  visibility: hidden;
  transform: translateY(10px);
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 1px solid var(--border-color);
}

.help-tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 20px;
  border: 6px solid transparent;
  border-top-color: var(--bg-secondary);
}

.help-corner:hover .help-tooltip {
  opacity: 1;
  visibility: visible;
  transform: translateY(0);
}

.tooltip-content p {
  margin: 0;
  padding: 4px 0;
}
</style>
