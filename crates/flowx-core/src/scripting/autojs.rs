use crate::engine::{AsyncCommandExecutor, CommandError, CommandResult};
use crate::scripting::JsRuntime;
use std::sync::Arc;

/// AutoJS compatible runtime
pub struct AutoJsRuntime {
    js: JsRuntime,
    executor: Option<Arc<dyn AsyncCommandExecutor>>,
}

impl AutoJsRuntime {
    pub fn new() -> Self {
        let mut js = JsRuntime::new();

        // Register global functions
        js.register_function("click", |_args| Ok(CommandResult::Success));
        js.register_function("swipe", |_args| Ok(CommandResult::Success));
        js.register_function("sleep", |_args| Ok(CommandResult::Success));
        js.register_function("text", |_args| Ok(CommandResult::Success));
        js.register_function("screenshot", |_args| Ok(CommandResult::Success));

        Self { js, executor: None }
    }

    pub fn set_executor(&mut self, executor: Arc<dyn AsyncCommandExecutor>) {
        self.executor = Some(executor);
    }

    /// Execute AutoJS script
    pub fn execute(&mut self, script: &str) -> Result<String, CommandError> {
        self.js.execute(script)
    }

    /// Get embedded AutoJS library code
    pub fn get_autojs_lib() -> &'static str {
        include_str!("../../../../runtime/autojs/global.js")
    }
}

/// AutoJS API documentation
pub mod api {
    /// Global functions available in AutoJS scripts
    pub const GLOBAL_FUNCTIONS: &[&str] = &[
        "click(x, y)",
        "swipe(x1, y1, x2, y2, duration)",
        "sleep(ms)",
        "text(str).click()",
        "desc(str).click()",
        "id(str).click()",
        "screenshot()",
        "toast(msg)",
        "log(msg)",
    ];

    /// App module functions
    pub const APP_FUNCTIONS: &[&str] = &[
        "app.launch(package)",
        "app.launchApp(name)",
        "app.getAppName(package)",
        "app.openAppSetting(package)",
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autojs_runtime_creation() {
        let runtime = AutoJsRuntime::new();
        assert!(runtime.executor.is_none());
    }

    #[test]
    fn test_autojs_api_constants() {
        assert!(api::GLOBAL_FUNCTIONS.len() > 0);
        assert!(api::APP_FUNCTIONS.len() > 0);
    }
}
