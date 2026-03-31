// 网络工具模块
// 包含网络相关的常量和通用功能

use reqwest::{header, Client};

/// 默认分片大小（256KB）
pub const CHUNK_SIZE: u64 = 256 * 1024;

/// 网络请求超时时间（秒）
pub const REQUEST_TIMEOUT_SECONDS: u64 = 30;

/// 创建HTTP客户端
///
/// 创建一个带有默认配置的HTTP客户端
/// - 超时时间：30秒
/// - 自动重定向：最多10次
/// - 连接池：默认配置
pub fn create_http_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECONDS))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .expect("创建HTTP客户端失败")
}

/// 创建带有认证头的请求构建器
///
/// 为请求添加认证信息
///
/// 参数：
/// - client: HTTP客户端
/// - device_id: 设备ID
/// - totp: TOTP值
/// - url: 请求URL
///
/// 返回：带有认证头的请求构建器
pub fn create_authenticated_request(
    client: &Client,
    device_id: &str,
    totp: &str,
    url: &str,
) -> reqwest::RequestBuilder {
    client
        .get(url)
        .header(
            header::AUTHORIZATION,
            format!("Bearer {}:{}", device_id, totp),
        )
        .header(header::USER_AGENT, "CAMFC-Client/1.0")
}
