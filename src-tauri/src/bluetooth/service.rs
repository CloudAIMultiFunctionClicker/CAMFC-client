use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use chrono;

use super::manager::{BluetoothManager, DeviceInfo};
use super::state::{BluetoothState, ConnectionStatus};

/// CPen设备服务层
/// 在基础的BluetoothManager之上封装CPen特定的业务逻辑
pub struct CpenService {
    /// 基础的蓝牙管理器
    manager: Arc<Mutex<BluetoothManager>>,
    
    /// 状态管理
    state: Arc<Mutex<BluetoothState>>,
    
    /// 后台任务句柄（定时刷新TOTP等）
    background_handle: Option<tokio::task::JoinHandle<()>>,
}

impl CpenService {
    /// 创建新的CPen服务
    pub fn new() -> Self {
        Self {
            manager: Arc::new(Mutex::new(BluetoothManager::new())),
            state: Arc::new(Mutex::new(BluetoothState::new())),
            background_handle: None,
        }
    }
    
    /// 自动扫描并连接CPen设备
    /// 这是核心连接逻辑，会在后台自动重试
    pub async fn auto_connect(&self) -> Result<(), String> {
        println!("开始自动连接CPen设备...");
        
        let mut state = self.state.lock().await;
        state.update_connection_status(ConnectionStatus::Connecting);
        drop(state); // 释放锁，避免死锁
        
        let mut manager = self.manager.lock().await;
        
        // 1. 扫描设备
        println!("扫描蓝牙设备...");
        let devices = manager.scan_devices(5000).await
            .map_err(|e| format!("扫描失败: {}", e))?;
        
        if devices.is_empty() {
            let mut state = self.state.lock().await;
            state.update_connection_status(ConnectionStatus::Disconnected);
            return Err("没有发现蓝牙设备".to_string());
        }
        
        // 2. 查找CPen设备
        let cpen_device = devices.iter().find(|d| {
            // 简单判断：设备名以"Cpen"开头（不区分大小写）
            // 修复：使用字符级别操作，避免UTF-8字节切片错误
            if d.name.chars().count() >= 4 {
                let prefix: String = d.name.chars().take(4).collect();
                prefix.to_lowercase() == "cpen"
            } else {
                false
            }
        });
        
        match cpen_device {
            Some(device) => {
                println!("找到CPen设备: {} ({})", device.name, device.address);
                
                // 3. 连接设备
                println!("正在连接...");
                manager.connect(&device.address).await
                    .map_err(|e| format!("连接失败: {}", e))?;
                
                println!("连接成功！");
                
                // 4. 更新状态
                let mut state = self.state.lock().await;
                state.set_connected_device(device.clone());
                state.update_connection_status(ConnectionStatus::Connected);
                
                // 5. 连接成功后获取设备ID
                // 先等一小会儿让设备稳定
                sleep(Duration::from_millis(500)).await;
                
                // 异步获取设备ID，不阻塞主流程
                let manager_clone = self.manager.clone();
                let state_clone = self.state.clone();
                let service_uuid = state.cpen_service_uuid.clone();
                let char_uuid = state.cpen_char_uuid.clone();
                
                tokio::spawn(async move {
                    println!("尝试获取设备ID...");
                    let mut mgr = manager_clone.lock().await;
                    
                    // 发送getId命令
                    match mgr.send(&service_uuid, &char_uuid, b"getId").await {
                        Ok(_) => {
                            // 接收响应
                            match mgr.recv(&service_uuid, &char_uuid).await {
                                Ok(response) => {
                                    let device_id = String::from_utf8_lossy(&response).to_string();
                                    println!("获取到设备ID: {}", device_id);
                                    
                                    let mut st = state_clone.lock().await;
                                    st.set_device_id(device_id);
                                }
                                Err(e) => {
                                    println!("获取设备ID响应失败: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("发送getId命令失败: {}", e);
                        }
                    }
                });
                
                Ok(())
            }
            None => {
                let mut state = self.state.lock().await;
                state.update_connection_status(ConnectionStatus::Disconnected);
                Err("未找到CPen设备".to_string())
            }
        }
    }
    
    /// 获取TOTP（主要功能）
    /// 优先使用缓存，如果缓存过期则从设备获取
    pub async fn get_totp(&self) -> Result<String, String> {
        println!("获取TOTP...");
        
        // 1. 检查缓存
        let cached_totp = {
            let state = self.state.lock().await;
            state.get_cached_totp()
        };
        
        if let Some(totp) = cached_totp {
            println!("使用缓存的TOTP");
            return Ok(totp);
        }
        
        // 2. 检查是否已连接
        let is_connected = {
            let state = self.state.lock().await;
            state.is_connected()
        };
        
        if !is_connected {
            // 尝试自动连接
            println!("未连接，尝试自动连接...");
            self.auto_connect().await?;
        }
        
        // 3. 从设备获取TOTP
        let mut manager = self.manager.lock().await;
        let state = self.state.lock().await;
        
        let service_uuid = state.get_service_uuid();
        let char_uuid = state.get_char_uuid();
        
        // 先发送setTime时间戳
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let set_time_command = format!("setTime:{}", timestamp);
        println!("发送setTime命令: {}", set_time_command);
        
        manager.send(service_uuid, char_uuid, set_time_command.as_bytes()).await
            .map_err(|e| format!("发送setTime失败: {}", e))?;
        
        // 等待设备处理
        sleep(Duration::from_millis(100)).await;
        
        // 然后发送getTotp命令
        manager.send(service_uuid, char_uuid, b"getTotp").await
            .map_err(|e| format!("发送getTotp失败: {}", e))?;
        
        // 接收响应
        let response = manager.recv(service_uuid, char_uuid).await
            .map_err(|e| format!("接收TOTP失败: {}", e))?;
        
        let totp = String::from_utf8(response)
            .map_err(|e| format!("TOTP响应不是有效UTF-8: {}", e))?;
        
        println!("获取到TOTP: {}", totp);
        
        // 4. 更新缓存（注意：需要获取state的可变引用）
        drop(manager); // 先释放manager锁
        let mut state_mut = self.state.lock().await;
        state_mut.update_totp_cache(totp.clone());
        
        Ok(totp)
    }
    
    /// 获取设备ID
    /// 如果已经缓存则直接返回，否则尝试从设备获取
    pub async fn get_device_id(&self) -> Result<String, String> {
        println!("获取设备ID...");
        
        // 1. 检查是否有缓存的设备ID
        let cached_id = {
            let state = self.state.lock().await;
            state.get_device_id()
        };
        
        if let Some(id) = cached_id {
            println!("使用缓存的设备ID: {}", id);
            return Ok(id);
        }
        
        // 2. 检查是否已连接
        let is_connected = {
            let state = self.state.lock().await;
            state.is_connected()
        };
        
        if !is_connected {
            // 尝试自动连接
            println!("未连接，尝试自动连接...");
            self.auto_connect().await?;
        }
        
        // 3. 从设备获取ID
        let mut manager = self.manager.lock().await;
        let state = self.state.lock().await;
        
        let service_uuid = state.get_service_uuid();
        let char_uuid = state.get_char_uuid();
        
        // 发送getId命令
        manager.send(service_uuid, char_uuid, b"getId").await
            .map_err(|e| format!("发送getId失败: {}", e))?;
        
        // 接收响应
        let response = manager.recv(service_uuid, char_uuid).await
            .map_err(|e| format!("接收设备ID失败: {}", e))?;
        
        let device_id = String::from_utf8(response)
            .map_err(|e| format!("设备ID响应不是有效UTF-8: {}", e))?;
        
        println!("获取到设备ID: {}", device_id);
        
        // 4. 更新缓存（需要state的可变引用）
        drop(manager); // 先释放manager锁
        let mut state_mut = self.state.lock().await;
        state_mut.set_device_id(device_id.clone());
        
        Ok(device_id)
    }
    
    /// 启动后台服务
    /// 包括定时刷新TOTP、自动重连等
    pub fn start_background_service(&mut self) {
        println!("启动CPen后台服务...");
        
        // 如果已经有后台任务，先停止
        self.stop_background_service();
        
        let manager = self.manager.clone();
        let state = self.state.clone();
        
        // 启动后台任务
        let handle = tokio::spawn(async move {
            println!("后台服务已启动");
            
            // 主循环
            loop {
                // 检查连接状态，如果需要则尝试重新连接
                {
                    let st = state.lock().await;
                    if !st.is_connected() {
                        // 这里可以添加自动重连逻辑
                        // TODO: 实现智能重连策略
                    }
                }
                
                // 每30秒刷新一次TOTP（如果已连接）
                {
                    let st = state.lock().await;
                    if st.is_connected() {
                        let mgr = manager.lock().await;
                        let service_uuid = st.get_service_uuid();
                        let char_uuid = st.get_char_uuid();
                        
                        // 异步刷新TOTP，不阻塞主循环
                        let mgr_clone = manager.clone();
                        let state_clone = state.clone();
                        let su = service_uuid.to_string();
                        let cu = char_uuid.to_string();
                        
                        tokio::spawn(async move {
                            let mut mgr2 = mgr_clone.lock().await;
                            
                            // 发送getTotp命令
                            if let Err(e) = mgr2.send(&su, &cu, b"getTotp").await {
                                println!("后台刷新TOTP失败: {}", e);
                                return;
                            }
                            
                            // 接收响应
                            match mgr2.recv(&su, &cu).await {
                                Ok(response) => {
                                    let totp = String::from_utf8_lossy(&response);
                                    println!("后台刷新TOTP成功: {}", totp);
                                    
                                    let mut st2 = state_clone.lock().await;
                                    st2.update_totp_cache(totp.to_string());
                                }
                                Err(e) => {
                                    println!("后台接收TOTP失败: {}", e);
                                }
                            }
                        });
                    }
                }
                
                // 等待30秒
                sleep(Duration::from_secs(30)).await;
            }
        });
        
        self.background_handle = Some(handle);
    }
    
    /// 停止后台服务
    pub fn stop_background_service(&mut self) {
        if let Some(handle) = self.background_handle.take() {
            handle.abort();
            println!("后台服务已停止");
        }
    }
    
    /// 断开连接
    pub async fn disconnect(&self) -> Result<(), String> {
        println!("断开CPen设备连接...");
        
        let mut state = self.state.lock().await;
        state.update_connection_status(ConnectionStatus::Disconnected);
        
        let mut manager = self.manager.lock().await;
        manager.disconnect().await
            .map_err(|e| format!("断开连接失败: {}", e))?;
        
        println!("断开连接成功");
        Ok(())
    }
}

impl Drop for CpenService {
    fn drop(&mut self) {
        // 清理时停止后台服务
        self.stop_background_service();
    }
}
