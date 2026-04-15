// 保留所有权利
//
// Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
// Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
//
// Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
// Email: abc.cxh2009@foxmail.com
//
// Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
// Email: 1220594170@qq.com
//
// Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
// Email: admin@mc666.top

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
    tracing::info!("开始初始化后端配置...");
    
    // 1. 先尝试从环境变量读取
    if let Some(config) = try_load_from_env().await {
        tracing::info!("从环境变量加载配置: {}", config.get_full_url());
        
        // 检测环境变量指定的服务器是否可用
        tracing::info!("检测环境变量指定的服务器是否可用...");
        if check_env_backend_available(&config).await {
            tracing::info!("环境变量指定的服务器可用");
            BACKEND_CONFIG.set(config)
                .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
            return Ok(());
        } else {
            tracing::info!("环境变量指定的服务器不可用，继续尝试其他配置源...");
        }
    }
    
    // 2. 环境变量不存在或不可用，尝试从远程 API 获取
    tracing::info!("尝试从远程 API 获取配置...");
    match try_load_from_remote().await {
        Ok(config) => {
            tracing::info!("从远程 API 加载配置: {}", config.get_full_url());
            BACKEND_CONFIG.set(config)
                .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
            Ok(())
        }
        Err(e) => {
            tracing::info!("远程配置加载失败: {}，使用默认配置", e);
            // 3. 远程获取失败，使用默认配置
            let default_config = BackendConfig {
                base_url: "http://localhost".to_string(),
                port: 7548,
            };
            tracing::info!("使用默认配置: {}", default_config.get_full_url());
            BACKEND_CONFIG.set(default_config)
                .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
            Ok(())
        }
    }
}

// 检测环境变量指定的服务器是否可用
async fn check_env_backend_available(config: &BackendConfig) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("创建HTTP客户端失败: {}", e);
            return false;
        }
    };
    
    // 优先检测 HTTPS
    let https_test_url = format!("{}:{}/test", config.base_url.replace("http://", "https://"), config.port);
    tracing::info!("检测 HTTPS 后端可用性: {}", https_test_url);
    
    match client.get(&https_test_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                            tracing::info!("HTTPS 后端可用");
                            return true;
                        }
                    }
                    Err(e) => {
                        tracing::info!("读取 HTTPS 响应失败: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::info!("检测 HTTPS 后端失败: {}", e);
        }
    }
    
    tracing::info!("HTTPS 后端不可用，尝试 HTTP");
    
    // HTTPS 失败，尝试 HTTP
    let http_test_url = format!("{}:{}/test", config.base_url, config.port);
    tracing::info!("检测 HTTP 后端可用性: {}", http_test_url);
    
    match client.get(&http_test_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                            tracing::info!("HTTP 后端可用");
                            return true;
                        }
                    }
                    Err(e) => {
                        tracing::info!("读取 HTTP 响应失败: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::info!("检测 HTTP 后端失败: {}", e);
        }
    }
    
    tracing::info!("HTTP 和 HTTPS 后端都不可用");
    false
}

// 尝试从环境变量加载配置
async fn try_load_from_env() -> Option<BackendConfig> {
    // 尝试从 .env 文件加载环境变量
    dotenv::dotenv().ok();
    
    let base_url = std::env::var("CAMFC_BASE").ok()?;
    let port_str = std::env::var("CAMFC_PORT").ok()?;
    
    let port = port_str.parse::<u16>().ok()?;
    
    // 解析 base_url，确保有协议前缀
    let base_url = if base_url.starts_with("http://") || base_url.starts_with("https://") {
        base_url.clone()
    } else {
        format!("http://{}", base_url)
    };
    
    tracing::info!("从环境变量加载配置: {}", base_url);
    tracing::info!("端口号: {}", port);
    
    // 如果已经有 https://，直接返回
    if base_url.starts_with("https://") {
        return Some(BackendConfig {
            base_url,
            port,
        });
    }
    
    // 尝试 HTTPS，如果失败则使用 HTTP
    let https_url = base_url.replace("http://", "https://");
    
    // 创建客户端进行检测
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("创建HTTP客户端失败: {}", e);
            return Some(BackendConfig {
                base_url,
                port,
            });
        }
    };
    
    // 优先检测 HTTPS
    let https_test_url = format!("{}:{}/test", https_url, port);
    tracing::info!("检测 HTTPS 后端可用性: {}", https_test_url);
    
    match client.get(&https_test_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                            tracing::info!("HTTPS 后端可用，使用 HTTPS 连接");
                            return Some(BackendConfig {
                                base_url: https_url,
                                port,
                            });
                        }
                    }
                    Err(e) => {
                        tracing::info!("读取 HTTPS 响应失败: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::info!("检测 HTTPS 后端失败: {}", e);
        }
    }
    
    tracing::info!("HTTPS 后端不可用，尝试 HTTP");
    
    // HTTPS 失败，尝试 HTTP
    let http_test_url = format!("{}:{}/test", base_url, port);
    tracing::info!("检测 HTTP 后端可用性: {}", http_test_url);
    
    match client.get(&http_test_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                            tracing::info!("HTTP 后端可用，使用 HTTP 连接");
                            return Some(BackendConfig {
                                base_url,
                                port,
                            });
                        }
                    }
                    Err(e) => {
                        tracing::info!("读取 HTTP 响应失败: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::info!("检测 HTTP 后端失败: {}", e);
        }
    }
    
    tracing::info!("HTTP 和 HTTPS 后端都不可用，使用配置中的 URL");
    
    // 都不可用，返回配置中的 URL（可能是 HTTP）
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
    
    tracing::info!("远程配置解析成功，收到 {} 个候选地址", remote_config.base_url.len());
    
    // 依次检测每个候选地址的可用性
    for (index, candidate) in remote_config.base_url.iter().enumerate() {
        tracing::info!("检测候选地址 [{}/{}]: {}", index + 1, remote_config.base_url.len(), candidate);
        
        // 对每个候选地址，优先检测 HTTPS，然后检测 HTTP
        let https_candidate = if candidate.starts_with("http://") {
            candidate.replace("http://", "https://")
        } else {
            candidate.to_string()
        };
        
        // 优先检测 HTTPS
        if check_backend_available(&client, &https_candidate).await {
            tracing::info!("HTTPS 候选地址可用: {}", https_candidate);
            
            // 解析 base_url 和 port
            let (base_url, port) = parse_backend_url(&https_candidate)?;
            
            return Ok(BackendConfig {
                base_url,
                port,
            });
        } else {
            tracing::info!("HTTPS 候选地址不可用: {}", https_candidate);
        }
        
        // HTTPS 失败，尝试 HTTP
        if check_backend_available(&client, candidate).await {
            tracing::info!("HTTP 候选地址可用: {}", candidate);
            
            // 解析 base_url 和 port
            let (base_url, port) = parse_backend_url(candidate)?;
            
            return Ok(BackendConfig {
                base_url,
                port,
            });
        } else {
            tracing::info!("HTTP 候选地址不可用: {}", candidate);
        }
    }
    
    Err(anyhow::anyhow!("所有候选地址都不可用"))
}

// 解析后端 URL，返回 (base_url, port)
fn parse_backend_url(url: &str) -> Result<(String, u16)> {
    let url = url.trim();
    
    // 先处理协议前缀
    let (protocol, rest) = if let Some(rest) = url.strip_prefix("https://") {
        ("https://", rest)
    } else if let Some(rest) = url.strip_prefix("http://") {
        ("http://", rest)
    } else {
        ("http://", url)
    };
    
    // 在剩余部分中查找端口
    if let Some((host, port_str)) = rest.split_once(':') {
        let port = port_str.parse::<u16>()
            .context(format!("无效的端口号: {}", port_str))?;
        
        let base_url = format!("{}{}", protocol, host);
        Ok((base_url, port))
    } else {
        // 没有端口，默认使用 8005
        let base_url = format!("{}{}", protocol, rest);
        Ok((base_url, 8005))
    }
}

// 检测后端是否可用
async fn check_backend_available(client: &reqwest::Client, backend_url: &str) -> bool {
    let (base_url, port) = match parse_backend_url(backend_url) {
        Ok(result) => result,
        Err(e) => {
            tracing::info!("解析后端 URL 失败: {} - {}", backend_url, e);
            return false;
        }
    };
    
    let test_url = format!("{}:{}/test", base_url, port);
    tracing::info!("检测后端可用性: {}", test_url);
    
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
                            tracing::info!("后端可用，返回合法 JSON: {}", text);
                            true
                        } else {
                            tracing::info!("后端响应不是合法 JSON: {}", text);
                            false
                        }
                    }
                    Err(e) => {
                        tracing::info!("读取响应失败: {}", e);
                        false
                    }
                }
            } else {
                tracing::info!("后端返回错误状态: {}", response.status());
                false
            }
        }
        Err(e) => {
            tracing::info!("检测后端失败: {}", e);
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
