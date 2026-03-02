use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppStorage {
    pub active_uploads: Vec<String>,
    pub active_downloads: Vec<String>,
    pub notes: String,
    pub theme: String,
}

impl AppStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

async fn get_storage_path() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .context("获取应用数据目录失败")?
        .join("CAMFC");
    
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).await
            .context(format!("创建数据目录失败: {:?}", data_dir))?;
    }
    
    Ok(data_dir.join("app_data.json"))
}

pub async fn load_storage() -> Result<AppStorage> {
    let path = get_storage_path().await?;
    
    if !path.exists() {
        return Ok(AppStorage::new());
    }
    
    let content = fs::read_to_string(&path).await
        .context("读取存储文件失败")?;
    
    let storage: AppStorage = serde_json::from_str(&content)
        .unwrap_or_default();
    
    Ok(storage)
}

pub async fn save_storage(storage: &AppStorage) -> Result<()> {
    let path = get_storage_path().await?;
    
    let content = serde_json::to_string_pretty(storage)
        .context("序列化存储数据失败")?;
    
    fs::write(&path, content).await
        .context("写入存储文件失败")?;
    
    Ok(())
}

#[tauri::command]
pub async fn load_app_data(key: String) -> Result<String, String> {
    let storage = load_storage().await
        .map_err(|e| format!("加载数据失败: {}", e))?;
    
    let value = match key.as_str() {
        "active_uploads" => serde_json::to_string(&storage.active_uploads)
            .unwrap_or_else(|_| "[]".to_string()),
        "active_downloads" => serde_json::to_string(&storage.active_downloads)
            .unwrap_or_else(|_| "[]".to_string()),
        "notes" => storage.notes,
        "theme" => storage.theme,
        _ => "".to_string(),
    };
    
    Ok(value)
}

#[tauri::command]
pub async fn save_app_data(key: String, value: String) -> Result<(), String> {
    let mut storage = load_storage().await
        .map_err(|e| format!("加载数据失败: {}", e))?;
    
    match key.as_str() {
        "active_uploads" => {
            storage.active_uploads = serde_json::from_str(&value)
                .unwrap_or_default();
        }
        "active_downloads" => {
            storage.active_downloads = serde_json::from_str(&value)
                .unwrap_or_default();
        }
        "notes" => {
            storage.notes = value;
        }
        "theme" => {
            storage.theme = value;
        }
        _ => {}
    }
    
    save_storage(&storage).await
        .map_err(|e| format!("保存数据失败: {}", e))?;
    
    Ok(())
}
