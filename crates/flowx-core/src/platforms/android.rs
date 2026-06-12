use crate::engine::{AsyncCommandExecutor, Command as EngineCommand, CommandError, CommandResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tungstenite::{connect, Message};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use url::Url;

type WsStream = tungstenite::WebSocket<MaybeTlsStream<TcpStream>>;

/// Android client that communicates with the Android app via WebSocket
/// The Android app runs an Accessibility Service with WebSocket server on port 6789
pub struct AndroidClient {
    device_id: String,
    ws_url: String,
    socket: Mutex<Option<WsStream>>,
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
    pub fn new(device_id: String) -> Self {
        let ws_url = if device_id.starts_with("android://") {
            let addr = device_id.trim_start_matches("android://");
            format!("ws://{}", addr)
        } else {
            format!("ws://localhost:6789")
        };

        Self {
            device_id,
            ws_url,
            socket: Mutex::new(None),
        }
    }

    /// Ensure WebSocket connection is established
    fn ensure_connected(&self) -> Result<(), CommandError> {
        let mut socket_guard = self.socket.lock().unwrap();

        // Check if already connected
        if socket_guard.is_some() {
            return Ok(());
        }

        // Parse URL
        let url = Url::parse(&self.ws_url).map_err(|e| CommandError::PlatformError {
            message: format!("Invalid WebSocket URL: {}", e),
        })?;

        // Connect to WebSocket
        let (socket, _response) = connect(url).map_err(|e| CommandError::PlatformError {
            message: format!("Failed to connect to WebSocket: {}", e),
        })?;

        *socket_guard = Some(socket);
        eprintln!("[AndroidClient] Connected to {}", self.ws_url);

        Ok(())
    }

    /// Send a command via WebSocket and get response (reuses connection)
    fn send_command(&self, command: Command) -> Result<Response, CommandError> {
        // Ensure connected
        self.ensure_connected()?;

        let mut socket_guard = self.socket.lock().unwrap();

        // Get mutable reference to socket
        let socket = match socket_guard.as_mut() {
            Some(s) => s,
            None => return Err(CommandError::PlatformError {
                message: "Socket not connected".to_string(),
            }),
        };

        // Serialize command to JSON
        let json = serde_json::to_string(&command).map_err(|e| CommandError::PlatformError {
            message: format!("Failed to serialize command: {}", e),
        })?;

        // Send command
        if let Err(e) = socket.send(Message::Text(json)) {
            // Connection failed, clear socket
            *socket_guard = None;
            return Err(CommandError::PlatformError {
                message: format!("Failed to send WebSocket message: {}", e),
            });
        }

        // Read response
        let msg = match socket.read() {
            Ok(m) => m,
            Err(e) => {
                // Connection failed, clear socket
                *socket_guard = None;
                return Err(CommandError::PlatformError {
                    message: format!("Failed to read WebSocket response: {}", e),
                });
            }
        };

        // Parse response
        let response_text = msg.to_text().map_err(|e| CommandError::PlatformError {
            message: format!("Invalid WebSocket response format: {}", e),
        })?;

        let response: Response =
            serde_json::from_str(response_text).map_err(|e| CommandError::PlatformError {
                message: format!("Failed to parse response JSON: {}", e),
            })?;

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
                Ok((1080, 1920))
            }
        } else {
            Err(CommandError::PlatformError {
                message: "Failed to get screen size".to_string(),
            })
        }
    }

    /// Take a screenshot
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
                let base64_str = data.as_str().ok_or_else(|| CommandError::PlatformError {
                    message: "Screenshot data is not a string".to_string(),
                })?;

                use base64::{Engine as _, engine::general_purpose};
                let png_data = general_purpose::STANDARD
                    .decode(base64_str)
                    .map_err(|e| CommandError::PlatformError {
                        message: format!("Failed to decode screenshot: {}", e),
                    })?;

                Ok(png_data)
            } else {
                Err(CommandError::PlatformError {
                    message: "Screenshot data missing".to_string(),
                })
            }
        } else {
            Err(CommandError::PlatformError {
                message: "Screenshot failed".to_string(),
            })
        }
    }

    /// Find element by text
    pub fn find_element(&self, text: &str) -> Result<Option<(i32, i32)>, CommandError> {
        let command = Command {
            cmd_type: "find_element".to_string(),
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
                let info: ElementInfo =
                    serde_json::from_value(data).map_err(|e| CommandError::PlatformError {
                        message: format!("Failed to parse element info: {}", e),
                    })?;
                Ok(Some((info.x, info.y)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl AsyncCommandExecutor for AndroidClient {
    async fn execute_async(&self, command: EngineCommand) -> Result<CommandResult, CommandError> {
        let cmd = match command {
            EngineCommand::Click { x, y } => Command {
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
            },
            EngineCommand::Swipe { from, to, duration_ms } => Command {
                cmd_type: "swipe".to_string(),
                x: None,
                y: None,
                x1: Some(from.x),
                y1: Some(from.y),
                x2: Some(to.x),
                y2: Some(to.y),
                duration: Some(duration_ms),
                text: None,
                key: None,
            },
            EngineCommand::InputText { text } => Command {
                cmd_type: "input_text".to_string(),
                x: None,
                y: None,
                x1: None,
                y1: None,
                x2: None,
                y2: None,
                duration: None,
                text: Some(text),
                key: None,
            },
            EngineCommand::PressKey { key: key_val } => Command {
                cmd_type: "press_key".to_string(),
                x: None,
                y: None,
                x1: None,
                y1: None,
                x2: None,
                y2: None,
                duration: None,
                text: None,
                key: Some(format!("{:?}", key_val)),
            },
            _ => {
                return Err(CommandError::InvalidArgument {
                    param: "command".to_string(),
                    reason: format!("Unsupported command: {:?}", command),
                })
            }
        };

        let response = self.send_command(cmd)?;

        if response.status == "success" {
            Ok(CommandResult::Success)
        } else {
            Err(CommandError::PlatformError {
                message: "Command failed".to_string(),
            })
        }
    }
}
