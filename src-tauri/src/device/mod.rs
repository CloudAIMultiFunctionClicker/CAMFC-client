//! 设备管理模块
//!
//! 提供 Cpen 设备的业务逻辑层，包括：
//! - 设备扫描和识别
//! - 自动连接管理
//! - TOTP 获取和缓存
//! - 设备 ID 管理

// 实际的设备管理器实现
mod cpen_device_manager;

// 重新导出公共类型
pub use cpen_device_manager::CpenDeviceManager;
