import { ref, watch } from 'vue'
import { useBluetoothStore } from '../stores/bluetooth.js'
import { showToast } from './useToast.js'
import {
  scanCpenDevices,
  connectCpenDevice
} from '../components/data/bluetooth.js'

const TOTP_REFRESH_INTERVAL = 30000
let totpRefreshInterval = null

export const startTotpRefresh = async () => {
  if (totpRefreshInterval) {
    clearInterval(totpRefreshInterval)
  }

  totpRefreshInterval = setInterval(async () => {
    try {
      const { getTotp } = await import('../components/data/bluetooth.js')
      await getTotp()
      console.log('[TOTP] 后台 TOTP 缓存刷新成功')
    } catch (error) {
      console.warn('[TOTP] TOTP 缓存刷新失败:', error.message)
    }
  }, TOTP_REFRESH_INTERVAL)
}

export const stopTotpRefresh = () => {
  if (totpRefreshInterval) {
    clearInterval(totpRefreshInterval)
    totpRefreshInterval = null
  }
}

export function useBluetooth() {
  const bluetoothStore = useBluetoothStore()

  const isScanning = ref(false)
  const cpenDevices = ref([])
  const isConnectingDevice = ref(false)
  const selectedDevice = ref(null)

  const isConnected = () => bluetoothStore.isConnected()
  const bluetoothStatus = () => bluetoothStore.bluetoothStatus

  const scanDevices = async () => {
    try {
      isScanning.value = true
      const cpenList = await scanCpenDevices()
      cpenDevices.value = cpenList
      return cpenList
    } catch (error) {
      console.error('扫描设备失败:', error)
      showToast('扫描设备失败: ' + error.message)
      return []
    } finally {
      isScanning.value = false
    }
  }

  const selectDevice = async (device) => {
    if (isConnectingDevice.value) return

    selectedDevice.value = device
    isConnectingDevice.value = true

    try {
      bluetoothStore.setStatus('connecting')

      const connectPromise = connectCpenDevice(device.address)
      const timeoutPromise = new Promise((_, reject) => {
        setTimeout(() => reject(new Error('连接超时')), 10000)
      })

      await Promise.race([connectPromise, timeoutPromise])

      bluetoothStore.setStatus('connected')
      showToast('设备连接成功！')
      return true
    } catch (error) {
      console.error('连接设备失败:', error)
      bluetoothStore.setError('连接失败')
      showToast('连接失败：' + error.message)
      return false
    } finally {
      isConnectingDevice.value = false
    }
  }

  const resetConnection = () => {
    bluetoothStore.reset()
    selectedDevice.value = null
  }

  return {
    isScanning,
    cpenDevices,
    isConnectingDevice,
    selectedDevice,
    isConnected,
    bluetoothStatus,
    scanDevices,
    selectDevice,
    resetConnection,
    startTotpRefresh,
    stopTotpRefresh
  }
}
