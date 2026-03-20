use async_trait::async_trait;
use reqwest::Client;
use serde_json;

use crate::core::{FileService, AppResult, AppError, FileInfo, Pagination};

/// 文件服务实现
/// 
/// 简单的文件服务，支持基本的文件操作
pub struct FileServiceImpl {
    client: Client,
    base_url: String,
}

impl FileServiceImpl {
    /// 创建文件服务
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    /// 构建完整URL
    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
    
    /// 添加认证头
    fn add_auth_headers(&self, request: reqwest::RequestBuilder, auth_info: &crate::core::AuthInfo) -> reqwest::RequestBuilder {
        request
            .header("Id", &auth_info.device_id)
            .header("Totp", &auth_info.totp)
    }
}

#[async_trait]
impl FileService for FileServiceImpl {
    async fn list_files(&self, path: &str, pagination: &Pagination) -> AppResult<Vec<FileInfo>> {
        let url = self.build_url("/files");
        
        let response = self.client.get(&url)
            .query(&["path", path])
            .query(&["page", &pagination.page.to_string()])
            .query(&["page_size", &pagination.page_size.to_string()])
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "获取文件列表失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        let files: Vec<FileInfo> = response.json()
            .await
            .map_err(|e| AppError::Network(format!("解析响应失败: {}", e)))?;
        
        Ok(files)
    }
    
    async fn create_directory(&self, path: &str) -> AppResult<()> {
        let url = self.build_url("/files/directories");
        
        let payload = serde_json::json!({"path": path});
        
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "创建目录失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        Ok(())
    }
    
    async fn delete(&self, path: &str) -> AppResult<()> {
        let url = self.build_url(&format!("/files/{}", urlencoding::encode(path)));
        
        let response = self.client.delete(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "删除失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        Ok(())
    }
    
    async fn rename(&self, old_path: &str, new_path: &str) -> AppResult<()> {
        let url = self.build_url("/files/rename");
        
        let payload = serde_json::json!({
            "old_path": old_path,
            "new_path": new_path
        });
        
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "重命名失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        Ok(())
    }
    
    async fn move_item(&self, from_path: &str, to_path: &str) -> AppResult<()> {
        let url = self.build_url("/files/move");
        
        let payload = serde_json::json!({
            "from_path": from_path,
            "to_path": to_path
        });
        
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "移动失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        Ok(())
    }
    
    async fn copy(&self, from_path: &str, to_path: &str) -> AppResult<()> {
        let url = self.build_url("/files/copy");
        
        let payload = serde_json::json!({
            "from_path": from_path,
            "to_path": to_path
        });
        
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "复制失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        Ok(())
    }
    
    async fn download(&self, path: &str) -> AppResult<Vec<u8>> {
        let url = self.build_url(&format!("/files/download/{}", urlencoding::encode(path)));
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "下载失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        let data = response.bytes()
            .await
            .map_err(|e| AppError::Network(format!("读取数据失败: {}", e)))?
            .to_vec();
        
        Ok(data)
    }
    
    async fn upload(&self, path: &str, data: &[u8]) -> AppResult<()> {
        let url = self.build_url(&format!("/files/upload/{}", urlencoding::encode(path)));
        
        let response = self.client.post(&url)
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "上传失败".to_string());
            return Err(AppError::FileOperation(error_msg));
        }
        
        Ok(())
    }
}