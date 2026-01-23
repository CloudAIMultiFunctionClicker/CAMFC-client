/**
 * 蓝牙状态管理store（重构版）
 * 只管理蓝牙连接状态，不存储TOTP和设备ID
 * 
 * 重构说明：TOTP和设备ID改为直接从Rust命令获取，不再通过store缓存
 * 这样保证数据实时性，简化状态管理
 * 
 * TODO: 考虑是否还需要deviceInfo，可能也改为直接从Rust获取？
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useBluetoothStore = defineStore('bluetooth', () => {
  // 蓝牙状态：连接中、已连接、断开
  const bluetoothStatus = ref('disconnected') // disconnected/connecting/connected
  
  // 设备信息（可能还需要，先保留）
  const deviceInfo = ref(null)
  
  // 错误信息，有错误才显示
  const error = ref(null)
  
  // 简单状态判断
  const isConnected = () => bluetoothStatus.value === 'connected'
  const isConnecting = () => bluetoothStatus.value === 'connecting'
  
  // 更新状态
  const setStatus = (status) => {
    bluetoothStatus.value = status
    if (status !== 'error') {
      error.value = null // 状态正常就清掉错误
    }
  }
  
  const setDeviceInfo = (info) => {
    deviceInfo.value = info
  }
  
  const setError = (err) => {
    error.value = err
    bluetoothStatus.value = 'error'
  }
  
  // 重置状态（断开连接时用）
  const reset = () => {
    bluetoothStatus.value = 'disconnected'
    deviceInfo.value = null
    error.value = null
    console.log('蓝牙状态已重置：断开连接')
  }
  
  return {
    // 状态
    bluetoothStatus,
    deviceInfo,
    error,
    
    // 计算属性
    isConnected,
    isConnecting,
    
    // 方法
    setStatus,
    setDeviceInfo,
    setError,
    reset
  }
})
