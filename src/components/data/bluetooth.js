/**
 * 简化版蓝牙模块
 * 
 * 重构原则：
 * 1. 所有连接逻辑移到Rust端
 * 2. 前端只调用两个简单API
 * 3. 状态管理由Rust端负责
 * 4. 前端不处理业务逻辑，只做UI展示
 * 
 * 注意：Rust端的CpenService会处理所有复杂逻辑：
 * - 自动扫描并连接CPen设备
 * - 30秒TOTP缓存
 * - 设备ID缓存
 * - 自动重连机制
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 获取TOTP - 核心API #1
 * 
 * 前端只需要调用这个函数，所有复杂逻辑都在Rust端处理：
 * 1. 检查30秒TOTP缓存
 * 2. 如果未连接，自动扫描并连接CPen设备
 * 3. 发送getTotp命令获取最新TOTP
 * 4. 更新缓存并返回结果
 * 
 * @returns {Promise<string>} TOTP字符串
 */
export async function getTotp() {
  try {
    console.info('调用getTotp()...')
    
    // 直接调用Rust端的get_totp命令
    // Rust端的CpenService会处理所有复杂逻辑
    const totp = await invoke('get_totp')
    
    console.info(`获取到TOTP: ${totp}`)
    
    // 用户要求：把返回值打印在console
    console.log(`TOTP: ${totp}`)
    
    return totp
  } catch (error) {
    console.error('获取TOTP失败:', error)
    throw new Error(`获取TOTP失败: ${error}`)
  }
}

/**
 * 获取设备ID - 核心API #2
 * 
 * 前端只需要调用这个函数，所有复杂逻辑都在Rust端处理：
 * 1. 检查设备ID缓存
 * 2. 如果未连接，自动扫描并连接CPen设备
 * 3. 发送getId命令获取设备UUID
 * 4. 更新缓存并返回结果
 * 
 * @returns {Promise<string>} 设备ID字符串
 */
export async function getDeviceId() {
  try {
    console.info('调用getDeviceId()...')
    
    // 直接调用Rust端的get_device_id命令
    // Rust端的CpenService会处理所有复杂逻辑
    const deviceId = await invoke('get_device_id')
    
    console.info(`获取到设备ID: ${deviceId}`)
    return deviceId
  } catch (error) {
    console.error('获取设备ID失败:', error)
    throw new Error(`获取设备ID失败: ${error}`)
  }
}

/**
 * 断开连接（可选，保持兼容性）
 * 
 * 断开当前已连接的蓝牙设备
 * Rust端会自动清理相关状态（TOTP缓存、设备ID等）
 * 
 * @returns {Promise<string>} 断开连接结果
 */
export async function disconnectCurrentDevice() {
  try {
    console.info('断开设备连接...')
    
    const result = await invoke('disconnect_current_device')
    
    console.info(`断开连接结果: ${result}`)
    return result
  } catch (error) {
    console.error('断开连接失败:', error)
    throw new Error(`断开连接失败: ${error}`)
  }
}

/**
 * 启动后台服务（可选，内部使用）
 * 
 * 应用启动时自动调用，不需要前端关心
 * Rust端会启动定时刷新TOTP、自动重连等后台任务
 * 
 * @returns {Promise<void>}
 */
export async function startBackgroundService() {
  try {
    console.info('启动蓝牙后台服务...')
    
    await invoke('start_background_service')
    console.info('后台服务已启动')
  } catch (error) {
    console.warn('启动后台服务失败:', error)
    // 失败不抛出错误，后台服务不是必需的
  }
}

// 导出所有函数
export default {
  getTotp,           // 核心API #1
  getDeviceId,       // 核心API #2
  disconnectCurrentDevice, // 可选API
  startBackgroundService  // 内部API
}
