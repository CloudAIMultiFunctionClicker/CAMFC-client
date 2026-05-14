

import { invoke } from '@tauri-apps/api/core'

export async function getTotp() {
  try {
    console.info('开始获取TOTP...')

    const totp = await invoke('get_totp')

    console.info(`成功获取TOTP: ${totp}`)

    console.log(`TOTP: ${totp}`)

    return totp
  } catch (error) {
    console.error(`获取TOTP失败: ${error}`)

    throw new Error(`获取TOTP失败: ${error}`)
  }
}

export async function scanCpenDevices() {
  try {
    console.info('开始扫描Cpen设备...')

    const devices = await invoke('scan_cpen_devices')

    console.info(`扫描完成，找到 ${devices.length} 个Cpen设备`)

    return devices
  } catch (error) {
    console.error(`扫描Cpen设备失败: ${error}`)
    throw new Error(`扫描失败: ${error}`)
  }
}

export async function scanAllBluetoothDevices() {
  try {
    console.info('开始扫描所有蓝牙设备...')

    const devices = await invoke('scan_all_bluetooth_devices')

    console.info(`扫描完成，找到 ${devices.length} 个蓝牙设备`)

    return devices
  } catch (error) {
    console.error(`扫描蓝牙设备失败: ${error}`)

    console.warn('scan_all_bluetooth_devices 命令可能未实现，返回空数组')
    return []
  }
}

export async function connectCpenDevice(address) {
  try {
    console.info(`开始连接Cpen设备: ${address}`)

    const deviceInfo = await invoke('connect_cpen_device', { address })

    console.info(`连接成功: ${deviceInfo.name} (${deviceInfo.address})`)

    return deviceInfo
  } catch (error) {
    console.error(`连接Cpen设备失败: ${error}`)
    throw new Error(`连接失败: ${error}`)
  }
}

export async function getDeviceId() {
  try {
    console.info('开始获取设备ID...')

    const deviceId = await invoke('get_device_id')

    console.info(`成功获取设备ID: ${deviceId}`)

    return deviceId
  } catch (error) {
    console.error(`获取设备ID失败: ${error}`)
    throw new Error(`获取设备ID失败: ${error}`)
  }
}

export async function getConnectionStatus() {
  try {
    console.info('获取连接状态...')

    const status = await invoke('get_connection_status')

    console.info(`连接状态: ${status}`)

    return status
  } catch (error) {
    console.error(`获取连接状态失败: ${error}`)

    return `状态获取失败: ${error}`
  }
}

export async function isConnected() {
  try {

    const connected = await invoke('is_connected')

    return connected
  } catch (error) {
    console.error(`检查连接状态失败: ${error}`)

    return false
  }
}

export async function disconnect() {
  try {
    console.info('断开蓝牙连接...')

    await invoke('disconnect')

    console.info('断开连接成功')
  } catch (error) {
    console.error(`断开连接失败: ${error}`)

    console.warn('断开连接失败，但继续执行')
  }
}

export async function cleanup() {
  try {
    console.info('清理蓝牙资源...')

    await invoke('cleanup')

    console.info('清理完成')
  } catch (error) {
    console.error(`清理失败: ${error}`)

  }
}

export async function testBluetooth() {
  try {
    console.info('测试蓝牙功能（加强版）...')

    const connected = await isConnected()

    const status = await getConnectionStatus()

    return {
      available: true,
      connected,
      status,
      message: connected ? '蓝牙功能正常，设备已连接' : '蓝牙功能正常，但设备未连接'
    }
  } catch (error) {
    console.error(`蓝牙测试失败: ${error}`)
    return {
      available: false,
      connected: false,
      status: 'error',
      message: `蓝牙测试失败: ${error}`,
      error: error.toString()
    }
  }
}

export async function pressWinKey() {
  try {
    await invoke('press_win_key')
  } catch (error) {
    console.error(`模拟右箭头键失败: ${error}`)
    throw new Error(`模拟右箭头键失败: ${error}`)
  }
}

export async function pressLeftKey() {
  try {
    await invoke('press_left_key')
  } catch (error) {
    console.error(`模拟左箭头键失败: ${error}`)
    throw new Error(`模拟左箭头键失败: ${error}`)
  }
}

export default {
  getTotp,
  scanCpenDevices,
  connectCpenDevice,
  getDeviceId,
  getConnectionStatus,
  isConnected,
  disconnect,
  cleanup,
  testBluetooth,
  pressWinKey,
  pressLeftKey
}

