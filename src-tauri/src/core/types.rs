use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 设备信息
/// 
/// 原代码里就有，直接搬过来，加几个注释
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub address: String,
    pub services: Vec<Uuid>,
}

/// 连接状态
/// 
/// 比原来那个更细点，方便调试
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConnectionState {
    /// 未连接
    Disconnected,
    /// 正在扫描
    Scanning,
    /// 正在连接
    Connecting,
    /// 已连接
    Connected,
    /// 连接断开（异常）
    DisconnectedError,
}

impl ConnectionState {
    /// 转换为中文描述
    /// 
    /// 前端显示用，懒得在前端再写一遍
    pub fn to_chinese(&self) -> &'static str {
        match self {
            Self::Disconnected => "未连接",
            Self::Scanning => "扫描中",
            Self::Connecting => "连接中",
            Self::Connected => "已连接",
            Self::DisconnectedError => "连接异常断开",
        }
    }
    
    /// 是否已连接
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected)
    }
    
    /// 是否正在处理中
    pub fn is_processing(&self) -> bool {
        matches!(self, Self::Scanning | Self::Connecting)
    }
}

/// 蓝牙响应类型
/// 
/// 原来分散在几个地方，集中一下
#[derive(Debug, Clone, PartialEq)]
pub enum BluetoothResponse {
    /// 设置时间响应
    SetTime(String),
    /// 获取TOTP响应
    GetTotp(String),
    /// 获取设备ID响应
    GetId(String),
    /// 按钮事件
    ButtonEvent(u8),
}

/// 缓存条目
/// 
/// 通用缓存结构，TOTP那个太特化了
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: std::time::Instant,
    pub ttl: std::time::Duration,
}

impl<T> CacheEntry<T> {
    /// 创建新缓存条目
    pub fn new(data: T, ttl: std::time::Duration) -> Self {
        Self {
            data,
            timestamp: std::time::Instant::now(),
            ttl,
        }
    }
    
    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }
    
    /// 获取数据（如果未过期）
    pub fn get(&self) -> Option<&T> {
        if self.is_expired() {
            None
        } else {
            Some(&self.data)
        }
    }
}

/// 分页参数
/// 
/// 文件列表那些地方要用，先定义好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub total: Option<u64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
            total: None,
        }
    }
}

/// 文件信息
/// 
/// 比原来更完整点，省得后面改来改去
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_directory: bool,
    pub modified_time: Option<i64>,
    pub created_time: Option<i64>,
}

/// 认证信息
/// 
/// 原来那个太简单了，扩展一下
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    pub device_id: String,
    pub totp: String,
    pub user_id: Option<String>,
    pub expires_at: Option<i64>,
}

impl AuthInfo {
    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            let now = chrono::Utc::now().timestamp();
            now >= expires
        } else {
            false
        }
    }
}

/// 后端配置
/// 
/// 集中管理配置，别到处硬编码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub base_url: String,
    pub port: u16,
    pub timeout: u64,
    pub retry_count: u32,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost".to_string(),
            port: 8080,
            timeout: 5000,
            retry_count: 3,
        }
    }
}