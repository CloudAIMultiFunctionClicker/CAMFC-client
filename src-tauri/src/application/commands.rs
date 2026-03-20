use std::sync::OnceLock;
use tokio::sync::Mutex;

use crate::core::{AppResult, AppError, DeviceInfo, AuthService};
use crate::application::services::AppService;

static APP_SERVICE: OnceLock<Mutex<AppService>> = OnceLock::new();

pub async fn get_app_service() -> AppResult<&'static Mutex<AppService>> {
    if let Some(service) = APP_SERVICE.get() {
        return Ok(service);
    }

    let backend_url = "http://localhost:8080";
    let new_service = AppService::new(backend_url).await?;

    APP_SERVICE.set(Mutex::new(new_service))
        .map_err(|_| AppError::Unknown("应用服务已初始化".to_string()))?;

    Ok(APP_SERVICE.get().unwrap())
}

#[tauri::command]
pub async fn get_totp() -> Result<String, String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.get_totp().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_device_id() -> Result<String, String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.get_device_id().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn scan_cpen_devices() -> Result<Vec<DeviceInfo>, String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.scan_devices(10000).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connect_cpen_device(address: String) -> Result<DeviceInfo, String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.connect_device(&address).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn disconnect() -> Result<(), String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.disconnect_device().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cleanup() -> Result<(), String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.disconnect_device().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_connection_status() -> Result<String, String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    Ok(service.get_connection_state().to_chinese().to_string())
}

#[tauri::command]
pub async fn is_connected() -> Result<bool, String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    Ok(service.is_connected())
}

#[tauri::command]
pub async fn get_backend_config() -> Result<serde_json::Value, String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    Ok(serde_json::json!({
        "base_url": service.get_backend_url(),
        "port": 8080,
        "full_url": service.get_backend_url()
    }))
}

#[tauri::command]
pub async fn login(username: String, password: String) -> Result<(), String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.get_auth_service().login(&username, &password).await
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    let service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    service.get_auth_service().logout().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user_info() -> Result<serde_json::Value, String> {
    let _service = get_app_service().await
        .map_err(|e| e.to_string())?
        .lock().await;

    Ok(serde_json::json!({}))
}

#[tauri::command]
pub async fn change_password(_old_password: String, _new_password: String) -> Result<(), String> {
    Ok(())
}