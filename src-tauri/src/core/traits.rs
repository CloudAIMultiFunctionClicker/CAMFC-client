use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::error::{AppResult};
use crate::core::types::{DeviceInfo, ConnectionState, FileInfo, Pagination};

/// 蓝牙适配器trait
///
/// 抽象蓝牙操作，方便后面mock和测试
#[async_trait]
pub trait BluetoothAdapter: Send + Sync {
    /// 启用蓝牙
    async fn enable(&self) -> AppResult<()>;

    /// 禁用蓝牙
    async fn disable(&self) -> AppResult<()>;

    /// 扫描设备
    async fn scan_devices(&self, timeout_ms: u64) -> AppResult<Vec<DeviceInfo>>;

    /// 连接设备
    async fn connect(&self, address: &str) -> AppResult<DeviceInfo>;

    /// 断开连接
    async fn disconnect(&self) -> AppResult<()>;

    /// 检查是否已连接
    async fn is_connected(&self) -> AppResult<bool>;

    /// 获取连接状态
    fn get_connection_state(&self) -> ConnectionState;

    /// 发送数据
    async fn send_data(&self, data: &[u8]) -> AppResult<()>;

    /// 接收数据
    async fn receive_data(&self, timeout_ms: u64) -> AppResult<Vec<u8>>;
}

/// 存储trait
/// 
/// 抽象存储操作，支持多种后端
#[async_trait]
pub trait Storage: Send + Sync {
    /// 保存数据
    async fn save(&self, key: &str, data: &[u8]) -> AppResult<()>;
    
    /// 读取数据
    async fn load(&self, key: &str) -> AppResult<Option<Vec<u8>>>;
    
    /// 删除数据
    async fn delete(&self, key: &str) -> AppResult<()>;
    
    /// 检查是否存在
    async fn exists(&self, key: &str) -> AppResult<bool>;
    
    /// 列出所有键
    async fn list_keys(&self, pattern: &str) -> AppResult<Vec<String>>;
}

/// 配置管理trait
/// 
/// 统一配置接口
pub trait ConfigManager: Send + Sync {
    /// 获取配置值
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> AppResult<Option<T>>;
    
    /// 设置配置值
    fn set<T: serde::Serialize>(&self, key: &str, value: &T) -> AppResult<()>;
    
    /// 删除配置
    fn remove(&self, key: &str) -> AppResult<()>;
    
    /// 重新加载配置
    fn reload(&self) -> AppResult<()>;
}

/// 认证服务trait
/// 
/// 抽象认证逻辑
#[async_trait]
pub trait AuthService: Send + Sync {
    /// 登录
    async fn login(&self, username: &str, password: &str) -> AppResult<String>;
    
    /// 登出
    async fn logout(&self) -> AppResult<()>;
    
    /// 验证token
    async fn validate_token(&self, token: &str) -> AppResult<bool>;
    
    /// 刷新token
    async fn refresh_token(&self, old_token: &str) -> AppResult<String>;
    
    /// 获取用户信息
    async fn get_user_info(&self, token: &str) -> AppResult<serde_json::Value>;
}

/// 文件服务trait
/// 
/// 文件操作抽象
#[async_trait]
pub trait FileService: Send + Sync {
    /// 列出文件
    async fn list_files(&self, path: &str, pagination: &Pagination) -> AppResult<Vec<FileInfo>>;
    
    /// 创建目录
    async fn create_directory(&self, path: &str) -> AppResult<()>;
    
    /// 删除文件/目录
    async fn delete(&self, path: &str) -> AppResult<()>;
    
    /// 重命名
    async fn rename(&self, old_path: &str, new_path: &str) -> AppResult<()>;
    
    /// 移动
    async fn move_item(&self, from_path: &str, to_path: &str) -> AppResult<()>;
    
    /// 复制
    async fn copy(&self, from_path: &str, to_path: &str) -> AppResult<()>;
    
    /// 下载文件
    async fn download(&self, path: &str) -> AppResult<Vec<u8>>;
    
    /// 上传文件
    async fn upload(&self, path: &str, data: &[u8]) -> AppResult<()>;
}

/// 设备管理trait
/// 
/// 设备相关业务抽象
#[async_trait]
pub trait DeviceManager: Send + Sync {
    /// 扫描设备
    async fn scan_devices(&self, timeout_ms: u64) -> AppResult<Vec<DeviceInfo>>;
    
    /// 连接设备
    async fn connect_device(&self, address: &str) -> AppResult<DeviceInfo>;
    
    /// 断开设备
    async fn disconnect_device(&self) -> AppResult<()>;
    
    /// 获取当前设备
    fn get_current_device(&self) -> Option<DeviceInfo>;
    
    /// 获取连接状态
    fn get_connection_state(&self) -> ConnectionState;
    
    /// 发送命令
    async fn send_command(&self, command: &[u8]) -> AppResult<Vec<u8>>;
    
    /// 获取TOTP
    async fn get_totp(&self) -> AppResult<String>;
    
    /// 获取设备ID
    async fn get_device_id(&self) -> AppResult<String>;
}

/// 事件发布trait
/// 
/// 事件总线抽象
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// 发布事件
    async fn publish(&self, event_type: &str, data: serde_json::Value) -> AppResult<()>;
    
    /// 订阅事件
    async fn subscribe(&self, event_type: &str, handler: Box<dyn EventHandler>) -> AppResult<String>;
    
    /// 取消订阅
    async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()>;
}

/// 事件处理trait
/// 
/// 事件处理器
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// 处理事件
    async fn handle(&self, event_type: &str, data: serde_json::Value) -> AppResult<()>;
    
    /// 获取处理器名称
    fn name(&self) -> &str;
}

/// 日志trait
/// 
/// 日志抽象，方便后面换实现
pub trait Logger: Send + Sync {
    /// 调试日志
    fn debug(&self, message: &str);
    
    /// 信息日志
    fn info(&self, message: &str);
    
    /// 警告日志
    fn warn(&self, message: &str);
    
    /// 错误日志
    fn error(&self, message: &str);
    
    /// 带上下文的日志
    fn with_context(&self, context: serde_json::Value) -> Box<dyn Logger>;
}