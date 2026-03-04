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
