// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn scan_bluetooth_devices() -> Result<Vec<String>, String> {
    use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
    use btleplug::platform::Manager;
    use std::time::Duration;
    use tokio::time::sleep;
    
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
        .invoke_handler(tauri::generate_handler![greet, scan_bluetooth_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
