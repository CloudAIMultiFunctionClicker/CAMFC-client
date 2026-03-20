use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;

use crate::core::{DeviceManager, BluetoothAdapter, AppResult, AppError, DeviceInfo, ConnectionState, CacheEntry};

/// 设备命令常量
const CMD_SET_TIME: &[u8] = b"setTime";
const CMD_GET_TOTP: &[u8] = b"getTotp";
const CMD_GET_ID: &[u8] = b"getId";

/// TOTP缓存时间（30秒）
const TOTP_CACHE_TTL_SECS: u64 = 30;

/// 设备管理器实现
/// 
/// 原来那个CpenDeviceManager太复杂，拆分一下职责
pub struct DeviceManagerImpl {
    bluetooth_adapter: Arc<dyn BluetoothAdapter>,
    current_device: RwLock<Option<DeviceInfo>>,
    connection_state: RwLock<ConnectionState>,
    totp_cache: RwLock<Option<CacheEntry<String>>>,
}

impl DeviceManagerImpl {
    /// 创建设备管理器
    pub fn new(bluetooth_adapter: Arc<dyn BluetoothAdapter>) -> Self {
        Self {
            bluetooth_adapter,
            current_device: RwLock::new(None),
            connection_state: RwLock::new(ConnectionState::Disconnected),
            totp_cache: RwLock::new(None),
        }
    }
    
    /// 更新连接状态
    async fn update_connection_state(&self, state: ConnectionState) {
        *self.connection_state.write().await = state;
    }
    
    /// 检查设备连接
    async fn check_device_connection(&self) -> AppResult<()> {
        let is_connected = self.bluetooth_adapter.is_connected().await?;
        
        if !is_connected {
            self.update_connection_state(ConnectionState::Disconnected).await;
            self.current_device.write().await.take();
            return Err(AppError::DeviceConnection("设备已断开连接".to_string()));
        }
        
        Ok(())
    }
    
    /// 发送命令并等待响应
    async fn send_command_with_response(&self, command: &[u8]) -> AppResult<Vec<u8>> {
        self.check_device_connection().await?;
        
        // 发送命令
        self.bluetooth_adapter.send_data(command).await?;
        
        // 等待响应（3秒超时）
        let response = self.bluetooth_adapter.receive_data(3000).await?;
        
        if response.is_empty() {
            return Err(AppError::DeviceConnection("设备无响应".to_string()));
        }
        
        Ok(response)
    }
    
    /// 获取TOTP（带缓存）
    async fn get_totp_internal(&self) -> AppResult<String> {
        // 检查缓存
        {
            let cache = self.totp_cache.read().await;
            if let Some(cache_entry) = cache.as_ref() {
                if let Some(totp) = cache_entry.get() {
                    return Ok(totp.clone());
                }
            }
        }
        
        // 从设备获取
        let response = self.send_command_with_response(CMD_GET_TOTP).await?;
        let totp = String::from_utf8_lossy(&response).trim().to_string();
        
        // 验证TOTP格式（应该是6位数字）
        if totp.len() != 6 || !totp.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::DeviceConnection(format!("无效的TOTP格式: {}", totp)));
        }
        
        // 缓存结果
        let cache_entry = CacheEntry::new(
            totp.clone(),
            Duration::from_secs(TOTP_CACHE_TTL_SECS)
        );
        
        *self.totp_cache.write().await = Some(cache_entry);
        
        Ok(totp)
    }
    
    /// 获取设备ID
    async fn get_device_id_internal(&self) -> AppResult<String> {
        let response = self.send_command_with_response(CMD_GET_ID).await?;
        let device_id = String::from_utf8_lossy(&response).trim().to_string();
        
        // 验证设备ID格式（应该是UUID）
        if device_id.len() < 10 {
            return Err(AppError::DeviceConnection(format!("无效的设备ID格式: {}", device_id)));
        }
        
        Ok(device_id)
    }
    
    /// 同步时间
    async fn sync_time(&self) -> AppResult<()> {
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let command = [CMD_SET_TIME, timestamp.as_bytes()].concat();
        
        let response = self.send_command_with_response(&command).await?;
        let result = String::from_utf8_lossy(&response);
        
        if result.trim() != "OK" {
            return Err(AppError::DeviceConnection(format!("时间同步失败: {}", result)));
        }
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl DeviceManager for DeviceManagerImpl {
    async fn scan_devices(&self, timeout_ms: u64) -> AppResult<Vec<DeviceInfo>> {
        self.update_connection_state(ConnectionState::Scanning).await;
        
        let result = self.bluetooth_adapter.scan_devices(timeout_ms).await;
        
        // 恢复状态
        let current_state = if self.bluetooth_adapter.is_connected().await? {
            ConnectionState::Connected
        } else {
            ConnectionState::Disconnected
        };
        
        self.update_connection_state(current_state).await;
        
        result
    }
    
    async fn connect_device(&self, address: &str) -> AppResult<DeviceInfo> {
        // 如果已连接其他设备，先断开
        if self.bluetooth_adapter.is_connected().await? {
            self.disconnect_device().await?;
        }
        
        self.update_connection_state(ConnectionState::Connecting).await;
        
        // 连接设备
        let device_info = self.bluetooth_adapter.connect(address).await?;
        
        // 更新当前设备
        *self.current_device.write().await = Some(device_info.clone());
        self.update_connection_state(ConnectionState::Connected).await;
        
        // 同步时间
        if let Err(e) = self.sync_time().await {
            // 时间同步失败不中断连接，但记录错误
            eprintln!("时间同步失败: {}", e);
        }
        
        Ok(device_info)
    }
    
    async fn disconnect_device(&self) -> AppResult<()> {
        self.update_connection_state(ConnectionState::Disconnected).await;
        
        // 清除缓存
        *self.totp_cache.write().await = None;
        *self.current_device.write().await = None;
        
        // 断开蓝牙连接
        self.bluetooth_adapter.disconnect().await?;
        
        Ok(())
    }
    
    fn get_current_device(&self) -> Option<DeviceInfo> {
        // 这里用blocking_lock，因为只是读取
        self.current_device.blocking_read().clone()
    }
    
    fn get_connection_state(&self) -> ConnectionState {
        *self.connection_state.blocking_read()
    }
    
    async fn send_command(&self, command: &[u8]) -> AppResult<Vec<u8>> {
        self.send_command_with_response(command).await
    }
    
    async fn get_totp(&self) -> AppResult<String> {
        self.get_totp_internal().await
    }
    
    async fn get_device_id(&self) -> AppResult<String> {
        self.get_device_id_internal().await
    }
}