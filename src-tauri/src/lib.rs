// 蓝牙模块导入
mod bluetooth;
mod cpen_device_manager;

// 使用新的Cpen设备管理器作为业务逻辑层
use cpen_device_manager::CpenDeviceManager;

// 导入同步原语
// 原来用tokio::sync::Mutex，继续用这个，适合异步环境
use tokio::sync::Mutex;
use std::sync::OnceLock;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 全局Cpen设备管理器实例
// 用OnceLock确保只初始化一次，Mutex保证线程安全
// 思考：原来的蓝牙管理器现在作为底层被CpenDeviceManager使用
static CPEN_DEVICE_MANAGER: OnceLock<Mutex<CpenDeviceManager>> = OnceLock::new();

/// 初始化Cpen设备管理器（懒初始化，实际用的时候再初始化）
fn get_cpen_device_manager() -> Result<&'static Mutex<CpenDeviceManager>, String> {
    // 使用get()检查是否已初始化，如果没有则初始化
    if let Some(manager) = CPEN_DEVICE_MANAGER.get() {
        return Ok(manager);
    }
    
    // 初始化新的管理器
    println!("自动初始化Cpen设备管理器...");
    let new_manager = Mutex::new(CpenDeviceManager::new());
    
    // 设置到OnceLock中
    CPEN_DEVICE_MANAGER.set(new_manager)
        .map_err(|_| "Cpen设备管理器已初始化".to_string())?;
    
    // 现在可以安全地获取引用
    Ok(CPEN_DEVICE_MANAGER.get().unwrap())
}

// 注意：以下旧的复杂命令已删除，业务逻辑已迁移到CpenDeviceManager中
// - scan_bluetooth_devices
// - connect_to_device
// - send_command_to_device (原来的复杂版本)
// - get_totp_from_device (原来的版本)
// - disconnect_current_device
// - simple_scan_devices
// - 各种监听命令

// 现在只暴露少数几个简洁的命令给前端

/// 获取TOTP（主要业务功能）
/// 
/// 前端只需要调用这个命令，所有业务逻辑都在Rust端处理：
/// 1. 自动扫描蓝牙设备
/// 2. 自动识别Cpen设备（根据名前缀）
/// 3. 保证只连接一个Cpen设备（重要！）
/// 4. 自动发送setTime和getTotp命令
/// 5. 30秒TOTP缓存
/// 
/// 返回值：TOTP字符串，或包含错误信息的字符串
#[tauri::command]
async fn get_totp() -> Result<String, String> {
    println!("前端调用get_totp命令...");
    
    let mut manager = get_cpen_device_manager()?.lock().await;
    
    match manager.get_totp().await {
        Ok(totp) => {
            // 成功获取TOTP，返回给前端
            println!("TOTP获取成功，返回给前端");
            Ok(totp)
        }
        Err(e) => {
            // 获取失败，返回错误信息
            println!("TOTP获取失败: {}", e);
            Err(format!("获取TOTP失败: {}", e))
        }
    }
}

/// 获取设备ID（设备UUID）
/// 
/// 前端调用这个命令获取设备唯一标识。
/// 内部会自动处理连接、发送getId命令等。
/// 
/// 返回值：设备ID字符串，或包含错误信息的字符串
#[tauri::command]
async fn get_device_id() -> Result<String, String> {
    println!("前端调用get_device_id命令...");
    
    let mut manager = get_cpen_device_manager()?.lock().await;
    
    match manager.get_device_id().await {
        Ok(device_id) => {
            println!("设备ID获取成功，返回给前端");
            Ok(device_id)
        }
        Err(e) => {
            println!("设备ID获取失败: {}", e);
            Err(format!("获取设备ID失败: {}", e))
        }
    }
}

/// 获取连接状态
/// 
/// 前端可以调用这个命令获取当前连接状态。
/// 返回格式化的状态字符串，包含设备信息。
/// 
/// 思考：这个命令比较简单，不会尝试连接设备，只返回当前状态。
#[tauri::command]
async fn get_connection_status() -> Result<String, String> {
    println!("前端调用get_connection_status命令...");
    
    let manager = get_cpen_device_manager()?.lock().await;
    
    let status = manager.get_connection_status();
    println!("当前连接状态: {}", status);
    
    Ok(status)
}

/// 断开连接并清理资源
/// 
/// 前端可以调用这个命令手动断开蓝牙连接。
/// 会清理所有缓存和连接状态。
/// 
/// 注意：断开后，下次调用get_totp或get_device_id会自动重新连接。
#[tauri::command]
async fn disconnect() -> Result<(), String> {
    println!("前端调用disconnect命令...");
    
    let mut manager = get_cpen_device_manager()?.lock().await;
    
    match manager.disconnect().await {
        Ok(_) => {
            println!("断开连接成功");
            Ok(())
        }
        Err(e) => {
            println!("断开连接失败: {}", e);
            Err(format!("断开连接失败: {}", e))
        }
    }
}

/// 清理所有蓝牙资源
/// 
/// 这个命令可以用于应用退出时，或者需要完全重置蓝牙状态时。
/// 比disconnect更彻底，但一般用disconnect就够了。
#[tauri::command]
async fn cleanup() -> Result<(), String> {
    println!("前端调用cleanup命令...");
    
    // 实际上和disconnect差不多，就叫cleanup保持兼容性
    let mut manager = get_cpen_device_manager()?.lock().await;
    
    match manager.disconnect().await {
        Ok(_) => {
            println!("清理成功");
            Ok(())
        }
        Err(e) => {
            println!("清理失败: {}", e);
            Err(format!("清理失败: {}", e))
        }
    }
}

// 注意：以下旧的命令已删除，因为业务逻辑已迁移到CpenDeviceManager
// - simple_scan_devices
// - start_listening_for_data
// - stop_listening_for_data  
// - is_listening_for_data
//
// 如果前端需要设备扫描功能，可以考虑加一个简单的scan命令，但用户说尽量简化接口。
// 先不加，等有需求再说。

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,  // 保留测试用的greet命令
            get_totp,           // 主要功能：获取TOTP
            get_device_id,      // 获取设备ID
            get_connection_status, // 获取连接状态
            disconnect,         // 断开连接
            cleanup,            // 清理资源
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
