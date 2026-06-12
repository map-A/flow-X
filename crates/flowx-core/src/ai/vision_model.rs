//! Vision model for screen understanding

use crate::ai::models::BaseModel;
use crate::ai::prompts::SCREEN_UNDERSTANDING_PROMPT;
use crate::ai::{AIError, AIResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenContext {
    pub app: String,
    pub screen_type: String,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    #[serde(rename = "type")]
    pub element_type: String,
    pub text: String,
    pub position: String,
}

pub struct VisionModel {
    model: Box<dyn BaseModel>,
}

impl VisionModel {
    pub fn new(model: Box<dyn BaseModel>) -> Self {
        Self { model }
    }

    pub fn understand_screen(&self, screenshot: &[u8]) -> AIResult<ScreenContext> {
        let response = self
            .model
            .vision_chat(SCREEN_UNDERSTANDING_PROMPT, screenshot, 0.7)?;

        let context: ScreenContext = serde_json::from_str(&response)
            .map_err(|e| AIError::ActionParse(format!("Failed to parse screen context: {}", e)))?;

        Ok(context)
    }

    pub fn find_element(&self, screenshot: &[u8], description: &str) -> AIResult<Option<Element>> {
        let prompt = format!(
            "请在这张截图中找到「{}」元素。如果找到，以 JSON 格式返回元素信息：\
            {{\"type\": \"类型\", \"text\": \"文本\", \"position\": \"位置\"}}。\
            如果没找到，返回 null。",
            description
        );

        let response = self.model.vision_chat(&prompt, screenshot, 0.7)?;

        if response.trim() == "null" || response.trim().is_empty() {
            return Ok(None);
        }

        let element: Element = serde_json::from_str(&response)
            .map_err(|e| AIError::ActionParse(format!("Failed to parse element: {}", e)))?;

        Ok(Some(element))
    }
}
