# DEV-001: Rust核心引擎开发

**任务ID**：DEV-001  
**负责角色**：Rust核心开发  
**优先级**：P0（最高）  
**预计工时**：80小时  
**依赖**：ARCH-001, ARCH-002  
**阶段**：Phase 1 (Week 3-6)

---

## 任务目标

实现FlowX的Rust核心引擎，包括指令执行器、设备控制模块和基础工具函数。

---

## 技术栈

- Rust 1.75+
- tokio (异步运行时)
- serde (序列化)
- anyhow / thiserror (错误处理)
- tracing (日志)

---

## 交付物

### 1. 项目骨架搭建

```bash
# 创建Rust workspace
cargo new --lib flowx-core
cd flowx-core

# Cargo.toml
[workspace]
members = [
    "crates/flowx-core",
    "crates/flowx-android",
    "crates/flowx-python",
    "crates/flowx-cli",
]
```

#### 1.1 flowx-core/Cargo.toml

```toml
[package]
name = "flowx-core"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
async-trait = "0.1"

[dev-dependencies]
tokio-test = "0.4"
```

### 2. 核心模块实现

#### 2.1 指令定义 (`engine/command.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Click { x: i32, y: i32 },
    LongClick { x: i32, y: i32, duration_ms: u64 },
    Swipe { from: Point, to: Point, duration_ms: u64 },
    InputText { text: String },
    PressKey { key: Key },
    FindElement { selector: Selector },
    FindElements { selector: Selector },
    Screenshot { region: Option<Rect> },
    OpenApp { package: String },
    OCR { region: Option<Rect> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Selector {
    Text(String),
    TextContains(String),
    Id(String),
    ClassName(String),
    XPath(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Key {
    Back,
    Home,
    Menu,
    Enter,
    Delete,
}
```

#### 2.2 指令结果 (`engine/result.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandResult {
    Success,
    Element(Element),
    Elements(Vec<Element>),
    Image(ImageData),
    Texts(Vec<OcrText>),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub text: Option<String>,
    pub bounds: Rect,
    pub class_name: Option<String>,
    pub clickable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    RGB,
    RGBA,
    PNG,
    JPEG,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrText {
    pub content: String,
    pub bounds: Rect,
    pub confidence: f32,
}
```

#### 2.3 错误处理 (`engine/error.rs`)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowXError {
    #[error("Element not found: {selector:?}")]
    ElementNotFound { selector: String },
    
    #[error("Operation timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("Platform error: {message}")]
    PlatformError { message: String },
    
    #[error("Permission denied: {permission}")]
    PermissionDenied { permission: String },
    
    #[error("Invalid argument: {param} - {reason}")]
    InvalidArgument { param: String, reason: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, FlowXError>;
```

#### 2.4 指令执行器 (`engine/executor.rs`)

```rust
use async_trait::async_trait;
use crate::engine::{Command, CommandResult, Result};
use crate::platform::Platform;

pub struct CommandExecutor {
    platform: Box<dyn Platform>,
}

impl CommandExecutor {
    pub fn new(platform: Box<dyn Platform>) -> Self {
        Self { platform }
    }
    
    pub async fn execute(&self, command: Command) -> Result<CommandResult> {
        tracing::info!("Executing command: {:?}", command);
        
        let result = self.platform.execute(command).await?;
        
        tracing::info!("Command completed: {:?}", result);
        Ok(result)
    }
    
    pub async fn execute_batch(&self, commands: Vec<Command>) -> Vec<Result<CommandResult>> {
        let mut results = Vec::new();
        for command in commands {
            results.push(self.execute(command).await);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_executor() {
        // 单元测试
    }
}
```

### 3. 设备控制模块 (`device/mod.rs`)

```rust
pub struct Device {
    executor: CommandExecutor,
}

impl Device {
    pub fn new(executor: CommandExecutor) -> Self {
        Self { executor }
    }
    
    pub async fn click(&self, x: i32, y: i32) -> Result<()> {
        self.executor.execute(Command::Click { x, y }).await?;
        Ok(())
    }
    
    pub async fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, duration_ms: u64) -> Result<()> {
        self.executor.execute(Command::Swipe {
            from: Point { x: x1, y: y1 },
            to: Point { x: x2, y: y2 },
            duration_ms,
        }).await?;
        Ok(())
    }
    
    pub async fn find_element(&self, selector: Selector) -> Result<Element> {
        match self.executor.execute(Command::FindElement { selector }).await? {
            CommandResult::Element(element) => Ok(element),
            _ => Err(FlowXError::ElementNotFound { selector: "unknown".to_string() }),
        }
    }
    
    pub async fn screenshot(&self) -> Result<ImageData> {
        match self.executor.execute(Command::Screenshot { region: None }).await? {
            CommandResult::Image(image) => Ok(image),
            _ => Err(FlowXError::PlatformError { message: "Screenshot failed".to_string() }),
        }
    }
}
```

### 4. 工具函数 (`utils/mod.rs`)

```rust
pub mod geometry {
    use crate::engine::{Point, Rect};
    
    pub fn rect_center(rect: &Rect) -> Point {
        Point {
            x: rect.x + rect.width as i32 / 2,
            y: rect.y + rect.height as i32 / 2,
        }
    }
    
    pub fn point_in_rect(point: &Point, rect: &Rect) -> bool {
        point.x >= rect.x
            && point.x <= rect.x + rect.width as i32
            && point.y >= rect.y
            && point.y <= rect.y + rect.height as i32
    }
}

pub mod retry {
    use std::time::Duration;
    use tokio::time::sleep;
    
    pub async fn retry_with_timeout<F, T, E>(
        mut f: F,
        max_attempts: u32,
        delay: Duration,
    ) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        for attempt in 0..max_attempts {
            match f() {
                Ok(result) => return Ok(result),
                Err(e) if attempt == max_attempts - 1 => return Err(e),
                Err(_) => sleep(delay).await,
            }
        }
        unreachable!()
    }
}
```

### 5. 模块组织 (`lib.rs`)

```rust
pub mod engine;
pub mod device;
pub mod platform;
pub mod utils;

pub use engine::{Command, CommandResult, Selector};
pub use device::Device;
pub use platform::Platform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

---

## 实现步骤

### Week 3: 基础框架
- [ ] 创建项目结构
- [ ] 实现Command枚举
- [ ] 实现CommandResult和错误类型
- [ ] 编写单元测试

### Week 4: 执行器
- [ ] 实现CommandExecutor
- [ ] 实现Device高级API
- [ ] 添加日志和追踪
- [ ] 编写集成测试框架

### Week 5-6: 工具和优化
- [ ] 实现工具函数
- [ ] 性能优化
- [ ] 文档和示例
- [ ] 代码审查

---

## 验收标准

- [ ] 所有代码编译通过 (`cargo build`)
- [ ] 单元测试通过 (`cargo test`)
- [ ] 代码格式化 (`cargo fmt`)
- [ ] Clippy检查通过 (`cargo clippy`)
- [ ] 文档完整 (`cargo doc`)
- [ ] 性能基准测试（点击延迟<50ms）

---

## 性能要求

| 操作 | 目标延迟 |
|------|---------|
| Command创建 | <1ms |
| Command序列化 | <5ms |
| 指令执行（不含平台） | <10ms |

---

## 测试用例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_serialization() {
        let cmd = Command::Click { x: 100, y: 200 };
        let json = serde_json::to_string(&cmd).unwrap();
        let cmd2: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, cmd2);
    }
    
    #[test]
    fn test_rect_center() {
        let rect = Rect { x: 0, y: 0, width: 100, height: 100 };
        let center = utils::geometry::rect_center(&rect);
        assert_eq!(center.x, 50);
        assert_eq!(center.y, 50);
    }
}
```

---

## 参考资料

- [Tokio异步编程](https://tokio.rs/)
- [Rust错误处理最佳实践](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Serde序列化指南](https://serde.rs/)

---

**创建日期**：2026-06-10  
**最后更新**：2026-06-10  
**状态**：待开始
