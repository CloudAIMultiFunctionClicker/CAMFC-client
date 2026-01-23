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
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { 
  getTotp,  // 新的核心API：获取TOTP
  getDeviceId,  // 新的核心API：获取设备ID
  startBackgroundService  // 启动后台服务
} from '../components/data/bluetooth.js'
import { showToast } from '../components/layout/showToast.js'

console.info('InitialView - 蓝牙连接界面（使用新版API）')

const router = useRouter()

// 倒计时相关状态
const showCountdown = ref(false)
const countdownSeconds = ref(5)
const countdownProgress = ref(100)
let countdownTimer = null

// 状态计算
const isConnecting = ref(false)
const isConnected = ref(false)
const error = ref(null)

// 设备连接状态（通过调用API间接判断）
function updateConnectionStatus(connected, connecting = false, err = null) {
  isConnecting.value = connecting
  isConnected.value = connected
  error.value = err
  
  if (connected) {
    console.log('设备已连接，开始5秒倒计时')
    startCountdown()
  }
}

/**
 * 启动后台服务
 * 应用启动时调用，让Rust端处理定时刷新等后台任务
 */
function setupBackgroundService() {
  console.log('设置后台服务...')
  startBackgroundService().catch(err => {
    console.warn('后台服务启动失败，但不影响主要功能:', err)
  })
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
 * 开始连接蓝牙设备（新版）
 * 直接调用bluetooth.js的新API，不通过Pinia
 * 
 * 流程：
 * 1. 更新状态为"连接中"
 * 2. 调用getTotp() API（Rust端会自动连接）
 * 3. 成功后调用getDeviceId()获取设备ID
 * 4. 更新状态并显示结果
 */
async function startConnection() {
  try {
    console.log('开始自动连接Cpen设备（使用新版API）...')
    updateConnectionStatus(false, true, null)  // 连接中
    
    // 1. 调用getTotp() - Rust端会自动连接设备并获取TOTP
    const totp = await getTotp()
    console.log('获取TOTP成功:', totp)
    
      // 2. 成功获取TOTP后，更新状态为已连接
      updateConnectionStatus(true, false, null)
      
      // 3. 获取设备ID（直接调用API，不需要存储到Pinia）
      try {
        const deviceId = await getDeviceId()
        console.log('获取设备ID成功:', deviceId)
        // 注意：现在不再存储到Pinia，因为所有地方都直接调用API
      } catch (idError) {
        console.warn('获取设备ID失败，但不影响连接状态:', idError)
        // 设备ID获取失败不影响整体连接状态
      }
    
    // 5. 显示成功提示
    showToast('设备连接成功！')
    
  } catch (err) {
    console.error('连接过程中出错:', err)
    // 处理错误
    updateConnectionStatus(false, false, err.message)
    showToast(`连接失败: ${err.message || '未知错误'}`)
  }
}

/**
 * 重置状态（现在不再需要，因为状态由Rust端管理）
 * 保留函数定义但注释掉，以防其他地方调用
 */
function resetState() {
  console.log('resetState已禁用：状态现在由Rust端管理')
  // bluetoothStore.reset() // 不再需要
}

// 组件挂载时设置
onMounted(() => {
  console.log('InitialView mounted，启动后台服务并开始连接')
  
  // 1. 启动后台服务（让Rust端处理定时刷新等）
  setupBackgroundService()
  
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
