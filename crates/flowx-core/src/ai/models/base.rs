//! Base model trait

use crate::ai::AIResult;

pub trait BaseModel: Send + Sync {
    fn chat(&self, messages: &[Message], temperature: f32) -> AIResult<String>;
    fn vision_chat(&self, prompt: &str, image: &[u8], temperature: f32) -> AIResult<String>;
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}
