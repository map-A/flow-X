//! Task planner for generating action sequences

use crate::ai::models::{BaseModel, Message};
use crate::ai::prompts::{task_planning_prompt, SYSTEM_PROMPT};
use crate::ai::utils::action::{parse_action_sequence, Action};
use crate::ai::vision_model::ScreenContext;
use crate::ai::{AIError, AIResult};

pub struct TaskPlanner {
    model: Box<dyn BaseModel>,
}

impl TaskPlanner {
    pub fn new(model: Box<dyn BaseModel>) -> Self {
        Self { model }
    }

    pub fn plan(&self, instruction: &str, screen_context: &ScreenContext) -> AIResult<Vec<Action>> {
        let screen_json = serde_json::to_string_pretty(screen_context).map_err(|e| {
            AIError::ActionParse(format!("Failed to serialize screen context: {}", e))
        })?;

        let prompt = task_planning_prompt(instruction, &screen_json);
        let messages = vec![Message::user(prompt)];

        let response = self.model.chat(&messages, 0.7)?;

        let actions = parse_action_sequence(&response)
            .map_err(|e| AIError::ActionParse(format!("Failed to parse actions: {}", e)))?;

        Ok(actions)
    }
}
