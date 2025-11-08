use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlConfig {
    pub model_provider: String,
    #[serde(default)]
    pub ollama_base_url: String,
    #[serde(default)]
    pub ollama_model: String,
    #[serde(default)]
    pub openai_api_key: Option<String>,
    #[serde(default)]
    pub anthropic_api_key: Option<String>,
    #[serde(default)]
    pub interactive_mode: bool,
    #[serde(default)]
    pub auto_fix_enabled: bool,
    #[serde(default)]
    pub max_retry_attempts: u32,
    #[serde(default)]
    pub excluded_files: Vec<String>,
    #[serde(default)]
    pub excluded_dirs: Vec<String>,
    #[serde(default)]
    pub custom_tools: HashMap<String, CustomTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTool {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub enabled: bool,
}

impl YamlConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(content: &str) -> Result<Self> {
        match serde_yaml::from_str::<Self>(content) {
            Ok(config) => Ok(config),
            Err(e) => {
                tracing::error!("Failed to parse YAML config: {}", e);
                Ok(Self::default())
            }
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.model_provider.is_empty() {
            return Err(anyhow::anyhow!("model_provider is required"));
        }

        match self.model_provider.as_str() {
            "ollama" => {
                if self.ollama_base_url.is_empty() {
                    return Err(anyhow::anyhow!("ollama_base_url is required for Ollama provider"));
                }
            }
            "openai" => {
                if self.openai_api_key.is_none() || self.openai_api_key.as_ref().unwrap().is_empty() {
                    return Err(anyhow::anyhow!("openai_api_key is required for OpenAI provider"));
                }
            }
            "anthropic" => {
                if self.anthropic_api_key.is_none() || self.anthropic_api_key.as_ref().unwrap().is_empty() {
                    return Err(anyhow::anyhow!("anthropic_api_key is required for Anthropic provider"));
                }
            }
            _ => return Err(anyhow::anyhow!("Unknown model provider: {}", self.model_provider)),
        }

        Ok(())
    }

    pub fn should_exclude_file(&self, file_path: &str) -> bool {
        for pattern in &self.excluded_files {
            if file_path.contains(pattern) {
                return true;
            }
        }
        false
    }

    pub fn should_exclude_dir(&self, dir_path: &str) -> bool {
        for pattern in &self.excluded_dirs {
            if dir_path.contains(pattern) {
                return true;
            }
        }
        false
    }
}

impl Default for YamlConfig {
    fn default() -> Self {
        Self {
            model_provider: "ollama".to_string(),
            ollama_base_url: "http://localhost:11434".to_string(),
            ollama_model: "mistral".to_string(),
            openai_api_key: None,
            anthropic_api_key: None,
            interactive_mode: false,
            auto_fix_enabled: false,
            max_retry_attempts: 3,
            excluded_files: vec![
                "*.min.js".to_string(),
                "*.lock".to_string(),
            ],
            excluded_dirs: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "__pycache__".to_string(),
                "target".to_string(),
            ],
            custom_tools: HashMap::new(),
        }
    }
}
