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

#[derive(Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub size: i64,
    pub modified_at: String,
}

#[derive(Deserialize)]
pub struct OllamaTagsResponse {
    pub models: Vec<OllamaModel>,
}

pub struct OllamaClient {
    pub base_url: String,
    pub model: String,
    pub available_models: Vec<String>,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        OllamaClient {
            base_url,
            model,
            available_models: Vec::new(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn auto_detect() -> Result<Self> {
        let base_url = "http://localhost:11434".to_string();
        let client = reqwest::Client::new();
        
        let url = format!("{}/api/tags", base_url);
        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let data: OllamaTagsResponse = response.json().await?;
            let available_models: Vec<String> = 
                data.models.iter().map(|m| m.name.clone()).collect();
            
            let model = available_models.first().cloned()
                .unwrap_or_else(|| "mistral".to_string());

            tracing::info!("Auto-detected Ollama models: {:?}", available_models);
            tracing::info!("Using default model: {}", model);

            Ok(OllamaClient {
                base_url,
                model,
                available_models,
                client,
            })
        } else {
            Err(anyhow::anyhow!("Failed to connect to Ollama"))
        }
    }

    pub async fn get_available_models(&mut self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let data: OllamaTagsResponse = response.json().await?;
            self.available_models = data.models.iter().map(|m| m.name.clone()).collect();
            Ok(self.available_models.clone())
        } else {
            Err(anyhow::anyhow!("Failed to fetch models"))
        }
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
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
