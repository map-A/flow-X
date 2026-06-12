use async_trait::async_trait;
use flowx_core::engine::{AsyncCommandExecutor, Command, CommandError, CommandResult};
use flowx_core::platforms::AndroidClient;
use pyo3::prelude::*;

/// 支持多平台的执行器
pub enum MockExecutor {
    Android(AndroidClient),
    #[cfg(target_os = "macos")]
    MacOS(flowx_core::platforms::MacOSClient),
    #[cfg(target_os = "windows")]
    Windows(flowx_core::platforms::WindowsClient),
    #[cfg(target_os = "ios")]
    IOS(flowx_core::platforms::IOSClient),
}

impl MockExecutor {
    pub fn new(device_id: String) -> Self {
        #[cfg(target_os = "macos")]
        {
            if device_id == "macos" {
                return Self::MacOS(flowx_core::platforms::MacOSClient::new().unwrap());
            }
        }

        #[cfg(target_os = "windows")]
        {
            if device_id == "windows" {
                return Self::Windows(flowx_core::platforms::WindowsClient::new());
            }
        }

        #[cfg(target_os = "ios")]
        {
            if device_id.starts_with("http://") {
                return Self::IOS(flowx_core::platforms::IOSClient::new(device_id).unwrap());
            }
        }

        // Android devices use WebSocket (android://host:port or just device id)
        Self::Android(AndroidClient::new(device_id))
    }
}

#[async_trait]
impl AsyncCommandExecutor for MockExecutor {
    async fn execute_async(&self, command: Command) -> Result<CommandResult, CommandError> {
        match self {
            Self::Android(client) => client.execute_async(command).await,
            #[cfg(target_os = "macos")]
            Self::MacOS(client) => client.execute_async(command).await,
            #[cfg(target_os = "windows")]
            Self::Windows(client) => client.execute_async(command).await,
            #[cfg(target_os = "ios")]
            Self::IOS(client) => client.execute_async(command).await,
        }
    }
}

/// Python wrapper for MockExecutor
#[pyclass(name = "MockExecutor")]
pub struct PyMockExecutor {
    executor: MockExecutor,
}

#[pymethods]
impl PyMockExecutor {
    #[new]
    fn new(device_id: String) -> Self {
        Self {
            executor: MockExecutor::new(device_id),
        }
    }
}
