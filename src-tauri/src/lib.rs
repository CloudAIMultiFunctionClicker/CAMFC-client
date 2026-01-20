// 使用简化的蓝牙模块
mod bluetooth;
use bluetooth::BluetoothManager;

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
static BLUETOOTH_MANAGER: OnceLock<Mutex<BluetoothManager>> = OnceLock::new();

/// 初始化蓝牙管理器
#[tauri::command]
async fn init_bluetooth_manager() -> Result<(), String> {
    println!("初始化蓝牙管理器...");
    
    if BLUETOOTH_MANAGER.get().is_some() {
        println!("蓝牙管理器已经初始化过了");
        return Ok(());
    }
    
    let manager = BluetoothManager::new();
    
    BLUETOOTH_MANAGER.set(Mutex::new(manager))
        .map_err(|_| "蓝牙管理器设置失败".to_string())?;
    
    println!("蓝牙管理器初始化成功");
    Ok(())
}

/// 获取全局蓝牙管理器的引用
fn get_bluetooth_manager() -> Result<&'static Mutex<BluetoothManager>, String> {
    BLUETOOTH_MANAGER.get().ok_or("蓝牙管理器未初始化，请先调用init_bluetooth_manager".to_string())
}

/// 扫描蓝牙设备
#[tauri::command]
async fn scan_bluetooth_devices() -> Result<Vec<String>, String> {
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    // 扫描5秒
    let devices = manager.scan_devices(5000).await
        .map_err(|e| format!("扫描失败: {}", e))?;
    
    if devices.is_empty() {
        Ok(vec!["没有发现蓝牙设备".to_string()])
    } else {
        let device_strings: Vec<String> = devices
            .iter()
            .map(|d| format!("{} - {}", d.name, d.address))
            .collect();
        Ok(device_strings)
    }
}


/// 连接蓝牙设备
/// 参数是 deviceInfo 字符串，格式 "设备名 - 地址"
/// 我们只提取地址部分使用
#[tauri::command]
async fn connect_to_device(device_info: String) -> Result<String, String> {
    println!("开始连接设备: {}", device_info);
    
    // 解析地址，格式是 "设备名 - 地址"
    let address = if let Some(addr) = device_info.split(" - ").nth(1) {
        addr.to_string()
    } else {
        // 如果格式不对，就用整个字符串作为地址
        device_info.clone()
    };
    
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    manager.connect(&address).await
        .map_err(|e| format!("连接失败: {}", e))?;
    
    Ok(format!("成功连接到设备: {}", device_info))
}

/// 发送命令到设备并获取响应
/// 使用Cpen设备的默认UUID
#[tauri::command]
async fn send_command_to_device(command: String) -> Result<String, String> {
    println!("发送命令: {}", command);
    
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    // Cpen设备UUID
    let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
    let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";
    
    // 发送命令
    manager.send(service_uuid, char_uuid, command.as_bytes()).await
        .map_err(|e| format!("发送失败: {}", e))?;
    
    // 接收响应
    let response = manager.recv(service_uuid, char_uuid).await
        .map_err(|e| format!("接收失败: {}", e))?;
    
    String::from_utf8(response)
        .map_err(|e| format!("响应数据不是有效UTF-8: {}", e))
}

/// 获取TOTP（快捷命令）
#[tauri::command]
async fn get_totp_from_device() -> Result<String, String> {
    println!("开始获取TOTP...");
    
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    let service_uuid = "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4";
    let char_uuid = "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4";
    
    manager.send(service_uuid, char_uuid, b"getTotp").await
        .map_err(|e| format!("发送失败: {}", e))?;
    
    let response = manager.recv(service_uuid, char_uuid).await
        .map_err(|e| format!("接收失败: {}", e))?;
    
    String::from_utf8(response)
        .map_err(|e| format!("响应数据不是有效UTF-8: {}", e))
}

/// 断开当前设备连接
#[tauri::command]
async fn disconnect_current_device() -> Result<String, String> {
    println!("准备断开设备连接...");
    
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    manager.disconnect().await
        .map_err(|e| format!("断开失败: {}", e))?;
    
    Ok("成功断开设备连接".to_string())
}

/// 清理蓝牙管理器
#[tauri::command]
async fn cleanup_bluetooth_manager() -> Result<(), String> {
    println!("清理蓝牙管理器...");
    
    if let Ok(manager_mutex) = get_bluetooth_manager() {
        let mut manager = manager_mutex.lock().await;
        let _ = manager.disconnect().await;
    }
    
    println!("蓝牙管理器清理完成");
    Ok(())
}

/// 简单的设备扫描功能（基础版本）
#[tauri::command]
async fn simple_scan_devices() -> Result<Vec<String>, String> {
    println!("执行简单设备扫描...");
    
    let mut manager = get_bluetooth_manager()?.lock().await;
    
    let devices = manager.scan_devices(5000).await
        .map_err(|e| format!("扫描失败: {}", e))?;
    
    let device_strings: Vec<String> = devices
        .iter()
        .map(|d| format!("{} - {}", d.name, d.address))
        .collect();
    
    println!("扫描完成，发现 {} 个设备", device_strings.len());
    Ok(device_strings)
}

// 监听功能暂时移除，bluetooth.rs没有实现
#[tauri::command]
async fn start_listening_for_data(_app: tauri::AppHandle) -> Result<(), String> {
    Err("数据监听功能未实现".to_string())
}



#[tauri::command]
async fn stop_listening_for_data() -> Result<(), String> {
    Err("数据监听功能未实现".to_string())
}

#[tauri::command]
async fn is_listening_for_data() -> Result<bool, String> {
    Ok(false)
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
            init_bluetooth_manager,
            cleanup_bluetooth_manager,
            start_listening_for_data,
            stop_listening_for_data,
            is_listening_for_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
