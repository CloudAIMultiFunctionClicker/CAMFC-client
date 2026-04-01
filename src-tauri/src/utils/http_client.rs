//! HTTP 客户端工厂模块
//!
//! 提供统一的 HTTP 客户端创建功能，避免重复创建客户端实例
//! 所有 HTTP 请求应该复用同一个客户端实例
//!
//! 使用示例：
//! ```rust
//! let client = create_http_client()?;
//! ```

use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

/// 创建 HTTP 客户端
///
/// 创建具有合理超时设置的 HTTP 客户端实例
/// 建议在整个应用生命周期中复用同一个客户端
///
/// # 返回值
/// 返回配置好的 reqwest::Client 实例
///
/// # 配置说明
/// - 超时时间：30 秒
/// - 连接超时：10 秒
/// - 使用 Rustls TLS（跨平台兼容）
pub fn create_http_client() -> Result<Client> {
    // 创建 HTTP 客户端构建器
    let client = Client::builder()
        // 设置请求超时时间（整个请求的最大时间）
        .timeout(Duration::from_secs(30))
        // 设置连接超时时间
        .connect_timeout(Duration::from_secs(10))
        // 使用 Rustls TLS（不依赖系统 OpenSSL）
        .use_rustls_tls()
        // 构建客户端
        .build()?;
    
    Ok(client)
}

/// 创建带有自定义超时的 HTTP 客户端
///
/// 当需要不同的超时设置时使用此函数
///
/// # 参数
/// * `timeout_secs` - 超时时间（秒）
///
/// # 返回值
/// 返回配置好的 reqwest::Client 实例
pub fn create_http_client_with_timeout(timeout_secs: u64) -> Result<Client> {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .connect_timeout(Duration::from_secs(10))
        .use_rustls_tls()
        .build()?;
    
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_http_client() {
        // 测试正常创建 HTTP 客户端
        let client = create_http_client();
        assert!(client.is_ok(), "HTTP 客户端创建失败");
    }
    
    #[test]
    fn test_create_http_client_with_timeout() {
        // 测试创建自定义超时的客户端
        let client = create_http_client_with_timeout(60);
        assert!(client.is_ok(), "自定义超时客户端创建失败");
    }
}
