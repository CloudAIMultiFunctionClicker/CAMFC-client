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
    <!-- 动态标题 -->
    <h1 class="title" :class="{ 'error-title': showConnectionFailed }">
      {{ getTitleText() }}
    </h1>
    
    <!-- 弹跳进度条（连接中显示） -->
    <div class="progress-container" v-if="isConnecting">
      <div class="bouncing-progress"></div>
    </div>
    
    <!-- 5秒倒计时进度条（连接成功后显示） -->
    <div class="countdown-container" v-if="isConnected && showCountdown">
      <h2 class="countdown-title">连接成功！5秒后跳转到主页面

      </h2>
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
      <p v-if="showConnectionFailed" class="error">{{ getErrorText() }}</p>
      <p v-if="isConnected && !showCountdown" class="success">
        设备连接成功！
      </p>
      <!-- 开发者模式提示 -->
      <p v-if="isDeveloperMode" class="info">
        <strong>开发者模式：</strong>使用环境变量CPEN_ID和CPEN_KEY
      </p>
    </div>
    
    <!-- 操作按钮 -->
    <div class="action-buttons">
      <!-- 连接失败或超时时显示"再次尝试"按钮 -->
      <button 
        v-if="showConnectionFailed && !isConnecting" 
        class="retry-btn"
        @click="retryConnection"
        :disabled="isConnecting"
      >
        {{ isConnecting ? '连接中...' : '再次尝试' }}
      </button>
      
      <!-- 连接成功时显示"进入主页面"按钮 -->
      <button 
        v-if="isConnected && !showCountdown" 
        class="enter-btn"
        @click="jumpToFileView"
      >
        进入主界面
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useBluetoothStore } from '../stores/bluetooth.js'
import { 
  getTotp,
  getDeviceId,
  getConnectionStatus,
  disconnect,
  cleanup
} from '../components/data/bluetooth.js'
import { showToast } from '../components/layout/showToast.js'

console.info('InitialView - 蓝牙连接界面（简化版，业务逻辑在Rust端）')

const router = useRouter()
const bluetoothStore = useBluetoothStore()

// 倒计时相关状态
const showCountdown = ref(false)
const countdownSeconds = ref(5)
const countdownProgress = ref(100)
let countdownTimer = null

// 状态计算（直接从store获取连接状态和错误信息）
// 注意：TOTP和deviceId不再从store读取，改为直接调用Rust命令
const isConnecting = computed(() => bluetoothStore.isConnecting())
const isConnected = computed(() => bluetoothStore.isConnected())
const error = computed(() => bluetoothStore.error)

// 超时状态
const connectionTimedOut = ref(false)

// 重试次数计数（用于调试和循环显示）
const retryCount = ref(0)

// 连接失败状态（包括错误和超时）
const showConnectionFailed = computed(() => {
  // 当有错误，或者连接超过一定时间但仍然没有连接成功时显示"无法连接"
  return error.value || connectionTimedOut.value
})

// 开发者模式检测（通过错误信息判断）
const isDeveloperMode = computed(() => {
  // 如果错误信息中包含"开发者模式"，说明是开发者模式
  return error.value && error.value.includes('开发者模式')
})

// 蓝牙硬件不可用检测
const isBluetoothHardwareUnavailable = computed(() => {
  // 检查错误信息是否包含蓝牙硬件相关关键词
  if (!error.value) return false
  const err = error.value.toLowerCase()
  return err.includes('蓝牙硬件') || 
         err.includes('no bluetooth') || 
         err.includes('未找到蓝牙设备') ||
         err.includes('没有适配器') ||
         err.includes('adapter not found')
})

// 蓝牙未开启检测
const isBluetoothDisabled = computed(() => {
  if (!error.value) return false
  const err = error.value.toLowerCase()
  return err.includes('蓝牙未开启') || 
         err.includes('bluetooth disabled') ||
         err.includes('蓝牙检测失败')
})

// 获取标题文本
function getTitleText() {
  if (showConnectionFailed.value) {
    if (isBluetoothHardwareUnavailable.value) {
      return '应用不可用'
    } else if (isBluetoothDisabled.value) {
      return '蓝牙未开启'
    } else {
      return '无法连接'
    }
  }
  return '等待蓝牙连接Cpen设备'
}

// 获取错误文本
function getErrorText() {
  if (!error.value) return '无法连接到Cpen设备'
  
  // 如果是开发者模式错误，显示友好提示
  if (error.value.includes('开发者模式')) {
    return '开发者模式：请设置CPEN_ID和CPEN_KEY环境变量'
  }
  
  // 蓝牙硬件不可用
  if (isBluetoothHardwareUnavailable.value) {
    return `未检测到蓝牙硬件\n\n请确保：\n1. 计算机支持蓝牙功能\n2. 蓝牙硬件已正确安装\n3. 蓝牙驱动程序已更新\n\n应用需要蓝牙设备才能正常工作。`
  }
  
  // 蓝牙未开启
  if (isBluetoothDisabled.value) {
    return `蓝牙未开启\n\n请开启蓝牙后重试。`
  }
  
  // 默认错误处理
  if (error.value.includes('未找到Cpen设备')) {
    return `未找到Cpen设备\n\n请确保：\n1. Cpen设备已开机\n2. 设备在蓝牙范围内\n3. 设备名以'Cpen'开头`
  }
  
  // 其他错误
  return error.value
}

// 监听连接成功状态
watch(isConnected, (newVal) => {
  if (newVal) {
    // 设备连接成功，开始倒计时
    console.log('设备已连接，开始5秒倒计时')
    startCountdown()
    // 连接成功，清除超时状态
    connectionTimedOut.value = false
  }
})

/**
 * 更新Pinia状态（简化版）
 * 
 * 重构后，业务逻辑在Rust端，前端只需要：
 * 1. 调用Rust命令获取结果
 * 2. 根据结果更新连接状态
 * 3. 处理UI反馈
 * 
 * 注意：TOTP和deviceId不再存入store，直接使用
 */
function updateStoreFromResult(result) {
  if (result.success) {
    // 成功连接，更新状态
    bluetoothStore.setStatus('connected')
    if (result.deviceInfo) {
      bluetoothStore.setDeviceInfo(result.deviceInfo)
    }
  } else {
    // 连接失败，设置错误状态
    bluetoothStore.setError(result.error || '连接失败')
  }
}

/**
 * 开始5秒倒计时
 * 计划连接成功后显示5秒倒计时，然后跳转到文件管理
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
  console.log('跳转到主页面')
  showCountdown.value = false  // 先隐藏倒计时UI
  router.push('/main')
}

/**
 * 开始连接蓝牙设备（简化版）
 * 
 * 重构后，所有业务逻辑在Rust端：
 * 1. 调用getTotp()会自动扫描、连接、获取TOTP
 * 2. 如果失败，会抛出错误
 * 3. 成功后，我们可以获取设备ID等信息
 * 
 * 注意：这个函数现在只调用简单的接口，不处理复杂逻辑
 */
async function startConnection() {
  try {
    console.log('开始自动连接Cpen设备（简化版）...')
    
    // 重置超时状态
    connectionTimedOut.value = false
    
    // 先设置状态为连接中
    bluetoothStore.setStatus('connecting')
    
    // 设置超时检测 - 15秒后如果还没有连接成功，就显示"无法连接"
    // 思考：15秒够吗？Rust端扫描5秒，再加上连接、发送命令的时间...
    // 先这样吧，不够再调
    const timeoutDuration = 15000 // 15秒
    let timeoutId = null
    
    // 用户提到"当没有cpen连接已经超时并且toast已经出来了"
    // 所以我们需要在超时后显示"无法连接"而不仅仅是等待状态
    const timeoutPromise = new Promise((_, reject) => {
      timeoutId = setTimeout(() => {
        console.warn('蓝牙连接超时（15秒）')
        connectionTimedOut.value = true
        // 超时时设置错误状态，让界面显示"无法连接"
        bluetoothStore.setError('连接超时')
        showToast('连接超时，请重试')
        reject(new Error('连接超时'))
      }, timeoutDuration)
    })
    
    // 连接任务
    const connectionPromise = (async () => {
      // 1. 获取TOTP（这会自动扫描、连接设备、发送命令）
      const totp = await getTotp()
      
      console.log('TOTP获取成功，设备已连接:', totp)
      
      // 2. 更新状态为已连接
      bluetoothStore.setStatus('connected')
      // 注意：TOTP不再存入store，直接使用返回值
      
      // 3. 尝试获取设备ID（可选）
      try {
        const deviceId = await getDeviceId()
        console.log('设备ID获取成功:', deviceId)
        // 注意：deviceId也不再存入store，直接使用返回值
      } catch (idError) {
        console.warn('获取设备ID失败，但不影响连接:', idError)
        // 设备ID获取失败不影响整体连接状态
      }
      
      // 4. 获取连接状态信息（包含设备名）
      try {
        const status = await getConnectionStatus()
        console.log('连接状态:', status)
        // 可以尝试从状态信息中提取设备名，但先简单处理
        bluetoothStore.setDeviceInfo(status)
      } catch (statusError) {
        console.warn('获取连接状态失败:', statusError)
      }
      
      // 5. 显示成功提示
      showToast('设备连接成功！')
      
      console.log('连接过程完成')
      
      // 如果是开发者模式，添加额外提示
      if (isDeveloperMode.value) {
        showToast('开发者模式：使用环境变量')
      }
    })()
    
    // 使用Promise.race等待连接完成或超时
    await Promise.race([connectionPromise, timeoutPromise])
    
    // 清除超时定时器（如果还没触发）
    if (timeoutId) {
      clearTimeout(timeoutId)
    }
    
  } catch (error) {
    console.error('连接过程中出错:', error)
    
    // 处理错误
    const errorMsg = error.message || error.toString()
    
    console.log('连接错误详情:', errorMsg)
    
    // 分析错误信息，提供更具体的错误提示
    let displayErrorMsg = errorMsg
    
    // 检查是否是蓝牙硬件问题
    if (errorMsg.includes('未找到蓝牙设备') || 
        errorMsg.includes('No Bluetooth') || 
        errorMsg.includes('找不到') ||
        errorMsg.includes('没有适配器') ||
        errorMsg.includes('adapter not found')) {
      displayErrorMsg = '蓝牙硬件不可用：未检测到蓝牙设备'
    } 
    // 检查是否是蓝牙未开启
    else if (errorMsg.includes('蓝牙未开启') || 
             errorMsg.includes('bluetooth disabled') ||
             errorMsg.includes('蓝牙检测失败')) {
      displayErrorMsg = '蓝牙未开启：请检查蓝牙设置'
    }
    // 检查是否是开发者模式相关错误
    else if (errorMsg.includes('开发者模式')) {
      displayErrorMsg = errorMsg // 保留原始错误信息
    }
    // 其他错误统一处理
    else {
      displayErrorMsg = '连接失败：' + errorMsg
    }
    
    // 注意：超时时已经设置过错误了，这里不再重复设置
    if (!connectionTimedOut.value) {
      bluetoothStore.setError(displayErrorMsg)
      
      // 根据错误类型显示不同的toast
      if (displayErrorMsg.includes('蓝牙硬件不可用')) {
        showToast('未检测到蓝牙硬件，应用不可用')
      } else if (displayErrorMsg.includes('蓝牙未开启')) {
        showToast('蓝牙未开启，请检查蓝牙设置')
      } else if (displayErrorMsg.includes('开发者模式')) {
        showToast('开发者模式：请检查环境变量')
      } else {
        showToast('连接失败，请重试')
      }
    }
    
    // 设置状态为断开连接
    bluetoothStore.setStatus('disconnected')
  }
}

/**
 * 再次尝试连接
 * 用户点击"再次尝试"按钮时调用
 * 
 * 照逻辑：如果再次失败，循环显示错误信息和重试按钮
 * 所以这个函数会重置状态并重新尝试，形成循环
 */
async function retryConnection() {
  console.log('用户点击再次尝试连接')
  
  // 增加重试计数
  retryCount.value++
  console.log(`第 ${retryCount.value} 次尝试连接`)
  
  // 重置错误状态和超时状态
  bluetoothStore.setError(null)
  connectionTimedOut.value = false
  
  // 开始新的连接尝试
  await startConnection()
}

// 组件挂载时设置
onMounted(() => {
  console.log('InitialView mounted，开始连接设备')
  
  // 重置状态，确保从干净状态开始
  bluetoothStore.reset()
  connectionTimedOut.value = false
  retryCount.value = 0
  
  // 延迟一下，确保UI渲染完成
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
  transition: color 0.3s ease;
}

.error-title {
  color: var(--accent-red);
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
  font-weight: bold;
}

.info {
  color: var(--text-secondary);
}

/* 操作按钮样式 */
.action-buttons {
  margin-top: 30px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
}

.retry-btn {
  padding: 12px 30px;
  background-color: var(--accent-blue);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 150px;
}

.retry-btn:hover:not(:disabled) {
  background-color: #4a8bd6;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(74, 139, 214, 0.3);
}

.retry-btn:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
  opacity: 0.7;
}

.enter-btn {
  padding: 12px 30px;
  background-color: #55aa55;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 150px;
}

.enter-btn:hover {
  background-color: #4a994a;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(85, 170, 85, 0.3);
}
</style>
