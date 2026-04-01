//! 事件发射模块
//!
//! 提供前端事件发射功能，包括：
//! - 按钮事件
//! - 截图命令
//! - 蓝牙断开事件

// 实际的事件发射实现（公共模块）
pub mod event_emitter;

// 重新导出公共函数
pub use event_emitter::{
    set_app_handle,
    emit_button_event,
    emit_screenshot_command,
    emit_show_note_command,
    emit_open_cloud_command,
    emit_bluetooth_disconnect,
};
