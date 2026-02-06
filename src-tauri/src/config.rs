// 配置模块
// 负责管理后端域名和端口的配置
//
// 优先级：
// 1. 环境变量 CAMFC_BASE 和 CAMFC_PORT
// 2. 远程配置 https://me.011420.xyz/api/camfc/data.json
// 3. 默认值 http://localhost:8005

use std::sync::OnceLock;
use serde::Deserialize;
use anyhow::{Result, Context};

// 远程配置响应结构
#[derive(Debug, Deserialize)]
struct RemoteConfig {
    base_url: Vec<String>,
}

// 后端配置
#[derive(Debug, Clone)]
pub struct BackendConfig {
    pub base_url: String,
    pub port: u16,
}

impl BackendConfig {
    // 获取完整的 URL（包含端口）
    pub fn get_full_url(&self) -> String {
        format!("{}:{}", self.base_url, self.port)
    }
}

// 全局配置实例
static BACKEND_CONFIG: OnceLock<BackendConfig> = OnceLock::new();

// 初始化配置
pub async fn init_config() -> Result<()> {
    println!("开始初始化后端配置...");
    
    // 1. 先尝试从环境变量读取
    if let Some(config) = try_load_from_env() {
        println!("从环境变量加载配置: {}", config.get_full_url());
        BACKEND_CONFIG.set(config)
            .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
        return Ok(());
    }
    
    // 2. 环境变量不存在，尝试从远程 API 获取
    println!("环境变量未配置，尝试从远程 API 获取配置...");
    match try_load_from_remote().await {
        Ok(config) => {
            println!("从远程 API 加载配置: {}", config.get_full_url());
            BACKEND_CONFIG.set(config)
                .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
            Ok(())
        }
        Err(e) => {
            println!("远程配置加载失败: {}，使用默认配置", e);
            // 3. 远程获取失败，使用默认配置
            let default_config = BackendConfig {
                base_url: "http://localhost".to_string(),
                port: 8005,
            };
            println!("使用默认配置: {}", default_config.get_full_url());
            BACKEND_CONFIG.set(default_config)
                .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
            Ok(())
        }
    }
}

// 尝试从环境变量加载配置
fn try_load_from_env() -> Option<BackendConfig> {
    let base_url = std::env::var("CAMFC_BASE").ok()?;
    let port_str = std::env::var("CAMFC_PORT").ok()?;
    
    let port = port_str.parse::<u16>().ok()?;
    
    Some(BackendConfig {
        base_url,
        port,
    })
}

// 尝试从远程 API 加载配置
async fn try_load_from_remote() -> Result<BackendConfig> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .context("创建HTTP客户端失败")?;
    
    let url = "https://me.011420.xyz/api/camfc/data.json";
    println!("请求远程配置: {}", url);
    
    let response = client
        .get(url)
        .send()
        .await
        .context("请求远程配置失败")?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "远程配置请求失败: {} - {}", 
            status, 
            error_text
        ));
    }
    
    let remote_config: RemoteConfig = response
        .json()
        .await
        .context("解析远程配置失败")?;
    
    println!("远程配置解析成功，收到 {} 个候选地址", remote_config.base_url.len());
    
    // 依次检测每个候选地址的可用性
    for (index, candidate) in remote_config.base_url.iter().enumerate() {
        println!("检测候选地址 [{}/{}]: {}", index + 1, remote_config.base_url.len(), candidate);
        
        if check_backend_available(&client, candidate).await {
            println!("候选地址可用: {}", candidate);
            
            // 解析 base_url 和 port
            let (base_url, port) = parse_backend_url(candidate)?;
            
            return Ok(BackendConfig {
                base_url,
                port,
            });
        } else {
            println!("候选地址不可用: {}", candidate);
        }
    }
    
    Err(anyhow::anyhow!("所有候选地址都不可用"))
}

// 解析后端 URL，返回 (base_url, port)
fn parse_backend_url(url: &str) -> Result<(String, u16)> {
    let url = url.trim();
    
    // 如果包含端口，则分离
    if let Some((host, port_str)) = url.split_once(':') {
        let port = port_str.parse::<u16>()
            .context(format!("无效的端口号: {}", port_str))?;
        
        // 检查是否已经有协议前缀
        let base_url = if host.starts_with("http://") || host.starts_with("https://") {
            host.to_string()
        } else {
            format!("http://{}", host)
        };
        
        Ok((base_url, port))
    } else {
        // 没有端口，默认使用 8005
        let base_url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("http://{}", url)
        };
        
        Ok((base_url, 8005))
    }
}

// 检测后端是否可用
async fn check_backend_available(client: &reqwest::Client, backend_url: &str) -> bool {
    let (base_url, port) = match parse_backend_url(backend_url) {
        Ok(result) => result,
        Err(e) => {
            println!("解析后端 URL 失败: {} - {}", backend_url, e);
            return false;
        }
    };
    
    let test_url = format!("{}:{}/test", base_url, port);
    println!("检测后端可用性: {}", test_url);
    
    match client
        .get(&test_url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                // 尝试解析响应为 JSON
                match response.text().await {
                    Ok(text) => {
                        // 检查是否是合法的 JSON
                        if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                            println!("后端可用，返回合法 JSON: {}", text);
                            true
                        } else {
                            println!("后端响应不是合法 JSON: {}", text);
                            false
                        }
                    }
                    Err(e) => {
                        println!("读取响应失败: {}", e);
                        false
                    }
                }
            } else {
                println!("后端返回错误状态: {}", response.status());
                false
            }
        }
        Err(e) => {
            println!("检测后端失败: {}", e);
            false
        }
    }
}

// 获取后端配置（必须在 init_config 之后调用）
pub fn get_backend_config() -> Result<&'static BackendConfig> {
    BACKEND_CONFIG.get()
        .ok_or_else(|| anyhow::anyhow!("后端配置未初始化，请先调用 init_config"))
}

// 获取完整的后端 URL（便捷函数）
pub fn get_backend_url() -> Result<String> {
    Ok(get_backend_config()?.get_full_url())
}
