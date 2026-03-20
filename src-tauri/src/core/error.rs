use std::fmt;
use thiserror::Error;

/// 应用统一错误类型
/// 
/// 试过用String但太不直观，还是enum清楚点
/// 所有业务错误都收敛到这里，前端好处理
#[derive(Error, Debug, Clone)]
pub enum AppError {
    /// 蓝牙相关错误
    #[error("蓝牙错误: {0}")]
    Bluetooth(String),
    
    /// 设备连接错误
    #[error("设备连接错误: {0}")]
    DeviceConnection(String),
    
    /// 认证错误
    #[error("认证失败: {0}")]
    Authentication(String),
    
    /// 文件操作错误
    #[error("文件错误: {0}")]
    FileOperation(String),
    
    /// 网络请求错误
    #[error("网络错误: {0}")]
    Network(String),
    
    /// 配置错误
    #[error("配置错误: {0}")]
    Configuration(String),
    
    /// 存储错误
    #[error("存储错误: {0}")]
    Storage(String),
    
    /// 无效参数
    #[error("无效参数: {0}")]
    InvalidParameter(String),
    
    /// 操作超时
    #[error("操作超时: {0}")]
    Timeout(String),
    
    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl AppError {
    /// 转换为前端友好的错误信息
    /// 
    /// 不想把内部细节暴露给前端，简单点
    pub fn to_user_friendly(&self) -> String {
        match self {
            Self::Bluetooth(msg) => format!("蓝牙连接失败: {}", msg),
            Self::DeviceConnection(msg) => format!("设备连接失败: {}", msg),
            Self::Authentication(msg) => format!("认证失败: {}", msg),
            Self::FileOperation(msg) => format!("文件操作失败: {}", msg),
            Self::Network(msg) => format!("网络连接失败: {}", msg),
            Self::Configuration(msg) => format!("配置错误: {}", msg),
            Self::Storage(msg) => format!("存储错误: {}", msg),
            Self::InvalidParameter(msg) => format!("参数错误: {}", msg),
            Self::Timeout(msg) => format!("操作超时: {}", msg),
            Self::Unknown(msg) => format!("发生未知错误: {}", msg),
        }
    }
    
    /// 判断是否为可恢复错误
    /// 
    /// 用于决定是否需要重试，经验之谈
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Network(_) | Self::Timeout(_) => true,
            Self::Bluetooth(msg) => msg.contains("timeout") || msg.contains("retry"),
            _ => false,
        }
    }
}

/// 方便从其他错误类型转换
/// 
/// 省得每次都map_err，写着烦
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Storage(format!("IO错误: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Storage(format!("JSON解析错误: {}", err))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network(format!("HTTP请求错误: {}", err))
    }
}

// TODO: 看看还需要哪些From实现，先用着再说

/// 结果类型别名，写着省事
pub type AppResult<T> = Result<T, AppError>;