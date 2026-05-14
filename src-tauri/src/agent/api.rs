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
        let base_url = get_backend_url().unwrap_or_else(|_| "http://localhost:7548".to_string());
        ApiClient {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn check_health(&self) -> Result<bool> {
        let url = format!("{}/api/agent/health", self.base_url);

        match self.client.get(&url).timeout(std::time::Duration::from_secs(5)).send().await {
            Ok(response) => Ok(response.status().is_success()),
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
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await?;

        if response.status().is_success() {
            let result = response.json::<PredictResponse>().await?;
            Ok(result)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Server error: {}", error_text))
        }
    }
}
