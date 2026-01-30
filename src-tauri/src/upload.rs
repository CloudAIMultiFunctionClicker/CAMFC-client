// 文件上传模块
// 负责分片上传文件到云盘后端，支持断点续传
//
// 思考：参考下载模块的结构，但上传更复杂一些
// 1. 前端提供文件，Rust负责分片和上传
// 2. 支持4MB分片（与后端API一致）
// 3. 支持断点续传，可以查询已上传分片
// 4. 提供上传进度信息

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use reqwest::{Client, multipart};

// 导入下载模块中的AuthInfo
use crate::download::AuthInfo;

// 基础URL - 和下载模块保持一致
const BASE_URL: &str = "http://localhost:8005";
// 默认分片大小 4MB - 和后端API保持一致
const CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4MB

// 上传状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UploadStatus {
    Pending,      // 等待开始
    Uploading,    // 上传中
    Paused,       // 已暂停
    Completed,    // 已完成
    Error(String), // 错误
}

// 上传进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadProgress {
    pub upload_id: String,         // 上传会话ID
    pub filename: String,          // 文件名
    pub total_size: u64,           // 总大小
    pub uploaded: u64,             // 已上传大小
    pub status: UploadStatus,      // 上传状态
    pub chunks_total: u32,         // 总分片数
    pub chunks_completed: u32,     // 已完成分片数
    pub speed_kbps: f64,           // 上传速度 KB/s
}

// 上传响应数据结构
#[derive(Debug, Deserialize)]
struct InitUploadResponse {
    upload_id: String,
    // 这里可能还有其他字段，根据后端API调整
}

#[derive(Debug, Deserialize)]
struct UploadStatusResponse {
    uploaded_chunks: Vec<u32>,
    // 可能还有其他状态信息
}

// 分片上传器
pub struct ChunkUploader {
    client: Client,
    auth_info: AuthInfo,
}

impl ChunkUploader {
    // 创建新的上传器
    pub fn new(auth_info: AuthInfo) -> Result<Self> {
        // 创建HTTP客户端，设置合适的超时时间
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("创建HTTP客户端失败")?;
            
        Ok(Self { client, auth_info })
    }
    
    // 初始化上传 - 调用 /upload/init
    // 后端不需要任何参数，只需要认证头
    pub async fn init_upload(&self, _filename: &str, _total_size: u64) -> Result<String> {
        let url = format!("{}/upload/init", BASE_URL);
        
        // 获取认证头
        let headers = self.auth_info.get_auth_header()?;
        
        // 发送POST请求，不需要body
        let response = self.client
            .post(&url)
            .headers(headers)
            .send()
            .await
            .context("初始化上传失败")?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "初始化上传失败: {} - {}", 
                status, 
                error_text
            ));
        }
        
        // 解析响应，获取 upload_id
        let response_data: InitUploadResponse = response
            .json()
            .await
            .context("解析初始化响应失败")?;
            
        println!("上传初始化成功，获取到 upload_id: {}", response_data.upload_id);
        Ok(response_data.upload_id)
    }
    
    // 上传单个分片 - 调用 /upload/chunk
    pub async fn upload_chunk(
        &self,
        upload_id: &str,
        chunk_index: u32,
        chunk_data: &[u8],
    ) -> Result<()> {
        let url = format!("{}/upload/chunk", BASE_URL);
        
        // 获取认证头
        let headers = self.auth_info.get_auth_header()?;
        
        // 构建multipart表单，只包含文件数据
        // upload_id 和 index 作为查询参数传递
        let form = multipart::Form::new()
            .part("file", multipart::Part::bytes(chunk_data.to_vec()).file_name(format!("chunk_{:04}", chunk_index)));
        
        // 发送请求，使用查询参数传递 upload_id 和 index
        let response = self.client
            .post(&url)
            .query(&[
                ("upload_id", upload_id),
                ("index", &chunk_index.to_string()),
            ])
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .context("上传分片失败")?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "上传分片 {} 失败: {} - {}", 
                chunk_index,
                status, 
                error_text
            ));
        }
        
        println!("分片 {} 上传成功", chunk_index);
        Ok(())
    }
    
    // 完成上传 - 调用 /upload/finish
    pub async fn finish_upload(
        &self,
        upload_id: &str,
        filename: &str,
        total_chunks: u32,
        target_path: Option<&str>,
    ) -> Result<String> {
        eprintln!("[finish_upload] 开始处理，upload_id={}, filename={}, total_chunks={}, target_path={:?}", 
                 upload_id, filename, total_chunks, target_path);
        
        let url = format!("{}/upload/finish", BASE_URL);
        
        // 获取认证头
        let headers = self.auth_info.get_auth_header()?;
        
        // 构建查询参数
        let total_chunks_str = total_chunks.to_string();
        let mut params = vec![
            ("upload_id", upload_id),
            ("filename", filename),
            ("total_chunks", &total_chunks_str),
        ];
        
        // 如果提供了目标路径，添加到参数中
        if let Some(path) = target_path {
            eprintln!("[finish_upload] 添加目标路径: {}", path);
            params.push(("target_path", path));
        }
        
        eprintln!("[finish_upload] 发送请求到: {}", url);
        eprintln!("[finish_upload] 参数: {:?}", params);
        
        // 发送POST请求
        let response = self.client
            .post(&url)
            .headers(headers)
            .query(&params)
            .send()
            .await
            .context("完成上传失败")?;
            
        eprintln!("[finish_upload] 收到响应状态: {:?}", response.status());
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "完成上传失败: {} - {}", 
                status, 
                error_text
            ));
        }
        
        // 解析响应，获取文件ID等信息
        let response_text = response.text().await.context("读取完成响应失败")?;
        eprintln!("[finish_upload] 上传完成响应: {}", response_text);
        
        Ok(format!("上传完成: {}", filename))
    }
    
    // 查询上传状态 - 调用 /upload/status/{upload_id}
    pub async fn get_upload_status(&self, upload_id: &str) -> Result<Vec<u32>> {
        let url = format!("{}/upload/status/{}", BASE_URL, upload_id);
        
        // 获取认证头
        let headers = self.auth_info.get_auth_header()?;
        
        // 发送GET请求
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .context("查询上传状态失败")?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "查询上传状态失败: {} - {}", 
                status, 
                error_text
            ));
        }
        
        // 解析响应，获取已上传分片列表
        let status_data: UploadStatusResponse = response
            .json()
            .await
            .context("解析上传状态失败")?;
            
        Ok(status_data.uploaded_chunks)
    }
}

// 上传任务管理器
pub struct UploadTask {
    upload_id: String,
    filename: String,
    file_path: PathBuf,
    total_size: u64,
    uploaded_size: Arc<AtomicU64>,
    status: Arc<Mutex<UploadStatus>>,
    uploader: ChunkUploader,
    chunks_total: u32,
    target_path: Option<String>,
}

impl UploadTask {
    // 创建新的上传任务
    pub async fn new(
        file_path: PathBuf,
        auth_info: AuthInfo,
        target_path: Option<&str>,
    ) -> Result<Self> {
        // 获取文件名
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("无法获取文件名")?
            .to_string();
        
        // 获取文件大小
        let total_size = fs::metadata(&file_path).await
            .context("获取文件大小失败")?
            .len();
        
        // 创建上传器
        let uploader = ChunkUploader::new(auth_info)?;
        
        // 初始化上传，获取upload_id
        let upload_id = uploader.init_upload(&filename, total_size).await?;
        
        // 计算总分片数
        let chunks_total = if total_size > 0 {
            ((total_size as f64) / (CHUNK_SIZE as f64)).ceil() as u32
        } else {
            1
        };
        
        println!("创建上传任务: {}, 大小: {} 字节, 分片数: {}", filename, total_size, chunks_total);
        
        Ok(Self {
            upload_id: upload_id.clone(),
            filename,
            file_path,
            total_size,
            uploaded_size: Arc::new(AtomicU64::new(0)),
            status: Arc::new(Mutex::new(UploadStatus::Pending)),
            uploader,
            chunks_total,
            target_path: target_path.map(|s| s.to_string()),
        })
    }
    
    // 开始上传（或恢复上传）
    pub async fn start(&self) -> Result<()> {
        // 更新状态为上传中
        *self.status.lock().await = UploadStatus::Uploading;
        
        println!("开始上传文件: {}, upload_id: {}", self.filename, self.upload_id);
        
        // 查询已上传分片，实现断点续传
        let uploaded_chunks = self.uploader.get_upload_status(&self.upload_id).await
            .unwrap_or_else(|_| vec![]); // 如果查询失败，当做没有已上传分片
        
        println!("已上传分片: {:?}", uploaded_chunks);
        
        // 打开文件
        let mut file = File::open(&self.file_path).await
            .context("打开文件失败")?;
        
        // 计算已上传大小
        let mut already_uploaded = 0u64;
        for &chunk_index in &uploaded_chunks {
            let chunk_start = (chunk_index as u64) * CHUNK_SIZE;
            let chunk_end = if chunk_index == self.chunks_total - 1 {
                self.total_size - 1
            } else {
                chunk_start + CHUNK_SIZE - 1
            };
            already_uploaded += chunk_end - chunk_start + 1;
        }
        
        // 更新已上传大小
        self.uploaded_size.store(already_uploaded, Ordering::SeqCst);
        
        println!("已上传大小: {} 字节", already_uploaded);
        
        // 分片上传
        for chunk_index in 0..self.chunks_total {
            // 跳过已上传的分片
            if uploaded_chunks.contains(&chunk_index) {
                println!("分片 {} 已上传，跳过", chunk_index);
                continue;
            }
            
            // 检查状态，如果暂停了就退出循环
            {
                let status = self.status.lock().await;
                match *status {
                    UploadStatus::Paused => {
                        println!("上传已暂停");
                        return Ok(());
                    }
                    UploadStatus::Error(_) => {
                        // 如果已经有错误，直接返回
                        return Ok(());
                    }
                    _ => {}
                }
            }
            
            // 计算分片范围
            let start = (chunk_index as u64) * CHUNK_SIZE;
            let end = if chunk_index == self.chunks_total - 1 {
                self.total_size - 1
            } else {
                start + CHUNK_SIZE - 1
            };
            
            let chunk_size = (end - start + 1) as usize;
            
            // 读取分片数据
            file.seek(std::io::SeekFrom::Start(start)).await
                .context("移动文件指针失败")?;
            
            let mut chunk_data = vec![0u8; chunk_size];
            let bytes_read = file.read_exact(&mut chunk_data).await
                .context("读取分片数据失败")?;
            
            if bytes_read != chunk_size {
                return Err(anyhow::anyhow!(
                    "读取分片数据大小不匹配: 期望 {}, 实际 {}", 
                    chunk_size, 
                    bytes_read
                ));
            }
            
            // 分片重试机制
            let mut last_error = None;
            for retry_count in 0..3 { // 最多重试3次
                match self.uploader.upload_chunk(
                    &self.upload_id,
                    chunk_index,
                    &chunk_data,
                ).await {
                    Ok(_) => {
                        // 更新进度
                        eprintln!("[start] 分片 {} 上传成功，准备更新进度", chunk_index);
                        self.uploaded_size.fetch_add(chunk_size as u64, Ordering::SeqCst);
                        eprintln!("[start] 获得锁，更新进度");
                        
                        let current_uploaded = self.uploaded_size.load(Ordering::SeqCst);
                        eprintln!("[start] 分片 {}/{} 上传成功 ({}/{} 字节)，当前进度: {}/{} 字节", 
                            chunk_index + 1, 
                            self.chunks_total,
                            chunk_size,
                            chunk_size,
                            current_uploaded,
                            self.total_size
                        );
                        
                        last_error = None;
                        break; // 成功，跳出重试循环
                    }
                    Err(e) => {
                        println!("上传分片 {} 失败: {}, 重试 {}/3", chunk_index, e, retry_count + 1);
                        last_error = Some(e);
                        // 等待一下再重试
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
            
            // 检查重试后是否还有错误
            if let Some(e) = last_error {
                *self.status.lock().await = UploadStatus::Error(format!("分片 {} 上传失败: {}", chunk_index, e));
                return Err(anyhow::anyhow!("分片 {} 上传失败: {}", chunk_index, e));
            }
        }
        
        // 所有分片上传完成，调用完成接口
        eprintln!("[start] 所有分片上传完成，共 {} 个分片，准备调用 finish_upload", self.chunks_total);
        
        match self.uploader.finish_upload(&self.upload_id, &self.filename, self.chunks_total, self.target_path.as_deref()).await {
            Ok(result) => {
                eprintln!("[start] 上传完成: {}", result);
                *self.status.lock().await = UploadStatus::Completed;
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("[start] 完成上传失败: {}", e);
                eprintln!("错误: {}", error_msg);
                *self.status.lock().await = UploadStatus::Error(error_msg.clone());
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }
    
    // 暂停上传
    pub async fn pause(&self) {
        *self.status.lock().await = UploadStatus::Paused;
        println!("上传已暂停");
    }
    
    // 获取上传进度
    pub async fn get_progress(&self) -> UploadProgress {
        let uploaded = self.uploaded_size.load(Ordering::SeqCst);
        let status = self.status.lock().await.clone();
        
        // 简单计算速度（暂时用0，后续可以添加时间计算）
        let speed_kbps = 0.0;
        
        UploadProgress {
            upload_id: self.upload_id.clone(),
            filename: self.filename.clone(),
            total_size: self.total_size,
            uploaded,
            status,
            chunks_total: self.chunks_total,
            chunks_completed: if self.total_size > 0 {
                ((uploaded as f64) / (self.total_size as f64) * (self.chunks_total as f64)) as u32
            } else {
                0
            },
            speed_kbps,
        }
    }
}