//! 蓝牙模块
//! 
//! 模块化设计：
//! - manager.rs: 基础蓝牙管理器（保留原有功能）
//! - state.rs: 状态管理（TOTP缓存、连接状态等）
//! - service.rs: CPen设备服务层（产品特定逻辑）
//! 
//! 设计原则：基础功能与产品逻辑分离，便于复用和测试。

pub mod manager;
pub mod state;
pub mod service;

// 重新导出常用类型
pub use manager::{BluetoothManager, DeviceInfo};
pub use state::{BluetoothState, ConnectionStatus};
pub use service::CpenService;
