# FlowX API 文档

**版本**: v1.0.0  
**日期**: 2026-06-11

---

## Python API

### 快速开始

```python
import flowx

# 连接设备
device = flowx.Device.connect("macos")  # macOS
device = flowx.Device.connect("emulator-5554")  # Android ADB
device = flowx.Device.connect("android://192.168.1.100")  # Android Accessibility

# 基础操作
device.click(100, 200)
device.swipe(100, 500, 100, 200, duration=300)
device.input_text("Hello World")
device.press_key("Enter")

# macOS 专用
device.open_app("Safari")
device.press_key("CommandT")  # ⌘+T

# 屏幕信息
width, height = device.get_screen_size()
screenshot = device.screenshot()
```

---

## Rust Core API

### Device 模块

```rust
use flowx_core::device::Device;
use flowx_core::engine::{Command, CommandResult};

// 创建设备实例
let executor = create_executor("device_id");
let device = Device::new(executor);

// 异步操作
device.click(100, 200).await?;
device.swipe(100, 500, 100, 200, 300).await?;
device.input_text("test").await?;
```

### 平台模块

#### Android
```rust
use flowx_core::platforms::android::AdbClient;

let client = AdbClient::new(Some("emulator-5554".to_string()));
client.shell("input tap 100 200")?;
```

#### macOS
```rust
use flowx_core::platforms::macos::MacOSClient;

let client = MacOSClient::new()?;
client.execute_async(Command::Click { x: 100, y: 200 }).await?;
```

### Vision 模块

```rust
use flowx_vision::VisionEngine;

let engine = VisionEngine::new();

// 模板匹配
let result = engine.find_template(&haystack, &needle, 0.8)?;
if let Some((x, y, confidence)) = result {
    println!("Found at ({}, {}) with confidence {}", x, y, confidence);
}

// 颜色查找
let pixels = engine.find_color(&image, (255, 0, 0), 10)?;
```

---

## Command 类型

```rust
pub enum Command {
    Click { x: i32, y: i32 },
    Swipe { from: Point, to: Point, duration_ms: u64 },
    InputText { text: String },
    PressKey { key: Key },
    OpenApp { name: String },
    Screenshot { region: Option<Rect> },
    GetScreenSize,
}
```

---

## 错误处理

```rust
pub enum CommandError {
    ElementNotFound { selector: String },
    Timeout { operation: String, timeout_ms: u64 },
    PlatformError { message: String },
    PermissionDenied { permission: String },
    InvalidArgument { param: String, reason: String },
}
```

---

## 平台支持

| 平台 | 连接方式 | 示例 |
|------|---------|------|
| macOS | 本地 | `flowx.Device.connect("macos")` |
| Android | ADB | `flowx.Device.connect("emulator-5554")` |
| Android | Accessibility | `flowx.Device.connect("android://192.168.1.100")` |
| Windows | 本地 | `flowx.Device.connect("windows")` |
| iOS | WDA | `flowx.Device.connect("http://localhost:8100")` |

---

## 示例

### 自动化浏览器
```python
import flowx

device = flowx.Device.connect("macos")
device.open_app("Microsoft Edge")
device.press_key("CommandT")
device.input_text("anthropic.com")
device.press_key("Enter")
```

### Android 自动化
```python
import flowx

device = flowx.Device.connect("emulator-5554")
device.click(540, 1000)
device.swipe(540, 1500, 540, 800, 300)
device.input_text("test")
```

---

**更多示例**: 查看 `scripts/` 目录
