/**
 * 蓝牙状态管理
 * 只管理连接状态，设备信息直接从 Rust 获取
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'
import { emit } from '@tauri-apps/api/event'

export const useBluetoothStore = defineStore('bluetooth', () => {
  // 连接状态：disconnected / connecting / connected
  const bluetoothStatus = ref('disconnected')
  const deviceInfo = ref(null)  // 临时存一下设备信息
  const error = ref(null)  // 有错误才显示
  
  // 状态判断
  const isConnected = () => bluetoothStatus.value === 'connected'
  const isConnecting = () => bluetoothStatus.value === 'connecting'
  
  // 通知悬浮窗（失败也不影响主流程）
  const notifyStatus = async (connected) => {
    try {
      await emit('connection-status', connected)
    } catch (e) {
      console.log('发送连接状态事件失败:', e)
    }
  }
  
  // 更新状态
  const setStatus = async (status) => {
    bluetoothStatus.value = status
    if (status !== 'error') error.value = null
    
    await notifyStatus(status === 'connected')
  }
  
  const setDeviceInfo = (info) => {
    deviceInfo.value = info
  }
  
  const setError = (err) => {
    error.value = err
    bluetoothStatus.value = 'error'
    notifyStatus(false)
  }
  
  // 重置（断开时用）
  const reset = () => {
    bluetoothStatus.value = 'disconnected'
    deviceInfo.value = null
    error.value = null
    notifyStatus(false)
  }
  
  return {
    bluetoothStatus,
    deviceInfo,
    error,
    isConnected,
    isConnecting,
    setStatus,
    setDeviceInfo,
    setError,
    reset
  }
})
