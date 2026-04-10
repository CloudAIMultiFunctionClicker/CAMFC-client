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

// AI 图片分析模块
// 负责图片哈希校验、索引管理和重新解析功能
//
// 功能：
// 1. 图片哈希计算（SHA256）
// 2. 索引管理（analysis_index.json）
// 3. 频率限制保护（30秒锁定）

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex::encode as hex_encode;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;
use std::io::Read;

use crate::storage::get_app_data_dir;

const REANALYZE_LOCK_SECONDS: u64 = 30;
const AI_INDEX_DIR: &str = ".ai_index";
const AI_INDEX_FILE: &str = "analysis_index.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReanalyzeLock {
    pub locked: bool,
    pub locked_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysisStatus {
    pub file_paths: Vec<String>,
    pub analyzed: bool,
    pub analysis_file: Option<String>,
    pub last_analyzed_at: Option<String>,
    pub reanalyze_lock: ReanalyzeLock,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisIndex {
    pub images: HashMap<String, ImageAnalysisStatus>,
    pub metadata: IndexMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub created_at: String,
    pub last_updated: String,
    pub total_images: usize,
    pub analyzed_images: usize,
}

impl AnalysisIndex {
    pub fn new() -> Self {
        let now = get_timestamp();
        Self {
            images: HashMap::new(),
            metadata: IndexMetadata {
                created_at: now.clone(),
                last_updated: now,
                total_images: 0,
                analyzed_images: 0,
            },
        }
    }
}

pub fn get_app_data_dir_for_ai() -> Result<PathBuf> {
    let data_dir = get_app_data_dir().map_err(|e| anyhow::anyhow!(e))?;
    Ok(data_dir.join(AI_INDEX_DIR))
}

pub fn get_index_path() -> Result<PathBuf> {
    Ok(get_app_data_dir_for_ai()?.join(AI_INDEX_FILE))
}

pub async fn load_index() -> Result<AnalysisIndex> {
    let path = get_index_path()?;
    
    if !path.exists() {
        return Ok(AnalysisIndex::new());
    }
    
    let content = fs::read_to_string(&path).await
        .context("读取索引文件失败")?;
    
    let index: AnalysisIndex = serde_json::from_str(&content)
        .context("解析索引文件失败")?;
    
    Ok(index)
}

pub async fn save_index(index: &AnalysisIndex) -> Result<()> {
    let path = get_index_path()?;
    
    fs::create_dir_all(get_app_data_dir_for_ai()?).await
        .context("创建 AI 索引目录失败")?;
    
    let content = serde_json::to_string_pretty(index)
        .context("序列化索引失败")?;
    
    fs::write(&path, content).await
        .context("写入索引文件失败")?;
    
    Ok(())
}

pub fn calculate_file_hash(file_path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(file_path)
        .context(format!("无法打开文件: {:?}", file_path))?;
    
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)
            .context("读取文件失败")?;
        
        if bytes_read == 0 {
            break;
        }
        
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(hex_encode(hasher.finalize()))
}

pub fn calculate_base64_hash(base64_data: &str) -> Result<String> {
    use base64::Engine;
    let engine = base64::engine::general_purpose::STANDARD;
    let bytes = engine.decode(base64_data)
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

pub async fn get_file_hash(file_path: &str) -> Result<String> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(anyhow::anyhow!("文件不存在: {}", file_path));
    }
    
    calculate_file_hash(path)
}

pub async fn get_file_size(file_path: &str) -> Result<u64> {
    let metadata = fs::metadata(file_path).await
        .context(format!("获取文件元数据失败: {}", file_path))?;
    
    Ok(metadata.len())
}

pub async fn add_or_update_image(
    index: &mut AnalysisIndex,
    file_path: &str,
    file_hash: &str,
) -> Result<()> {
    let now = get_timestamp();
    let now_formatted = format_timestamp(now.parse::<u64>().unwrap_or(0));
    
    let entry = index.images.entry(file_hash.to_string())
        .or_insert_with(|| {
            index.metadata.total_images += 1;
            ImageAnalysisStatus {
                file_paths: Vec::new(),
                analyzed: false,
                analysis_file: None,
                last_analyzed_at: None,
                reanalyze_lock: ReanalyzeLock {
                    locked: false,
                    locked_at: None,
                    expires_at: None,
                },
                status: "pending".to_string(),
                created_at: now_formatted.clone(),
            }
        });
    
    if !entry.file_paths.contains(&file_path.to_string()) {
        entry.file_paths.push(file_path.to_string());
    }
    
    index.metadata.last_updated = now_formatted;
    
    Ok(())
}

pub async fn mark_as_analyzed(
    index: &mut AnalysisIndex,
    file_hash: &str,
    analysis_file: &str,
) -> Result<()> {
    let now = get_timestamp();
    let now_formatted = format_timestamp(now.parse::<u64>().unwrap_or(0));
    
    if let Some(entry) = index.images.get_mut(file_hash) {
        entry.analyzed = true;
        entry.analysis_file = Some(analysis_file.to_string());
        entry.last_analyzed_at = Some(now_formatted.clone());
        entry.status = "completed".to_string();
        index.metadata.analyzed_images += 1;
    }
    
    index.metadata.last_updated = now_formatted;
    
    Ok(())
}

pub async fn lock_for_reanalyze(
    index: &mut AnalysisIndex,
    file_hash: &str,
) -> Result<()> {
    let now = get_timestamp();
    let now_formatted = format_timestamp(now.parse::<u64>().unwrap_or(0));
    let expires_at = format_timestamp(now.parse::<u64>().unwrap_or(0) + REANALYZE_LOCK_SECONDS);
    
    if let Some(entry) = index.images.get_mut(file_hash) {
        entry.reanalyze_lock = ReanalyzeLock {
            locked: true,
            locked_at: Some(now_formatted.clone()),
            expires_at: Some(expires_at.clone()),
        };
        entry.status = "processing".to_string();
    }
    
    index.metadata.last_updated = now_formatted;
    
    Ok(())
}

pub async fn unlock_after_reanalyze(
    index: &mut AnalysisIndex,
    file_hash: &str,
) -> Result<()> {
    let now = get_timestamp();
    let now_formatted = format_timestamp(now.parse::<u64>().unwrap_or(0));
    
    if let Some(entry) = index.images.get_mut(file_hash) {
        entry.reanalyze_lock = ReanalyzeLock {
            locked: false,
            locked_at: None,
            expires_at: None,
        };
        entry.status = "completed".to_string();
    }
    
    index.metadata.last_updated = now_formatted;
    
    Ok(())
}

pub async fn check_reanalyze_lock(
    index: &AnalysisIndex,
    file_hash: &str,
) -> Result<bool> {
    if let Some(entry) = index.images.get(file_hash) {
        if entry.reanalyze_lock.locked {
            if let Some(expires_at) = &entry.reanalyze_lock.expires_at {
                let expires_secs = expires_at.parse::<u64>()
                    .unwrap_or(0);
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                
                if now < expires_secs {
                    let remaining = expires_secs - now;
                    return Err(anyhow::anyhow!(
                        "解析锁未过期，请等待 {} 秒",
                        remaining
                    ));
                }
            }
        }
    }
    
    Ok(true)
}

pub async fn is_file_analyzed(
    index: &AnalysisIndex,
    file_hash: &str,
) -> Result<bool> {
    if let Some(entry) = index.images.get(file_hash) {
        Ok(entry.analyzed)
    } else {
        Ok(false)
    }
}

pub async fn get_image_status(
    index: &AnalysisIndex,
    file_hash: &str,
) -> Result<Option<ImageAnalysisStatus>> {
    Ok(index.images.get(file_hash).cloned())
}

pub async fn get_hash_for_file_path(
    index: &AnalysisIndex,
    file_path: &str,
) -> Result<Option<String>> {
    for (hash, entry) in &index.images {
        if entry.file_paths.contains(&file_path.to_string()) {
            return Ok(Some(hash.clone()));
        }
    }
    Ok(None)
}

pub async fn check_file_exists_in_index(index: &AnalysisIndex, file_path: &str) -> Result<bool> {
    for entry in index.images.values() {
        if entry.file_paths.contains(&file_path.to_string()) {
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn get_analysis_info_for_file(
    index: &AnalysisIndex,
    file_path: &str,
) -> Result<Option<(String, ImageAnalysisStatus)>> {
    for (hash, entry) in &index.images {
        if entry.file_paths.contains(&file_path.to_string()) {
            return Ok(Some((hash.clone(), entry.clone())));
        }
    }
    Ok(None)
}

pub async fn get_hash_for_filename(
    index: &AnalysisIndex,
    filename: &str,
) -> Result<Option<String>> {
    for (hash, entry) in &index.images {
        for file_path in &entry.file_paths {
            if file_path.ends_with(filename) {
                return Ok(Some(hash.clone()));
            }
        }
    }
    Ok(None)
}
