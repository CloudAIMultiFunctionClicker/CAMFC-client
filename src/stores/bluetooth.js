/**
 * 蓝牙状态管理store
 * 用来存蓝牙连接状态和TOTP
 * 
 * 本来想搞复杂点，但用户说莫复杂化，那就简单搞
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useBluetoothStore = defineStore('bluetooth', () => {
  // 蓝牙状态：连接中、已连接、断开
  const bluetoothStatus = ref('disconnected') // disconnected/connecting/connected
  
  // 当前TOTP值，null表示没有或者无效
  const currentTotp = ref(null)
  
  // 设备信息
  const deviceInfo = ref(null)
  
  // 错误信息，有错误才显示
  const error = ref(null)
  
  // 简单状态判断
  const isConnected = () => bluetoothStatus.value === 'connected'
  const isConnecting = () => bluetoothStatus.value === 'connecting'
  const hasTotp = () => currentTotp.value !== null
  
  // 更新状态
  const setStatus = (status) => {
    bluetoothStatus.value = status
    if (status !== 'error') {
      error.value = null // 状态正常就清掉错误
    }
  }
  
  const setTotp = (totp) => {
    currentTotp.value = totp
  }
  
  const setDeviceInfo = (info) => {
    deviceInfo.value = info
  }
  
  const setError = (err) => {
    error.value = err
    bluetoothStatus.value = 'error'
  }
  
  // 重置状态
  const reset = () => {
    bluetoothStatus.value = 'disconnected'
    currentTotp.value = null
    deviceInfo.value = null
    error.value = null
  }
  
  return {
    // 状态
    bluetoothStatus,
    currentTotp,
    deviceInfo,
    error,
    
    // 计算属性
    isConnected,
    isConnecting,
    hasTotp,
    
    // 方法
    setStatus,
    setTotp,
    setDeviceInfo,
    setError,
    reset
  }
})
