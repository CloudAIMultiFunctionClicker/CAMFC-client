use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppStorage {
    #[serde(flatten)]
    pub data: HashMap<String, String>,
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
    
    let value = storage.data.get(&key).cloned().unwrap_or_default();
    
    Ok(value)
}

#[tauri::command]
pub async fn save_app_data(key: String, value: String) -> Result<(), String> {
    let mut storage = load_storage().await
        .map_err(|e| format!("加载数据失败: {}", e))?;
    
    storage.data.insert(key, value);
    
    save_storage(&storage).await
        .map_err(|e| format!("保存数据失败: {}", e))?;
    
    Ok(())
}

pub fn get_app_data_dir() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| "获取应用数据目录失败".to_string())?
        .join("CAMFC");
    
    Ok(data_dir)
}

#[tauri::command]
pub async fn get_notes_temp_path() -> Result<String, String> {
    let data_dir = get_app_data_dir()?;
    
    let note_dir = data_dir.join(".note");
    if !note_dir.exists() {
        fs::create_dir_all(&note_dir)
            .await
            .map_err(|e| format!("创建笔记目录失败: {}", e))?;
    }
    
    let notes_path = note_dir.join("notes.json");
    Ok(notes_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn read_notes_content() -> Result<String, String> {
    let notes_path = get_notes_temp_path().await?;
    let notes_path = PathBuf::from(notes_path);
    
    if !notes_path.exists() {
        return Ok(String::new());
    }
    
    let content = fs::read_to_string(&notes_path)
        .await
        .map_err(|e| format!("读取笔记失败: {}", e))?;
    
    Ok(content)
}

#[tauri::command]
pub async fn write_notes_content(content: String) -> Result<(), String> {
    let notes_path = get_notes_temp_path().await?;
    let notes_path = PathBuf::from(notes_path);
    
    fs::write(&notes_path, content)
        .await
        .map_err(|e| format!("写入笔记失败: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_download_file_path(file_id: String) -> Result<String, String> {
    let data_dir = get_app_data_dir()?;
    let file_path = data_dir.join(&file_id);
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_notes_to_file(notes_json: String) -> Result<String, String> {
    let file_path = rfd::FileDialog::new()
        .set_file_name(&format!("camfc-notes-{}.json", chrono::Local::now().format("%Y-%m-%d")))
        .add_filter("JSON", &["json"])
        .save_file();
    
    match file_path {
        Some(path) => {
            std::fs::write(&path, notes_json)
                .map_err(|e| format!("写入文件失败: {}", e))?;
            Ok(path.to_string_lossy().to_string())
        }
        None => Err("用户取消操作".to_string())
    }
}

#[tauri::command]
pub async fn import_notes_from_file() -> Result<String, String> {
    let file_path = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file();
    
    match file_path {
        Some(path) => {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("读取文件失败: {}", e))?;
            Ok(content)
        }
        None => Err("用户取消操作".to_string())
    }
}

#[tauri::command]
pub async fn clear_local_notes() -> Result<bool, String> {
    let data_dir = get_app_data_dir()?;
    let note_dir = data_dir.join(".note");
    
    if !note_dir.exists() {
        return Ok(true); // 目录不存在，认为清理成功
    }
    
    // 删除整个.note目录及其内容
    match fs::remove_dir_all(&note_dir).await {
        Ok(_) => {
            println!("本地笔记文件已清理: {:?}", note_dir);
            Ok(true)
        }
        Err(e) => {
            println!("清理本地笔记文件失败: {:?}", e);
            Err(format!("清理本地笔记文件失败: {}", e))
        }
    }
}
