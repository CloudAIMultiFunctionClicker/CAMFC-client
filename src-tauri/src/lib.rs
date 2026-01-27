// 蓝牙模块导入
mod bluetooth;
mod cpen_device_manager;
// 下载模块导入
mod download;

// 使用新的Cpen设备管理器作为业务逻辑层
use cpen_device_manager::CpenDeviceManager;
use download::{DownloadTask, AuthInfo, get_app_data_dir};

// 导入同步原语
// 原来用tokio::sync::Mutex，继续用这个，适合异步环境
use tokio::sync::Mutex;
use std::sync::OnceLock;
use std::collections::HashMap;
use std::sync::Arc;

// 下载任务管理器
static DOWNLOAD_TASKS: OnceLock<Mutex<HashMap<String, Arc<download::DownloadTask>>>> = OnceLock::new();

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

/// 检查是否已建立稳定连接
/// 
/// 前端可以调用这个命令检查连接是否真的还活着。
/// 返回布尔值：true表示已建立稳定连接，false表示未连接或连接已断开。
/// 
/// 注意：这个方法会实际检查蓝牙物理连接状态，而不仅仅是内存中的记录。
#[tauri::command]
async fn is_connected() -> Result<bool, String> {
    println!("前端调用is_connected命令...");
    
    let mut manager = get_cpen_device_manager()?.lock().await;
    
    match manager.is_connected().await {
        Ok(connected) => {
            println!("连接状态检查结果: {}", if connected { "已连接" } else { "未连接" });
            Ok(connected)
        }
        Err(e) => {
            println!("检查连接状态失败: {}", e);
            // 检查失败时，保守返回false，表示连接不可用
            Err(format!("检查连接状态失败: {}", e))
        }
    }
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

// 下载相关命令

/// 下载文件
/// 
/// 前端调用这个命令下载文件到应用内目录
/// 会自动从蓝牙设备获取认证信息，支持分片下载和断点续传
/// 
/// 注意：file_id参数应该是完整的云盘路径，例如"ds/下载.png"而不是"下载.png"
/// 因为后端API需要完整的路径信息：http://localhost:8005/download/ds/下载.png
/// 
/// 这个版本支持真正的分片下载和断点续传
#[tauri::command]
async fn download_file(file_id: String) -> Result<String, String> {
    println!("前端调用download_file命令，文件路径: {}", file_id);
    
    // 先获取设备ID和TOTP
    let device_id = get_device_id().await.map_err(|e| format!("获取设备ID失败: {}", e))?;
    let totp = get_totp().await.map_err(|e| format!("获取TOTP失败: {}", e))?;
    
    // 创建认证信息
    let auth_info = AuthInfo {
        device_id,
        totp,
    };
    
    // 获取下载目录
    let download_dir = get_app_data_dir()
        .await
        .map_err(|e| format!("获取下载目录失败: {}", e))?;
    
    // 创建保存路径 - 从文件路径中提取文件名
    let timestamp = chrono::Utc::now().timestamp();
    let file_name = std::path::Path::new(&file_id)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&file_id);
    let save_path = download_dir.join(format!("{}_{}", file_name, timestamp));
    
    println!("创建下载任务: {} -> {:?}", file_id, save_path);
    
    // 创建下载任务
    let task = DownloadTask::new(file_id.clone(), save_path.clone(), auth_info)
        .await
        .map_err(|e| format!("创建下载任务失败: {}", e))?;
    
    // 将任务保存到全局管理器中
    let task_arc = Arc::new(task);
    
    // 初始化下载任务管理器
    let download_tasks = DOWNLOAD_TASKS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut tasks_map = download_tasks.lock().await;
    tasks_map.insert(file_id.clone(), task_arc.clone());
    
    println!("下载任务已添加到管理器，开始后台下载...");
    
    // 在后台异步执行下载，不阻塞前端响应
    let task_for_spawn = task_arc.clone();
    let file_id_for_spawn = file_id.clone();
    let save_path_for_spawn = save_path.clone();
    
    tokio::spawn(async move {
        println!("后台下载任务开始: {}", file_id_for_spawn);
        
        match task_for_spawn.start().await {
            Ok(_) => {
                println!("后台下载完成: {}，保存到: {:?}", file_id_for_spawn, save_path_for_spawn);
                
                // 下载完成后更新状态为完成
                // 状态已经在start()方法中更新了
            }
            Err(e) => {
                println!("后台下载失败: {}，错误: {}", file_id_for_spawn, e);
            }
        }
    });
    
    // 立即返回，不等待下载完成
    let result = format!("下载已开始，文件将保存到: {:?}，可使用get_download_progress查询进度", save_path);
    println!("{}", result);
    Ok(result)
}

/// 获取下载进度
/// 
/// 从下载任务管理器中获取真实的下载进度信息
/// 如果任务不存在，返回一个默认的进度信息
#[tauri::command]
async fn get_download_progress(file_id: String) -> Result<serde_json::Value, String> {
    println!("前端调用get_download_progress命令，文件ID: {}", file_id);
    
    // 尝试从下载任务管理器中获取任务
    let download_tasks = DOWNLOAD_TASKS.get_or_init(|| Mutex::new(HashMap::new()));
    let tasks_map = download_tasks.lock().await;
    
    if let Some(task) = tasks_map.get(&file_id) {
        // 获取真实的进度信息
        let progress = task.get_progress().await;
        
        // 将进度信息转换为JSON
        let status_str = match &progress.status {
            download::DownloadStatus::Pending => "Pending",
            download::DownloadStatus::Downloading => "Downloading",
            download::DownloadStatus::Paused => "Paused",
            download::DownloadStatus::Completed => "Completed",
            download::DownloadStatus::Error(err_msg) => {
                // 错误信息包含在状态字符串中
                return Ok(serde_json::json!({
                    "file_id": progress.file_id,
                    "file_name": progress.file_name,
                    "total_size": progress.total_size,
                    "downloaded": progress.downloaded,
                    "status": format!("Error: {}", err_msg),
                    "chunks_total": progress.chunks_total,
                    "chunks_completed": progress.chunks_completed,
                    "speed_kbps": progress.speed_kbps,
                    "progress_percentage": if progress.total_size > 0 {
                        (progress.downloaded as f64 / progress.total_size as f64 * 100.0).round() as u32
                    } else {
                        0
                    },
                }));
            }
        };
        
        println!("获取到真实下载进度: {} - {}%", file_id, 
            if progress.total_size > 0 {
                (progress.downloaded as f64 / progress.total_size as f64 * 100.0).round() as u32
            } else {
                0
            });
        
        return Ok(serde_json::json!({
            "file_id": progress.file_id,
            "file_name": progress.file_name,
            "total_size": progress.total_size,
            "downloaded": progress.downloaded,
            "status": status_str,
            "chunks_total": progress.chunks_total,
            "chunks_completed": progress.chunks_completed,
            "speed_kbps": progress.speed_kbps,
            "progress_percentage": if progress.total_size > 0 {
                (progress.downloaded as f64 / progress.total_size as f64 * 100.0).round() as u32
            } else {
                0
            },
        }));
    }
    
    // 如果任务不存在，返回一个默认的进度信息
    println!("下载任务 {} 不存在，返回默认进度信息", file_id);
    
    Ok(serde_json::json!({
        "file_id": file_id,
        "file_name": "未知文件",
        "total_size": 0,
        "downloaded": 0,
        "status": "Pending",
        "chunks_total": 0,
        "chunks_completed": 0,
        "speed_kbps": 0.0,
        "progress_percentage": 0,
    }))
}

/// 暂停下载
/// 
/// TODO: 需要下载任务管理器来实现真正的暂停功能
/// 先简单返回成功
#[tauri::command]
async fn pause_download(file_id: String) -> Result<(), String> {
    println!("前端调用pause_download命令，文件ID: {}", file_id);
    
    // 暂时简单实现
    println!("下载暂停功能待实现");
    Ok(())
}

/// 恢复下载
/// 
/// TODO: 需要下载任务管理器来实现真正的恢复功能
/// 先简单返回成功
#[tauri::command]
async fn resume_download(file_id: String) -> Result<(), String> {
    println!("前端调用resume_download命令，文件ID: {}", file_id);
    
    // 暂时简单实现
    println!("下载恢复功能待实现");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,  // 保留测试用的greet命令
            get_totp,           // 主要功能：获取TOTP
            get_device_id,      // 获取设备ID
            get_connection_status, // 获取连接状态
            is_connected,       // 检查是否已建立稳定连接
            disconnect,         // 断开连接
            cleanup,            // 清理资源
            // 下载相关命令
            download_file,
            get_download_progress,
            pause_download,
            resume_download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
