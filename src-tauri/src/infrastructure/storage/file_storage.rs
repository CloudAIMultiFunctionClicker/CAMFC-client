use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::core::{Storage, AppResult, AppError};

/// 文件系统存储实现
/// 
/// 简单的文件存储，够用就行
pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    /// 创建新的文件存储
    pub fn new(base_path: PathBuf) -> AppResult<Self> {
        // 确保目录存在
        std::fs::create_dir_all(&base_path)
            .map_err(|e| AppError::Storage(format!("创建存储目录失败: {}", e)))?;
            
        Ok(Self { base_path })
    }
    
    /// 获取完整路径
    fn get_full_path(&self, key: &str) -> PathBuf {
        // 简单的路径拼接，后面考虑更安全的处理
        self.base_path.join(key)
    }
    
    /// 确保父目录存在
    async fn ensure_parent_dir(&self, path: &PathBuf) -> AppResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Storage(format!("创建父目录失败: {}", e)))?;
        }
        Ok(())
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn save(&self, key: &str, data: &[u8]) -> AppResult<()> {
        let path = self.get_full_path(key);
        self.ensure_parent_dir(&path).await?;
        
        let mut file = fs::File::create(&path)
            .await
            .map_err(|e| AppError::Storage(format!("创建文件失败: {}", e)))?;
            
        file.write_all(data)
            .await
            .map_err(|e| AppError::Storage(format!("写入数据失败: {}", e)))?;
            
        file.flush()
            .await
            .map_err(|e| AppError::Storage(format!("刷新文件失败: {}", e)))?;
            
        Ok(())
    }
    
    async fn load(&self, key: &str) -> AppResult<Option<Vec<u8>>> {
        let path = self.get_full_path(key);
        
        if !path.exists() {
            return Ok(None);
        }
        
        let data = fs::read(&path)
            .await
            .map_err(|e| AppError::Storage(format!("读取文件失败: {}", e)))?;
            
        Ok(Some(data))
    }
    
    async fn delete(&self, key: &str) -> AppResult<()> {
        let path = self.get_full_path(key);
        
        if path.exists() {
            fs::remove_file(&path)
                .await
                .map_err(|e| AppError::Storage(format!("删除文件失败: {}", e)))?;
        }
        
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> AppResult<bool> {
        let path = self.get_full_path(key);
        Ok(path.exists())
    }
    
    async fn list_keys(&self, pattern: &str) -> AppResult<Vec<String>> {
        // 简单的实现，后面考虑用glob模式
        let mut keys = Vec::new();
        
        // 这里应该递归遍历目录，但先简单处理
        // TODO: 实现完整的目录遍历
        
        Ok(keys)
    }
}

/// JSON存储封装
/// 
/// 在文件存储基础上加一层JSON序列化
pub struct JsonStorage<T: Storage> {
    inner: T,
}

impl<T: Storage> JsonStorage<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    
    /// 保存JSON数据
    pub async fn save_json<V: Serialize>(&self, key: &str, value: &V) -> AppResult<()> {
        let data = serde_json::to_vec(value)
            .map_err(|e| AppError::Storage(format!("JSON序列化失败: {}", e)))?;
            
        self.inner.save(key, &data).await
    }
    
    /// 加载JSON数据
    pub async fn load_json<V: for<'de> Deserialize<'de>>(&self, key: &str) -> AppResult<Option<V>> {
        match self.inner.load(key).await? {
            Some(data) => {
                let value = serde_json::from_slice(&data)
                    .map_err(|e| AppError::Storage(format!("JSON反序列化失败: {}", e)))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}