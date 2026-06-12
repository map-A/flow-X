//! Generic multimodal model implementation

use super::base::{BaseModel, Message};
use crate::ai::{AIError, AIResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MultimodalModel {
    config: ModelConfig,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub model_name: String,
    pub api_endpoint: String,
    pub api_key: String,
    pub timeout: u64,
    pub temperature: f32,
    pub top_p: Option<f32>,
}

impl MultimodalModel {
    pub fn new(config: ModelConfig) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    pub fn from_file(path: &str) -> AIResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| AIError::IO(e))?;

        let config: HashMap<String, HashMap<String, toml::Value>> = toml::from_str(&content)
            .map_err(|e| AIError::ModelAPI(format!("Config parse error: {}", e)))?;

        let model_config = config
            .get("model")
            .ok_or_else(|| AIError::ModelAPI("Missing [model] section".to_string()))?;

        let cfg = ModelConfig {
            provider: model_config
                .get("provider")
                .and_then(|v| v.as_str())
                .unwrap_or("zhipu")
                .to_string(),
            model_name: model_config
                .get("model_name")
                .and_then(|v| v.as_str())
                .unwrap_or("glm-4v")
                .to_string(),
            api_endpoint: model_config
                .get("api_endpoint")
                .and_then(|v| v.as_str())
                .ok_or_else(|| AIError::ModelAPI("Missing api_endpoint".to_string()))?
                .to_string(),
            api_key: Self::resolve_env_var(
                model_config
                    .get("api_key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AIError::ModelAPI("Missing api_key".to_string()))?,
            ),
            timeout: model_config
                .get("timeout")
                .and_then(|v| v.as_integer())
                .unwrap_or(60) as u64,
            temperature: model_config
                .get("temperature")
                .and_then(|v| v.as_float())
                .unwrap_or(0.7) as f32,
            top_p: model_config
                .get("top_p")
                .and_then(|v| v.as_float())
                .map(|v| v as f32),
        };

        Ok(Self::new(cfg))
    }

    fn resolve_env_var(value: &str) -> String {
        if value.starts_with("${") && value.ends_with("}") {
            let var_name = &value[2..value.len() - 1];
            std::env::var(var_name).unwrap_or_else(|_| value.to_string())
        } else {
            value.to_string()
        }
    }
}

impl BaseModel for MultimodalModel {
    fn chat(&self, messages: &[Message], temperature: f32) -> AIResult<String> {
        let mut payload = serde_json::json!({
            "model": self.config.model_name,
            "messages": messages.iter().map(|m| {
                serde_json::json!({
                    "role": m.role,
                    "content": m.content
                })
            }).collect::<Vec<_>>(),
            "temperature": temperature,
        });

        if let Some(top_p) = self.config.top_p {
            payload["top_p"] = serde_json::json!(top_p);
        }

        let response = self
            .client
            .post(&self.config.api_endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AIError::ModelAPI(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AIError::ModelAPI(format!(
                "API error: {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .map_err(|e| AIError::ModelAPI(format!("Failed to parse response: {}", e)))?;

        result["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AIError::ModelAPI("Invalid response format".to_string()))
            .map(|s| s.to_string())
    }

    fn vision_chat(&self, prompt: &str, image: &[u8], temperature: f32) -> AIResult<String> {
        let image_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image);
        let image_url = format!("data:image/png;base64,{}", image_b64);

        let payload = serde_json::json!({
            "model": self.config.model_name,
            "messages": [{
                "role": "user",
                "content": [
                    {"type": "text", "text": prompt},
                    {"type": "image_url", "image_url": {"url": image_url}}
                ]
            }],
            "temperature": temperature,
        });

        let response = self
            .client
            .post(&self.config.api_endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AIError::ModelAPI(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AIError::ModelAPI(format!(
                "API error: {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .map_err(|e| AIError::ModelAPI(format!("Failed to parse response: {}", e)))?;

        result["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AIError::ModelAPI("Invalid response format".to_string()))
            .map(|s| s.to_string())
    }
}
