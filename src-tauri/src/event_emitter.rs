// 保留所有权利
//
// Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
// Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
//
// Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
// Email: abc.cxh2009@foxmail.com
//
// Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
// Email: 1220594170@qq.com
//
// Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
// Email: admin@mc666.top

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tauri::Emitter;

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
        let _ = handle.emit("button-event", event);
    }
}

/// 发射蓝牙断开事件
pub fn emit_bluetooth_disconnect() {
    if let Some(handle) = get_app_handle() {
        println!("[EVENT] 发射蓝牙断开事件");
        let _ = handle.emit("bluetooth-disconnect", ());
    }
}
