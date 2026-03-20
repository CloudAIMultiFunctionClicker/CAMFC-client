use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::core::{ConfigManager, AppResult, AppError};

/// 内存配置管理器
/// 
/// 先搞个简单的，后面再加持久化
pub struct MemoryConfigManager {
    config: RwLock<HashMap<String, serde_json::Value>>,
}

impl MemoryConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self {
            config: RwLock::new(HashMap::new()),
        }
    }
    
    /// 从JSON加载配置
    pub fn from_json(json: &str) -> AppResult<Self> {
        let config: HashMap<String, serde_json::Value> = serde_json::from_str(json)
            .map_err(|e| AppError::Configuration(format!("JSON解析失败: {}", e)))?;
            
        Ok(Self {
            config: RwLock::new(config),
        })
    }
    
    /// 导出为JSON
    pub fn to_json(&self) -> AppResult<String> {
        let config = self.config.read()
            .map_err(|e| AppError::Configuration(format!("读取配置锁失败: {}", e)))?;
            
        serde_json::to_string(&*config)
            .map_err(|e| AppError::Configuration(format!("JSON序列化失败: {}", e)))
    }
}

impl Default for MemoryConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigManager for MemoryConfigManager {
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> AppResult<Option<T>> {
        let config = self.config.read()
            .map_err(|e| AppError::Configuration(format!("读取配置锁失败: {}", e)))?;
            
        match config.get(key) {
            Some(value) => {
                let typed_value = serde_json::from_value(value.clone())
                    .map_err(|e| AppError::Configuration(format!("类型转换失败: {}", e)))?;
                Ok(Some(typed_value))
            }
            None => Ok(None),
        }
    }
    
    fn set<T: serde::Serialize>(&self, key: &str, value: &T) -> AppResult<()> {
        let json_value = serde_json::to_value(value)
            .map_err(|e| AppError::Configuration(format!("序列化失败: {}", e)))?;
            
        let mut config = self.config.write()
            .map_err(|e| AppError::Configuration(format!("写入配置锁失败: {}", e)))?;
            
        config.insert(key.to_string(), json_value);
        Ok(())
    }
    
    fn remove(&self, key: &str) -> AppResult<()> {
        let mut config = self.config.write()
            .map_err(|e| AppError::Configuration(format!("写入配置锁失败: {}", e)))?;
            
        config.remove(key);
        Ok(())
    }
    
    fn reload(&self) -> AppResult<()> {
        // 内存配置不需要重新加载
        Ok(())
    }
}

/// 文件配置管理器
/// 
/// 支持持久化的配置管理
pub struct FileConfigManager {
    inner: MemoryConfigManager,
    file_path: std::path::PathBuf,
}

impl FileConfigManager {
    /// 创建新的文件配置管理器
    pub fn new(file_path: std::path::PathBuf) -> AppResult<Self> {
        // 如果文件存在，加载配置
        let inner = if file_path.exists() {
            let content = std::fs::read_to_string(&file_path)
                .map_err(|e| AppError::Configuration(format!("读取配置文件失败: {}", e)))?;
                
            if content.trim().is_empty() {
                MemoryConfigManager::new()
            } else {
                MemoryConfigManager::from_json(&content)?
            }
        } else {
            MemoryConfigManager::new()
        };
        
        Ok(Self {
            inner,
            file_path,
        })
    }
    
    /// 保存到文件
    pub fn save_to_file(&self) -> AppResult<()> {
        let json = self.inner.to_json()?;
        
        // 确保父目录存在
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::Configuration(format!("创建配置目录失败: {}", e)))?;
        }
        
        std::fs::write(&self.file_path, json)
            .map_err(|e| AppError::Configuration(format!("写入配置文件失败: {}", e)))
    }
}

impl ConfigManager for FileConfigManager {
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> AppResult<Option<T>> {
        self.inner.get(key)
    }
    
    fn set<T: serde::Serialize>(&self, key: &str, value: &T) -> AppResult<()> {
        self.inner.set(key, value)?;
        self.save_to_file()?;
        Ok(())
    }
    
    fn remove(&self, key: &str) -> AppResult<()> {
        self.inner.remove(key)?;
        self.save_to_file()?;
        Ok(())
    }
    
    fn reload(&self) -> AppResult<()> {
        // 重新从文件加载
        let content = std::fs::read_to_string(&self.file_path)
            .map_err(|e| AppError::Configuration(format!("读取配置文件失败: {}", e)))?;
            
        let new_config = MemoryConfigManager::from_json(&content)?;
        
        // 替换内部配置
        *self.inner.config.write()
            .map_err(|e| AppError::Configuration(format!("写入配置锁失败: {}", e)))? = 
            new_config.config.read()
                .map_err(|e| AppError::Configuration(format!("读取配置锁失败: {}", e)))?
                .clone();
                
        Ok(())
    }
}