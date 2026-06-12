use crate::engine::{Command, CommandResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 多设备管理器
pub struct MultiDevice {
    devices: Arc<RwLock<HashMap<String, DeviceHandle>>>,
}

/// 设备句柄
pub struct DeviceHandle {
    pub id: String,
    pub platform: String,
    pub status: DeviceStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceStatus {
    Connected,
    Busy,
    Disconnected,
}

impl MultiDevice {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册设备
    pub async fn register(&self, id: String, platform: String) {
        let mut devices = self.devices.write().await;
        devices.insert(
            id.clone(),
            DeviceHandle {
                id,
                platform,
                status: DeviceStatus::Connected,
            },
        );
    }

    /// 注销设备
    pub async fn unregister(&self, id: &str) {
        let mut devices = self.devices.write().await;
        devices.remove(id);
    }

    /// 获取所有设备
    pub async fn list(&self) -> Vec<DeviceHandle> {
        let devices = self.devices.read().await;
        devices.values().cloned().collect()
    }

    /// 并发执行命令
    pub async fn execute_parallel(
        &self,
        device_ids: Vec<String>,
        command: Command,
    ) -> Vec<(String, Result<CommandResult, String>)> {
        let mut tasks = Vec::new();

        for device_id in device_ids {
            let _cmd = command.clone();
            let id = device_id.clone();

            let task = tokio::spawn(async move {
                // 模拟执行
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                (id.clone(), Ok(CommandResult::Success))
            });

            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }

        results
    }

    /// 设置设备状态
    pub async fn set_status(&self, id: &str, status: DeviceStatus) {
        let mut devices = self.devices.write().await;
        if let Some(device) = devices.get_mut(id) {
            device.status = status;
        }
    }
}

impl Clone for DeviceHandle {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            platform: self.platform.clone(),
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_device() {
        let multi = MultiDevice::new();
        multi
            .register("device1".to_string(), "android".to_string())
            .await;

        let devices = multi.list().await;
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].id, "device1");
    }

    #[tokio::test]
    async fn test_unregister_device() {
        let multi = MultiDevice::new();
        multi
            .register("device1".to_string(), "android".to_string())
            .await;
        multi.unregister("device1").await;

        let devices = multi.list().await;
        assert_eq!(devices.len(), 0);
    }

    #[tokio::test]
    async fn test_parallel_execution() {
        let multi = MultiDevice::new();
        multi
            .register("device1".to_string(), "android".to_string())
            .await;
        multi
            .register("device2".to_string(), "android".to_string())
            .await;

        let results = multi
            .execute_parallel(
                vec!["device1".to_string(), "device2".to_string()],
                Command::Click { x: 100, y: 200 },
            )
            .await;

        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_set_status() {
        let multi = MultiDevice::new();
        multi
            .register("device1".to_string(), "android".to_string())
            .await;
        multi.set_status("device1", DeviceStatus::Busy).await;

        let devices = multi.list().await;
        assert_eq!(devices[0].status, DeviceStatus::Busy);
    }
}
