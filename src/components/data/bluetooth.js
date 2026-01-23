/**
 * 蓝牙设备接口模块（简化版）
 * 
 * 重构说明：
 * 1. 所有业务逻辑已迁移到Rust端的CpenDeviceManager
 * 2. 前端只调用少数几个简单的Tauri命令
 * 3. 保证全局只连接一个Cpen设备（由Rust端实现）
 * 4. 保留原有功能完全不变
 * 
 * 思考：这样前端代码就简单多了，不需要处理扫描、连接、缓存等复杂逻辑。
 * Rust端会处理所有事情，前端只需要调用命令并显示结果。
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 获取TOTP
 * 
 * 调用Rust端的get_totp命令，内部会：
 * 1. 自动扫描蓝牙设备
 * 2. 自动识别Cpen设备（根据名前缀）
 * 3. 保证只连接一个Cpen设备
 * 4. 发送setTime和getTotp命令
 * 5. 30秒TOTP缓存
 * 
 * 返回值：TOTP字符串，或者错误信息
 * 
 * 注意：这个函数会阻塞直到TOTP获取完成（包括扫描和连接时间）
 * 
 * @returns {Promise<string>} TOTP字符串
 */
export async function getTotp() {
  try {
    console.info('开始获取TOTP...')
    
    // 调用Rust命令，所有业务逻辑都在Rust端处理
    const totp = await invoke('get_totp')
    
    console.info(`成功获取TOTP: ${totp}`)
    
    // 计划：把返回值打印在console
    console.log(`TOTP: ${totp}`)
    
    return totp
  } catch (error) {
    console.error(`获取TOTP失败: ${error}`)
    // 直接抛出错误，让调用者处理
    throw new Error(`获取TOTP失败: ${error}`)
  }
}

/**
 * 获取设备ID（设备UUID）
 * 
 * 调用Rust端的get_device_id命令，内部会：
 * 1. 自动连接设备（如果还没连接）
 * 2. 发送getId命令
 * 3. 缓存设备ID
 * 
 * 注意：这个函数通常比getTotp快，因为不需要发送setTime命令
 * 
 * @returns {Promise<string>} 设备ID字符串
 */
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

/**
 * 获取连接状态
 * 
 * 调用Rust端的get_connection_status命令
 * 返回当前连接状态的描述字符串
 * 
 * 这个命令不会尝试连接设备，只返回当前状态
 * 
 * @returns {Promise<string>} 连接状态描述
 */
export async function getConnectionStatus() {
  try {
    console.info('获取连接状态...')
    
    const status = await invoke('get_connection_status')
    
    console.info(`连接状态: ${status}`)
    
    return status
  } catch (error) {
    console.error(`获取连接状态失败: ${error}`)
    // 状态获取失败也返回一个默认状态
    return `状态获取失败: ${error}`
  }
}

/**
 * 检查是否已建立稳定连接
 * 
 * 调用Rust端的is_connected命令
 * 返回布尔值：true表示已建立稳定连接，false表示未连接或连接已断开
 * 
 * 注意：这个方法会实际检查蓝牙物理连接状态，而不仅仅是内存中的记录
 * 可以用来在操作前验证连接是否真的还活着
 * 
 * @returns {Promise<boolean>} 是否已建立稳定连接
 */
export async function isConnected() {
  try {
    console.info('检查稳定连接状态...')
    
    const connected = await invoke('is_connected')
    
    console.info(`稳定连接状态: ${connected ? '已连接' : '未连接'}`)
    
    return connected
  } catch (error) {
    console.error(`检查连接状态失败: ${error}`)
    // 检查失败时，保守返回false
    return false
  }
}

/**
 * 断开蓝牙连接
 * 
 * 调用Rust端的disconnect命令
 * 断开当前连接并清理所有缓存
 * 
 * 注意：断开后，下次调用getTotp或getDeviceId会自动重新连接
 * 
 * @returns {Promise<void>}
 */
export async function disconnect() {
  try {
    console.info('断开蓝牙连接...')
    
    await invoke('disconnect')
    
    console.info('断开连接成功')
  } catch (error) {
    console.error(`断开连接失败: ${error}`)
    // 断开失败不抛出错误，因为可能已经断开了
    console.warn('断开连接失败，但继续执行')
  }
}

/**
 * 清理蓝牙资源
 * 
 * 调用Rust端的cleanup命令
 * 比disconnect更彻底，但一般用disconnect就够了
 * 
 * 这个函数可以在应用退出时调用
 * 
 * @returns {Promise<void>}
 */
export async function cleanup() {
  try {
    console.info('清理蓝牙资源...')
    
    await invoke('cleanup')
    
    console.info('清理完成')
  } catch (error) {
    console.error(`清理失败: ${error}`)
    // 清理失败也继续，不抛出错误
  }
}

/**
 * 测试蓝牙功能（加强版）
 * 
 * 通过检查连接状态来测试蓝牙功能是否正常
 * 现在使用isConnected来验证稳定连接状态
 * 
 * @returns {Promise<{available: boolean, status: string, connected: boolean}>}
 */
export async function testBluetooth() {
  try {
    console.info('测试蓝牙功能（加强版）...')
    
    // 首先检查稳定连接状态
    const connected = await isConnected()
    
    // 获取详细的连接状态描述
    const status = await getConnectionStatus()
    
    return {
      available: true,
      connected, // 新增：稳定连接状态
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

// 导出所有函数（简化版）
export default {
  getTotp,
  getDeviceId,
  getConnectionStatus,
  isConnected,
  disconnect,
  cleanup,
  testBluetooth
}

// 注意：以下旧函数已删除，因为业务逻辑已迁移到Rust端：
// - scanDevices
// - connectDevice
// - findCpenDevices
// - autoConnectCpen
// - autoConnectAndGetTotp
// - autoConnectAndGetTotpWithPinia
// - simpleScan
// - sendCommandToDevice
// - recv
// - startListeningForData
// - stopListeningForData
// - isListeningForData
// - registerPiniaCallbacks
// - startTotpRefreshTimer
// - stopTotpRefreshTimer
// 
// 如果需要这些功能，应该扩展Rust端的接口，而不是在前端实现业务逻辑。