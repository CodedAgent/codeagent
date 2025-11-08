use super::client::{LLMClient, LLMResponse};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: Option<String>,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicApiResponse {
    content: Vec<AnthropicContent>,
    usage: AnthropicUsage,
}

#[derive(Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

pub struct AnthropicClient {
    api_key: String,
    model: String,
    base_url: String,
    client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(api_key: String, model: String) -> Self {
        AnthropicClient {
            api_key,
            model,
            base_url: "https://api.anthropic.com/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn call_api(
        &self,
        system: Option<String>,
        messages: Vec<AnthropicMessage>,
    ) -> Result<AnthropicApiResponse> {
        let url = format!("{}/messages", self.base_url);
        
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 2000,
            system,
            messages,
        };

        let response = self.client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let data: AnthropicApiResponse = response.json().await?;
            Ok(data)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("Anthropic API error: {}", error_text))
        }
    }
}

#[async_trait::async_trait]
impl LLMClient for AnthropicClient {
    async fn generate(&self, prompt: &str) -> Result<LLMResponse> {
        let messages = vec![
            AnthropicMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let system = Some("You are a helpful AI coding assistant.".to_string());
        let api_response = self.call_api(system, messages).await?;
        
        let content = api_response
            .content
            .get(0)
            .map(|c| c.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Anthropic"))?;

        let total_tokens = api_response.usage.input_tokens + api_response.usage.output_tokens;

        Ok(LLMResponse {
            content,
            tokens_used: Some(total_tokens),
            model: self.model.clone(),
            provider: "Anthropic".to_string(),
        })
    }

    async fn generate_with_context(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LLMResponse> {
        let messages = vec![
            AnthropicMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];

        let system = Some(system_prompt.to_string());
        let api_response = self.call_api(system, messages).await?;
        
        let content = api_response
            .content
            .get(0)
            .map(|c| c.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Anthropic"))?;

        let total_tokens = api_response.usage.input_tokens + api_response.usage.output_tokens;

        Ok(LLMResponse {
            content,
            tokens_used: Some(total_tokens),
            model: self.model.clone(),
            provider: "Anthropic".to_string(),
        })
    }

    fn provider_name(&self) -> &str {
        "Anthropic"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn health_check(&self) -> Result<bool> {
        match self.generate("ping").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
