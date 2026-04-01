//! 认证工具模块
//!
//! 提供统一的认证头创建功能，避免在各模块重复实现
//!
//! 使用示例：
//! ```rust
//! let headers = create_auth_headers(&device_id, &totp)?;
//! ```

use anyhow::Result;
use reqwest::header;

/// 创建认证头信息
///
/// 将设备 ID 和 TOTP 组合成 JSON 格式的认证头
///
/// # 参数
/// * `device_id` - 设备唯一标识
/// * `totp` - 动态验证码
///
/// # 返回值
/// 返回包含认证信息的 HeaderMap，可直接用于 HTTP 请求
///
/// # 错误
/// 返回错误当 JSON 序列化或 Header 创建失败时
pub fn create_auth_headers(device_id: &str, totp: &str) -> Result<header::HeaderMap> {
    // 构造认证 JSON：{"Id": "xxx", "Totp": "xxx"}
    let auth_json = serde_json::json!({
        "Id": device_id,
        "Totp": totp
    }).to_string();
    
    // 创建 HeaderMap 并设置 Authorization 头
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&auth_json)?
    );
    
    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_auth_headers() {
        // 测试正常创建认证头
        let headers = create_auth_headers("test-device-id", "123456").unwrap();
        
        // 验证 Authorization 头存在
        assert!(headers.contains_key(header::AUTHORIZATION));
        
        // 验证认证头内容
        let auth_value = headers.get(header::AUTHORIZATION).unwrap();
        let auth_str = auth_value.to_str().unwrap();
        assert!(auth_str.contains("test-device-id"));
        assert!(auth_str.contains("123456"));
    }
    
    #[test]
    fn test_create_auth_headers_with_special_chars() {
        // 测试包含特殊字符的情况
        let headers = create_auth_headers("device-with-special-chars", "654321").unwrap();
        assert!(headers.contains_key(header::AUTHORIZATION));
    }
}
