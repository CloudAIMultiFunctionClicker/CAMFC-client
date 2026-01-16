// 导入蓝牙管理器模块
mod bluetooth_manager;
use bluetooth_manager::{BluetoothManager, BluetoothOperations, BluetoothDevice};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}




/// 扫描蓝牙设备
/// 
/// 这个命令会扫描周围的蓝牙设备并返回列表
/// 用新的蓝牙管理器实现，更简洁
#[tauri::command]
async fn scan_bluetooth_devices() -> Result<Vec<String>, String> {
    use std::time::Duration;
    
    // 创建蓝牙管理器
    let mut manager = BluetoothManager::new();
    
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
/// 用新的蓝牙管理器实现
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
    
    // 创建蓝牙管理器
    let mut manager = BluetoothManager::new();
    
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, scan_bluetooth_devices, main_func, connect_to_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 主功能：扫描并自动连接Cpen设备
/// 
/// 这个函数会：
/// 1. 扫描蓝牙设备
/// 2. 查找Cpen设备
/// 3. 如果找到，自动连接
/// 4. 返回结果
/// 
/// 注意：即使没有找到Cpen设备，也会返回扫描结果
/// 这是用户要求的"扫描到Cpen也要输出"
#[tauri::command]
async fn main_func() -> Result<String, String> {
    use std::time::Duration;
    
    println!("开始执行主功能：扫描并连接Cpen设备");
    
    // 创建蓝牙管理器
    let mut manager = BluetoothManager::new();
    
    // 1. 扫描设备，持续3秒
    println!("正在扫描蓝牙设备...");
    let devices = match manager.scan_devices(Duration::from_secs(3)).await {
        Ok(devices) => devices,
        Err(e) => {
            let error_msg = format!("扫描设备失败: {}", e);
            println!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    // 2. 输出扫描结果
    println!("扫描完成，共发现 {} 个设备:", devices.len());
    for (i, device) in devices.iter().enumerate() {
        println!("设备 {}: {}", i + 1, device.display_info());
    }
    
    // 3. 查找Cpen设备
    let cpen_devices: Vec<BluetoothDevice> = manager.find_cpen_devices(&devices);
    
    // 4. 处理结果
    if cpen_devices.is_empty() {
        // 没找到Cpen设备
        println!("未发现Cpen开头的设备，但有其他设备");
        
        let result = format!("扫描完成，未发现Cpen设备。共扫描到 {} 个其他设备。", devices.len());
        Ok(result)
    } else {
        // 找到Cpen设备
        println!("发现 {} 个Cpen设备", cpen_devices.len());
        
        // 尝试连接第一个Cpen设备
        // 思考：如果有多个Cpen设备，应该连哪个？
        // 先连第一个吧，以后可以改进
        let target_device = &cpen_devices[0];
        println!("正在尝试连接Cpen设备: {}", target_device.display_info());
        
        match manager.connect_device(target_device).await {
            Ok(_) => {
                let result = format!("扫描完成，发现并成功连接Cpen设备: {}。共扫描到 {} 个设备。", 
                    target_device.display_info(), devices.len());
                println!("{}", result);
                Ok(result)
            }
            Err(e) => {
                // 连接失败，但仍然返回扫描结果
                let result = format!("扫描完成，发现Cpen设备但连接失败: {}。错误: {}。共扫描到 {} 个设备。", 
                    target_device.display_info(), e, devices.len());
                println!("{}", result);
                Ok(result) // 注意：这里返回Ok，不是Err，因为扫描本身成功了
            }
        }
    }
}
