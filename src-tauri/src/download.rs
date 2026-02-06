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
const BASE_URL: &str = "http://localhost:8005";
// 默认分片大小 4MB - 和后端保持一致
const CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4MB
// 下载目录名称
const DOWNLOAD_DIR: &str = "C:\\Users\\user";

// 文件类型分类
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Image,      // 图片
    Video,      // 视频
    Audio,      // 音频
    Document,   // 文档
    Archive,    // 压缩包
    Code,       // 代码
    Other,      // 其他
}

// 文件类型对应的文件夹名称
impl FileType {
    pub fn folder_name(&self) -> &'static str {
        match self {
            FileType::Image => "图片",
            FileType::Video => "视频",
            FileType::Audio => "音频",
            FileType::Document => "文档",
            FileType::Archive => "压缩包",
            FileType::Code => "代码",
            FileType::Other => "其他",
        }
    }
}

// 根据文件扩展名判断文件类型
pub fn get_file_type_from_extension(ext: &str) -> FileType {
    let ext_lower = ext.to_lowercase();
    
    match ext_lower.as_str() {
        // 图片
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" => FileType::Image,
        
        // 视频
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => FileType::Video,
        
        // 音频
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" => FileType::Audio,
        
        // 文档
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "odt" | "ods" | "odp" => FileType::Document,
        
        // 压缩包
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => FileType::Archive,
        
        // 代码
        "js" | "ts" | "html" | "css" | "json" | "xml" | "py" | "java" | "cpp" | "c" | "h" | "rs" | "go" | "php" | "rb" | "swift" | "kt" => FileType::Code,
        
        // 其他
        _ => FileType::Other,
    }
}

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
        file_id: &str,  // 注意：file_id应该是完整的云盘路径，如"ds/下载.png"
        _chunk_index: u32,
        range_start: u64,
        range_end: u64,
    ) -> Result<Vec<u8>> {
        // 构建URL：file_id应该包含完整路径
        // 例如：file_id = "ds/下载.png" -> URL = "http://localhost:8005/download/ds/下载.png"
        let encoded_file_id = urlencoding::encode(file_id);
        let url = format!("{}/download/{}", BASE_URL, encoded_file_id);
        
        println!("下载请求URL: {}", url);
        println!("原始文件路径: {}", file_id);
        
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
        // 根据API文档，应该使用HEAD /download/{file_path} 获取文件元数据
        // 例如：file_id = "ds/下载.png" -> URL = "http://localhost:8005/download/ds/下载.png"
        let encoded_file_id = urlencoding::encode(file_id);
        let url = format!("{}/download/{}", BASE_URL, encoded_file_id);
        
        println!("获取文件元数据URL (HEAD): {}", url);
        println!("原始文件路径: {}", file_id);
        
        let headers = self.auth_info.get_auth_header()?;
        
        // 发送HEAD请求获取文件元数据
        let response = self.client
            .head(&url)
            .headers(headers)
            .send()
            .await
            .context("获取文件元数据失败")?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = if status == reqwest::StatusCode::NOT_FOUND {
                "文件不存在".to_string()
            } else {
                response.text().await.unwrap_or_default()
            };
            return Err(anyhow::anyhow!(
                "获取文件元数据失败: {} - {}", 
                status, 
                error_text
            ));
        }
        
        // 从响应头获取文件大小
        let content_length = response
            .headers()
            .get(header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
            
        // 从文件路径中提取文件名
        let filename = std::path::Path::new(file_id)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(file_id)
            .to_string();
        
        println!("获取到文件元数据: 文件名={}, 大小={}字节", filename, content_length);
        
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
        
        // 获取文件元数据 - file_id应该包含完整的云盘路径
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
        
        // 检查哪些分片已经下载（断点续传）
        // 如果文件已存在，检查已下载的大小，跳过已下载的分片
        let mut starting_chunk = 0;
        let mut already_downloaded = 0;
        
        if self.save_path.exists() {
            let file_size = fs::metadata(&self.save_path).await
                .context("检查已下载文件失败")?
                .len();
            
            already_downloaded = file_size;
            starting_chunk = (file_size / CHUNK_SIZE) as u32;
            
            println!("发现已下载文件: {} 字节，从分片 {} 开始继续下载", 
                already_downloaded, starting_chunk);
            
            // 更新已下载大小
            let mut downloaded = self.downloaded_size.lock().await;
            *downloaded = already_downloaded;
        } else {
            println!("开始新下载，文件不存在");
        }
        
        // 分片下载，增加重试机制
        for chunk_index in starting_chunk..chunks_count {
            // 检查状态，如果暂停了就退出循环
            {
                let status = self.status.lock().await;
                match *status {
                    DownloadStatus::Paused => {
                        println!("下载已暂停");
                        return Ok(());
                    }
                    DownloadStatus::Error(_) => {
                        // 如果已经有错误，直接返回
                        return Ok(());
                    }
                    _ => {}
                }
            }
            
            // 计算分片范围
            let start = (chunk_index as u64) * CHUNK_SIZE;
            let end = if chunk_index == chunks_count - 1 {
                self.total_size - 1
            } else {
                start + CHUNK_SIZE - 1
            };
            
            // 分片重试机制
            let mut last_error = None;
            for retry_count in 0..3 { // 最多重试3次
                match self.downloader.download_chunk(
                    &self.file_id,
                    chunk_index,
                    start,
                    end,
                ).await {
                    Ok(chunk_data) => {
                        // 检查分片大小是否合理
                        let expected_size = (end - start + 1) as usize;
                        let actual_size = chunk_data.len();
                        
                        // 最后一个分片可能小于CHUNK_SIZE，这是正常的
                        let is_last_chunk = chunk_index == chunks_count - 1;
                        if !is_last_chunk && actual_size != expected_size {
                            println!("警告: 分片 {} 大小异常，期望 {} 字节，实际 {} 字节", 
                                chunk_index, expected_size, actual_size);
                            // 继续处理，不中断下载
                        }
                        
                        // 写入文件
                        if let Err(e) = self.write_chunk(start, &chunk_data).await {
                            println!("写入分片 {} 失败: {}, 重试 {}/3", chunk_index, e, retry_count + 1);
                            last_error = Some(e);
                            continue; // 写入失败也重试
                        }
                        
                        // 更新进度
                        let mut downloaded = self.downloaded_size.lock().await;
                        *downloaded += actual_size as u64;
                        
                        println!("分片 {}/{} 下载完成 ({}/{} 字节)，当前进度: {}/{} 字节", 
                            chunk_index + 1, 
                            chunks_count,
                            actual_size,
                            expected_size,
                            *downloaded,
                            self.total_size
                        );
                        
                        last_error = None;
                        break; // 成功，跳出重试循环
                    }
                    Err(e) => {
                        println!("下载分片 {} 失败: {}, 重试 {}/3", chunk_index, e, retry_count + 1);
                        last_error = Some(e);
                        // 等待一下再重试
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
            
            // 检查重试后是否还有错误
            if let Some(e) = last_error {
                *self.status.lock().await = DownloadStatus::Error(format!("分片 {} 下载失败: {}", chunk_index, e));
                return Err(anyhow::anyhow!("分片 {} 下载失败: {}", chunk_index, e));
            }
        }
        
        // 下载完成，验证文件完整性
        println!("文件下载完成: {}，开始验证完整性...", self.file_name);
        
        // 检查文件大小是否正确
        let file_size = fs::metadata(&self.save_path).await
            .context("获取文件元数据失败")?
            .len();
        
        if file_size != self.total_size {
            let error_msg = format!("文件大小不匹配: 期望 {} 字节，实际 {} 字节", self.total_size, file_size);
            println!("错误: {}", error_msg);
            *self.status.lock().await = DownloadStatus::Error(error_msg.clone());
            return Err(anyhow::anyhow!(error_msg));
        }
        
        println!("文件大小验证通过: {} 字节", file_size);
        
        // 尝试计算文件哈希进行基本校验
        // 注意：这个校验只是本地校验，无法验证与服务器端是否一致
        match calculate_file_hash(&self.save_path).await {
            Ok(hash) => {
                println!("文件SHA256哈希: {}", hash);
                // 这里可以记录哈希值，将来可以与服务器端对比
            }
            Err(e) => {
                println!("警告: 无法计算文件哈希: {}", e);
                // 不中断下载，只是记录警告
            }
        }
        
        // 更新状态为完成
        *self.status.lock().await = DownloadStatus::Completed;
        println!("文件下载和验证完成: {}", self.file_name);
        
        Ok(())
    }
    
    // 写入分片到文件
    async fn write_chunk(&self, offset: u64, data: &[u8]) -> Result<()> {
        // 确保父目录存在
        if let Some(parent) = self.save_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await
                    .context(format!("创建父目录失败: {:?}", parent))?;
            }
        }
        
        // 如果文件不存在，创建新文件
        if !self.save_path.exists() {
            File::create(&self.save_path).await
                .context(format!("创建文件失败: {:?}", self.save_path))?;
        }
        
        // 以读写模式打开文件，允许追加
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&self.save_path)
            .await
            .context(format!("打开文件失败: {:?}, offset: {}", self.save_path, offset))?;
        
        // 获取当前文件大小
        let file_size = file.metadata().await
            .context("获取文件元数据失败")?
            .len();
        
        // 检查offset是否合理
        // offset应该 <= file_size，否则说明有分片间隙
        // 但这种情况在断点续传中可能发生（比如之前下载中断了）
        if offset > file_size {
            println!("警告: offset {} > 文件大小 {}，可能存在分片间隙，扩展文件", offset, file_size);
            // 这里不处理，seek会扩展文件
        }
        
        // 移动到指定位置
        file.seek(std::io::SeekFrom::Start(offset)).await
            .context("移动文件指针失败")?;
        
        // 验证当前位置是否正确
        let actual_pos = file.stream_position().await
            .context("获取当前位置失败")?;
        if actual_pos != offset {
            return Err(anyhow::anyhow!(
                "文件位置不匹配: 期望 {}，实际 {}", 
                offset, 
                actual_pos
            ));
        }
        
        // 写入数据
        file.write_all(data).await
            .context("写入文件失败")?;
        
        // 确保数据写入磁盘
        file.flush().await
            .context("刷新文件失败")?;
        
        // 验证写入后的文件大小
        let new_file_size = file.metadata().await
            .context("获取更新后的文件元数据失败")?
            .len();
        
        let expected_new_size = std::cmp::max(offset + data.len() as u64, file_size);
        if new_file_size < expected_new_size {
            println!("警告: 写入后文件大小 {} < 期望大小 {}", new_file_size, expected_new_size);
        }
        
        Ok(())
    }
    
    // 暂停下载
    pub async fn pause(&self) {
        *self.status.lock().await = DownloadStatus::Paused;
        println!("下载已暂停");
    }
    
    // 验证文件完整性 - 公开方法，可以在下载后调用
    pub async fn verify_file_integrity(&self) -> Result<bool> {
        println!("开始验证文件完整性: {}", self.file_name);
        
        // 检查文件是否存在
        if !self.save_path.exists() {
            return Err(anyhow::anyhow!("文件不存在: {:?}", self.save_path));
        }
        
        // 检查文件大小
        let file_size = fs::metadata(&self.save_path).await
            .context("获取文件元数据失败")?
            .len();
        
        if file_size != self.total_size {
            println!("文件大小不匹配: 期望 {} 字节，实际 {} 字节", self.total_size, file_size);
            return Ok(false);
        }
        
        println!("文件大小验证通过: {} 字节", file_size);
        
        // 计算文件哈希
        let hash = calculate_file_hash(&self.save_path).await?;
        println!("文件SHA256哈希: {}", hash);
        
        // TODO: 这里应该与服务器端的哈希对比
        // 暂时只返回大小校验结果
        
        Ok(true)
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