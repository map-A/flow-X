//! Model configuration

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub class: String,
    pub requires_api_key: bool,
    pub cost_per_call: f64,
}

pub struct ModelConfig;

impl ModelConfig {
    pub fn supported_models() -> Vec<(&'static str, ModelInfo)> {
        vec![(
            "glm-4v",
            ModelInfo {
                class: "MultimodalModel".to_string(),
                requires_api_key: true,
                cost_per_call: 0.1,
            },
        )]
    }

    pub fn get_api_key() -> Option<String> {
        env::var("FLOWX_API_KEY")
            .or_else(|_| env::var("GLM_API_KEY"))
            .ok()
    }

    pub fn validate_model(model_name: &str) -> bool {
        Self::supported_models()
            .iter()
            .any(|(name, _)| *name == model_name)
    }

    pub fn get_model_info(model_name: &str) -> Option<ModelInfo> {
        Self::supported_models()
            .into_iter()
            .find(|(name, _)| *name == model_name)
            .map(|(_, info)| info)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("API key not found")]
    MissingAPIKey,

    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Response parse error: {0}")]
    Parse(String),
}
