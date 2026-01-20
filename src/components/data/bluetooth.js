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
 * recv()函数 - 类似Python的socket.recv()
 * 阻塞等待接收蓝牙数据，直到收到数据或超时
 * 
 * 思考：用户想要类似Python recv()的函数，简单直接
 * 不像"开始监听"那么复杂，就是等待数据而已
 * 
 * @param {number} timeout - 超时时间，默认2000ms
 * @returns {Promise<string>} 收到的数据，超时返回空字符串
 */
export async function recv(timeout = 2000) {
  try {
    // 先确保蓝牙管理器初始化
    await ensureBluetoothManagerInitialized()
    
    console.log(`recv()开始等待数据，超时${timeout}ms`)
    // 不显示toast，避免频繁打扰用户，recv应该低调点
    
    return new Promise((resolve) => {
      let gotData = false
      let unlistenFunc = null
      
      // 超时定时器
      const timeoutTimer = setTimeout(() => {
        if (!gotData) {
          console.log(`recv()超时${timeout}ms，没收到数据`)
          // 超时不显示toast，避免干扰
          
          // 清理监听
          if (unlistenFunc) {
            try {
              unlistenFunc()
            } catch (e) {
              console.log('清理监听时有点小问题:', e)
            }
          }
          resolve('')  // 返回空字符串表示超时
        }
      }, timeout)
      
      // 监听Rust端发来的事件
      // 用动态import避免循环依赖问题
      import('@tauri-apps/api/event').then(({ listen }) => {
        // 监听蓝牙数据事件
        listen('bluetooth-data', (event) => {
          if (!gotData) {
            gotData = true
            clearTimeout(timeoutTimer)
            
            const data = event.payload
            console.log('recv()收到数据:', data)
            
            // 返回数据
            const result = typeof data === 'string' ? data : JSON.stringify(data)
            resolve(result)
            
            // 自动取消监听
            if (unlistenFunc) {
              try {
                unlistenFunc()
              } catch (e) {
                console.log('取消监听时有点小问题:', e)
              }
            }
          }
        }).then(unlisten => {
          // 保存取消监听的函数
          unlistenFunc = unlisten
        }).catch(err => {
          console.warn('设置recv监听时出错:', err)
          // 出错的话，等200ms就返回空
          setTimeout(() => {
            if (!gotData) {
              gotData = true
              clearTimeout(timeoutTimer)
              console.log('recv监听设置失败，返回空数据')
              resolve('')
            }
          }, 200)
        })
      }).catch(err => {
        console.error('导入事件模块失败:', err)
        // 导入失败，直接返回空
        if (!gotData) {
          gotData = true
          clearTimeout(timeoutTimer)
          console.log('事件模块加载失败，recv返回空数据')
          resolve('')
        }
      })
    })
    
  } catch (error) {
    console.error('recv()函数出错:', error)
    // recv失败不throw，直接返回空字符串，简单点
    return ''
  }
}

/**
 * 开始监听设备数据（阻塞式）
 * 这个函数现在只是recv()的包装，保持兼容性
 * 
 * @returns {Promise<string>} 收到的数据，如果超时返回空字符串
 */
export async function startListeningForData() {
  console.log('startListeningForData调用recv()...')
  // 调用Rust端启动监听（如果有必要的话）
  try {
    await ensureBluetoothManagerInitialized()
    await invoke('start_listening_for_data')
  } catch (err) {
    console.warn('调用Rust端启动监听失败:', err)
    // 继续，recv函数不依赖这个
  }
  
  return recv(2000)  // 默认2000ms超时
}

/**
 * 停止监听设备数据
 * 对于recv()这种阻塞函数，这个函数其实没啥用
 * 因为recv()会自己结束（超时或收到数据）
 * 但保留着吧，也许有其他用途
 * 
 * @returns {Promise<void>}
 */
export async function stopListeningForData() {
  try {
    await ensureBluetoothManagerInitialized()
    
    console.log('尝试停止监听...')
    // 不显示toast了，recv相关操作要低调
    
    await invoke('stop_listening_for_data')
    
    console.log('监听已停止')
  } catch (error) {
    console.warn('停止监听时出错:', error)
    // 出错就出错吧，不显示toast了
  }
}

/**
 * 检查是否正在监听数据
 * 对于阻塞式监听，这个函数其实不太准
 * 因为阻塞函数执行期间就是在监听，但Rust端可能有自己的状态
 * 保留这个函数吧，也许有用
 * 
 * @returns {Promise<boolean>} 是否正在监听
 */
export async function isListeningForData() {
  try {
    await ensureBluetoothManagerInitialized()
    
    const isListening = await invoke('is_listening_for_data')
    
    console.log(`Rust端说监听状态是: ${isListening}`)
    return isListening
  } catch (error) {
    console.warn('检查监听状态出错:', error)
    return false  // 出错就当作没在监听
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
      await new Promise(resolve => setTimeout(resolve, 1000))
      
      // 6. 获取TOTP
      let totp
      try {
        totp = await getTotpFromDevice()
        
        // 7. 成功获取TOTP后，开始监听设备数据（用户新需求）
        // 现在用recv()函数来监听
        try {
          console.log('TOTP获取成功，开始recv()监听数据...')
          // 这里不等待recv()，因为它是阻塞的
          // 启动一个异步任务来监听
          recv(2000).then(data => {
            if (data) {
              console.log('自动监听收到数据:', data)
            } else {
              console.log('自动监听超时，没收到数据')
            }
          }).catch(err => {
            console.warn('自动监听出错:', err)
          })
          console.log('已启动recv()监听任务')
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
          console.log('TOTP获取失败，但尝试recv()监听数据...')
          recv(2000).then(data => {
            if (data) {
              console.log('自动监听收到数据:', data)
            } else {
              console.log('自动监听超时，没收到数据')
            }
          }).catch(err => {
            console.warn('自动监听出错:', err)
          })
          console.log('已启动recv()监听任务')
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
  recv,                   // 新增：recv()函数，类似Python的socket.recv()
  startListeningForData,  // 保留兼容性
  stopListeningForData,   // 保留兼容性
  isListeningForData      // 保留兼容性
}
