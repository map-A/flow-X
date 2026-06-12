//! Action executor for executing planned actions

use crate::ai::utils::action::{validate_action, Action};
use crate::ai::{AIError, AIResult};
use crate::engine::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub action: Action,
    pub error: Option<String>,
}

pub struct ActionExecutor;

impl ActionExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_action(&self, action: &Action) -> AIResult<ExecutionResult> {
        if !validate_action(action) {
            return Ok(ExecutionResult {
                success: false,
                action: action.clone(),
                error: Some("Invalid action format".to_string()),
            });
        }

        // Convert action to command
        let command = self.action_to_command(action)?;

        // Here we would execute the command through device
        // For now, just return success
        Ok(ExecutionResult {
            success: true,
            action: action.clone(),
            error: None,
        })
    }

    pub fn execute_sequence(&self, actions: &[Action]) -> Vec<ExecutionResult> {
        actions
            .iter()
            .map(|action| {
                self.execute_action(action)
                    .unwrap_or_else(|e| ExecutionResult {
                        success: false,
                        action: action.clone(),
                        error: Some(e.to_string()),
                    })
            })
            .collect()
    }

    fn action_to_command(&self, action: &Action) -> AIResult<Command> {
        use crate::engine::Point;

        match action.action.as_str() {
            "click" => {
                // Would need screen coordinates from element finder
                Ok(Command::Click { x: 0, y: 0 })
            }
            "input" => {
                let text = action.text.as_ref().ok_or_else(|| {
                    AIError::ActionExecution("Missing text for input action".to_string())
                })?;
                Ok(Command::InputText { text: text.clone() })
            }
            "swipe" => {
                // Would need to convert direction to coordinates
                Ok(Command::Swipe {
                    from: Point { x: 500, y: 1000 },
                    to: Point { x: 500, y: 200 },
                    duration_ms: action.duration.unwrap_or(300),
                })
            }
            _ => Err(AIError::ActionExecution(format!(
                "Unsupported action: {}",
                action.action
            ))),
        }
    }
}
