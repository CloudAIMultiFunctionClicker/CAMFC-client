<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<template>
  <div class="initial-container">
    <!-- 标题 -->
    <h1 class="title">等待蓝牙连接Cpen设备</h1>
    
    <!-- 弹跳进度条（连接中显示） -->
    <div class="progress-container" v-if="isConnecting">
      <div class="bouncing-progress"></div>
    </div>
    
    <!-- 5秒倒计时进度条（连接成功后显示） -->
    <div class="countdown-container" v-if="isConnected && showCountdown">
      <h2 class="countdown-title">连接成功！5秒后跳转到文件管理</h2>
      <div class="countdown-bar">
        <div 
          class="countdown-progress" 
          :style="{ width: countdownProgress + '%' }"
        ></div>
      </div>
      <p class="countdown-text">{{ countdownSeconds }}秒</p>
      <!-- 跳过按钮，可以手动跳过等待 -->
      <button class="skip-btn" @click="skipCountdown">跳过等待</button>
    </div>
    
    <!-- 状态显示 -->
    <div class="status-info">
      <p v-if="isConnecting">正在扫描并连接蓝牙设备...</p>
      <p v-if="error" class="error">错误：{{ error }}</p>
      <p v-if="isConnected && !showCountdown" class="success">
        设备连接成功！
      </p>
    </div>
    
    <!-- 调试用按钮，可以隐藏 -->
    <div class="debug-buttons" v-if="false">
      <button @click="startConnection">手动连接</button>
      <button @click="resetState">重置</button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useBluetoothStore } from '../stores/bluetooth.js'
import { 
  autoConnectAndGetTotpWithPinia, 
  registerPiniaCallbacks 
} from '../components/data/bluetooth.js'
import { showToast } from '../components/layout/showToast.js'

console.info('InitialView - 蓝牙连接界面（带Pinia回调）')

const router = useRouter()
const bluetoothStore = useBluetoothStore()

// 倒计时相关状态
const showCountdown = ref(false)
const countdownSeconds = ref(5)
const countdownProgress = ref(100)
let countdownTimer = null

// 状态计算（直接用store的计算属性）
const isConnecting = computed(() => bluetoothStore.isConnecting())
const isConnected = computed(() => bluetoothStore.isConnected())
const hasTotp = computed(() => bluetoothStore.hasTotp())
const currentTotp = computed(() => bluetoothStore.currentTotp)
const error = computed(() => bluetoothStore.error)

// 监听连接成功状态
watch(isConnected, (newVal) => {
  if (newVal) {
    // 设备连接成功，开始倒计时
    console.log('设备已连接，开始5秒倒计时')
    startCountdown()
  }
})

/**
 * 注册Pinia回调给bluetooth.js用
 * 这样bluetooth.js能直接更新store状态，不用我们手动操作
 * 
 * 思考：这样设计是为了避免模块间的循环依赖
 * bluetooth.js不能直接import store，store也不能直接调用bluetooth.js
 * 用回调函数解耦，感觉挺巧妙的
 */
function setupPiniaCallbacks() {
  registerPiniaCallbacks(
    // TOTP更新回调
    (totp) => {
      console.log('Pinia回调：更新TOTP', totp)
      bluetoothStore.setTotp(totp)
    },
    // 状态更新回调
    (status) => {
      console.log('Pinia回调：更新状态', status)
      bluetoothStore.setStatus(status)
    },
    // 设备信息更新回调
    (deviceInfo) => {
      console.log('Pinia回调：更新设备信息', deviceInfo)
      bluetoothStore.setDeviceInfo(deviceInfo)
    },
    // 设备ID更新回调（新增）
    (deviceId) => {
      console.log('Pinia回调：更新设备ID', deviceId)
      bluetoothStore.setDeviceId(deviceId)
    }
  )
  console.log('Pinia回调注册完成（包括设备ID回调）')
}

/**
 * 开始5秒倒计时
 * 用户要求连接成功后显示5秒倒计时，然后跳转到文件管理
 * 
 * 实现思路：每秒更新一次，更新进度条和剩余时间
 * TODO: 倒计时结束后要跳转路由，还要清理定时器避免内存泄漏
 */
function startCountdown() {
  showCountdown.value = true
  countdownSeconds.value = 5
  countdownProgress.value = 100
  
  // 清理之前的定时器（安全起见）
  if (countdownTimer) {
    clearInterval(countdownTimer)
  }
  
  countdownTimer = setInterval(() => {
    countdownSeconds.value--
    countdownProgress.value = countdownSeconds.value * 20 // 5秒对应100%
    
    console.log(`倒计时剩余: ${countdownSeconds.value}秒`)
    
    if (countdownSeconds.value <= 0) {
      // 倒计时结束，跳转到文件管理
      clearInterval(countdownTimer)
      jumpToFileView()
    }
  }, 1000)
}

/**
 * 手动跳过倒计时
 * 用户可能不想等5秒，所以加个跳过按钮
 */
function skipCountdown() {
  console.log('用户跳过倒计时')
  
  if (countdownTimer) {
    clearInterval(countdownTimer)
  }
  
  jumpToFileView()
}

/**
 * 跳转到文件管理路由
 * 直接用router.push跳转
 */
function jumpToFileView() {
  console.log('跳转到文件管理页面')
  showCountdown.value = false  // 先隐藏倒计时UI
  router.push('/fileView')
}

/**
 * 开始连接蓝牙设备（新版本）
 * 调用带Pinia更新的函数，它会自动通过回调更新store状态
 * 我们这里主要处理UI反馈和错误显示
 * 
 * 注意：用户说不需要显示TOTP，所以去掉相关提示
 */
async function startConnection() {
  try {
    console.log('开始自动连接Cpen设备（带Pinia回调）...')
    // 注意：不需要手动设置状态，因为bluetooth.js会通过回调自动更新
    
    const result = await autoConnectAndGetTotpWithPinia()
    
    // 检查结果，显示相应提示
    if (result.success) {
      console.log('连接过程完成:', result.message)
      // 成功连接，显示简单提示（不要TOTP）
      showToast('设备连接成功！')
    } else {
      console.warn('连接失败:', result.message)
      // 注意：错误信息已经在回调中更新到store了
      // 这里只显示toast提示
      showToast(`连接失败: ${result.message || '未知错误'}`)
    }
  } catch (err) {
    console.error('连接过程中出错:', err)
    // 处理意外错误
    bluetoothStore.setError(err.message || '未知错误')
    showToast(`连接出错: ${err.message || '未知错误'}`)
  }
}

/**
 * 重置状态
 * 调试用，可以不用
 */
function resetState() {
  bluetoothStore.reset()
}

// 组件挂载时设置
onMounted(() => {
  console.log('InitialView mounted，设置Pinia回调并开始连接')
  
  // 1. 先注册回调函数
  setupPiniaCallbacks()
  
  // 2. 延迟一下，确保UI渲染完成
  setTimeout(() => {
    startConnection()
  }, 500)
})
</script>

<style scoped>
.initial-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: calc(100vh - 65px); /* 减去header高度 */
  text-align: center;
  padding: 20px;
}

.title {
  font-size: 24px;
  margin-bottom: 40px;
  color: var(--text-primary);
}

.progress-container {
  margin-bottom: 30px;
}

/* 弹跳进度条样式 */
.bouncing-progress {
  width: 60px;
  height: 60px;
  background-color: var(--accent-blue);
  border-radius: 50%;
  animation: bounce 1s infinite ease-in-out;
}

/* 弹跳动画 */
@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-20px);
  }
}

/* 倒计时容器 */
.countdown-container {
  margin: 30px 0;
  padding: 20px;
  background-color: var(--bg-secondary);
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  max-width: 400px;
  width: 100%;
}

.countdown-title {
  font-size: 20px;
  margin-bottom: 20px;
  color: #55aa55;
}

/* 倒计时进度条 */
.countdown-bar {
  width: 100%;
  height: 20px;
  background-color: var(--bg-primary);
  border-radius: 10px;
  overflow: hidden;
  margin-bottom: 10px;
  border: 1px solid var(--border-color);
}

.countdown-progress {
  height: 100%;
  background-color: #55aa55;
  border-radius: 10px;
  transition: width 0.3s ease;
  /* 渐变效果 */
  background: linear-gradient(90deg, #55aa55, #7ccc7c);
}

.countdown-text {
  font-size: 24px;
  font-weight: bold;
  color: var(--text-primary);
  margin: 10px 0;
}

/* 跳过按钮 */
.skip-btn {
  margin-top: 15px;
  padding: 8px 20px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.2s;
}

.skip-btn:hover {
  background-color: #4a8bd6;
}

.status-info {
  margin-top: 20px;
  min-height: 60px;
}

.status-info p {
  margin: 10px 0;
  font-size: 16px;
}

.success {
  color: #55aa55;
  font-weight: bold;
}

.error {
  color: var(--accent-red);
}

.info {
  color: var(--text-secondary);
}

.debug-buttons {
  margin-top: 20px;
}

.debug-buttons button {
  margin: 0 10px;
  padding: 8px 16px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
}

.debug-buttons button:hover {
  background-color: var(--hover-bg);
}
</style>
