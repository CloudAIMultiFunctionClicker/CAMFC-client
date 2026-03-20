use std::sync::Arc;

use crate::core::{AppResult, DeviceInfo, ConnectionState, DeviceManager, AuthService, FileService};
use crate::domain::device::DeviceManagerImpl;
use crate::domain::auth::AuthServiceImpl;
use crate::domain::file::FileServiceImpl;
use crate::infrastructure::bluetooth::BtleplugAdapter;

/// 应用服务
/// 
/// 整合各个领域服务，提供统一的业务逻辑入口
pub struct AppService {
    device_manager: Arc<DeviceManagerImpl>,
    auth_service: Arc<AuthServiceImpl>,
    file_service: Arc<FileServiceImpl>,
    backend_url: String,
}

impl AppService {
    /// 创建应用服务
    pub async fn new(backend_url: &str) -> AppResult<Self> {
        // 创建设备管理器
        let bluetooth_adapter = Arc::new(BtleplugAdapter::new().await?);
        let device_manager = Arc::new(DeviceManagerImpl::new(bluetooth_adapter));
        
        // 创建认证服务
        let auth_service = Arc::new(AuthServiceImpl::new(backend_url));
        
        // 创建文件服务
        let file_service = Arc::new(FileServiceImpl::new(backend_url));
        
        Ok(Self {
            device_manager,
            auth_service,
            file_service,
            backend_url: backend_url.to_string(),
        })
    }
    
    /// 获取设备管理器
    pub fn get_device_manager(&self) -> Arc<DeviceManagerImpl> {
        self.device_manager.clone()
    }
    
    /// 获取认证服务
    pub fn get_auth_service(&self) -> Arc<AuthServiceImpl> {
        self.auth_service.clone()
    }
    
    /// 获取文件服务
    pub fn get_file_service(&self) -> Arc<FileServiceImpl> {
        self.file_service.clone()
    }
    
    /// 获取后端URL
    pub fn get_backend_url(&self) -> &str {
        &self.backend_url
    }
    
    // 以下是业务方法
    
    /// 获取TOTP
    pub async fn get_totp(&self) -> AppResult<String> {
        self.device_manager.get_totp().await
    }
    
    /// 获取设备ID
    pub async fn get_device_id(&self) -> AppResult<String> {
        self.device_manager.get_device_id().await
    }
    
    /// 扫描设备
    pub async fn scan_devices(&self, timeout_ms: u64) -> AppResult<Vec<DeviceInfo>> {
        self.device_manager.scan_devices(timeout_ms).await
    }
    
    /// 连接设备
    pub async fn connect_device(&self, address: &str) -> AppResult<DeviceInfo> {
        self.device_manager.connect_device(address).await
    }
    
    /// 断开设备
    pub async fn disconnect_device(&self) -> AppResult<()> {
        self.device_manager.disconnect_device().await
    }
    
    /// 获取连接状态
    pub fn get_connection_state(&self) -> ConnectionState {
        self.device_manager.get_connection_state()
    }
    
    /// 检查是否已连接
    pub fn is_connected(&self) -> bool {
        self.device_manager.get_connection_state().is_connected()
    }
}