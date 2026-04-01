<template>
  <div class="initial-container">
    <div class="left-panel">
      <div class="left-content">
        <h1 class="app-title">CAMFC 客户端</h1>
        <p class="app-subtitle">智能蓝牙连接</p>
      </div>
    </div>
    
    <div class="right-panel">
      <div class="panel-header">
        <h2 class="panel-title">
          <i class="ri-smartphone-line"></i>
          Cpen 设备
        </h2>
        <div v-if="isScanning" class="scanning-indicator">
          <i class="ri-loader-4-line spinning"></i>
          <span>扫描中...</span>
        </div>
      </div>
      
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
      
      <div v-if="!isScanning && cpenDevices.length === 0" class="empty-state">
        <i class="ri-bluetooth-off-line"></i>
        <p>未发现 cpen 设备</p>
        <button class="retry-btn" @click="rescanDevices">
          重新扫描
        </button>
      </div>
    </div>
    
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

    <div class="help-corner">
      <span class="help-text">连接失败</span>
      <span class="help-icon">?</span>
      <div class="help-tooltip">
        <div class="tooltip-content">
          <p>① 确定设备处于配对状态</p>
          <p>② 确定电脑的蓝牙版本需要 5.0 及以上</p>
          <p>③ 其他问题请<a href="https://github.com/CloudAIMultiFunctionClicker/CAMFC-client/issues" target="_blank" rel="noopener noreferrer">向我们反馈</a></p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useBluetoothStore } from '../stores/bluetooth.js'
import { scanCpenDevices, connectCpenDevice } from '../components/data/bluetooth.js'
import { showToast } from '../components/layout/showToast.js'
import { loadAppData, saveAppData } from '../components/data/storage.js'

const router = useRouter()
const bluetoothStore = useBluetoothStore()

// 设备列表
const cpenDevices = ref([])
// 扫描和连接状态
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
const error = computed(() => bluetoothStore.error)

// 监听连接状态
watch(isConnected, (newVal) => {
  if (newVal) {
    console.log('设备已连接')
  }
})

// 格式化蓝牙地址
function formatAddress(address) {
  if (!address) return ''
  if (address.length > 12) {
    return address.substring(0, 8) + '...' + address.substring(address.length - 4)
  }
  return address
}

// 扫描蓝牙设备
async function scanDevices() {
  try {
    isScanning.value = true
    const cpenList = await scanCpenDevices()
    cpenDevices.value = cpenList
    
    if (cpenList.length === 0) {
      showToast('未发现 Cpen 设备')
    } else if (cpenList.length === 1) {
      showToast('发现 1 个 Cpen 设备，自动连接中...')
      await selectDevice(cpenList[0])
    } else {
      showToast(`发现 ${cpenList.length} 个 Cpen 设备`)
    }
  } catch (error) {
    console.error('扫描设备失败:', error)
    showToast('扫描设备失败：' + error.message)
  } finally {
    isScanning.value = false
  }
}

// 重新扫描
async function rescanDevices() {
  await scanDevices()
}

// 选择设备并连接
async function selectDevice(device) {
  if (isConnectingDevice.value) return
  
  selectedDevice.value = device
  isConnectingDevice.value = true
  
  try {
    bluetoothStore.setStatus('connecting')
    
    // 10 秒超时
    const connectPromise = connectCpenDevice(device.address)
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => reject(new Error('连接超时')), 10000)
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
    
    startCountdown()
  } catch (error) {
    console.error('连接设备失败:', error)
    bluetoothStore.setError('连接失败')
    showToast('连接失败：' + error.message)
  } finally {
    isConnectingDevice.value = false
  }
}

// 开始倒计时
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

// 跳过倒计时
function skipCountdown() {
  if (countdownTimer) {
    clearInterval(countdownTimer)
  }
  jumpToMain()
}

// 跳转到主页面
function jumpToMain() {
  showCountdown.value = false
  if (!bluetoothStore.isConnected()) {
    bluetoothStore.setStatus('connected')
  }
  setTimeout(() => {
    router.push('/main')
  }, 100)
}

// 组件挂载时自动扫描
onMounted(async () => {
  bluetoothStore.reset()
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

.left-panel {
  flex: 0 0 40%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px;
  background: var(--bg-secondary, #ffffff);
  border-right: 1px solid var(--border-color, #d0d7de);
}

.left-content {
  text-align: center;
  color: var(--text-primary, #24292f);
  margin-top: -80px;
}

.app-title {
  font-size: 42px;
  font-weight: 700;
  margin-bottom: 10px;
  color: var(--text-primary, #24292f);
}

.app-subtitle {
  font-size: 18px;
  color: var(--text-secondary, #57606a);
  margin-bottom: 40px;
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  animation: pulse 2s infinite;
}

.status-indicator.connected {
  background: var(--accent-green, #2da44e);
}

.status-indicator.scanning {
  background: var(--accent-yellow, #9a6700);
}

.status-indicator.error {
  background: var(--accent-red, #cf222e);
}

.status-indicator.disconnected {
  background: var(--text-muted, #8c959f);
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(1.1); }
}

.right-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 30px;
  margin: 20px;
  background: var(--bg-primary, #ffffff);
  border-radius: .375rem;
  border: 1px solid var(--border-color, #d0d7de);
  overflow-y: auto;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0px;
  padding-bottom: 0px;
  border-bottom: 2px solid var(--border-color, #d0d7de);
}

.scanning-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary, #57606a);
  font-size: 14px;
}

.scanning-indicator i {
  color: var(--accent-blue-dark, #0a3069);
}

.panel-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 22px;
  font-weight: 600;
  color: var(--text-primary, #24292f);
}

.panel-title i {
  color: var(--accent-blue-dark, #0a3069);
  font-size: 26px;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.spinning {
  animation: spin 1s linear infinite;
  display: inline-block;
}

.device-section {
  margin-bottom: 25px;
}

.device-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
  padding: 16px 0;
}

.device-card {
  background: var(--bg-secondary, #ffffff);
  border: 1px solid var(--border-color, #d0d7de);
  border-radius: .375rem;
  padding: 16px;
  transition: all 0.3s ease;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  gap: 15px;
}

.device-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 3px;
  background: var(--accent-blue, #0969da);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.device-card:hover {
  border-color: var(--accent-blue, #0969da);
  box-shadow: 0 4px 12px rgba(9, 105, 218, 0.15);
  transform: translateY(-2px);
}

.device-card:hover::before {
  opacity: 1;
}

.device-card.connecting {
  opacity: 0.7;
  pointer-events: none;
}

.device-card.selected {
  background: var(--selected-bg, #ddf4ff);
  border-color: var(--accent-blue, #0969da);
}

.device-card.selected::before {
  opacity: 1;
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
  color: var(--text-primary, #24292f);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.device-address {
  font-size: 12px;
  color: var(--text-secondary, #57606a);
}

.device-action {
  font-size: 20px;
  color: var(--text-secondary, #57606a);
  transition: all 0.3s ease;
}

.device-card:hover .device-action {
  color: var(--accent-blue-dark, #0a3069);
  transform: translateX(4px);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: var(--text-secondary, #57606a);
}

.empty-state i {
  font-size: 64px;
  margin-bottom: 20px;
  opacity: 0.7;
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
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: background 0.3s ease;
}

.retry-btn:hover {
  background: var(--accent-blue);
  filter: brightness(1.1);
}

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
  border-radius: .375rem;
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
  border-radius: .375rem;
  overflow: hidden;
  margin-bottom: 20px;
}

.countdown-progress {
  height: 100%;
  background: var(--accent-blue, #3178c6);
  border-radius: .375rem;
  transition: width 0.3s ease;
}

.skip-countdown-btn {
  padding: 12px 30px;
  background: var(--accent-blue);
  color: white;
  border: none;
  border-radius: .375rem;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.skip-countdown-btn:hover {
  filter: brightness(1.1);
}

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
  border-radius: .375rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: all 0.3s ease;
  z-index: 100;
}

.help-corner:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.help-icon {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: var(--bg-primary);
  color: var(--text-secondary, #57606a);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
}

.help-text {
  font-size: 14px;
  color: var(--text-secondary, #57606a);
  font-weight: 500;
}

.help-tooltip {
  position: absolute;
  bottom: 100%;
  left: 0;
  margin-bottom: 10px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border-radius: .375rem;
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

.tooltip-content a {
  color: var(--accent-blue);
  text-decoration: none;
  font-weight: 500;
}

.tooltip-content a:hover {
  text-decoration: underline;
}
</style>
