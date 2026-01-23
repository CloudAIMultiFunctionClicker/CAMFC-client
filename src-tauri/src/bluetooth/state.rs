use std::time::{Instant, Duration};
use uuid::Uuid;

// 从manager导入DeviceInfo，确保类型一致
use super::manager::DeviceInfo;

/// 连接状态
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// 蓝牙状态管理
pub struct BluetoothState {
    /// 当前连接的设备（如果有的话）
    pub connected_device: Option<DeviceInfo>,
    
    /// 设备ID（UUID），连接后从设备获取
    pub device_id: Option<String>,
    
    /// TOTP缓存：值 + 缓存时间
    totp_cache: Option<(String, Instant)>,
    
    /// 当前连接状态
    pub connection_status: ConnectionStatus,
    
    /// CPen设备的UUID（固定值）
    pub cpen_service_uuid: String,
    pub cpen_char_uuid: String,
}

impl BluetoothState {
    /// 创建新的状态实例
    pub fn new() -> Self {
        Self {
            connected_device: None,
            device_id: None,
            totp_cache: None,
            connection_status: ConnectionStatus::Disconnected,
            
            // CPen设备的UUID，从原代码复制过来的
            cpen_service_uuid: "d816e4c6-1b99-4da7-bcd5-7c37cc2642c4".to_string(),
            cpen_char_uuid: "d816e4c7-1b99-4da7-bcd5-7c37cc2642c4".to_string(),
        }
    }
    
    /// 获取缓存的TOTP（如果还在30秒有效期内）
    pub fn get_cached_totp(&self) -> Option<String> {
        match &self.totp_cache {
            Some((totp, timestamp)) => {
                let elapsed = timestamp.elapsed();
                if elapsed < Duration::from_secs(30) {
                    // 还在缓存期内
                    Some(totp.clone())
                } else {
                    // 超过30秒，缓存过期
                    None
                }
            }
            None => None
        }
    }
    
    /// 更新TOTP缓存
    pub fn update_totp_cache(&mut self, totp: String) {
        self.totp_cache = Some((totp, Instant::now()));
    }
    
    /// 清空TOTP缓存
    pub fn clear_totp_cache(&mut self) {
        self.totp_cache = None;
    }
    
    /// 更新连接状态
    pub fn update_connection_status(&mut self, status: ConnectionStatus) {
        self.connection_status = status.clone(); // 使用clone而不是move
        
        // 如果断开连接，清空相关状态
        if status == ConnectionStatus::Disconnected {
            self.connected_device = None;
            self.device_id = None;
            self.clear_totp_cache();
        }
    }
    
    /// 设置设备ID
    pub fn set_device_id(&mut self, device_id: String) {
        self.device_id = Some(device_id);
    }
    
    /// 获取设备ID
    pub fn get_device_id(&self) -> Option<String> {
        self.device_id.clone()
    }
    
    /// 是否已连接
    pub fn is_connected(&self) -> bool {
        self.connection_status == ConnectionStatus::Connected
    }
    
    /// 获取CPen服务UUID
    pub fn get_service_uuid(&self) -> &str {
        &self.cpen_service_uuid
    }
    
    /// 获取CPen特性UUID
    pub fn get_char_uuid(&self) -> &str {
        &self.cpen_char_uuid
    }
    
    /// 设置连接的设备信息
    pub fn set_connected_device(&mut self, device: DeviceInfo) {
        self.connected_device = Some(device);
    }
}
