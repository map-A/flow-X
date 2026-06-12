use crate::engine::{AsyncCommandExecutor, Command as EngineCommand, CommandError, CommandResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tungstenite::{connect, Message};
use url::Url;

/// Android client that communicates with the Android app via WebSocket
/// The Android app runs an Accessibility Service with WebSocket server on port 6789
pub struct AndroidClient {
    device_id: String,
    ws_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    #[serde(rename = "type")]
    cmd_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x1: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y1: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x2: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y2: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ElementInfo {
    x: i32,
    y: i32,
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScreenInfo {
    width: u32,
    height: u32,
}

impl AndroidClient {
    /// Create a new Android client
    /// device_id format: "android://host:port" (e.g., "android://localhost:6789")
    /// Note: The Android app WebSocket server runs on port 6789 by default
    pub fn new(device_id: String) -> Self {
        let ws_url = if device_id.starts_with("android://") {
            let addr = device_id.trim_start_matches("android://");
            format!("ws://{}", addr)
        } else {
            // Default to localhost:6789
            format!("ws://localhost:6789")
        };

        Self { device_id, ws_url }
    }

    /// Send a command via WebSocket and get response
    fn send_command(&self, command: Command) -> Result<Response, CommandError> {
        // Parse URL
        let url = Url::parse(&self.ws_url).map_err(|e| CommandError::PlatformError {
            message: format!("Invalid WebSocket URL: {}", e),
        })?;

        // Connect to WebSocket
        let (mut socket, _response) = connect(url).map_err(|e| CommandError::PlatformError {
            message: format!("Failed to connect to WebSocket: {}", e),
        })?;

        // Serialize command to JSON
        let json = serde_json::to_string(&command).map_err(|e| CommandError::PlatformError {
            message: format!("Failed to serialize command: {}", e),
        })?;

        // Send command
        socket
            .write_message(Message::Text(json))
            .map_err(|e| CommandError::PlatformError {
                message: format!("Failed to send WebSocket message: {}", e),
            })?;

        // Read response
        let msg = socket
            .read_message()
            .map_err(|e| CommandError::PlatformError {
                message: format!("Failed to read WebSocket response: {}", e),
            })?;

        // Parse response
        let response_text = msg.to_text().map_err(|e| CommandError::PlatformError {
            message: format!("Invalid WebSocket response format: {}", e),
        })?;

        let response: Response =
            serde_json::from_str(response_text).map_err(|e| CommandError::PlatformError {
                message: format!("Failed to parse response JSON: {}", e),
            })?;

        // Close connection
        let _ = socket.close(None);

        Ok(response)
    }

    /// Get device screen size
    pub fn get_screen_size(&self) -> Result<(u32, u32), CommandError> {
        let command = Command {
            cmd_type: "screen_size".to_string(),
            x: None,
            y: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: None,
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            if let Some(data) = response.data {
                let info: ScreenInfo =
                    serde_json::from_value(data).map_err(|e| CommandError::PlatformError {
                        message: format!("Failed to parse screen info: {}", e),
                    })?;
                Ok((info.width, info.height))
            } else {
                // Fallback to default resolution
                Ok((1080, 1920))
            }
        } else {
            Err(CommandError::PlatformError {
                message: format!("Get screen size failed: {}", response.status),
            })
        }
    }

    /// Take screenshot
    /// Returns PNG image data
    pub fn screenshot(&self) -> Result<Vec<u8>, CommandError> {
        let command = Command {
            cmd_type: "screenshot".to_string(),
            x: None,
            y: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: None,
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            if let Some(data) = response.data {
                // Expect base64 encoded PNG
                let base64_str = data.as_str().ok_or_else(|| CommandError::PlatformError {
                    message: "Screenshot data is not a string".to_string(),
                })?;

                let png_data =
                    base64::decode(base64_str).map_err(|e| CommandError::PlatformError {
                        message: format!("Failed to decode screenshot: {}", e),
                    })?;

                Ok(png_data)
            } else {
                Err(CommandError::PlatformError {
                    message: "No screenshot data returned".to_string(),
                })
            }
        } else {
            Err(CommandError::PlatformError {
                message: format!("Screenshot failed: {}", response.status),
            })
        }
    }

    /// Tap at coordinates
    pub fn tap(&self, x: i32, y: i32) -> Result<(), CommandError> {
        let command = Command {
            cmd_type: "click".to_string(),
            x: Some(x),
            y: Some(y),
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: None,
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            Ok(())
        } else {
            Err(CommandError::PlatformError {
                message: format!("Tap failed: {}", response.status),
            })
        }
    }

    /// Long press at coordinates
    pub fn long_press(&self, x: i32, y: i32, duration_ms: u64) -> Result<(), CommandError> {
        // Use swipe with same start/end point for long press
        self.swipe(x, y, x, y, duration_ms)
    }

    /// Swipe from one point to another
    pub fn swipe(
        &self,
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
        duration_ms: u64,
    ) -> Result<(), CommandError> {
        let command = Command {
            cmd_type: "swipe".to_string(),
            x: None,
            y: None,
            x1: Some(from_x),
            y1: Some(from_y),
            x2: Some(to_x),
            y2: Some(to_y),
            duration: Some(duration_ms),
            text: None,
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            Ok(())
        } else {
            Err(CommandError::PlatformError {
                message: format!("Swipe failed: {}", response.status),
            })
        }
    }

    /// Input text
    pub fn input_text(&self, text: &str) -> Result<(), CommandError> {
        let command = Command {
            cmd_type: "input_text".to_string(),
            x: None,
            y: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: Some(text.to_string()),
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            Ok(())
        } else {
            Err(CommandError::PlatformError {
                message: format!("Input text failed: {}", response.status),
            })
        }
    }

    /// Press key (e.g., "BACK", "HOME", "ENTER")
    pub fn press_key(&self, key: &str) -> Result<(), CommandError> {
        let command = Command {
            cmd_type: "press_key".to_string(),
            x: None,
            y: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: None,
            key: Some(key.to_string()),
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            Ok(())
        } else {
            Err(CommandError::PlatformError {
                message: format!("Press key failed: {}", response.status),
            })
        }
    }

    /// Find element by text and return its center coordinates
    pub fn find_element(&self, text: &str) -> Result<(i32, i32), CommandError> {
        let command = Command {
            cmd_type: "find".to_string(),
            x: None,
            y: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: Some(text.to_string()),
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            if let Some(data) = response.data {
                let element: ElementInfo =
                    serde_json::from_value(data).map_err(|e| CommandError::PlatformError {
                        message: format!("Failed to parse element info: {}", e),
                    })?;
                Ok((element.x, element.y))
            } else {
                Err(CommandError::PlatformError {
                    message: "Element found but no data returned".to_string(),
                })
            }
        } else if response.status == "not_found" {
            Err(CommandError::PlatformError {
                message: format!("Element not found: {}", text),
            })
        } else {
            Err(CommandError::PlatformError {
                message: format!("Find element failed: {}", response.status),
            })
        }
    }

    /// Get current package name
    pub fn current_package(&self) -> Result<String, CommandError> {
        let command = Command {
            cmd_type: "current_package".to_string(),
            x: None,
            y: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: None,
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            if let Some(data) = response.data {
                let package = data.as_str().ok_or_else(|| CommandError::PlatformError {
                    message: "Package name is not a string".to_string(),
                })?;
                Ok(package.to_string())
            } else {
                Err(CommandError::PlatformError {
                    message: "No package name returned".to_string(),
                })
            }
        } else {
            Err(CommandError::PlatformError {
                message: format!("Get current package failed: {}", response.status),
            })
        }
    }

    /// Get current activity name
    pub fn current_activity(&self) -> Result<String, CommandError> {
        let command = Command {
            cmd_type: "current_activity".to_string(),
            x: None,
            y: None,
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: None,
            key: None,
        };

        let response = self.send_command(command)?;

        if response.status == "success" {
            if let Some(data) = response.data {
                let activity = data.as_str().ok_or_else(|| CommandError::PlatformError {
                    message: "Activity name is not a string".to_string(),
                })?;
                Ok(activity.to_string())
            } else {
                Err(CommandError::PlatformError {
                    message: "No activity name returned".to_string(),
                })
            }
        } else {
            Err(CommandError::PlatformError {
                message: format!("Get current activity failed: {}", response.status),
            })
        }
    }
}

#[async_trait]
impl AsyncCommandExecutor for AndroidClient {
    async fn execute_async(&self, command: EngineCommand) -> Result<CommandResult, CommandError> {
        match command {
            EngineCommand::Click { x, y } => {
                self.tap(x, y)?;
                Ok(CommandResult::Success)
            }
            EngineCommand::Swipe {
                from,
                to,
                duration_ms,
            } => {
                self.swipe(from.x, from.y, to.x, to.y, duration_ms as u64)?;
                Ok(CommandResult::Success)
            }
            EngineCommand::InputText { text } => {
                self.input_text(&text)?;
                Ok(CommandResult::Success)
            }
            EngineCommand::Screenshot { .. } => {
                let png_data = self.screenshot()?;
                Ok(CommandResult::Image(crate::engine::Image {
                    data: png_data,
                    width: 1080,
                    height: 1920,
                    format: crate::engine::ImageFormat::RGB,
                }))
            }
            EngineCommand::GetScreenSize => {
                let (width, height) = self.get_screen_size()?;
                Ok(CommandResult::Size(width, height))
            }
            EngineCommand::PressKey { key } => {
                let key_str = match key {
                    crate::engine::Key::Enter => "ENTER",
                    crate::engine::Key::Back => "BACK",
                    crate::engine::Key::Delete => "DEL",
                    crate::engine::Key::Home => "HOME",
                    crate::engine::Key::Menu => "MENU",
                    crate::engine::Key::VolumeUp => "VOLUME_UP",
                    crate::engine::Key::VolumeDown => "VOLUME_DOWN",
                    crate::engine::Key::Up => "DPAD_UP",
                    crate::engine::Key::Down => "DPAD_DOWN",
                    crate::engine::Key::Left => "DPAD_LEFT",
                    crate::engine::Key::Right => "DPAD_RIGHT",
                    crate::engine::Key::Other(ref s) => s.as_str(),
                };
                self.press_key(key_str)?;
                Ok(CommandResult::Success)
            }
            EngineCommand::FindElement { selector } => {
                let search_text = match selector {
                    crate::engine::Selector::Text(text) => text,
                    crate::engine::Selector::Id(id) => id,
                    _ => String::new(),
                };
                let (x, y) = self.find_element(&search_text)?;
                Ok(CommandResult::Element(crate::engine::Element {
                    id: format!("{}_{}", x, y),
                    text: Some(search_text),
                    bounds: crate::engine::Rect {
                        x,
                        y,
                        width: 0,
                        height: 0,
                    },
                    class_name: None,
                    clickable: true,
                }))
            }
            _ => Ok(CommandResult::Success),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_client_creation() {
        let client = AndroidClient::new("android://localhost:6789".to_string());
        assert_eq!(client.ws_url, "ws://localhost:6789");
    }

    #[test]
    fn test_command_serialization() {
        let command = Command {
            cmd_type: "click".to_string(),
            x: Some(100),
            y: Some(200),
            x1: None,
            y1: None,
            x2: None,
            y2: None,
            duration: None,
            text: None,
            key: None,
        };

        let json = serde_json::to_string(&command).unwrap();
        assert!(json.contains("\"type\":\"click\""));
        assert!(json.contains("\"x\":100"));
        assert!(json.contains("\"y\":200"));
    }
}
