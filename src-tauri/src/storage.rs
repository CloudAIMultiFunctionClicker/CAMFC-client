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

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
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
    tracing::info!("[STORAGE] 加载设置: {}", key);
    
    let storage = load_storage().await
        .map_err(|e| format!("加载数据失败: {}", e))?;
    
    let value = storage.data.get(&key).cloned().unwrap_or_default();
    
    tracing::info!("[STORAGE] 加载设置完成: {} = {}", key, if value.is_empty() { "(空)" } else { "(有值)" });
    Ok(value)
}

#[tauri::command]
pub async fn save_app_data(key: String, value: String) -> Result<(), String> {
    tracing::info!("[STORAGE] 保存设置: {} = {}", key, value);
    
    let mut storage = load_storage().await
        .map_err(|e| format!("加载数据失败: {}", e))?;
    
    storage.data.insert(key, value);
    
    save_storage(&storage).await
        .map_err(|e| format!("保存数据失败: {}", e))?;
    
    tracing::info!("[STORAGE] 设置保存成功");
    Ok(())
}

pub fn get_app_data_dir() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| "获取应用数据目录失败".to_string())?
        .join("CAMFC");
    
    Ok(data_dir)
}

#[tauri::command]
pub async fn get_download_file_path(fileId: String) -> Result<String, String> {
    // 使用 download 模块的 get_app_data_dir 函数，它会考虑自定义下载路径
    let data_dir = crate::download::get_app_data_dir()
        .map_err(|e| format!("获取下载目录失败：{}", e))?;
    let file_path = data_dir.join(&fileId);
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn open_file(filePath: String) -> Result<(), String> {
    tracing::info!("[STORAGE] 打开文件：{}", filePath);
    
    // 使用 Windows 的 start 命令打开文件
    Command::new("cmd")
        .args(["/C", "start", "", &filePath])
        .spawn()
        .map_err(|e| format!("打开文件失败：{}", e))?;
    
    tracing::info!("[STORAGE] 文件打开命令已执行");
    Ok(())
}

#[tauri::command]
pub fn open_folder(folderPath: String) -> Result<(), String> {
    tracing::info!("[STORAGE] 打开文件所在文件夹：{}", folderPath);
    
    // 提取文件路径中的目录部分
    let path = PathBuf::from(&folderPath);
    let parent_dir = path.parent()
        .ok_or_else(|| format!("无法获取文件 {:?} 的父目录", folderPath))?;
    
    // 使用 Windows 的 explorer 命令打开文件夹
    Command::new("explorer")
        .arg(parent_dir)
        .spawn()
        .map_err(|e| format!("打开文件夹失败：{}", e))?;
    
    tracing::info!("[STORAGE] 文件夹打开命令已执行");
    Ok(())
}

use std::sync::OnceLock;

const DOWNLOAD_PATH_KEY: &str = "download_path";

static CUSTOM_DOWNLOAD_PATH: OnceLock<String> = OnceLock::new();

pub fn get_download_path_for_download() -> Result<String, String> {
    if let Some(path) = CUSTOM_DOWNLOAD_PATH.get() {
        return Ok(path.clone());
    }
    // 缓存未初始化时返回空字符串，使用默认路径
    Ok(String::new())
}

pub fn set_download_path_cache(path: &str) {
    CUSTOM_DOWNLOAD_PATH.set(path.to_string()).ok();
}

pub async fn load_download_path_to_cache() -> Result<String, String> {
    let storage = load_storage().await
        .map_err(|e| format!("加载存储失败: {}", e))?;
    
    let path = storage.data.get(DOWNLOAD_PATH_KEY).cloned().unwrap_or_default();
    
    if !path.is_empty() {
        set_download_path_cache(&path);
    }
    
    Ok(path)
}

#[tauri::command]
pub async fn get_custom_download_path() -> Result<String, String> {
    // 如果缓存已初始化，直接返回
    if let Some(path) = CUSTOM_DOWNLOAD_PATH.get() {
        return Ok(path.clone());
    }
    
    // 从存储加载
    let path = load_download_path_to_cache().await?;
    Ok(path)
}

#[tauri::command]
pub async fn set_custom_download_path(path: String) -> Result<(), String> {
    tracing::info!("[STORAGE] 设置自定义下载路径: {}", path);
    
    // 更新缓存
    set_download_path_cache(&path);
    
    let mut storage = load_storage().await
        .map_err(|e| format!("加载存储失败: {}", e))?;
    
    if path.is_empty() {
        storage.data.remove(DOWNLOAD_PATH_KEY);
    } else {
        storage.data.insert(DOWNLOAD_PATH_KEY.to_string(), path.clone());
    }
    
    save_storage(&storage).await
        .map_err(|e| format!("保存存储失败: {}", e))?;
    
    tracing::info!("[STORAGE] 自定义下载路径保存成功: {}", path);
    Ok(())
}

#[tauri::command]
pub fn get_default_download_path() -> Result<String, String> {
    let user_profile = std::env::var("USERPROFILE")
        .map_err(|e| format!("获取用户目录失败: {}", e))?;
    
    let download_path = PathBuf::from(user_profile).join("Downloads");
    
    Ok(download_path.to_string_lossy().to_string())
}
