// 导入蓝牙管理器模块
mod bluetooth_manager;
use bluetooth_manager::{BluetoothManager, BluetoothOperations, BluetoothDevice};

// 导入同步原语
// 注意：原来用std::sync::Mutex，但MutexGuard不是Send，会导致编译错误
// 改成tokio::sync::Mutex，它的MutexGuard是Send的，适合异步环境
use tokio::sync::Mutex;
use std::sync::OnceLock;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 全局蓝牙管理器实例
// 用OnceLock确保只初始化一次，Mutex保证线程安全
// 注意：这里必须用tokio::sync::Mutex，不能用std::sync::Mutex
// 因为std::sync::Mutex的MutexGuard不是Send，在async/await里会报错
static BLUETOOTH_MANAGER: OnceLock<Mutex<BluetoothManager>> = OnceLock::new();

/// 初始化蓝牙管理器
/// 
/// 这个命令会创建全局蓝牙管理器实例
/// JS端在开始蓝牙操作前应该先调用这个
/// 
/// 为啥要单独初始化？因为原来每个命令都new一个实例，状态不共享
/// 现在改成全局单例，状态就能保持了
#[tauri::command]
async fn init_bluetooth_manager() -> Result<(), String> {
    println!("初始化蓝牙管理器...");
    
    // 如果已经初始化过，就直接返回成功
    if BLUETOOTH_MANAGER.get().is_some() {
        println!("蓝牙管理器已经初始化过了");
        return Ok(());
    }
    
    // 创建新的管理器实例
    let manager = BluetoothManager::new();
    
    // 尝试设置到全局变量
    match BLUETOOTH_MANAGER.set(Mutex::new(manager)) {
        Ok(_) => {
            println!("蓝牙管理器初始化成功");
            Ok(())
        }
        Err(_) => {
            // 理论上不会到这里，因为上面已经检查过了
            println!("警告：蓝牙管理器设置失败，可能已经初始化过了");
            Ok(()) // 即使失败也返回成功，因为可能其他线程已经初始化了
        }
    }
}

/// 获取全局蓝牙管理器的引用
/// 
/// 内部用的辅助函数，检查管理器是否已初始化
/// 没初始化就返回错误
fn get_bluetooth_manager() -> Result<&'static Mutex<BluetoothManager>, String> {
    BLUETOOTH_MANAGER.get().ok_or("蓝牙管理器未初始化，请先调用init_bluetooth_manager".to_string())
}

/// 扫描蓝牙设备
/// 
/// 这个命令会扫描周围的蓝牙设备并返回列表
/// 用全局蓝牙管理器实现，状态可以保持
#[tauri::command]
async fn scan_bluetooth_devices() -> Result<Vec<String>, String> {
    use std::time::Duration;
    
    // 获取全局管理器
    // 注意：这里用.await而不是.unwrap()，因为tokio::sync::Mutex是异步的
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    // 扫描设备，持续3秒
    match manager.scan_devices(Duration::from_secs(3)).await {
        Ok(devices) => {
            // 转换成前端需要的格式
            let device_strings: Vec<String> = devices
                .iter()
                .map(|d| d.display_info())
                .collect();
            
            if device_strings.is_empty() {
                Ok(vec!["没有发现蓝牙设备".to_string()])
            } else {
                Ok(device_strings)
            }
        }
        Err(e) => {
            println!("扫描蓝牙设备失败: {}", e);
            Err(format!("扫描蓝牙设备失败: {}", e))
        }
    }
}

/// 连接蓝牙设备
/// 
/// 这个函数会尝试连接指定的设备
/// 参数是设备信息字符串，格式是 "设备名 - 地址"
/// 用全局蓝牙管理器实现
#[tauri::command]
async fn connect_to_device(device_info: String) -> Result<String, String> {
    println!("开始连接设备: {}", device_info);
    
    // 解析设备信息
    let parts: Vec<&str> = device_info.split(" - ").collect();
    let (name, address) = if parts.len() >= 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        // 如果格式不对，尝试解析
        // TODO: 这里可以更智能一点
        (device_info.clone(), device_info.clone())
    };
    
    // 创建蓝牙设备对象
    let device = BluetoothDevice::new(name, address);
    
    // 获取全局管理器
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    // 尝试连接设备
    match manager.connect_device(&device).await {
        Ok(_) => {
            let result = format!("成功连接到设备: {}", device_info);
            println!("{}", result);
            Ok(result)
        }
        Err(e) => {
            let error_msg = format!("连接设备失败: {}，设备: {}", e, device_info);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

/// 向已连接的设备发送命令
/// 
/// 这个命令会向当前已连接的蓝牙设备发送命令
/// 注意：需要先连接设备才能使用这个命令
/// 超时设为500ms，这是用户要求的
/// 现在用全局管理器，连接状态能保持
#[tauri::command]
async fn send_command_to_device(command: String) -> Result<String, String> {
    println!("准备发送命令到设备: {}", command);
    
    // 获取全局管理器
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    match manager.send_command(&command).await {
        Ok(data) => {
            if data.is_empty() {
                let result = format!("命令发送成功，但未收到响应数据");
                println!("{}", result);
                Ok(result)
            } else {
                // 尝试将字节数据转换为字符串
                match String::from_utf8(data) {
                    Ok(response_str) => {
                        let result = format!("命令发送成功，收到响应: {}", response_str);
                        println!("{}", result);
                        Ok(response_str) // 直接返回响应字符串，方便前端使用
                    }
                    Err(e) => {
                        let error_msg = format!("命令发送成功，但响应数据不是有效UTF-8: {}", e);
                        println!("{}", error_msg);
                        Err(error_msg)
                    }
                }
            }
        }
        Err(e) => {
            let error_msg = format!("发送命令失败: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

/// 获取TOTP（快捷命令）
/// 
/// 这个命令会发送"getTotp"到已连接的设备并返回TOTP
/// 这是用户要求的主要功能
/// 现在用全局管理器，连接状态能保持
#[tauri::command]
async fn get_totp_from_device() -> Result<String, String> {
    println!("开始获取TOTP...");
    
    // 获取全局管理器
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    match manager.send_get_totp().await {
        Ok(totp) => {
            let result = format!("成功获取TOTP: {}", totp);
            println!("{}", result);
            Ok(totp) // 直接返回TOTP字符串
        }
        Err(e) => {
            let error_msg = format!("获取TOTP失败: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

/// 断开当前设备连接
/// 
/// 断开当前已连接的蓝牙设备
/// 用全局管理器，可以断开当前连接
#[tauri::command]
async fn disconnect_current_device() -> Result<String, String> {
    println!("准备断开当前设备连接...");
    
    // 获取全局管理器
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    match manager.disconnect_current().await {
        Ok(_) => {
            let result = "成功断开设备连接".to_string();
            println!("{}", result);
            Ok(result)
        }
        Err(e) => {
            let error_msg = format!("断开连接失败: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

/// 清理蓝牙管理器
/// 
/// 断开连接并清理资源
/// JS端可以在不需要蓝牙时调用这个
#[tauri::command]
async fn cleanup_bluetooth_manager() -> Result<(), String> {
    println!("清理蓝牙管理器...");
    
    // 先尝试断开当前连接
    if let Ok(manager_mutex) = get_bluetooth_manager() {
        let mut manager = manager_mutex.lock().await;
        
        // 尝试断开连接
        match manager.disconnect_current().await {
            Ok(_) => println!("已断开设备连接"),
            Err(e) => println!("断开连接时出错: {} (可能本来就没连接)", e),
        }
    }
    
    // 注意：OnceLock不能直接清除内容，但我们可以忽略这个问题
    // 因为应用重启时会重新初始化
    // 如果真的有需要，可以考虑用Option包装，但有点复杂，先这样吧
    println!("蓝牙管理器清理完成");
    Ok(())
}

/// 开始监听设备数据
/// 
/// 这个命令会启动后台任务，持续监听来自已连接设备的数据包
/// 当收到数据时，会通过"bluetooth-data"事件发送到前端
/// 
/// 注意：需要先连接设备才能使用这个命令
#[tauri::command]
async fn start_listening_for_data(app: tauri::AppHandle) -> Result<(), String> {
    println!("开始监听设备数据...");
    
    // 获取全局管理器
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    match manager.start_data_listening(app).await {
        Ok(_) => {
            println!("数据监听启动成功");
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("启动数据监听失败: {}", e);
            println!("{}", error_msg);
            Err(error_msg)
        }
    }
}

/// 停止监听设备数据
/// 
/// 停止当前的数据监听任务
#[tauri::command]
async fn stop_listening_for_data() -> Result<(), String> {
    println!("停止监听设备数据...");
    
    // 获取全局管理器
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    manager.stop_data_listening().await;
    println!("数据监听已停止");
    Ok(())
}

/// 检查是否正在监听数据
#[tauri::command]
async fn is_listening_for_data() -> Result<bool, String> {
    // 获取全局管理器
    let manager = get_bluetooth_manager()?.lock().await;
    
    let is_listening = manager.is_listening_data();
    println!("数据监听状态: {}", is_listening);
    Ok(is_listening)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            scan_bluetooth_devices, 
            simple_scan_devices, 
            connect_to_device,
            send_command_to_device,
            get_totp_from_device,
            disconnect_current_device,
            init_bluetooth_manager,  // 新增：初始化命令
            cleanup_bluetooth_manager, // 新增：清理命令
            start_listening_for_data,  // 新增：开始监听数据
            stop_listening_for_data,   // 新增：停止监听数据
            is_listening_for_data,     // 新增：检查监听状态
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 简单的设备扫描功能（基础版本）
/// 
/// 这个函数只扫描设备并返回原始设备列表，不包含业务逻辑
/// 业务逻辑（如查找Cpen设备、连接决策）应该在前端实现
/// 这样符合"主要逻辑在前端"的设计原则
#[tauri::command]
async fn simple_scan_devices() -> Result<Vec<String>, String> {
    use std::time::Duration;
    
    println!("执行简单设备扫描（基础功能）...");
    
    // 获取全局管理器
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    // 扫描设备，持续3秒
    match manager.scan_devices(Duration::from_secs(3)).await {
        Ok(devices) => {
            // 转换成前端需要的格式
            let device_strings: Vec<String> = devices
                .iter()
                .map(|d| d.display_info())
                .collect();
            
            println!("扫描完成，发现 {} 个设备", device_strings.len());
            Ok(device_strings)
        }
        Err(e) => {
            println!("设备扫描失败: {}", e);
            Err(format!("设备扫描失败: {}", e))
        }
    }
}
