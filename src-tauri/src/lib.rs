use windows::Devices::Radios::{Radio, RadioAccessStatus, RadioKind};


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]

/// 查找并启用蓝牙设备
/// 
/// 返回值: 
/// - Ok(true): 蓝牙已成功启用或已经是开启状态
/// - Ok(false): 未找到蓝牙设备
/// - Err(...): 过程中发生错误
fn enable_bluetooth() -> Result<bool, Box<dyn std::error::Error>> {
    println!("正在查找蓝牙设备...");

    // 获取所有无线电设备
    let async_op = Radio::GetRadiosAsync()?;
    let radios = async_op.get()?;
    
    // 查找蓝牙设备
    let bluetooth_radio = find_bluetooth_radio(&radios);
    
    match bluetooth_radio {
        Some(radio) => {
            // 检查当前状态
            let current_state = radio.State()?;
            
            if current_state == windows::Devices::Radios::RadioState::On {
                println!("蓝牙已经是开启状态");
                Ok(true)
            } else {
                println!("正在启用蓝牙...");
                
                // 尝试启用蓝牙
                let result = radio.SetStateAsync(windows::Devices::Radios::RadioState::On)?.get()?;
                
                match result {
                    RadioAccessStatus::Allowed => {
                        println!("蓝牙启用成功！");
                        println!("新状态: {:?}", radio.State()?);
                        Ok(true)
                    }
                    RadioAccessStatus::DeniedBySystem => {
                        let err_msg = "系统拒绝访问蓝牙设备，可能的原因：管理员权限不足或系统策略限制";
                        eprintln!("错误：{}", err_msg);
                        Err(err_msg.into())
                    }
                    RadioAccessStatus::DeniedByUser => {
                        let err_msg = "用户拒绝访问蓝牙设备";
                        eprintln!("错误：{}", err_msg);
                        Err(err_msg.into())
                    }
                    RadioAccessStatus::Unspecified => {
                        let err_msg = "未知错误，无法启用蓝牙";
                        eprintln!("错误：{}", err_msg);
                        Err(err_msg.into())
                    }
                    _ => {
                        let err_msg = format!("未知的访问状态: {:?}", result);
                        eprintln!("错误：{}", err_msg);
                        Err(err_msg.into())
                    }
                }
            }
        }
        None => {
            eprintln!("未找到蓝牙设备");
            eprintln!("请确保：");
            eprintln!("1. 计算机支持蓝牙功能");
            eprintln!("2. 蓝牙硬件已正确安装");
            eprintln!("3. 蓝牙驱动程序已更新");
            Ok(false)
        }
    }
}


/// 在无线电设备列表中查找蓝牙设备
fn find_bluetooth_radio(radios: &windows::Foundation::Collections::IVectorView<Radio>) -> Option<Radio> {
    for radio in radios {
        // 直接使用radio，不需要?操作符
        if let Ok(kind) = radio.Kind() {
            if kind == RadioKind::Bluetooth {
                if let Ok(name) = radio.Name() {
                    println!("找到蓝牙设备: {}", name.to_string_lossy());
                }
                if let Ok(state) = radio.State() {
                    println!("当前状态: {:?}", state);
                }
                return Some(radio);
            }
        }
    }
    None
}



#[tauri::command]
async fn scan_bluetooth_devices() -> Result<Vec<String>, String> {
    use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
    use btleplug::platform::Manager;
    use std::time::Duration;
    use tokio::time::sleep;
    enable_bluetooth();
    
    // 获取蓝牙管理器
    let manager = Manager::new().await.map_err(|e| e.to_string())?;
    
    // 获取适配器列表
    let adapters = manager.adapters().await.map_err(|e| e.to_string())?;
    if adapters.is_empty() {
        return Ok(vec!["没有找到蓝牙适配器".to_string()]);
    }
    
    // 使用第一个适配器
    let central = adapters.into_iter().next().unwrap();
    
    // 开始扫描
    central.start_scan(ScanFilter::default()).await.map_err(|e| e.to_string())?;
    
    // 等待几秒让设备被发现
    sleep(Duration::from_secs(3)).await;
    
    // 获取发现的设备
    let peripherals = central.peripherals().await.map_err(|e| e.to_string())?;
    
    // 停止扫描
    central.stop_scan().await.map_err(|e| e.to_string())?;
    
    let mut devices = Vec::new();
    for peripheral in peripherals {
        let properties = peripheral.properties().await.map_err(|e| e.to_string())?;
        if let Some(props) = properties {
            let name = props.local_name.unwrap_or_else(|| "未知设备".to_string());
            let address = props.address.to_string();
            devices.push(format!("{} - {}", name, address));
        }
    }
    
    if devices.is_empty() {
        devices.push("没有发现蓝牙设备".to_string());
    }
    
    Ok(devices)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, scan_bluetooth_devices,main_func])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn main_func() -> Result<String, String> {
    // 1. 启用蓝牙
    enable_bluetooth()
        .map_err(|e| format!("启用蓝牙失败: {}", e))?;
    
    // 2. 扫描设备
    let devices = scan_bluetooth_devices().await
        .map_err(|e| format!("扫描设备失败: {}", e))?;
    
    // 3. 查找Cpen开头的设备
    let target_device = devices.iter()
        .find(|d| d.starts_with("Cpen"))
        .cloned()
        .ok_or("未找到Cpen开头的设备".to_string())?;
    
    // 4. 连接设备（需要实现连接逻辑）
    // ... 连接代码 ...
    
    Ok(format!("成功连接设备: {}", target_device))
}