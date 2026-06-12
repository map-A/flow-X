/// Scripting runtime module
///
/// Provides interfaces for Python and JavaScript runtime integration
use crate::engine::{Command, CommandError, CommandResult};

pub mod autojs;
pub mod js_runtime;

pub use autojs::AutoJsRuntime;
pub use js_runtime::JsRuntime;

/// Scripting runtime trait for executing scripts
pub trait ScriptingRuntime: Send + Sync {
    /// Execute script and return result
    fn execute_script(&self, source: &str) -> Result<CommandResult, CommandError>;

    /// Register command handler for script access
    fn register_command_handler(
        &mut self,
        handler: Box<dyn Fn(Command) -> Result<CommandResult, CommandError> + Send + Sync>,
    );
}
