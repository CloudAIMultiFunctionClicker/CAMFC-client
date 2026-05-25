use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::config::get_backend_url;

#[derive(Serialize)]
pub struct PredictRequest {
    pub instruction: String,
    pub screenshot: String,
    pub history: Vec<String>,
}

#[derive(Deserialize)]
pub struct PredictResponse {
    pub success: bool,
    pub output: String,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub model: String,
}

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        let base_url = get_backend_url().unwrap_or_else(|_| {
            tracing::warn!("无法获取后端配置，使用默认值 https://camfc.seven-cloud.cn:8005");
            "https://camfc.seven-cloud.cn:8005".to_string()
        });
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        
        ApiClient {
            client,
            base_url,
        }
    }

    pub async fn check_health(&self) -> Result<bool> {
        let url = format!("{}/api/agent/health", self.base_url);

        match self.client.get(&url).timeout(std::time::Duration::from_secs(10)).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            Err(_) => Ok(false),
        }
    }

    pub async fn predict(
        &self,
        instruction: &str,
        screenshot_base64: &str,
        history: &[String],
    ) -> Result<PredictResponse> {
        let url = format!("{}/api/agent/predict", self.base_url);

        let request = PredictRequest {
            instruction: instruction.to_string(),
            screenshot: screenshot_base64.to_string(),
            history: history.to_vec(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result = response.json::<PredictResponse>().await?;
            Ok(result)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("HTTP {}: {}", status, error_text))
        }
    }
}
