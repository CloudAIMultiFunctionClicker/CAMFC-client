use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

use crate::core::{AuthService, AppResult, AppError, AuthInfo};

/// 认证服务实现
/// 
/// 简单的认证服务，支持基本的登录登出功能
pub struct AuthServiceImpl {
    client: Client,
    base_url: String,
}

impl AuthServiceImpl {
    /// 创建认证服务
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
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, username: &str, password: &str) -> AppResult<String> {
        let url = self.build_url("/auth/login");
        
        let payload = serde_json::json!({
            "username": username,
            "password": password
        });
        
        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            let error_msg = response.text()
                .await
                .unwrap_or_else(|_| "登录失败".to_string());
            return Err(AppError::Authentication(error_msg));
        }
        
        let result: serde_json::Value = response.json()
            .await
            .map_err(|e| AppError::Network(format!("解析响应失败: {}", e)))?;
        
        let token = result.get("token")
            .and_then(|t| t.as_str())
            .ok_or_else(|| AppError::Authentication("未返回token".to_string()))?
            .to_string();
        
        Ok(token)
    }
    
    async fn logout(&self) -> AppResult<()> {
        // 这里可以实现登出逻辑，比如调用后端API
        // 暂时先返回成功
        Ok(())
    }
    
    async fn validate_token(&self, token: &str) -> AppResult<bool> {
        // 这里可以实现token验证逻辑
        // 暂时先返回true
        Ok(true)
    }
    
    async fn refresh_token(&self, old_token: &str) -> AppResult<String> {
        // 这里可以实现token刷新逻辑
        // 暂时返回原token
        Ok(old_token.to_string())
    }
    
    async fn get_user_info(&self, token: &str) -> AppResult<Value> {
        // 这里可以实现获取用户信息逻辑
        // 暂时返回空对象
        Ok(serde_json::json!({}))
    }
}