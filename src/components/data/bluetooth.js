/**
 * 蓝牙设备管理模块
 * 
 * 这个模块负责处理蓝牙相关的业务逻辑：
 * 1. 扫描蓝牙设备
 * 2. 查找Cpen设备
 * 3. 连接设备决策
 * 4. 管理设备状态
 * 
 * 注意：Rust端只提供基础的蓝牙交互功能，业务逻辑都在这里实现
 * 这样符合"主要逻辑在前端"的设计原则
 */

import { invoke } from '@tauri-apps/api/core'
import { showToast } from '../../composables/useToast'

// 蓝牙管理器初始化状态
let isBluetoothManagerInitialized = false

/**
 * 初始化蓝牙管理器
 * 
 * 调用Rust端的init_bluetooth_manager命令
 * 必须在其他蓝牙操作前调用，确保全局管理器实例存在
 * 
 * 思考：要不要做成自动的？还是让调用者显式调用？
 * 先做成自动的吧，在需要时自动初始化，简单点
 * 
 * @returns {Promise<void>}
 */
async function ensureBluetoothManagerInitialized() {
  if (isBluetoothManagerInitialized) {
    return
  }
  
  try {
    showToast('初始化蓝牙管理器...')
    await invoke('init_bluetooth_manager')
    isBluetoothManagerInitialized = true
    showToast('蓝牙管理器初始化完成')
  } catch (error) {
    console.error('蓝牙管理器初始化失败:', error)
    showToast(`蓝牙管理器初始化失败: ${error}`, '#ff0000')
    throw new Error(`蓝牙管理器初始化失败: ${error}`)
  }
}

/**
 * 清理蓝牙管理器
 * 
 * 调用Rust端的cleanup_bluetooth_manager命令
 * 断开连接并清理资源
 * 可以在不需要蓝牙时调用，或者在应用退出时调用
 * 
 * @returns {Promise<void>}
 */
export async function cleanupBluetoothManager() {
  try {
    showToast('清理蓝牙管理器...')
    await invoke('cleanup_bluetooth_manager')
    isBluetoothManagerInitialized = false
    showToast('蓝牙管理器清理完成')
  } catch (error) {
    console.error('蓝牙管理器清理失败:', error)
    showToast(`蓝牙管理器清理失败: ${error}`, '#ff0000')
    // 清理失败不抛出错误，因为可能只是小问题
  }
}

/**
 * 扫描蓝牙设备
 * 
 * 调用Rust端的scan_bluetooth_devices命令
 * 返回原始的蓝牙设备列表
 * 
 * @returns {Promise<Array<string>>} 蓝牙设备信息列表
 */
export async function scanDevices() {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    showToast('开始扫描蓝牙设备...')
    const devices = await invoke('scan_bluetooth_devices')
    showToast(`扫描完成，发现设备：${devices.join(', ')}`, '#55aa55')
    return devices
  } catch (error) {
    showToast(`扫描失败：${error}`, '#ff0000')
    throw new Error(`扫描失败: ${error}`)
  }
}

/**
 * 连接指定设备
 * 
 * 调用Rust端的connect_to_device命令
 * 只负责连接，不包含业务逻辑
 * 
 * @param {string} deviceInfo - 设备信息字符串，格式"设备名 - 地址"
 * @returns {Promise<string>} 连接结果
 */
export async function connectDevice(deviceInfo) {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    showToast(`开始连接设备: ${deviceInfo}`)
    const result = await invoke('connect_to_device', { deviceInfo })
    showToast(`连接结果: ${result}`, '#55aa55')
    return result
  } catch (error) {
    console.error('连接设备失败:', error)
    showToast(`连接设备失败: ${error}`, '#ff0000')
    throw new Error(`连接失败: ${error}`)
  }
}

/**
 * 从设备列表中查找Cpen设备
 * 
 * 这个是业务逻辑：判断哪些设备是Cpen设备
 * 根据设备名前缀"CPen"或"Cpen"来判断
 * 
 * @param {Array<string>} devices - 设备信息列表
 * @returns {Array<{name: string, address: string, displayInfo: string}>} Cpen设备列表
 */
export function findCpenDevices(devices) {
  if (!Array.isArray(devices)) {
    console.warn('设备列表不是数组:', devices)
    showToast('设备列表不是数组', '#ff0000')
    return []
  }
  
  // 思考：这里用正则还是startsWith？startsWith更简单
  // 但要注意大小写，有些设备可能叫"CPen"，有些是"Cpen"
  // 先统一转小写比较，避免大小写问题
  const cpenDevices = []
  
  devices.forEach(deviceStr => {
    // 设备字符串格式通常是"设备名 - 地址"
    // 但有时候可能只有设备名或地址，需要容错处理
    const parts = deviceStr.split(' - ')
    let name = deviceStr // 默认整个字符串作为名字
    let address = deviceStr // 默认整个字符串作为地址
    
    if (parts.length >= 2) {
      name = parts[0]
      address = parts[1]
    } else if (parts.length === 1) {
      // 只有一个部分，可能是只有名字或只有地址
      name = parts[0]
      address = parts[0] // 地址和名字相同
    }
    
    // 检查是否是Cpen设备
    // 注意：原代码检查的是前4个字符是否为'Cpen'，这里保持兼容
    if (name.slice(0, 4).toLowerCase() === 'cpen') {
      cpenDevices.push({
        name,
        address,
        displayInfo: deviceStr
      })
      showToast(`找到Cpen设备: ${deviceStr}`, '#55aa55')
    }
  })
  
  showToast(`共找到 ${cpenDevices.length} 个Cpen设备`, '#55aa55')
  return cpenDevices
}

/**
 * 自动连接Cpen设备（主要业务逻辑）
 * 
 * 这个函数实现了完整的业务逻辑：
 * 1. 扫描设备
 * 2. 查找Cpen设备
 * 3. 决定连接哪个（如果有多个）
 * 4. 尝试连接
 * 5. 返回详细结果
 * 
 * @returns {Promise<{success: boolean, message: string, scannedCount: number, cpenCount: number, connectedDevice?: string}>}
 */
export async function autoConnectCpen() {
  try {
    showToast('开始自动连接Cpen设备...')
    
    // 1. 扫描设备
    let devices
    try {
      devices = await scanDevices()
    } catch (scanError) {
      // 扫描失败，直接返回错误
      return {
        success: false,
        message: `扫描设备失败: ${scanError.message}`,
        scannedCount: 0,
        cpenCount: 0
      }
    }
    
    const scannedCount = devices.length
    
    // 2. 查找Cpen设备
    const cpenDevices = findCpenDevices(devices)
    const cpenCount = cpenDevices.length
    
    // 3. 处理结果
    if (cpenDevices.length === 0) {
      // 没找到Cpen设备
      const message = `扫描完成，未发现Cpen设备。共扫描到 ${scannedCount} 个其他设备。`
      showToast(message, '#ff0000')
      return {
        success: true, // 扫描本身成功了
        message,
        scannedCount,
        cpenCount
      }
    }
    
    // 4. 找到Cpen设备，决定连接哪个
    // 业务逻辑：如果有多个Cpen设备，连接第一个
    // TODO: 以后可以改进，比如让用户选择，或者连接信号最强的
    const targetDevice = cpenDevices[0]
    showToast(`发现 ${cpenCount} 个Cpen设备，尝试连接第一个: ${targetDevice.displayInfo}`)
    
    // 5. 尝试连接
    try {
      const connectResult = await connectDevice(targetDevice.displayInfo)
      
      const successMessage = `扫描完成，发现并成功连接Cpen设备: ${targetDevice.displayInfo}。共扫描到 ${scannedCount} 个设备。`
      showToast(successMessage, '#55aa55')
      
      return {
        success: true,
        message: successMessage,
        scannedCount,
        cpenCount,
        connectedDevice: targetDevice.displayInfo,
        rawResult: connectResult
      }
    } catch (connectError) {
      // 连接失败，但仍然返回扫描结果
      const failMessage = `扫描完成，发现Cpen设备但连接失败: ${targetDevice.displayInfo}。错误: ${connectError.message}。共扫描到 ${scannedCount} 个设备。`
      console.warn(failMessage)
      showToast(failMessage, '#ff0000')
      
      return {
        success: false,
        message: failMessage,
        scannedCount,
        cpenCount,
        attemptedDevice: targetDevice.displayInfo,
        error: connectError.message
      }
    }
    
  } catch (error) {
    // 未知错误
    console.error('自动连接Cpen过程中发生未知错误:', error)
    showToast(`自动连接失败: ${error.message}`, '#ff0000')
    return {
      success: false,
      message: `自动连接失败: ${error.message}`,
      scannedCount: 0,
      cpenCount: 0,
      error: error.message
    }
  }
}

/**
 * 简单扫描并返回设备列表（不自动连接）
 * 
 * 这个函数只扫描，不自动连接，适合手动操作场景
 * 
 * @returns {Promise<{success: boolean, devices: Array<string>, count: number, message: string}>}
 */
export async function simpleScan() {
  try {
    const devices = await scanDevices()
    return {
      success: true,
      devices,
      count: devices.length,
      message: `扫描完成，发现 ${devices.length} 个设备`
    }
  } catch (error) {
    showToast(`扫描失败: ${error.message}`, '#ff0000')
    return {
      success: false,
      devices: [],
      count: 0,
      message: `扫描失败: ${error.message}`
    }
  }
}

/**
 * 测试蓝牙连接
 * 
 * 这个函数可以用来测试蓝牙功能是否正常
 * 它执行一个简化的扫描过程，不进行实际连接
 * 
 * @returns {Promise<{bluetoothAvailable: boolean, testResult: string}>}
 */
export async function testBluetooth() {
  try {
    showToast('开始测试蓝牙功能...')
    
    // 尝试扫描设备，但只持续很短时间
    // 注意：scanDevices内部会调用Rust命令，Rust那边有固定的3秒扫描时间
    // 我们没办法控制扫描时间，因为那是Rust端实现的
    
    const result = await simpleScan()
    
    if (result.success) {
      return {
        bluetoothAvailable: true,
        testResult: `蓝牙功能正常，发现 ${result.count} 个设备`,
        deviceCount: result.count
      }
    } else {
      return {
        bluetoothAvailable: false,
        testResult: `蓝牙功能异常: ${result.message}`,
        error: result.message
      }
    }
  } catch (error) {
    showToast(`蓝牙测试失败: ${error.message}`, '#ff0000')
    return {
      bluetoothAvailable: false,
      testResult: `蓝牙测试失败: ${error.message}`,
      error: error.message
    }
  }
}

/**
 * 向已连接的设备发送命令
 * 
 * 这个函数会调用Rust端的send_command_to_device命令
 * 向当前已连接的蓝牙设备发送命令
 * 
 * 注意：需要先连接设备才能使用这个函数
 * 超时设为500ms，这是用户要求的
 * 
 * @param {string} command - 要发送的命令字符串
 * @returns {Promise<string>} 设备响应（如果有的话）
 */
export async function sendCommandToDevice(command) {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    showToast(`准备向设备发送命令: ${command}`)
    
    // 调用Rust端的send_command_to_device命令
    // 注意：超时已经在Rust端实现（500ms）
    const response = await invoke('send_command_to_device', { command })
    
    showToast(`命令发送完成，响应: ${response}`, '#55aa55')
    return response
  } catch (error) {
    console.error('发送命令失败:', error)
    showToast(`发送命令失败: ${error}`, '#ff0000')
    throw new Error(`发送命令失败: ${error}`)
  }
}

/**
 * 获取TOTP（主要功能）
 * 
 * 这个函数会：
 * 1. 向已连接的设备发送"getTotp"命令
 * 2. 接收TOTP响应
 * 3. 将TOTP打印到console（用户要求）
 * 4. 返回TOTP字符串
 * 
 * 注意：需要先连接设备才能使用这个函数
 * 超时设为500ms，这是用户要求的
 * 
 * @returns {Promise<string>} TOTP字符串
 */
export async function getTotpFromDevice() {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    showToast('开始获取TOTP...')
    
    // 调用Rust端的get_totp_from_device命令
    // 这个命令会处理完整的"getTotp"发送和接收流程
    const totp = await invoke('get_totp_from_device')
    
    showToast(`成功获取TOTP: ${totp}`, '#55aa55')
    
    // 用户要求：把返回值打印在console
    // 这里我们已经用console.info打印了，但再明确打印一次
    console.log(`TOTP: ${totp}`)
    
    return totp
  } catch (error) {
    console.error('获取TOTP失败:', error)
    showToast(`获取TOTP失败: ${error}`, '#ff0000')
    throw new Error(`获取TOTP失败: ${error}`)
  }
}

/**
 * 断开当前设备连接
 * 
 * 断开当前已连接的蓝牙设备
 * 
 * @returns {Promise<string>} 断开连接结果
 */
export async function disconnectCurrentDevice() {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    showToast('准备断开当前设备连接...')
    const result = await invoke('disconnect_current_device')
    showToast(`断开连接结果: ${result}`, '#55aa55')
    return result
  } catch (error) {
    console.error('断开连接失败:', error)
    showToast(`断开连接失败: ${error}`, '#ff0000')
    throw new Error(`断开连接失败: ${error}`)
  }
}

/**
 * 开始监听设备数据
 * 
 * 这个函数会调用Rust端的start_listening_for_data命令
 * 启动后台任务持续监听来自设备的数据包
 * 
 * 注意：需要先连接设备才能使用这个函数
 * 当收到数据时，会通过TAURI事件系统发送到前端
 * 
 * @returns {Promise<void>}
 */
export async function startListeningForData() {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    showToast('开始监听设备数据...')
    
    // 调用Rust端的start_listening_for_data命令
    // 注意：这个命令需要传入app handle，但TAURI会自动处理
    await invoke('start_listening_for_data')
    
    showToast('数据监听已启动', '#55aa55')
    console.log('数据监听已启动，等待设备发送数据...')
  } catch (error) {
    console.error('开始监听设备数据失败:', error)
    showToast(`开始监听失败: ${error}`, '#ff0000')
    throw new Error(`开始监听失败: ${error}`)
  }
}

/**
 * 停止监听设备数据
 * 
 * 这个函数会调用Rust端的stop_listening_for_data命令
 * 停止当前的数据监听任务
 * 
 * @returns {Promise<void>}
 */
export async function stopListeningForData() {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    showToast('停止监听设备数据...')
    
    // 调用Rust端的stop_listening_for_data命令
    await invoke('stop_listening_for_data')
    
    showToast('数据监听已停止', '#55aa55')
    console.log('数据监听已停止')
  } catch (error) {
    console.error('停止监听设备数据失败:', error)
    showToast(`停止监听失败: ${error}`, '#ff0000')
    throw new Error(`停止监听失败: ${error}`)
  }
}

/**
 * 检查是否正在监听数据
 * 
 * 这个函数会调用Rust端的is_listening_for_data命令
 * 返回当前的数据监听状态
 * 
 * @returns {Promise<boolean>} 是否正在监听数据
 */
export async function isListeningForData() {
  try {
    // 确保蓝牙管理器已初始化
    await ensureBluetoothManagerInitialized()
    
    // 调用Rust端的is_listening_for_data命令
    const isListening = await invoke('is_listening_for_data')
    
    console.log(`数据监听状态: ${isListening}`)
    return isListening
  } catch (error) {
    console.error('检查监听状态失败:', error)
    return false
  }
}

/**
 * 增强版自动连接Cpen设备并获取TOTP
 * 
 * 这个函数实现了完整的业务逻辑：
 * 1. 扫描设备
 * 2. 查找Cpen设备
 * 3. 连接第一个Cpen设备
 * 4. 发送"getTotp"命令获取TOTP
 * 5. 返回完整结果
 * 
 * 这是用户要求的主要功能：连接Cpen之后发送'getTotp'并且把返回值打印在console
 * 
 * @returns {Promise<{success: boolean, message: string, scannedCount: number, cpenCount: number, connectedDevice?: string, totp?: string, error?: string}>}
 */
export async function autoConnectAndGetTotp() {
  try {
    showToast('开始自动连接Cpen设备并获取TOTP...')
    
    // 1. 扫描设备
    let devices
    try {
      devices = await scanDevices()
    } catch (scanError) {
      return {
        success: false,
        message: `扫描设备失败: ${scanError.message}`,
        scannedCount: 0,
        cpenCount: 0
      }
    }
    
    const scannedCount = devices.length
    
    // 2. 查找Cpen设备
    const cpenDevices = findCpenDevices(devices)
    const cpenCount = cpenDevices.length
    
    // 3. 处理结果
    if (cpenDevices.length === 0) {
      const message = `扫描完成，未发现Cpen设备。共扫描到 ${scannedCount} 个其他设备。`
      showToast(message, '#ff0000')
      return {
        success: true, // 扫描本身成功了
        message,
        scannedCount,
        cpenCount
      }
    }
    
    // 4. 连接第一个Cpen设备
    const targetDevice = cpenDevices[0]
    showToast(`发现 ${cpenCount} 个Cpen设备，尝试连接第一个: ${targetDevice.displayInfo}`)
    
    try {
      const connectResult = await connectDevice(targetDevice.displayInfo)
      showToast('连接成功', '#55aa55')
      
      // 5. 连接成功后，等待一小会儿让设备稳定
      // 思考：要不要等？从MicroPython代码看，设备连接后应该立即可以通信
      // 但为了保险，等100ms吧
      await new Promise(resolve => setTimeout(resolve, 100))
      
      // 6. 获取TOTP
      let totp
      try {
        totp = await getTotpFromDevice()
        
        // 7. 成功获取TOTP后，开始监听设备数据（用户新需求）
        // 注意：这里不等待监听启动，因为监听是后台任务
        // 如果监听启动失败，不影响TOTP获取的成功状态
        try {
          await startListeningForData()
          console.log('TOTP获取成功后已自动开始监听设备数据')
        } catch (listenError) {
          console.warn('开始监听设备数据失败，但不影响TOTP获取:', listenError)
        }
        
        const successMessage = `扫描完成，成功连接Cpen设备并获取TOTP: ${totp}。设备: ${targetDevice.displayInfo}。共扫描到 ${scannedCount} 个设备。`
        showToast(successMessage, '#55aa55')
        
        return {
          success: true,
          message: successMessage,
          scannedCount,
          cpenCount,
          connectedDevice: targetDevice.displayInfo,
          totp,
          rawResult: connectResult
        }
      } catch (totpError) {
        // 连接成功但获取TOTP失败，仍然尝试开始监听（用户需求）
        try {
          await startListeningForData()
          console.log('TOTP获取失败但已开始监听设备数据')
        } catch (listenError) {
          console.warn('开始监听设备数据失败:', listenError)
        }
        
        const failMessage = `扫描完成，成功连接Cpen设备但获取TOTP失败: ${totpError.message}。设备: ${targetDevice.displayInfo}。共扫描到 ${scannedCount} 个设备。`
        console.warn(failMessage)
        showToast(failMessage, '#ff0000')
        
        return {
          success: false,
          message: failMessage,
          scannedCount,
          cpenCount,
          connectedDevice: targetDevice.displayInfo,
          error: totpError.message
        }
      }
      
    } catch (connectError) {
      // 连接失败
      const failMessage = `扫描完成，发现Cpen设备但连接失败: ${targetDevice.displayInfo}。错误: ${connectError.message}。共扫描到 ${scannedCount} 个设备。`
      console.warn(failMessage)
      showToast(failMessage, '#ff0000')
      
      return {
        success: false,
        message: failMessage,
        scannedCount,
        cpenCount,
        attemptedDevice: targetDevice.displayInfo,
        error: connectError.message
      }
    }
    
  } catch (error) {
    // 未知错误
    console.error('自动连接Cpen并获取TOTP过程中发生未知错误:', error)
    showToast(`自动连接失败: ${error.message}`, '#ff0000')
    return {
      success: false,
      message: `自动连接失败: ${error.message}`,
      scannedCount: 0,
      cpenCount: 0,
      error: error.message
    }
  }
}

// 导出所有函数
export default {
  scanDevices,
  connectDevice,
  findCpenDevices,
  autoConnectCpen,
  autoConnectAndGetTotp, // 新增：增强版自动连接并获取TOTP
  simpleScan,
  testBluetooth,
  sendCommandToDevice,
  getTotpFromDevice,
  disconnectCurrentDevice,
  cleanupBluetoothManager, // 新增：清理函数
  startListeningForData,  // 新增：开始监听数据
  stopListeningForData,   // 新增：停止监听数据
  isListeningForData      // 新增：检查监听状态
}
