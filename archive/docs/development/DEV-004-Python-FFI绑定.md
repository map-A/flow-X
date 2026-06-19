# DEV-004: Python FFI绑定

**任务ID**：DEV-004  
**负责角色**：Python/JS开发  
**优先级**：P0（最高）  
**预计工时**：60小时  
**依赖**：DEV-001, DEV-002  
**阶段**：Phase 2 (Week 5-7)

---

## 任务目标

使用PyO3实现Python到Rust的FFI绑定，提供Pythonic的API接口，让Python用户能够方便地使用FlowX。

---

## 技术栈

- PyO3 (Rust Python绑定)
- Python 3.10+
- maturin (构建工具)

---

## 交付物

### 1. 项目结构

```
flowx-python/
├── src/
│   ├── lib.rs              # PyO3绑定主入口
│   ├── device.rs           # Device类绑定
│   ├── element.rs          # Element类绑定
│   └── types.rs            # 类型转换
├── Cargo.toml
└── pyproject.toml

python/
└── flowx/
    ├── __init__.py         # Python包入口
    ├── device.py           # Device类型提示
    ├── ai.py               # AI模块（纯Python）
    ├── _core.pyi           # Rust绑定类型提示
    └── py.typed            # 类型标记文件
```

### 2. Cargo.toml配置

```toml
[package]
name = "flowx-python"
version = "0.1.0"
edition = "2021"

[lib]
name = "flowx_core"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }
flowx-core = { path = "../flowx-core" }
tokio = { version = "1.35", features = ["full"] }
```

### 3. PyO3绑定实现 (`lib.rs`)

```rust
use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;

mod device;
mod element;
mod types;

use device::PyDevice;
use element::PyElement;

/// FlowX Python模块
#[pymodule]
fn flowx_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDevice>()?;
    m.add_class::<PyElement>()?;
    Ok(())
}

/// 将Rust错误转换为Python异常
fn to_py_err(err: flowx_core::FlowXError) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}
```

### 4. Device类绑定 (`device.rs`)

```rust
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use flowx_core::{Device, Command, Selector, Platform};
use flowx_android::AndroidPlatform;
use crate::element::PyElement;
use crate::to_py_err;

#[pyclass]
pub struct PyDevice {
    device: Device,
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyDevice {
    /// 创建Android设备
    #[staticmethod]
    fn android(device_id: Option<String>) -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        
        let device = runtime.block_on(async {
            let mut platform = AndroidPlatform::new(device_id)
                .map_err(to_py_err)?;
            platform.initialize().await.map_err(to_py_err)?;
            
            let executor = CommandExecutor::new(Box::new(platform));
            Ok::<_, PyErr>(Device::new(executor))
        })?;
        
        Ok(Self { device, runtime })
    }
    
    /// 点击坐标
    fn click(&self, x: i32, y: i32) -> PyResult<()> {
        self.runtime.block_on(async {
            self.device.click(x, y).await.map_err(to_py_err)
        })
    }
    
    /// 长按
    fn long_click(&self, x: i32, y: i32, duration_ms: Option<u64>) -> PyResult<()> {
        let duration = duration_ms.unwrap_or(1000);
        self.runtime.block_on(async {
            self.device.long_click(x, y, duration).await.map_err(to_py_err)
        })
    }
    
    /// 滑动
    fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, duration_ms: Option<u64>) -> PyResult<()> {
        let duration = duration_ms.unwrap_or(300);
        self.runtime.block_on(async {
            self.device.swipe(x1, y1, x2, y2, duration).await.map_err(to_py_err)
        })
    }
    
    /// 输入文本
    fn input_text(&self, text: String) -> PyResult<()> {
        self.runtime.block_on(async {
            self.device.input_text(&text).await.map_err(to_py_err)
        })
    }
    
    /// 查找元素（按文本）
    fn find(&self, text: Option<String>, id: Option<String>, class_name: Option<String>) -> PyResult<PyElement> {
        let selector = if let Some(t) = text {
            Selector::Text(t)
        } else if let Some(i) = id {
            Selector::Id(i)
        } else if let Some(c) = class_name {
            Selector::ClassName(c)
        } else {
            return Err(PyRuntimeError::new_err("Must provide text, id, or class_name"));
        };
        
        self.runtime.block_on(async {
            let element = self.device.find_element(selector).await.map_err(to_py_err)?;
            Ok(PyElement::new(element, self.device.clone()))
        })
    }
    
    /// 截图
    fn screenshot(&self, py: Python) -> PyResult<PyObject> {
        let image_data = self.runtime.block_on(async {
            self.device.screenshot().await.map_err(to_py_err)
        })?;
        
        // 返回bytes
        let bytes = PyBytes::new(py, &image_data.data);
        Ok(bytes.into())
    }
    
    /// OCR识别
    fn ocr(&self) -> PyResult<Vec<(String, (i32, i32, u32, u32), f32)>> {
        self.runtime.block_on(async {
            let texts = self.device.ocr(None).await.map_err(to_py_err)?;
            
            Ok(texts.into_iter().map(|t| {
                (
                    t.content,
                    (t.bounds.x, t.bounds.y, t.bounds.width, t.bounds.height),
                    t.confidence
                )
            }).collect())
        })
    }
    
    /// 打开应用
    fn open_app(&self, package: String) -> PyResult<()> {
        self.runtime.block_on(async {
            self.device.open_app(&package).await.map_err(to_py_err)
        })
    }
    
    /// 等待元素出现
    fn wait_for(&self, text: String, timeout_ms: Option<u64>) -> PyResult<PyElement> {
        let timeout = timeout_ms.unwrap_or(10000);
        
        self.runtime.block_on(async {
            let element = self.device.wait_for_element(
                Selector::Text(text),
                timeout
            ).await.map_err(to_py_err)?;
            
            Ok(PyElement::new(element, self.device.clone()))
        })
    }
    
    /// 字符串表示
    fn __repr__(&self) -> String {
        format!("<FlowX Device: Android>")
    }
}
```

### 5. Element类绑定 (`element.rs`)

```rust
use pyo3::prelude::*;
use flowx_core::{Element, Device};
use crate::to_py_err;

#[pyclass]
pub struct PyElement {
    element: Element,
    device: Device,
}

impl PyElement {
    pub fn new(element: Element, device: Device) -> Self {
        Self { element, device }
    }
}

#[pymethods]
impl PyElement {
    /// 点击元素
    fn click(&self) -> PyResult<()> {
        let center = flowx_core::utils::geometry::rect_center(&self.element.bounds);
        
        // TODO: 需要runtime
        self.device.click(center.x, center.y).await.map_err(to_py_err)
    }
    
    /// 获取文本
    #[getter]
    fn text(&self) -> Option<String> {
        self.element.text.clone()
    }
    
    /// 获取bounds
    #[getter]
    fn bounds(&self) -> (i32, i32, u32, u32) {
        let b = &self.element.bounds;
        (b.x, b.y, b.width, b.height)
    }
    
    /// 是否可点击
    #[getter]
    fn clickable(&self) -> bool {
        self.element.clickable
    }
    
    fn __repr__(&self) -> String {
        format!(
            "<Element text={:?} bounds=({}, {}, {}, {})>",
            self.element.text,
            self.element.bounds.x,
            self.element.bounds.y,
            self.element.bounds.width,
            self.element.bounds.height
        )
    }
}
```

### 6. Python包实现 (`python/flowx/__init__.py`)

```python
"""
FlowX - AI驱动的智能自动化平台
"""

from .device import Device
from .ai import AI

__version__ = "0.1.0"
__all__ = ["Device", "AI"]
```

### 7. Device类型提示 (`python/flowx/device.py`)

```python
"""Device API with type hints"""

from typing import Optional, Tuple, List
from flowx_core import PyDevice, PyElement

class Device:
    """FlowX设备控制类"""
    
    def __init__(self, _device: PyDevice):
        self._device = _device
    
    @staticmethod
    def android(device_id: Optional[str] = None) -> "Device":
        """
        创建Android设备实例
        
        Args:
            device_id: 设备ID，None表示使用第一个连接的设备
            
        Returns:
            Device实例
            
        Example:
            >>> device = Device.android()
            >>> device.click(100, 200)
        """
        return Device(PyDevice.android(device_id))
    
    def click(self, x: int, y: int) -> None:
        """点击屏幕坐标"""
        self._device.click(x, y)
    
    def swipe(self, x1: int, y1: int, x2: int, y2: int, 
              duration_ms: int = 300) -> None:
        """滑动"""
        self._device.swipe(x1, y1, x2, y2, duration_ms)
    
    def input_text(self, text: str) -> None:
        """输入文本"""
        self._device.input_text(text)
    
    def find(self, text: Optional[str] = None,
             id: Optional[str] = None,
             class_name: Optional[str] = None) -> "Element":
        """
        查找元素
        
        Args:
            text: 元素文本
            id: 元素ID
            class_name: 元素类名
            
        Returns:
            Element实例
            
        Example:
            >>> button = device.find(text="确认")
            >>> button.click()
        """
        elem = self._device.find(text=text, id=id, class_name=class_name)
        return Element(elem, self)
    
    def screenshot(self) -> bytes:
        """截图，返回PNG格式bytes"""
        return self._device.screenshot()
    
    def ocr(self) -> List[Tuple[str, Tuple[int, int, int, int], float]]:
        """
        OCR识别
        
        Returns:
            List of (文本, (x, y, width, height), 置信度)
        """
        return self._device.ocr()
    
    def open_app(self, package: str) -> None:
        """打开应用"""
        self._device.open_app(package)
    
    def wait_for(self, text: str, timeout_ms: int = 10000) -> "Element":
        """等待元素出现"""
        elem = self._device.wait_for(text, timeout_ms)
        return Element(elem, self)


class Element:
    """UI元素"""
    
    def __init__(self, _element: PyElement, device: Device):
        self._element = _element
        self.device = device
    
    def click(self) -> None:
        """点击元素"""
        self._element.click()
    
    @property
    def text(self) -> Optional[str]:
        """元素文本"""
        return self._element.text
    
    @property
    def bounds(self) -> Tuple[int, int, int, int]:
        """元素边界 (x, y, width, height)"""
        return self._element.bounds
    
    @property
    def clickable(self) -> bool:
        """是否可点击"""
        return self._element.clickable
    
    def __repr__(self) -> str:
        return self._element.__repr__()
```

### 8. 构建配置 (`pyproject.toml`)

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "flowx"
version = "0.1.0"
description = "AI-driven automation platform"
requires-python = ">=3.10"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: 3",
]

[tool.maturin]
python-source = "python"
module-name = "flowx.flowx_core"
```

---

## 实现步骤

### Week 5: 基础绑定
- [ ] 设置PyO3项目
- [ ] 实现Device基础方法（click, swipe）
- [ ] 类型转换函数
- [ ] 编译测试

### Week 6: 完整API
- [ ] 实现Element类
- [ ] 实现所有Device方法
- [ ] 错误处理优化
- [ ] Python类型提示

### Week 7: 优化和测试
- [ ] 性能优化
- [ ] 内存管理
- [ ] 单元测试
- [ ] 使用文档

---

## 验收标准

- [ ] 编译成功：`maturin build --release`
- [ ] Python可导入：`import flowx`
- [ ] 所有API可用
- [ ] 类型提示完整
- [ ] 示例代码运行成功
- [ ] 性能测试通过（Python调用<1ms）

---

## 使用示例

```python
from flowx import Device

# 创建设备
device = Device.android()

# 基础操作
device.click(100, 200)
device.swipe(100, 500, 100, 100)

# 元素操作
button = device.find(text="确认")
print(f"找到按钮: {button.text} at {button.bounds}")
button.click()

# 链式调用
device.find(text="搜索框").click()
device.input_text("笔记本电脑")
device.find(text="搜索").click()

# 等待元素
result = device.wait_for(text="搜索结果", timeout_ms=5000)

# 截图和OCR
screenshot = device.screenshot()
with open("screen.png", "wb") as f:
    f.write(screenshot)

texts = device.ocr()
for text, bounds, confidence in texts:
    print(f"{text}: {confidence}")
```

---

## 参考资料

- [PyO3 用户指南](https://pyo3.rs/)
- [maturin 构建工具](https://github.com/PyO3/maturin)

---

**创建日期**：2026-06-10  
**最后更新**：2026-06-10  
**状态**：待开始
