//! AI Agent - main interface for natural language automation

use crate::ai::executor::{ActionExecutor, ExecutionResult};
use crate::ai::models::{BaseModel, MultimodalModel};
use crate::ai::planner::TaskPlanner;
use crate::ai::vision_model::VisionModel;
use crate::ai::{AIError, AIResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub steps: Vec<ExecutionResult>,
    pub error: Option<String>,
}

pub struct AIAgent {
    vision: VisionModel,
    planner: TaskPlanner,
    executor: ActionExecutor,
    max_steps: usize,
}

impl AIAgent {
    pub fn from_config(config_path: &str) -> AIResult<Self> {
        let model = MultimodalModel::from_file(config_path)?;

        let vision = VisionModel::new(Box::new(MultimodalModel::from_file(config_path)?));
        let planner = TaskPlanner::new(Box::new(MultimodalModel::from_file(config_path)?));
        let executor = ActionExecutor::new();

        Ok(Self {
            vision,
            planner,
            executor,
            max_steps: 20,
        })
    }

    pub fn from_default_config() -> AIResult<Self> {
        Self::from_config("flowx.toml")
    }

    pub fn execute(&self, instruction: &str, screenshot: &[u8]) -> AIResult<TaskResult> {
        let screen_context = self.vision.understand_screen(screenshot)?;
        let actions = self.planner.plan(instruction, &screen_context)?;

        if actions.len() > self.max_steps {
            return Err(AIError::ActionExecution(format!(
                "Too many steps: {} > {}",
                actions.len(),
                self.max_steps
            )));
        }

        let results = self.executor.execute_sequence(&actions);
        let success = results.iter().all(|r| r.success);
        let error = results
            .iter()
            .find(|r| !r.success)
            .and_then(|r| r.error.clone());

        Ok(TaskResult {
            success,
            steps: results,
            error,
        })
    }

    pub fn ask(&self, question: &str, screenshot: &[u8]) -> AIResult<String> {
        let model = MultimodalModel::from_default_config()?;
        model.vision_chat(question, screenshot, 0.7)
    }

    pub fn set_max_steps(&mut self, max_steps: usize) {
        self.max_steps = max_steps;
    }
}

impl MultimodalModel {
    fn from_default_config() -> AIResult<Self> {
        Self::from_file("flowx.toml")
    }
}
