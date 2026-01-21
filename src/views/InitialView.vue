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
    
    <!-- 弹跳进度条 -->
    <div class="progress-container">
      <div class="bouncing-progress"></div>
    </div>
    
    <!-- 状态显示 -->
    <div class="status-info">
      <p v-if="isConnecting">正在扫描并连接蓝牙设备...</p>
      <p v-if="error" class="error">错误：{{ error }}</p>
      <p v-if="isConnected && hasTotp" class="success">
        连接成功！TOTP: {{ currentTotp }}
      </p>
      <p v-else-if="isConnected && !hasTotp" class="info">
        设备已连接，正在获取TOTP...
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
import { ref, onMounted, computed } from 'vue'
import { useBluetoothStore } from '../stores/bluetooth.js'
import { 
  autoConnectAndGetTotpWithPinia, 
  registerPiniaCallbacks 
} from '../components/data/bluetooth.js'
import { showToast } from '../components/layout/showToast.js'

console.info('InitialView - 蓝牙连接界面（带Pinia回调）')

const bluetoothStore = useBluetoothStore()

// 状态计算（直接用store的计算属性）
const isConnecting = computed(() => bluetoothStore.isConnecting())
const isConnected = computed(() => bluetoothStore.isConnected())
const hasTotp = computed(() => bluetoothStore.hasTotp())
const currentTotp = computed(() => bluetoothStore.currentTotp)
const error = computed(() => bluetoothStore.error)

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
    }
  )
  console.log('Pinia回调注册完成')
}

/**
 * 开始连接蓝牙设备（新版本）
 * 调用带Pinia更新的函数，它会自动通过回调更新store状态
 * 我们这里主要处理UI反馈和错误显示
 */
async function startConnection() {
  try {
    console.log('开始自动连接Cpen设备（带Pinia回调）...')
    // 注意：不需要手动设置状态，因为bluetooth.js会通过回调自动更新
    
    const result = await autoConnectAndGetTotpWithPinia()
    
    // 检查结果，显示相应提示
    if (result.success) {
      console.log('连接过程完成:', result.message)
      
      // 如果成功获取到TOTP，显示成功提示
      if (result.totp) {
        showToast(`连接成功！TOTP: ${result.totp}`)
        console.log('TOTP获取成功:', result.totp)
      }
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
