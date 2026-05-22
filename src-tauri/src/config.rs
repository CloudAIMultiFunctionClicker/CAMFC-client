

use std::sync::OnceLock;
use serde::Deserialize;
use anyhow::{Result, Context};

#[derive(Debug, Deserialize)]
struct RemoteConfig {
    base_url: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BackendConfig {
    pub base_url: String,
    pub port: u16,
}

impl BackendConfig {

    pub fn get_full_url(&self) -> String {
        format!("{}:{}", self.base_url, self.port)
    }
}

static BACKEND_CONFIG: OnceLock<BackendConfig> = OnceLock::new();

pub async fn init_config() -> Result<()> {
    tracing::info!("开始初始化后端配置...");

    // 优先级 1: 尝试从环境变量读取
    if let Some(env_config) = try_load_from_env().await {
        tracing::info!("使用环境变量配置：{}", env_config.get_full_url());
        BACKEND_CONFIG.set(env_config)
            .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
        return Ok(());
    }

    // 优先级 2: 尝试从远程配置读取
    match try_load_from_remote().await {
        Ok(remote_config) => {
            tracing::info!("使用远程配置：{}", remote_config.get_full_url());
            BACKEND_CONFIG.set(remote_config)
                .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
            return Ok(());
        }
        Err(e) => {
            tracing::info!("加载远程配置失败：{}，使用硬编码默认配置", e);
        }
    }

    // 优先级 3: 使用硬编码默认配置
    let config = BackendConfig {
        base_url: "https://camfc.seven-cloud.cn".to_string(),
        port: 8005,
    };
    tracing::info!("使用硬编码默认配置：{}", config.get_full_url());
    BACKEND_CONFIG.set(config)
        .map_err(|_| anyhow::anyhow!("配置已初始化"))?;
    Ok(())
}

async fn check_env_backend_available(config: &BackendConfig) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("创建 HTTP 客户端失败：{}", e);
            return false;
        }
    };

    const MAX_RETRIES: u32 = 2;
    const RETRY_DELAY_MS: u64 = 500;

    for attempt in 1..=MAX_RETRIES {
        tracing::info!("检测后端服务器尝试 {}/{}", attempt, MAX_RETRIES);

        if attempt > 1 {
            tracing::info!("等待 {}ms 后重试", RETRY_DELAY_MS);
            tokio::time::sleep(std::time::Duration::from_millis(RETRY_DELAY_MS)).await;
        }

        let https_test_url = format!("{}:{}/test", config.base_url.replace("http://", "https://"), config.port);
        tracing::info!("检测 HTTPS 后端可用性：{}", https_test_url);

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
                            tracing::info!("读取 HTTPS 响应失败：{}", e);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::info!("检测 HTTPS 后端失败：{}", e);
            }
        }

        tracing::info!("HTTPS 后端不可用，尝试 HTTP");

        let http_test_url = format!("{}:{}/test", config.base_url, config.port);
        tracing::info!("检测 HTTP 后端可用性：{}", http_test_url);

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
                            tracing::info!("读取 HTTP 响应失败：{}", e);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::info!("检测 HTTP 后端失败：{}", e);
            }
        }

        if attempt < MAX_RETRIES {
            tracing::info!("第 {} 次尝试失败，准备重试", attempt);
        }
    }

    tracing::info!("HTTP 和 HTTPS 后端都不可用");
    false
}

async fn try_load_from_env() -> Option<BackendConfig> {
    // 尝试从多个可能的位置加载 .env 文件
    let env_loaded = dotenv::dotenv().ok().is_some();
    
    if !env_loaded {
        // 如果默认位置加载失败，尝试从可执行目录的父目录加载
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let env_path = exe_dir.join(".env");
                if env_path.exists() {
                    dotenv::from_path(&env_path).ok();
                }
            }
        }
        
        // 尝试从项目根目录加载（开发环境）
        let project_root = std::env::current_dir().ok()?;
        let env_path = project_root.join(".env");
        if env_path.exists() {
            dotenv::from_path(&env_path).ok();
        }
    }

    let base_url = std::env::var("CAMFC_BASE").ok()?;
    let port_str = std::env::var("CAMFC_PORT").ok()?;

    let port = port_str.parse::<u16>().ok()?;

    let base_url = if base_url.starts_with("http://") || base_url.starts_with("https://") {
        base_url.clone()
    } else {
        format!("http://{}", base_url)
    };

    tracing::info!("从环境变量加载配置：{}", base_url);
    tracing::info!("端口号：{}", port);

    if base_url.starts_with("https://") {
        return Some(BackendConfig {
            base_url,
            port,
        });
    }

    let https_url = base_url.replace("http://", "https://");

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("创建 HTTP 客户端失败：{}", e);
            return Some(BackendConfig {
                base_url,
                port,
            });
        }
    };

    let https_test_url = format!("{}:{}/test", https_url, port);
    tracing::info!("检测 HTTPS 后端可用性：{}", https_test_url);

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
                        tracing::info!("读取 HTTPS 响应失败：{}", e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::info!("检测 HTTPS 后端失败：{}", e);
        }
    }

    tracing::info!("HTTPS 后端不可用，尝试 HTTP");

    let http_test_url = format!("{}:{}/test", base_url, port);
    tracing::info!("检测 HTTP 后端可用性：{}", http_test_url);

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
                        tracing::info!("读取 HTTP 响应失败：{}", e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::info!("检测 HTTP 后端失败：{}", e);
        }
    }

    tracing::info!("HTTP 和 HTTPS 后端都不可用，使用配置中的 URL");

    Some(BackendConfig {
        base_url,
        port,
    })
}

async fn try_load_from_remote() -> Result<BackendConfig> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .context("创建 HTTP 客户端失败")?;

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
            "远程配置请求失败：{} - {}",
            status,
            error_text
        ));
    }

    let remote_config: RemoteConfig = response
        .json()
        .await
        .context("解析远程配置失败")?;

    tracing::info!("远程配置解析成功，收到 {} 个候选地址", remote_config.base_url.len());

    for (index, candidate) in remote_config.base_url.iter().enumerate() {
        tracing::info!("检测候选地址 [{}/{}]: {}", index + 1, remote_config.base_url.len(), candidate);

        let https_candidate = if candidate.starts_with("http://") {
            candidate.replace("http://", "https://")
        } else {
            candidate.to_string()
        };

        if check_backend_available(&client, &https_candidate).await {
            tracing::info!("HTTPS 候选地址可用：{}", https_candidate);

            let (base_url, port) = parse_backend_url(&https_candidate)?;

            return Ok(BackendConfig {
                base_url,
                port,
            });
        } else {
            tracing::info!("HTTPS 候选地址不可用：{}", https_candidate);
        }

        if check_backend_available(&client, candidate).await {
            tracing::info!("HTTP 候选地址可用：{}", candidate);

            let (base_url, port) = parse_backend_url(candidate)?;

            return Ok(BackendConfig {
                base_url,
                port,
            });
        } else {
            tracing::info!("HTTP 候选地址不可用：{}", candidate);
        }
    }

    Err(anyhow::anyhow!("所有候选地址都不可用"))
}

fn parse_backend_url(url: &str) -> Result<(String, u16)> {
    let url = url.trim();

    let (protocol, rest) = if let Some(rest) = url.strip_prefix("https://") {
        ("https://", rest)
    } else if let Some(rest) = url.strip_prefix("http://") {
        ("http://", rest)
    } else {
        ("http://", url)
    };

    if let Some((host, port_str)) = rest.split_once(':') {
        let port = port_str.parse::<u16>()
            .context(format!("无效的端口号：{}", port_str))?;

        let base_url = format!("{}{}", protocol, host);
        Ok((base_url, port))
    } else {

        let base_url = format!("{}{}", protocol, rest);
        Ok((base_url, 8005))
    }
}

async fn check_backend_available(client: &reqwest::Client, backend_url: &str) -> bool {
    let (base_url, port) = match parse_backend_url(backend_url) {
        Ok(result) => result,
        Err(e) => {
            tracing::info!("解析后端 URL 失败：{} - {}", backend_url, e);
            return false;
        }
    };

    const MAX_RETRIES: u32 = 2;
    const RETRY_DELAY_MS: u64 = 500;

    for attempt in 1..=MAX_RETRIES {
        tracing::info!("检测后端服务器尝试 {}/{}", attempt, MAX_RETRIES);

        if attempt > 1 {
            tracing::info!("等待 {}ms 后重试", RETRY_DELAY_MS);
            tokio::time::sleep(std::time::Duration::from_millis(RETRY_DELAY_MS)).await;
        }

        let test_url = format!("{}:{}/test", base_url, port);
        tracing::info!("检测后端可用性：{}", test_url);

        match client
            .get(&test_url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {

                    match response.text().await {
                        Ok(text) => {

                            if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                                tracing::info!("后端可用，返回合法 JSON: {}", text);
                                return true;
                            } else {
                                tracing::info!("后端响应不是合法 JSON: {}", text);
                            }
                        }
                        Err(e) => {
                            tracing::info!("读取响应失败：{}", e);
                        }
                    }
                } else {
                    tracing::info!("后端返回错误状态：{}", response.status());
                }
            }
            Err(e) => {
                tracing::info!("检测后端失败：{}", e);
            }
        }

        if attempt < MAX_RETRIES {
            tracing::info!("第 {} 次尝试失败，准备重试", attempt);
        }
    }

    tracing::info!("后端不可用");
    false
}

pub fn get_backend_config() -> Result<&'static BackendConfig> {
    BACKEND_CONFIG.get()
        .ok_or_else(|| anyhow::anyhow!("后端配置未初始化，请先调用 init_config"))
}

// 获取完整的后端 URL（便捷函数）
// 添加重试逻辑，等待配置初始化完成
pub fn get_backend_url() -> Result<String> {
    const MAX_RETRIES: u32 = 10;
    const RETRY_DELAY_MS: u64 = 100;
    
    for attempt in 1..=MAX_RETRIES {
        if let Some(config) = BACKEND_CONFIG.get() {
            return Ok(config.get_full_url());
        }
        
        if attempt < MAX_RETRIES {
            tracing::info!("等待配置初始化... ({}/{})", attempt, MAX_RETRIES);
            std::thread::sleep(std::time::Duration::from_millis(RETRY_DELAY_MS));
        }
    }
    
    // 如果等待后仍未初始化，尝试同步初始化
    tracing::warn!("配置未初始化，尝试同步加载...");
    match tokio::runtime::Runtime::new() {
        Ok(runtime) => {
            if let Err(e) = runtime.block_on(init_config()) {
                tracing::error!("同步配置初始化失败：{}", e);
            }
        }
        Err(e) => {
            tracing::error!("创建运行时失败：{}", e);
        }
    }
    
    BACKEND_CONFIG.get()
        .map(|c| c.get_full_url())
        .ok_or_else(|| anyhow::anyhow!("后端配置未初始化"))
}
