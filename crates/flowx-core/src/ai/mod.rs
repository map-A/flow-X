/// AI module for natural language automation
///
/// Provides intelligent automation with vision understanding and task planning.
mod agent;
mod executor;
mod models;
mod planner;
mod prompts;
mod utils;
mod vision_model;

pub use agent::{AIAgent, TaskResult};
pub use executor::ExecutionResult;
pub use models::{ModelConfig, ModelError};

use crate::engine::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("Model API error: {0}")]
    ModelAPI(String),

    #[error("Action parse error: {0}")]
    ActionParse(String),

    #[error("Action execution error: {0}")]
    ActionExecution(String),

    #[error("Command error: {0}")]
    Command(#[from] CommandError),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type AIResult<T> = Result<T, AIError>;
