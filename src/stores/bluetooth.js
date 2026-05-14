

import { defineStore } from 'pinia'
import { ref } from 'vue'
import { emit } from '@tauri-apps/api/event'

export const useBluetoothStore = defineStore('bluetooth', () => {

  const bluetoothStatus = ref('disconnected')

  const deviceInfo = ref(null)

  const error = ref(null)

  const isConnected = () => bluetoothStatus.value === 'connected'
  const isConnecting = () => bluetoothStatus.value === 'connecting'

  const setStatus = async (status) => {
    bluetoothStatus.value = status
    if (status !== 'error') {
      error.value = null
    }

    try {
      const isConnectedStatus = status === 'connected'
      await emit('connection-status', isConnectedStatus)
      console.log('已发送连接状态事件到悬浮窗:', isConnectedStatus)
    } catch (e) {
      console.log('发送连接状态事件失败（非Tauri环境）:', e)
    }
  }

  const setDeviceInfo = (info) => {
    deviceInfo.value = info
  }

  const setError = (err) => {
    error.value = err
    bluetoothStatus.value = 'error'

    try {
      emit('connection-status', false)
    } catch (e) {
      console.log('发送连接状态事件失败（非Tauri环境）:', e)
    }
  }

  const reset = () => {
    bluetoothStatus.value = 'disconnected'
    deviceInfo.value = null
    error.value = null
    console.log('蓝牙状态已重置：断开连接')

    try {
      emit('connection-status', false)
    } catch (e) {
      console.log('发送连接状态事件失败（非Tauri环境）:', e)
    }
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
