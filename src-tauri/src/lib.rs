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

/// 连接蓝牙设备
/// 
/// 这个函数会尝试连接指定的设备
/// 参数是设备名称（从扫描结果里拿到的完整字符串）
/// 
/// 注意：btleplug的连接API有点复杂，这里先实现一个基础版本
/// 实际使用中可能需要处理更多错误情况
#[tauri::command]
async fn connect_to_device(device_info: String) -> Result<String, String> {
    use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
    use btleplug::platform::Manager;
    use std::time::Duration;
    use tokio::time::sleep;
    
    println!("开始连接设备: {}", device_info);
    
    // 解析设备信息，格式是 "设备名 - 地址"
    // 不过我们主要用地址来连接，名字只是给人看的
    let parts: Vec<&str> = device_info.split(" - ").collect();
    let target_address = if parts.len() >= 2 {
        parts[1].to_string()
    } else {
        // 如果格式不对，就用整个字符串当地址
        device_info.clone()
    };
    
    // 先启用蓝牙（虽然main_func里已经启用了，但这里再确保一下）
    enable_bluetooth().map_err(|e| format!("启用蓝牙失败: {}", e))?;
    
    // 获取蓝牙管理器
    let manager = Manager::new().await.map_err(|e| e.to_string())?;
    let adapters = manager.adapters().await.map_err(|e| e.to_string())?;
    
    if adapters.is_empty() {
        return Err("没有可用的蓝牙适配器".to_string());
    }
    
    let central = adapters.into_iter().next().unwrap();
    
    // 开始扫描来找到目标设备
    println!("正在扫描以找到设备 {}...", target_address);
    central.start_scan(ScanFilter::default()).await.map_err(|e| e.to_string())?;
    
    // 等待一会儿让设备被发现
    sleep(Duration::from_secs(2)).await;
    
    // 获取所有发现的设备
    let peripherals = central.peripherals().await.map_err(|e| e.to_string())?;
    
    // 停止扫描
    central.stop_scan().await.map_err(|e| e.to_string())?;
    
    // 查找目标设备
    let mut target_peripheral = None;
    
    for peripheral in &peripherals {
        if let Ok(properties) = peripheral.properties().await {
            if let Some(props) = properties {
                let address = props.address.to_string();
                
                // 检查地址是否匹配
                if address == target_address {
                    println!("找到目标设备，地址: {}", address);
                    target_peripheral = Some(peripheral.clone());
                    break;
                }
            }
        }
    }
    
    match target_peripheral {
        Some(peripheral) => {
            println!("正在连接设备...");
            
            // 尝试连接
            // 这里有个问题：btleplug的connect方法可能需要适配器级别的权限
            // 先试试看，不行再想其他办法
            match peripheral.connect().await {
                Ok(_) => {
                    println!("连接成功！");
                    
                    // 检查连接状态
                    if peripheral.is_connected().await.map_err(|e| e.to_string())? {
                        // 可以在这里添加更多操作，比如发现服务、读取特征等
                        // 不过对于Cpen设备，可能需要特定的服务UUID
                        println!("设备已连接，可以开始通信");
                        
                        // TODO: 这里应该发现设备服务，但不知道Cpen的具体服务UUID
                        // 暂时先返回成功
                        return Ok(format!("成功连接到设备: {}", device_info));
                    } else {
                        println!("连接状态检查失败，可能实际没连上");
                        return Err("连接状态异常".to_string());
                    }
                }
                Err(e) => {
                    println!("连接失败: {}", e);
                    return Err(format!("连接失败: {}", e));
                }
            }
        }
        None => {
            println!("未找到设备: {}", target_address);
            Err(format!("未找到设备: {}", target_address))
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

#[tauri::command]
async fn main_func() -> Result<String, String> {
    // 1. 启用蓝牙
    enable_bluetooth()
        .map_err(|e| format!("启用蓝牙失败: {}", e))?;
    
    // 2. 扫描设备
    let devices = scan_bluetooth_devices().await
        .map_err(|e| format!("扫描设备失败: {}", e))?;
    
    // 3. 查找Cpen开头的设备
    // 这里先检查有没有Cpen设备，不管有没有找到都要输出结果
    // 用户要求：扫描到Cpen也要输出，所以即使没找到也不能直接报错
    let target_device = devices.iter()
        .find(|d| d.starts_with("Cpen"));
    
    // 4. 输出扫描结果
    println!("扫描完成，共发现 {} 个设备:", devices.len());
    for (i, device) in devices.iter().enumerate() {
        println!("设备 {}: {}", i + 1, device);
    }
    
    // 5. 如果有Cpen设备，尝试连接
    match target_device {
        Some(device_name) => {
            println!("发现Cpen设备: {}", device_name);
            
            // 尝试连接设备 - 现在有了实际的连接函数
            println!("正在尝试连接Cpen设备...");
            
            // 调用连接函数
            match connect_to_device(device_name.clone()).await {
                Ok(connection_result) => {
                    println!("连接成功: {}", connection_result);
                    Ok(format!("扫描完成，发现并成功连接Cpen设备: {}。共扫描到 {} 个设备。连接结果: {}", 
                        device_name, devices.len(), connection_result))
                }
                Err(e) => {
                    println!("连接失败: {}", e);
                    // 思考：连接失败要不要算作整个main_func失败？
                    // 我觉得不应该，因为扫描本身成功了，只是连接失败
                    // 但用户可能更关心连接结果，所以返回错误信息但格式是成功的？
                    // 先返回包含错误信息的成功结果吧，这样前端能看到发生了什么
                    Ok(format!("扫描完成，发现Cpen设备但连接失败: {}。错误: {}。共扫描到 {} 个设备。", 
                        device_name, e, devices.len()))
                }
            }
        }
        None => {
            // 没找到Cpen设备，但还是要输出扫描结果
            println!("未发现Cpen开头的设备，但有其他设备");
            
            // 思考：这里要不要返回错误？用户说"扫描到Cpen也要输出"
            // 我的理解是：即使没找到Cpen，也要输出扫描结果，所以返回成功但提示没找到
            Ok(format!("扫描完成，未发现Cpen设备。共扫描到 {} 个其他设备。", devices.len()))
        }
    }
}
