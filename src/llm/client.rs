use std::fmt;

#[derive(Debug, Clone)]
pub enum LLMProvider {
    Ollama {
        base_url: String,
        model: String,
    },
    OpenAI {
        api_key: String,
        model: String,
    },
    Anthropic {
        api_key: String,
        model: String,
    },
}

#[derive(Debug, Clone)]
pub struct LLMResponse {
    pub content: String,
    pub tokens_used: Option<u32>,
    pub model: String,
    pub provider: String,
}

#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> anyhow::Result<LLMResponse>;
    
    async fn generate_with_context(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> anyhow::Result<LLMResponse>;
    
    fn provider_name(&self) -> &str;
    
    fn model_name(&self) -> &str;
    
    async fn health_check(&self) -> anyhow::Result<bool>;
}

impl fmt::Display for LLMProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLMProvider::Ollama { model, .. } => write!(f, "Ollama ({})", model),
            LLMProvider::OpenAI { model, .. } => write!(f, "OpenAI ({})", model),
            LLMProvider::Anthropic { model, .. } => write!(f, "Anthropic ({})", model),
        }
    }
}
