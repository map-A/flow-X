use super::command::Command;
use super::types::{CommandError, CommandResult};
use async_trait::async_trait;

pub trait CommandExecutor {
    fn execute(&self, command: Command) -> Result<CommandResult, CommandError>;
}

#[async_trait]
pub trait AsyncCommandExecutor: Send + Sync {
    async fn execute_async(&self, command: Command) -> Result<CommandResult, CommandError>;

    async fn execute_batch(
        &self,
        commands: Vec<Command>,
    ) -> Vec<Result<CommandResult, CommandError>> {
        let mut results = Vec::with_capacity(commands.len());
        for command in commands {
            results.push(self.execute_async(command).await);
        }
        results
    }
}
