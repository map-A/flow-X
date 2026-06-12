use crate::engine::{Command, CommandError, CommandResult};
use async_trait::async_trait;

/// 统一的平台接口
#[async_trait]
pub trait Platform: Send + Sync {
    fn name(&self) -> &str;
    async fn initialize(&mut self) -> Result<(), CommandError>;
    async fn execute(&self, command: Command) -> Result<CommandResult, CommandError>;
    fn check_permission(&self, permission: &str) -> bool;
    async fn request_permission(&self, permission: &str) -> Result<bool, CommandError>;
    async fn cleanup(&mut self) -> Result<(), CommandError>;
}

pub trait PlatformCapabilities {
    fn supports_accessibility(&self) -> bool;
    fn supports_screenshot(&self) -> bool;
    fn supports_ocr(&self) -> bool;
    fn supports_gesture(&self) -> bool;
}

// Platform implementations
pub mod android;
pub use android::AndroidClient;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "ios")]
pub mod ios;

// Re-export platform clients
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "ios")]
pub use ios::*;
