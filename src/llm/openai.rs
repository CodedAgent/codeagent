use super::client::{LLMClient, LLMResponse};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIApiResponse {
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Deserialize)]
struct OpenAIUsage {
    total_tokens: u32,
}

pub struct OpenAIClient {
    api_key: String,
    model: String,
    base_url: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String, model: String) -> Self {
        OpenAIClient {
            api_key,
            model,
            base_url: "https://api.openai.com/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn call_api(&self, messages: Vec<OpenAIMessage>) -> Result<OpenAIApiResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 2000,
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let data: OpenAIApiResponse = response.json().await?;
            Ok(data)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("OpenAI API error: {}", error_text))
        }
    }
}

#[async_trait::async_trait]
impl LLMClient for OpenAIClient {
    async fn generate(&self, prompt: &str) -> Result<LLMResponse> {
        let messages = vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: "You are a helpful AI coding assistant.".to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let api_response = self.call_api(messages).await?;
        
        let content = api_response
            .choices
            .get(0)
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?;

        Ok(LLMResponse {
            content,
            tokens_used: Some(api_response.usage.total_tokens),
            model: self.model.clone(),
            provider: "OpenAI".to_string(),
        })
    }

    async fn generate_with_context(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LLMResponse> {
        let messages = vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];

        let api_response = self.call_api(messages).await?;
        
        let content = api_response
            .choices
            .get(0)
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?;

        Ok(LLMResponse {
            content,
            tokens_used: Some(api_response.usage.total_tokens),
            model: self.model.clone(),
            provider: "OpenAI".to_string(),
        })
    }

    fn provider_name(&self) -> &str {
        "OpenAI"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/models", self.base_url);
        
        match self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
