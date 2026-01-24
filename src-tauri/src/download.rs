// 文件下载模块
// 负责从云盘后端下载文件，支持分片和断点续传
//
// 思考：这个模块要实现的功能
// 1. 下载文件到应用内目录
// 2. 支持分片下载（默认4MB）
// 3. 支持断点续传
// 4. 提供下载进度信息

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncWriteExt, AsyncReadExt, AsyncSeekExt};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use reqwest::{Client, header};
use sha2::{Sha256, Digest};
use hex::encode as hex_encode;

// 基础URL - 和前端保持一致
const BASE_URL: &str = "http://cloud.api.ant-cave-2026.asia";
// 默认分片大小 4MB - 和后端保持一致
const CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4MB
// 下载目录名称
const DOWNLOAD_DIR: &str = "downloads";

// 下载状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,      // 等待开始
    Downloading,  // 下载中
    Paused,       // 已暂停
    Completed,    // 已完成
    Error(String), // 错误
}

// 下载进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub file_id: String,           // 文件ID
    pub file_name: String,         // 文件名
    pub total_size: u64,           // 总大小
    pub downloaded: u64,           // 已下载大小
    pub status: DownloadStatus,    // 下载状态
    pub chunks_total: u32,         // 总分片数
    pub chunks_completed: u32,     // 已完成分片数
    pub speed_kbps: f64,           // 下载速度 KB/s
}

// 认证信息 - 从蓝牙设备获取
#[derive(Debug, Clone)]
pub struct AuthInfo {
    pub device_id: String,  // 设备ID
    pub totp: String,       // 动态密码
}

impl AuthInfo {
    // 获取认证头信息
    pub fn get_auth_header(&self) -> Result<header::HeaderMap> {
        let auth_json = serde_json::json!({
            "Id": self.device_id,
            "Totp": self.totp
        }).to_string();
        
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_json)?
        );
        
        Ok(headers)
    }
}

// 分片下载器
pub struct ChunkDownloader {
    client: Client,
    auth_info: AuthInfo,
}

impl ChunkDownloader {
    // 创建新的下载器
    pub fn new(auth_info: AuthInfo) -> Result<Self> {
        // 创建HTTP客户端，设置合适的超时时间
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("创建HTTP客户端失败")?;
            
        Ok(Self { client, auth_info })
    }
    
    // 下载单个分片
    pub async fn download_chunk(
        &self,
        file_id: &str,
        chunk_index: u32,
        range_start: u64,
        range_end: u64,
    ) -> Result<Vec<u8>> {
        let url = format!("{}/download/{}", BASE_URL, file_id);
        
        // 构建Range头
        let range_header = format!("bytes={}-{}", range_start, range_end);
        
        // 获取认证头
        let mut headers = self.auth_info.get_auth_header()?;
        headers.insert(
            header::RANGE,
            header::HeaderValue::from_str(&range_header)?
        );
        
        // 发送请求
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .context("发送下载请求失败")?;
            
        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "下载请求失败: {} - {}", 
                status, 
                error_text
            ));
        }
        
        // 读取响应内容
        let chunk_data = response
            .bytes()
            .await
            .context("读取分片数据失败")?;
            
        Ok(chunk_data.to_vec())
    }
    
    // 获取文件元数据（大小等信息）
    pub async fn get_file_metadata(&self, file_id: &str) -> Result<(u64, String)> {
        let url = format!("{}/files/info/{}", BASE_URL, file_id);
        
        let headers = self.auth_info.get_auth_header()?;
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .context("获取文件元数据失败")?;
            
        if !response.status().is_success() {
            let status = response.status();
            return Err(anyhow::anyhow!(
                "获取文件元数据失败: {}", 
                status
            ));
        }
        
        // 解析响应JSON
        // TODO: 这里应该解析实际的响应结构，先简单返回一个默认值
        // 假设文件大小从Content-Length获取，名字从URL获取
        let content_length = response
            .headers()
            .get(header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
            
        let filename = format!("{}.bin", file_id); // 临时文件名
        
        Ok((content_length, filename))
    }
}

// 下载任务管理器
pub struct DownloadTask {
    file_id: String,
    file_name: String,
    save_path: PathBuf,
    total_size: u64,
    downloaded_size: Arc<Mutex<u64>>,
    status: Arc<Mutex<DownloadStatus>>,
    downloader: ChunkDownloader,
}

impl DownloadTask {
    // 创建新的下载任务
    pub async fn new(
        file_id: String,
        save_path: PathBuf,
        auth_info: AuthInfo,
    ) -> Result<Self> {
        // 创建下载器
        let downloader = ChunkDownloader::new(auth_info)?;
        
        // 获取文件元数据
        let (total_size, file_name) = downloader.get_file_metadata(&file_id).await?;
        
        // 确保保存目录存在
        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent).await
                .context("创建下载目录失败")?;
        }
        
        Ok(Self {
            file_id,
            file_name,
            save_path,
            total_size,
            downloaded_size: Arc::new(Mutex::new(0)),
            status: Arc::new(Mutex::new(DownloadStatus::Pending)),
            downloader,
        })
    }
    
    // 开始下载（或恢复下载）
    pub async fn start(&self) -> Result<()> {
        // 更新状态为下载中
        *self.status.lock().await = DownloadStatus::Downloading;
        
        // 计算分片信息
        let chunks_count = if self.total_size > 0 {
            ((self.total_size as f64) / (CHUNK_SIZE as f64)).ceil() as u32
        } else {
            1 // 如果不知道大小，就按一个分片处理
        };
        
        println!("开始下载文件: {}, 总分片数: {}", self.file_name, chunks_count);
        
        // 分片下载
        for chunk_index in 0..chunks_count {
            // 检查状态，如果暂停了就退出循环
            {
                let status = self.status.lock().await;
                if let DownloadStatus::Paused = *status {
                    println!("下载已暂停");
                    return Ok(());
                }
            }
            
            // 计算分片范围
            let start = (chunk_index as u64) * CHUNK_SIZE;
            let end = if chunk_index == chunks_count - 1 {
                self.total_size - 1
            } else {
                start + CHUNK_SIZE - 1
            };
            
            // 下载分片
            match self.downloader.download_chunk(
                &self.file_id,
                chunk_index,
                start,
                end,
            ).await {
                Ok(chunk_data) => {
                    // 写入文件
                    self.write_chunk(start, &chunk_data).await?;
                    
                    // 更新进度
                    let mut downloaded = self.downloaded_size.lock().await;
                    *downloaded += chunk_data.len() as u64;
                    
                    println!("分片 {}/{} 下载完成，当前进度: {}/{}", 
                        chunk_index + 1, 
                        chunks_count,
                        *downloaded,
                        self.total_size
                    );
                }
                Err(e) => {
                    // 下载失败，更新状态
                    *self.status.lock().await = DownloadStatus::Error(e.to_string());
                    return Err(e);
                }
            }
        }
        
        // 下载完成
        *self.status.lock().await = DownloadStatus::Completed;
        println!("文件下载完成: {}", self.file_name);
        
        Ok(())
    }
    
    // 写入分片到文件
    async fn write_chunk(&self, offset: u64, data: &[u8]) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&self.save_path)
            .await
            .context("打开文件失败")?;
            
        // 移动到正确位置
        file.seek(std::io::SeekFrom::Start(offset)).await
            .context("移动文件指针失败")?;
            
        // 写入数据
        file.write_all(data).await
            .context("写入文件失败")?;
            
        Ok(())
    }
    
    // 暂停下载
    pub async fn pause(&self) {
        *self.status.lock().await = DownloadStatus::Paused;
        println!("下载已暂停");
    }
    
    // 获取下载进度
    pub async fn get_progress(&self) -> DownloadProgress {
        let downloaded = *self.downloaded_size.lock().await;
        let status = self.status.lock().await.clone();
        
        let chunks_total = if self.total_size > 0 {
            ((self.total_size as f64) / (CHUNK_SIZE as f64)).ceil() as u32
        } else {
            0
        };
        
        let chunks_completed = if self.total_size > 0 {
            ((downloaded as f64) / (self.total_size as f64) * (chunks_total as f64)) as u32
        } else {
            0
        };
        
        DownloadProgress {
            file_id: self.file_id.clone(),
            file_name: self.file_name.clone(),
            total_size: self.total_size,
            downloaded,
            status,
            chunks_total,
            chunks_completed,
            speed_kbps: 0.0, // 暂时不计算速度，先实现基本功能
        }
    }
}

// 工具函数：获取应用数据目录
pub async fn get_app_data_dir() -> Result<PathBuf> {
    // 使用Tauri的路径API获取数据目录
    // 这里先用一个简单的临时方案
    let current_dir = std::env::current_dir()
        .context("获取当前目录失败")?;
    
    Ok(current_dir.join("data").join(DOWNLOAD_DIR))
}

// 工具函数：计算文件SHA256哈希
pub async fn calculate_file_hash(path: &Path) -> Result<String> {
    let mut file = File::open(path).await
        .context("打开文件失败")?;
    
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192]; // 8KB缓冲区
    
    loop {
        let bytes_read = file.read(&mut buffer).await
            .context("读取文件失败")?;
            
        if bytes_read == 0 {
            break;
        }
        
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(hex_encode(hasher.finalize()))
}