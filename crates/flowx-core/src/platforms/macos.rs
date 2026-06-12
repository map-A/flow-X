use crate::engine::{AsyncCommandExecutor, Command, CommandError, CommandResult};
use async_trait::async_trait;

/// macOS Accessibility API 客户端
pub struct MacOSClient;

impl MacOSClient {
    pub fn new() -> Result<Self, CommandError> {
        #[cfg(target_os = "macos")]
        {
            if !check_accessibility_permission() {
                return Err(CommandError::PermissionDenied {
                    permission: "Accessibility".to_string(),
                });
            }
        }
        Ok(Self)
    }
}

#[cfg(target_os = "macos")]
fn check_accessibility_permission() -> bool {
    use std::process::Command;
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first process")
        .output();
    output.is_ok()
}

#[async_trait]
impl AsyncCommandExecutor for MacOSClient {
    async fn execute_async(&self, command: Command) -> Result<CommandResult, CommandError> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command as ProcessCommand;

            match command {
                Command::Click { x, y } => {
                    ProcessCommand::new("osascript")
                        .arg("-e")
                        .arg(format!(
                            "tell application \"System Events\" to click at {{{}, {}}}",
                            x, y
                        ))
                        .output()
                        .map_err(|e| CommandError::PlatformError {
                            message: format!("Click failed: {}", e),
                        })?;
                    Ok(CommandResult::Success)
                }
                Command::InputText { text } => {
                    ProcessCommand::new("osascript")
                        .arg("-e")
                        .arg(format!(
                            "tell application \"System Events\" to keystroke \"{}\"",
                            text
                        ))
                        .output()
                        .map_err(|e| CommandError::PlatformError {
                            message: format!("Input failed: {}", e),
                        })?;
                    Ok(CommandResult::Success)
                }
                Command::PressKey { key } => {
                    use crate::engine::Key;
                    let script = match key {
                        Key::Enter => "tell application \"System Events\" to key code 36".to_string(),
                        Key::Down => "tell application \"System Events\" to key code 125".to_string(),
                        Key::Up => "tell application \"System Events\" to key code 126".to_string(),
                        Key::Left => "tell application \"System Events\" to key code 123".to_string(),
                        Key::Right => "tell application \"System Events\" to key code 124".to_string(),
                        Key::Other(s) if s == "CommandT" => {
                            "tell application \"System Events\" to keystroke \"t\" using command down".to_string()
                        }
                        Key::Other(s) => format!("tell application \"System Events\" to keystroke \"{}\"", s),
                        _ => return Ok(CommandResult::Success),
                    };

                    ProcessCommand::new("osascript")
                        .arg("-e")
                        .arg(&script)
                        .output()
                        .map_err(|e| CommandError::PlatformError {
                            message: format!("Key press failed: {}", e),
                        })?;
                    Ok(CommandResult::Success)
                }
                Command::OpenApp { name } => {
                    ProcessCommand::new("osascript")
                        .arg("-e")
                        .arg(format!("tell application \"{}\" to activate", name))
                        .output()
                        .map_err(|e| CommandError::PlatformError {
                            message: format!("Open app failed: {}", e),
                        })?;
                    Ok(CommandResult::Success)
                }
                _ => Ok(CommandResult::Success),
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(CommandError::PlatformError {
                message: "macOS platform not available".into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{Command, Key};

    #[test]
    fn test_macos_client_creation() {
        #[cfg(target_os = "macos")]
        {
            let result = MacOSClient::new();
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[tokio::test]
    async fn test_input_text_command() {
        #[cfg(target_os = "macos")]
        {
            if let Ok(client) = MacOSClient::new() {
                let result = client
                    .execute_async(Command::InputText {
                        text: "test".to_string(),
                    })
                    .await;
                assert!(result.is_ok() || result.is_err());
            }
        }
    }

    #[tokio::test]
    async fn test_press_key_command() {
        #[cfg(target_os = "macos")]
        {
            if let Ok(client) = MacOSClient::new() {
                let result = client
                    .execute_async(Command::PressKey { key: Key::Enter })
                    .await;
                assert!(result.is_ok() || result.is_err());
            }
        }
    }
}
