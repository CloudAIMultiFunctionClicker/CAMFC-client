// 使用模块化的蓝牙模块
mod bluetooth;
use bluetooth::CpenService;

// 导入同步原语
use tokio::sync::Mutex;
use std::sync::OnceLock;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 全局CPen服务实例
// 用OnceLock确保只初始化一次，Mutex保证线程安全
static CPEN_SERVICE: OnceLock<Mutex<CpenService>> = OnceLock::new();

/// 初始化CPen服务
fn init_cpen_service() -> Result<(), String> {
    println!("初始化CPen服务...");
    
    if CPEN_SERVICE.get().is_some() {
        println!("CPen服务已经初始化过了");
        return Ok(());
    }
    
    let service = CpenService::new();
    
    CPEN_SERVICE.set(Mutex::new(service))
        .map_err(|_| "CPen服务设置失败".to_string())?;
    
    println!("CPen服务初始化成功");
    Ok(())
}

/// 获取全局CPen服务的引用
fn get_cpen_service() -> Result<&'static Mutex<CpenService>, String> {
    // 如果服务未初始化，先初始化
    if CPEN_SERVICE.get().is_none() {
        init_cpen_service()?;
    }
    
    CPEN_SERVICE.get().ok_or("CPen服务初始化失败".to_string())
}

/// 获取TOTP - 核心API #1
/// 前端只需要调用这个函数，所有连接、缓存逻辑都在Rust端处理
#[tauri::command]
async fn get_totp() -> Result<String, String> {
    println!("前端调用get_totp()...");
    
    let service_mutex = get_cpen_service()?;
    let service = service_mutex.lock().await;
    
    // 调用CpenService的get_totp方法
    // 这个方法会：1.检查缓存 2.自动连接设备 3.获取TOTP 4.返回结果
    let totp = service.get_totp().await?;
    
    println!("get_totp()返回: {}", totp);
    Ok(totp)
}

/// 获取设备ID - 核心API #2  
/// 前端只需要调用这个函数，所有连接、缓存逻辑都在Rust端处理
#[tauri::command]
async fn get_device_id() -> Result<String, String> {
    println!("前端调用get_device_id()...");
    
    let service_mutex = get_cpen_service()?;
    let service = service_mutex.lock().await;
    
    // 调用CpenService的get_device_id方法
    // 这个方法会：1.检查缓存 2.自动连接设备 3.获取设备ID 4.返回结果
    let device_id = service.get_device_id().await?;
    
    println!("get_device_id()返回: {}", device_id);
    Ok(device_id)
}

/// 断开连接（可选，保持兼容性）
#[tauri::command]
async fn disconnect_current_device() -> Result<String, String> {
    println!("前端调用断开连接...");
    
    let service_mutex = get_cpen_service()?;
    let service = service_mutex.lock().await;
    
    match service.disconnect().await {
        Ok(_) => Ok("成功断开设备连接".to_string()),
        Err(e) => Err(format!("断开连接失败: {}", e)),
    }
}

/// 启动后台服务（可选，内部使用）
/// 应用启动时自动调用，不需要前端关心
#[tauri::command]
async fn start_background_service() -> Result<(), String> {
    println!("启动后台服务...");
    
    let service_mutex = get_cpen_service()?;
    let mut service = service_mutex.lock().await;
    
    service.start_background_service();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_totp,           // 核心API #1
            get_device_id,      // 核心API #2
            disconnect_current_device, // 可选API
            start_background_service, // 内部API
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
