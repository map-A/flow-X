use crate::engine::{AsyncCommandExecutor, Command, CommandError, CommandResult};
use async_trait::async_trait;

/// Windows UI Automation 客户端
pub struct WindowsClient;

impl WindowsClient {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AsyncCommandExecutor for WindowsClient {
    async fn execute_async(&self, command: Command) -> Result<CommandResult, CommandError> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command as ProcessCommand;

            match command {
                Command::Click { x, y } => {
                    // 使用 PowerShell 执行点击
                    ProcessCommand::new("powershell")
                        .args(&[
                            "-Command",
                            &format!(
                                "Add-Type -AssemblyName System.Windows.Forms; \
                                 [System.Windows.Forms.Cursor]::Position = New-Object System.Drawing.Point({}, {}); \
                                 [System.Windows.Forms.SendKeys]::SendWait('{{ENTER}}')",
                                x, y
                            )
                        ])
                        .output()
                        .map_err(|e| CommandError::PlatformError { message: e.to_string() })?;
                    Ok(CommandResult::Success)
                }
                Command::InputText { text } => {
                    ProcessCommand::new("powershell")
                        .args(&[
                            "-Command",
                            &format!(
                                "Add-Type -AssemblyName System.Windows.Forms; \
                                     [System.Windows.Forms.SendKeys]::SendWait('{}')",
                                text
                            ),
                        ])
                        .output()
                        .map_err(|e| CommandError::PlatformError {
                            message: e.to_string(),
                        })?;
                    Ok(CommandResult::Success)
                }
                _ => Ok(CommandResult::Success),
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err(CommandError::PlatformError {
                message: "Windows platform not available".into(),
            })
        }
    }
}
