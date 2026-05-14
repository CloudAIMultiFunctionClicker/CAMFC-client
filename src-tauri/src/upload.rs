

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

use crate::download::AuthInfo;

use crate::config;
use crate::activity_log::{ActivityLogManager, ActivityType};

const CHUNK_SIZE: u64 = 256 * 1024;

fn get_base_url() -> Result<String> {
    config::get_backend_url()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UploadStatus {
    Pending,
    Uploading,
    Paused,
    Completed,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadProgress {
    pub upload_id: String,
    pub filename: String,
    pub total_size: u64,
    pub uploaded: u64,
    pub status: UploadStatus,
    pub chunks_total: u32,
    pub chunks_completed: u32,
    pub speed_kbps: f64,
}

#[derive(Debug, Deserialize)]
struct InitUploadResponse {
    upload_id: String,

}

#[derive(Debug, Deserialize)]
struct UploadStatusResponse {
    uploaded_chunks: Vec<u32>,

}

pub struct ChunkUploader {
    client: Client,
    auth_info: AuthInfo,
}

impl ChunkUploader {

    pub fn new(auth_info: AuthInfo) -> Result<Self> {

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("创建HTTP客户端失败")?;

        Ok(Self { client, auth_info })
    }

    pub async fn init_upload(&self, _filename: &str, _total_size: u64) -> Result<String> {
        let base_url = get_base_url()?;
        let url = format!("{}/upload/init", base_url);

        let headers = self.auth_info.get_auth_header()?;

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

        let response_data: InitUploadResponse = response
            .json()
            .await
            .context("解析初始化响应失败")?;

        tracing::info!("上传初始化成功，获取到 upload_id: {}", response_data.upload_id);
        Ok(response_data.upload_id)
    }

    pub async fn upload_chunk(
        &self,
        upload_id: &str,
        chunk_index: u32,
        chunk_data: &[u8],
    ) -> Result<()> {
        let base_url = get_base_url()?;
        let url = format!("{}/upload/chunk", base_url);

        let headers = self.auth_info.get_auth_header()?;

        let form = multipart::Form::new()
            .part("file", multipart::Part::bytes(chunk_data.to_vec()).file_name(format!("chunk_{:04}", chunk_index)));

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

        tracing::info!("分片 {} 上传成功", chunk_index);
        Ok(())
    }

    pub async fn finish_upload(
        &self,
        upload_id: &str,
        filename: &str,
        total_chunks: u32,
        target_path: Option<&str>,
    ) -> Result<String> {
         tracing::error!("[finish_upload] 开始处理，upload_id={}, filename={}, total_chunks={}, target_path={:?}",
                 upload_id, filename, total_chunks, target_path);

        let base_url = get_base_url()?;
        let url = format!("{}/upload/finish", base_url);

        let headers = self.auth_info.get_auth_header()?;

        let total_chunks_str = total_chunks.to_string();
        let mut params = vec![
            ("upload_id", upload_id),
            ("filename", filename),
            ("total_chunks", &total_chunks_str),
        ];

        if let Some(path) = target_path {
             tracing::error!("[finish_upload] 添加目标路径: {}", path);
            params.push(("target_path", path));
        }

         tracing::error!("[finish_upload] 发送请求到: {}", url);
         tracing::error!("[finish_upload] 参数: {:?}", params);

        let response = self.client
            .post(&url)
            .headers(headers)
            .query(&params)
            .send()
            .await
            .context("完成上传失败")?;

         tracing::error!("[finish_upload] 收到响应状态: {:?}", response.status());

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "完成上传失败: {} - {}",
                status,
                error_text
            ));
        }

        let response_text = response.text().await.context("读取完成响应失败")?;
         tracing::error!("[finish_upload] 上传完成响应: {}", response_text);

        Ok(format!("上传完成: {}", filename))
    }

    pub async fn get_upload_status(&self, upload_id: &str) -> Result<Vec<u32>> {
        let base_url = get_base_url()?;
        let url = format!("{}/upload/status/{}", base_url, upload_id);

        let headers = self.auth_info.get_auth_header()?;

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

        let status_data: UploadStatusResponse = response
            .json()
            .await
            .context("解析上传状态失败")?;

        Ok(status_data.uploaded_chunks)
    }
}

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
    user_uuid: Option<String>,
}

impl UploadTask {

    pub async fn new(
        file_path: PathBuf,
        auth_info: AuthInfo,
        target_path: Option<&str>,
        user_uuid: Option<String>,
    ) -> Result<Self> {

        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("无法获取文件名")?
            .to_string();

        let total_size = fs::metadata(&file_path).await
            .context("获取文件大小失败")?
            .len();

        let uploader = ChunkUploader::new(auth_info)?;

        let upload_id = uploader.init_upload(&filename, total_size).await?;

        let chunks_total = if total_size > 0 {
            ((total_size as f64) / (CHUNK_SIZE as f64)).ceil() as u32
        } else {
            1
        };

        tracing::info!("创建上传任务: {}, 大小: {} 字节, 分片数: {}", filename, total_size, chunks_total);

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
            user_uuid,
        })
    }

    pub async fn start(&self) -> Result<()> {

        {
            let status = self.status.lock().await;
            if matches!(*status, UploadStatus::Uploading) {
                tracing::info!("上传任务已在进行中，跳过重复启动");
                return Ok(());
            }
        }

        *self.status.lock().await = UploadStatus::Uploading;

        tracing::info!("开始上传文件: {}, upload_id: {}", self.filename, self.upload_id);

        let uploaded_chunks = self.uploader.get_upload_status(&self.upload_id).await
            .unwrap_or_else(|_| vec![]);

        tracing::info!("已上传分片: {:?}", uploaded_chunks);

        let mut file = File::open(&self.file_path).await
            .context("打开文件失败")?;

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

        self.uploaded_size.store(already_uploaded, Ordering::SeqCst);

        tracing::info!("已上传大小: {} 字节", already_uploaded);

        for chunk_index in 0..self.chunks_total {

            if uploaded_chunks.contains(&chunk_index) {
                tracing::info!("分片 {} 已上传，跳过", chunk_index);
                continue;
            }

            {
                let status = self.status.lock().await;
                match *status {
                    UploadStatus::Paused => {
                        tracing::info!("上传已暂停，当前分片: {}", chunk_index);
                        return Ok(());
                    }
                    UploadStatus::Error(_) => {

                        return Ok(());
                    }
                    _ => {}
                }
            }

            let start = (chunk_index as u64) * CHUNK_SIZE;
            let end = if chunk_index == self.chunks_total - 1 {
                self.total_size - 1
            } else {
                start + CHUNK_SIZE - 1
            };

            let chunk_size = (end - start + 1) as usize;

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

            {
                let status = self.status.lock().await;
                if matches!(*status, UploadStatus::Paused) {
                    tracing::info!("上传已暂停（读取数据后），当前分片: {}", chunk_index);
                    return Ok(());
                }
            }

            let mut last_error = None;
            for retry_count in 0..3 {
                match self.uploader.upload_chunk(
                    &self.upload_id,
                    chunk_index,
                    &chunk_data,
                ).await {
                    Ok(_) => {

                        self.uploaded_size.fetch_add(chunk_size as u64, Ordering::SeqCst);

                        let current_uploaded = self.uploaded_size.load(Ordering::SeqCst);
                        tracing::info!("分片 {}/{} 上传成功，当前进度: {}/{} 字节",
                            chunk_index + 1,
                            self.chunks_total,
                            current_uploaded,
                            self.total_size
                        );

                        last_error = None;
                        break;
                    }
                    Err(e) => {
                        tracing::info!("上传分片 {} 失败: {}, 重试 {}/3", chunk_index, e, retry_count + 1);
                        last_error = Some(e);

                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }

            if let Some(e) = last_error {
                *self.status.lock().await = UploadStatus::Error(format!("分片 {} 上传失败: {}", chunk_index, e));
                return Err(anyhow::anyhow!("分片 {} 上传失败: {}", chunk_index, e));
            }
        }

        tracing::info!("所有分片上传完成，共 {} 个分片，准备调用 finish_upload", self.chunks_total);

        match self.uploader.finish_upload(&self.upload_id, &self.filename, self.chunks_total, self.target_path.as_deref()).await {
            Ok(result) => {
                tracing::info!("上传完成: {}", result);
                *self.status.lock().await = UploadStatus::Completed;

                if let Some(ref user_uuid) = self.user_uuid {
                    if let Err(e) = record_upload_activity_for_user(user_uuid, &self.filename, self.total_size).await {
                        tracing::warn!("记录上传活动失败: {}", e);
                    } else {
                        tracing::info!("已记录上传活动: {}", self.filename);
                    }
                }

                Ok(())
            }
            Err(e) => {
                let error_msg = format!("完成上传失败: {}", e);
                tracing::error!("错误: {}", error_msg);
                *self.status.lock().await = UploadStatus::Error(error_msg.clone());
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }

    pub async fn pause(&self) {
        *self.status.lock().await = UploadStatus::Paused;
        tracing::info!("上传已暂停");
    }

    pub async fn get_progress(&self) -> UploadProgress {
        let uploaded = self.uploaded_size.load(Ordering::SeqCst);
        let status = self.status.lock().await.clone();

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

pub async fn record_upload_activity_for_user(user_uuid: &str, file_path: &str, file_size: u64) -> Result<()> {
    let manager = ActivityLogManager::new(user_uuid.to_string());
    manager.add_activity(ActivityType::Upload, file_path, file_size).await
        .map_err(|e| anyhow::anyhow!("记录上传活动失败: {}", e))
}