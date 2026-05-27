

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tauri::{Emitter, Manager};

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

pub fn set_app_handle(handle: tauri::AppHandle) {
    let _ = APP_HANDLE.set(handle);
}

pub fn get_app_handle() -> Option<&'static tauri::AppHandle> {
    APP_HANDLE.get()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ButtonEvent {
    pub event_type: String,
    pub timestamp: i64,
}

pub fn emit_button_event(event_type: &str) {
    if let Some(handle) = get_app_handle() {
        let event = ButtonEvent {
            event_type: event_type.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            tracing::info!("[EVENT] 发送按钮事件到主窗口：{}", event_type);
            let _ = main_window.emit("button-event", event);
        } else {
            tracing::warn!("主窗口不存在，无法发送按钮事件");
        }
    }
}

/// 发射截图命令事件
pub fn emit_screenshot_command() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射截图命令事件");
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("screenshot-command", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送截图命令");
        }
    }
}

/// 发射显示主窗口 + note 页面事件
pub fn emit_show_note_command() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射显示主窗口 + note 命令事件");
        let _ = handle.emit("show-note-command", ());
    }
}

/// 发射打开云盘页面事件
pub fn emit_open_cloud_command() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射打开云盘命令事件");
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("open-cloud-command", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送打开云盘命令");
        }
    }
}

/// 发射跳转到笔记列表事件
pub fn emit_navigate_to_notes() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射跳转到笔记列表事件");
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("navigate-to-notes", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送跳转笔记列表命令");
        }
    }
}

/// 发射跳转到课堂记录事件
pub fn emit_navigate_to_meetings() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射跳转到课堂记录事件");
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("navigate-to-meetings", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送跳转课堂记录命令");
        }
    }
}

/// 发射新建笔记事件
pub fn emit_create_note() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射新建笔记事件");
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("create-note", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送新建笔记命令");
        }
    }
}

/// 发射会议切换事件（开会/下会）
pub fn emit_toggle_meeting() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射会议切换事件");
        // 显示会议切换通知
        let _ = crate::notification::show_notification("会议模式", "已切换会议模式");
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("toggle-meeting", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送会议切换命令");
        }
    }
}

/// 发射蓝牙断开事件
pub fn emit_bluetooth_disconnect() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射蓝牙断开事件");
        // 只发送到主窗口
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("bluetooth-disconnect", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送蓝牙断开事件");
        }
    }
}

/// 发射音量增加事件
pub fn emit_volume_up() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射音量增加事件");
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("volume-up", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送音量增加事件");
        }
    }
}

/// 发射音量减少事件
pub fn emit_volume_down() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射音量减少事件");
        if let Some(main_window) = handle.get_webview_window("main") {
            let _ = main_window.emit("volume-down", ());
        } else {
            tracing::warn!("主窗口不存在，无法发送音量减少事件");
        }
    }
}

/// 发射打开 agent 窗口事件
pub fn emit_open_agent_window() {
    if let Some(handle) = get_app_handle() {
        tracing::info!("[EVENT] 发射打开 agent 窗口事件");
        // 检查 agent 窗口是否已存在
        if let Some(existing_window) = handle.get_webview_window("agent-window") {
            tracing::info!("[EVENT] agent 窗口已存在，显示并聚焦");
            let _ = existing_window.show();
            let _ = existing_window.set_focus();
        } else {
            tracing::info!("[EVENT] 创建新的 agent 窗口");
            // 在主窗口中发送事件，让前端创建窗口
            if let Some(main_window) = handle.get_webview_window("main") {
                let _ = main_window.emit("open-agent-window", ());
            }
        }
    }
}
