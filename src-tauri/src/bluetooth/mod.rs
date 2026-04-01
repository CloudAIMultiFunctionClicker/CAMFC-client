//! 蓝牙通信模块
//!
//! 提供蓝牙底层通信功能，包括：
//! - 蓝牙设备扫描
//! - 连接和断开
//! - 数据发送和接收
//! - 通知监听

// 实际的蓝牙实现
mod bluetooth;

// 重新导出公共类型
pub use bluetooth::{
    BluetoothManager,
    DeviceInfo,
    BluetoothResponse,
    ResponseType,
};
