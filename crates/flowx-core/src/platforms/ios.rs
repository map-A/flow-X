use crate::engine::{AsyncCommandExecutor, Command, CommandError, CommandResult};
use async_trait::async_trait;
use serde_json::json;

/// iOS WebDriverAgent 客户端
pub struct IOSClient {
    device_ip: String,
    wda_port: u16,
}

impl IOSClient {
    pub fn new(device_ip: String) -> Self {
        Self {
            device_ip,
            wda_port: 8100,
        }
    }

    fn base_url(&self) -> String {
        format!("http://{}:{}", self.device_ip, self.wda_port)
    }
}

#[async_trait]
impl AsyncCommandExecutor for IOSClient {
    async fn execute_async(&self, command: Command) -> Result<CommandResult, CommandError> {
        let client = reqwest::Client::new();

        match command {
            Command::Click { x, y } => {
                let url = format!("{}/session/1/wda/tap/0", self.base_url());
                let body = json!({ "x": x, "y": y });

                client.post(&url).json(&body).send().await.map_err(|e| {
                    CommandError::PlatformError {
                        message: format!("iOS tap failed: {}", e),
                    }
                })?;

                Ok(CommandResult::Success)
            }

            Command::Swipe {
                from,
                to,
                duration_ms,
            } => {
                let url = format!("{}/session/1/wda/touch/perform", self.base_url());
                let body = json!({
                    "actions": [{
                        "action": "press",
                        "options": { "x": from.x, "y": from.y }
                    }, {
                        "action": "wait",
                        "options": { "ms": duration_ms }
                    }, {
                        "action": "moveTo",
                        "options": { "x": to.x, "y": to.y }
                    }, {
                        "action": "release"
                    }]
                });

                client.post(&url).json(&body).send().await.map_err(|e| {
                    CommandError::PlatformError {
                        message: format!("iOS swipe failed: {}", e),
                    }
                })?;

                Ok(CommandResult::Success)
            }

            Command::InputText { text } => {
                let url = format!("{}/session/1/wda/keys", self.base_url());
                let body = json!({ "value": [text] });

                client.post(&url).json(&body).send().await.map_err(|e| {
                    CommandError::PlatformError {
                        message: format!("iOS input failed: {}", e),
                    }
                })?;

                Ok(CommandResult::Success)
            }

            _ => Ok(CommandResult::Success),
        }
    }
}
