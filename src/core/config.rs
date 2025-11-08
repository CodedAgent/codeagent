use std::path::PathBuf;
use anyhow::Result;

pub struct ProjectConfig {
    pub project_root: PathBuf,
    pub model_provider: ModelProvider,
}

#[derive(Clone)]
pub enum ModelProvider {
    Ollama { base_url: String },
    OpenAI { api_key: String },
    Anthropic { api_key: String },
}

impl ProjectConfig {
    pub fn load(project_root: PathBuf) -> Result<Self> {
        let config_path = project_root.join(".codeagent.yml");
        
        let provider = if config_path.exists() {
            Self::load_from_file(&config_path)?
        } else {
            ModelProvider::Ollama {
                base_url: "http://localhost:11434".to_string(),
            }
        };

        Ok(ProjectConfig {
            project_root,
            model_provider: provider,
        })
    }

    fn load_from_file(_path: &std::path::Path) -> Result<ModelProvider> {
        Ok(ModelProvider::Ollama {
            base_url: "http://localhost:11434".to_string(),
        })
    }
}

pub fn init_project(path: &std::path::Path) -> Result<()> {
    let config_path = path.join(".codeagent.yml");
    
    let default_config = r#"# CodeAgent Configuration
model_provider: ollama
ollama_base_url: "http://localhost:11434"
ollama_model: "mistral"
"#;

    std::fs::write(&config_path, default_config)?;
    tracing::info!("Initialized CodeAgent project at {:?}", path);
    
    Ok(())
}
