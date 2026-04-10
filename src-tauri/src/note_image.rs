// 保留所有权利
//
// Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
// Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
//
// Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
// Email: abc.cxh2009@foxmail.com
//
// Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
// Email: 1220596931@qq.com
//
// Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
// Email: admin@mc666.top

// 笔记图片管理模块
// 负责笔记中的图片上传、存储和管理
//
// 功能：
// 1. 上传 base64 图片
// 2. 获取用户的所有笔记图片
// 3. 删除笔记图片
// 4. 图片哈希计算（用于去重）

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex::encode as hex_encode;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;
use base64::{Engine, engine::general_purpose::STANDARD};
use tracing;

use crate::storage::get_app_data_dir;
use crate::config;

const NOTE_IMAGES_DIR: &str = ".note/.images";
const NOTE_IMAGES_INDEX: &str = "images_index.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub filename: String,
    pub size: u64,
    pub hash: String,
    pub created_at: String,
    pub path: String,
    pub user_uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagesIndex {
    pub images: HashMap<String, ImageInfo>,
    pub metadata: IndexMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub created_at: String,
    pub last_updated: String,
    pub total_images: usize,
}

impl ImagesIndex {
    pub fn new() -> Self {
        let now = get_timestamp();
        Self {
            images: HashMap::new(),
            metadata: IndexMetadata {
                created_at: now.clone(),
                last_updated: now,
                total_images: 0,
            },
        }
    }
}

pub fn get_app_data_dir_for_images() -> Result<PathBuf> {
    let data_dir = get_app_data_dir().map_err(|e| anyhow::anyhow!(e))?;
    Ok(data_dir.join(NOTE_IMAGES_DIR))
}

pub fn get_images_index_path() -> Result<PathBuf> {
    Ok(get_app_data_dir_for_images()?.join(NOTE_IMAGES_INDEX))
}

pub async fn load_images_index() -> Result<ImagesIndex> {
    let path = get_images_index_path()?;
    
    if !path.exists() {
        return Ok(ImagesIndex::new());
    }
    
    let content = fs::read_to_string(&path).await
        .context("读取图片索引文件失败")?;
    
    let index: ImagesIndex = serde_json::from_str(&content)
        .context("解析图片索引文件失败")?;
    
    Ok(index)
}

pub async fn save_images_index(index: &ImagesIndex) -> Result<()> {
    let path = get_images_index_path()?;
    
    fs::create_dir_all(get_app_data_dir_for_images()?).await
        .context("创建图片索引目录失败")?;
    
    let content = serde_json::to_string_pretty(index)
        .context("序列化图片索引失败")?;
    
    fs::write(&path, content).await
        .context("写入图片索引文件失败")?;
    
    Ok(())
}

pub fn calculate_base64_hash(base64_data: &str) -> Result<String> {
    let bytes = STANDARD.decode(base64_data)
        .context("解码 base64 数据失败")?;
    
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    
    Ok(hex_encode(hasher.finalize()))
}

pub fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
}

pub fn format_timestamp(secs: u64) -> String {
    chrono::NaiveDateTime::from_timestamp_opt(secs as i64, 0)
        .map(|dt| dt.to_string())
        .unwrap_or_else(|| "1970-01-01 00:00:00".to_string())
}

pub async fn save_base64_image(user_uuid: &str, base64_data: &str) -> Result<String> {
    // 计算图片哈希
    let hash = calculate_base64_hash(base64_data)?;
    
    // 检查是否已存在
    let mut index = load_images_index().await?;
    
    if let Some(existing_image) = index.images.get(&hash) {
        tracing::info!("图片已存在，跳过上传: {}", existing_image.filename);
        return Ok(existing_image.filename.clone());
    }
    
    // 生成文件名（使用哈希）
    let filename = format!("{}.png", hash);
    let file_path = get_app_data_dir_for_images()?
        .join(&filename);
    
    // 解码 base64 并保存图片
    let bytes = STANDARD.decode(base64_data)
        .context("解码 base64 数据失败")?;
    
    fs::write(&file_path, bytes).await
        .context("保存图片文件失败")?;
    
    // 获取文件大小
    let metadata = fs::metadata(&file_path).await
        .context("获取文件元数据失败")?;
    
    // 更新索引
    let now = get_timestamp();
    let now_formatted = format_timestamp(now.parse::<u64>().unwrap_or(0));
    
    let image_info = ImageInfo {
        filename: filename.clone(),
        size: metadata.len(),
        hash: hash.clone(),
        created_at: now_formatted.clone(),
        path: format!("/note/image/{}/{}", user_uuid, filename),
        user_uuid: user_uuid.to_string(),
    };
    
    index.images.insert(hash, image_info);
    index.metadata.total_images += 1;
    index.metadata.last_updated = now_formatted;
    
    save_images_index(&index).await?;
    
    tracing::info!("图片保存成功: {}", filename);
    
    Ok(filename)
}

pub async fn get_user_images(user_uuid: &str) -> Result<serde_json::Value> {
    let index = load_images_index().await?;
    
    let mut user_images = Vec::new();
    
    for (_, image) in &index.images {
        if image.user_uuid == user_uuid {
            user_images.push(serde_json::json!({
                "filename": image.filename,
                "size": image.size,
                "hash": image.hash,
                "created_at": image.created_at,
                "path": image.path
            }));
        }
    }
    
    Ok(serde_json::json!({
        "success": true,
        "total": user_images.len(),
        "images": user_images
    }))
}

pub async fn delete_user_image(user_uuid: &str, filename: &str) -> Result<bool> {
    let mut index = load_images_index().await?;
    
    // 查找图片
    let mut found_hash = None;
    for (hash, image) in &index.images {
        if image.user_uuid == user_uuid && image.filename == filename {
            found_hash = Some(hash.clone());
            break;
        }
    }
    
    if let Some(hash) = found_hash {
        // 删除文件
        let file_path = get_app_data_dir_for_images()?
            .join(filename);
        
        if file_path.exists() {
            fs::remove_file(&file_path).await
                .context("删除图片文件失败")?;
        }
        
        // 从索引中移除
        index.images.remove(&hash);
        index.metadata.total_images -= 1;
        
        let now = get_timestamp();
        index.metadata.last_updated = format_timestamp(now.parse::<u64>().unwrap_or(0));
        
        save_images_index(&index).await?;
        
        tracing::info!("图片删除成功: {}", filename);
        
        return Ok(true);
    }
    
    Ok(false)
}
