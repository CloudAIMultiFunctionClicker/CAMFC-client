/**
 * CAMFC Client - 蓝牙设备接口模块
 * 
 * 所有业务逻辑都在 Rust 端的 CpenDeviceManager 实现
 * 前端只负责调用简单的 Tauri 命令
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 获取 TOTP 验证码
 * @returns {Promise<string>} TOTP 字符串
 * 
 * Rust 端会自动处理：扫描、识别、连接、发送命令、缓存
 */
export async function getTotp() {
  try {
    console.info('开始获取 TOTP...')
    const totp = await invoke('get_totp')
    console.info(`成功获取 TOTP: ${totp}`)
    return totp
  } catch (error) {
    console.error(`获取 TOTP 失败：${error}`)
    throw new Error(`获取 TOTP 失败：${error}`)
  }
}

/**
 * 扫描 Cpen 设备（不连接）
 * @returns {Promise<Array<{name: string, address: string}>>} 设备列表
 */
export async function scanCpenDevices() {
  try {
    console.info('开始扫描 Cpen 设备...')
    const devices = await invoke('scan_cpen_devices')
    console.info(`扫描完成，找到 ${devices.length} 个 Cpen 设备`)
    return devices
  } catch (error) {
    console.error(`扫描 Cpen 设备失败：${error}`)
    throw new Error(`扫描失败：${error}`)
  }
}

/**
 * 扫描所有蓝牙设备
 * @returns {Promise<Array<{name: string, address: string}>>} 设备列表
 * 
 * 注意：如果命令未实现，返回空数组
 */
export async function scanAllBluetoothDevices() {
  try {
    console.info('开始扫描所有蓝牙设备...')
    const devices = await invoke('scan_all_bluetooth_devices')
    console.info(`扫描完成，找到 ${devices.length} 个蓝牙设备`)
    return devices
  } catch (error) {
    console.error(`扫描蓝牙设备失败：${error}`)
    console.warn('命令可能未实现，返回空数组')
    return []
  }
}

/**
 * 连接到指定 Cpen 设备
 * @param {string} address 设备蓝牙地址
 * @returns {Promise<{name: string, address: string}>} 设备信息
 */
export async function connectCpenDevice(address) {
  try {
    console.info(`开始连接 Cpen 设备：${address}`)
    const deviceInfo = await invoke('connect_cpen_device', { address })
    console.info(`连接成功：${deviceInfo.name} (${deviceInfo.address})`)
    return deviceInfo
  } catch (error) {
    console.error(`连接 Cpen 设备失败：${error}`)
    throw new Error(`连接失败：${error}`)
  }
}

/**
 * 获取设备 ID（UUID）
 * @returns {Promise<string>} 设备 ID 字符串
 */
export async function getDeviceId() {
  try {
    console.info('开始获取设备 ID...')
    const deviceId = await invoke('get_device_id')
    console.info(`成功获取设备 ID: ${deviceId}`)
    return deviceId
  } catch (error) {
    console.error(`获取设备 ID 失败：${error}`)
    throw new Error(`获取设备 ID 失败：${error}`)
  }
}

/**
 * 获取当前连接状态描述
 * @returns {Promise<string>} 状态描述字符串
 */
export async function getConnectionStatus() {
  try {
    console.info('获取连接状态...')
    const status = await invoke('get_connection_status')
    console.info(`连接状态：${status}`)
    return status
  } catch (error) {
    console.error(`获取连接状态失败：${error}`)
    return `状态获取失败：${error}`
  }
}

/**
 * 检查是否已建立稳定连接
 * @returns {Promise<boolean>} true=已连接，false=未连接
 */
export async function isConnected() {
  try {
    console.info('检查稳定连接状态...')
    const connected = await invoke('is_connected')
    console.info(`稳定连接状态：${connected ? '已连接' : '未连接'}`)
    return connected
  } catch (error) {
    console.error(`检查连接状态失败：${error}`)
    return false
  }
}

/**
 * 断开蓝牙连接
 * @returns {Promise<void>}
 */
export async function disconnect() {
  try {
    console.info('断开蓝牙连接...')
    await invoke('disconnect')
    console.info('断开连接成功')
  } catch (error) {
    console.error(`断开连接失败：${error}`)
    console.warn('断开连接失败，但继续执行')
  }
}

/**
 * 清理蓝牙资源（比 disconnect 更彻底）
 * @returns {Promise<void>}
 */
export async function cleanup() {
  try {
    console.info('清理蓝牙资源...')
    await invoke('cleanup')
    console.info('清理完成')
  } catch (error) {
    console.error(`清理失败：${error}`)
  }
}

/**
 * 测试蓝牙功能
 * @returns {Promise<{available: boolean, status: string, connected: boolean}>}
 */
export async function testBluetooth() {
  try {
    console.info('测试蓝牙功能...')
    const connected = await isConnected()
    const status = await getConnectionStatus()
    
    return {
      available: true,
      connected,
      status,
      message: connected ? '蓝牙功能正常，设备已连接' : '蓝牙功能正常，但设备未连接'
    }
  } catch (error) {
    console.error(`蓝牙测试失败：${error}`)
    return {
      available: false,
      connected: false,
      status: 'error',
      message: `蓝牙测试失败：${error}`,
      error: error.toString()
    }
  }
}

/**
 * 模拟按下并松开右箭头键
 * @returns {Promise<void>}
 */
export async function pressWinKey() {
  try {
    await invoke('press_win_key')
  } catch (error) {
    console.error(`模拟右箭头键失败：${error}`)
    throw new Error(`模拟右箭头键失败：${error}`)
  }
}

/**
 * 模拟按下并松开左箭头键
 * @returns {Promise<void>}
 */
export async function pressLeftKey() {
  try {
    await invoke('press_left_key')
  } catch (error) {
    console.error(`模拟左箭头键失败：${error}`)
    throw new Error(`模拟左箭头键失败：${error}`)
  }
}

// 导出所有函数
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
