use crate::engine::{CommandError, CommandResult};
use std::collections::HashMap;

/// Simple JavaScript runtime mock
pub struct JsRuntime {
    globals: HashMap<String, String>,
}

impl JsRuntime {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    /// Execute JavaScript code
    pub fn execute(&mut self, script: &str) -> Result<String, CommandError> {
        // Mock implementation - just return success
        Ok(format!("Executed: {}", script.lines().next().unwrap_or("")))
    }

    /// Register native function
    pub fn register_function<F>(&mut self, name: &str, _callback: F)
    where
        F: Fn(&[String]) -> Result<CommandResult, CommandError> + 'static,
    {
        self.globals
            .insert(name.to_string(), "function".to_string());
    }

    /// Evaluate expression
    pub fn eval(&self, expr: &str) -> Result<String, CommandError> {
        Ok(format!("Result: {}", expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_runtime_basic() {
        let mut runtime = JsRuntime::new();
        let result = runtime.execute("console.log('hello')");
        assert!(result.is_ok());
    }
}
