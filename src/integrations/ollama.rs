use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaClient {
    pub base_url: String,
    pub model: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        OllamaClient {
            base_url,
            model,
            client: reqwest::Client::new(),
        }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        tracing::info!("Generating response from Ollama model: {}", self.model);
        
        let url = format!("{}/api/generate", self.base_url);
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let data: OllamaResponse = response.json().await?;
            Ok(data.response)
        } else {
            Err(anyhow::anyhow!(
                "Ollama API error: {}",
                response.status()
            ))
        }
    }

    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
