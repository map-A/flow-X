use crate::executor::MockExecutor;
use flowx_core::device::Device;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass(name = "Device")]
pub struct PyDevice {
    device: Device<MockExecutor>,
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyDevice {
    #[staticmethod]
    fn connect(device_id: &str) -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;

        let executor = MockExecutor::new(device_id.to_string());
        let device = Device::new(executor);

        Ok(PyDevice { device, runtime })
    }

    fn click(&self, x: i32, y: i32) -> PyResult<()> {
        self.runtime.block_on(async {
            self.device
                .click(x, y)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Click failed: {}", e)))
        })
    }

    #[pyo3(signature = (x1, y1, x2, y2, duration_ms=None))]
    fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, duration_ms: Option<u64>) -> PyResult<()> {
        let duration = duration_ms.unwrap_or(300);
        self.runtime.block_on(async {
            self.device
                .swipe(x1, y1, x2, y2, duration)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Swipe failed: {}", e)))
        })
    }

    fn input_text(&self, text: &str) -> PyResult<()> {
        self.runtime.block_on(async {
            self.device
                .input_text(text)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Input text failed: {}", e)))
        })
    }

    fn press_key(&self, key: &str) -> PyResult<()> {
        self.runtime.block_on(async {
            self.device
                .press_key(key)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Press key failed: {}", e)))
        })
    }

    fn open_app(&self, name: &str) -> PyResult<()> {
        self.runtime.block_on(async {
            self.device
                .open_app(name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Open app failed: {}", e)))
        })
    }

    fn screenshot(&self, py: Python) -> PyResult<PyScreenshot> {
        self.runtime.block_on(async {
            let image = self
                .device
                .screenshot(None)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Screenshot failed: {}", e)))?;
            Ok(PyScreenshot {
                width: image.width,
                height: image.height,
                data: PyBytes::new_bound(py, &image.data).into(),
            })
        })
    }

    fn get_screen_size(&self) -> PyResult<(u32, u32)> {
        self.runtime.block_on(async {
            self.device
                .get_screen_size()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Get screen size failed: {}", e)))
        })
    }
}

#[pyclass(name = "Screenshot")]
pub struct PyScreenshot {
    #[pyo3(get)]
    pub width: u32,
    #[pyo3(get)]
    pub height: u32,
    #[pyo3(get)]
    pub data: Py<PyBytes>,
}
