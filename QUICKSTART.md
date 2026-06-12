# FlowX 架构快速参考

## 核心概念

### 1. 统一指令集（Command）

所有操作都通过 `Command` 枚举表示：

```rust
use flowx_core::engine::Command;

// 点击坐标
let cmd = Command::Click { x: 100, y: 200 };

// 查找元素
let cmd = Command::FindElement {
    selector: Selector::Text("确认".to_string())
};

// 截图
let cmd = Command::Screenshot { region: None };
```

### 2. 平台抽象（Platform）

所有平台实现 `Platform` trait：

```rust
use flowx_core::platform::{Platform, PlatformFactory, PlatformType};

// 创建 Android 平台
let platform = PlatformFactory::create(
    PlatformType::Android { device_id: None }
).await?;

// 执行指令
let result = platform.execute(command).await?;
```

### 3. 异步执行

所有操作都是异步的：

```rust
use flowx_core::engine::AsyncCommandExecutor;

#[tokio::main]
async fn main() -> Result<()> {
    let platform = PlatformFactory::create(
        PlatformType::Android { device_id: None }
    ).await?;
    
    let result = platform.execute(Command::Click { x: 100, y: 200 }).await?;
    Ok(())
}
```

## 选择器（Selector）

```rust
use flowx_core::engine::Selector;

// 文本匹配
Selector::Text("确认".to_string())
Selector::TextContains("确认".to_string())

// ID 匹配
Selector::Id("btn_confirm".to_string())

// 组合选择器
Selector::And(
    Box::new(Selector::Text("确认".to_string())),
    Box::new(Selector::ClassName("Button".to_string()))
)
```

## 指令类别

### 触摸操作
- `Click { x, y }` - 点击坐标
- `LongClick { x, y, duration_ms }` - 长按
- `Swipe { from, to, duration_ms }` - 滑动
- `Drag { from, to }` - 拖拽

### 元素查找
- `FindElement { selector }` - 查找单个元素
- `FindElements { selector }` - 查找多个元素
- `WaitForElement { selector, timeout_ms }` - 等待元素出现

### 屏幕操作
- `Screenshot { region }` - 截图
- `GetScreenSize` - 获取屏幕尺寸
- `ScrollTo { direction, distance }` - 滚动

### 输入操作
- `InputText { text }` - 输入文本
- `PressKey { key }` - 按键

### 应用管理
- `OpenApp { package }` - 打开应用
- `CloseApp { package }` - 关闭应用
- `GetCurrentApp` - 获取当前应用

### 视觉识别
- `OCR { region }` - 文字识别
- `FindImage { template, threshold }` - 图像匹配
- `FindColor { color, region }` - 颜色查找

## 错误处理

```rust
use flowx_core::engine::CommandError;

match platform.execute(command).await {
    Ok(result) => println!("成功: {:?}", result),
    Err(CommandError::ElementNotFound { selector }) => {
        println!("元素未找到: {}", selector);
    }
    Err(CommandError::Timeout { operation, timeout_ms }) => {
        println!("操作超时: {} ({}ms)", operation, timeout_ms);
    }
    Err(e) => println!("其他错误: {}", e),
}
```

## 返回值类型

```rust
use flowx_core::engine::CommandResult;

match result {
    CommandResult::Success => println!("操作成功"),
    CommandResult::Element(element) => {
        println!("找到元素: {:?}", element.text);
    }
    CommandResult::Elements(elements) => {
        println!("找到 {} 个元素", elements.len());
    }
    CommandResult::Image(image) => {
        println!("图像大小: {}x{}", image.width, image.height);
    }
    CommandResult::Size(w, h) => {
        println!("屏幕尺寸: {}x{}", w, h);
    }
    _ => {}
}
```

## 实现新平台

```rust
use async_trait::async_trait;
use flowx_core::platform::Platform;
use flowx_core::engine::{Command, CommandResult, CommandError};

pub struct MyPlatform {
    // 平台特定字段
}

#[async_trait]
impl Platform for MyPlatform {
    fn name(&self) -> &str {
        "MyPlatform"
    }

    async fn initialize(&mut self) -> Result<(), CommandError> {
        // 初始化逻辑
        Ok(())
    }

    async fn execute(&self, command: Command) -> Result<CommandResult, CommandError> {
        match command {
            Command::Click { x, y } => {
                // 实现点击
                Ok(CommandResult::Success)
            }
            _ => Err(CommandError::PlatformError {
                message: "Not implemented".to_string()
            })
        }
    }

    fn check_permission(&self, permission: &str) -> bool {
        true
    }

    async fn request_permission(&self, permission: &str) -> Result<bool, CommandError> {
        Ok(true)
    }

    async fn cleanup(&mut self) -> Result<(), CommandError> {
        Ok(())
    }
}
```

## 常见用法示例

### 示例 1：点击元素

```rust
// 查找元素
let result = platform.execute(Command::FindElement {
    selector: Selector::Text("确认".to_string())
}).await?;

// 提取元素
if let CommandResult::Element(element) = result {
    // 点击元素中心
    let x = element.bounds.x + element.bounds.width as i32 / 2;
    let y = element.bounds.y + element.bounds.height as i32 / 2;
    
    platform.execute(Command::Click { x, y }).await?;
}
```

### 示例 2：批量执行

```rust
use flowx_core::engine::AsyncCommandExecutor;

let commands = vec![
    Command::Click { x: 100, y: 200 },
    Command::InputText { text: "Hello".to_string() },
    Command::PressKey { key: Key::Enter },
];

let results = platform.execute_batch(commands).await;

for result in results {
    match result {
        Ok(r) => println!("成功: {:?}", r),
        Err(e) => println!("失败: {}", e),
    }
}
```

### 示例 3：条件编译

```rust
#[cfg(feature = "android")]
use flowx_android::AndroidPlatform;

#[cfg(feature = "ios")]
use flowx_ios::IOSPlatform;

// 根据 feature 选择平台
#[cfg(feature = "android")]
let platform = AndroidPlatform::new(None)?;

#[cfg(feature = "ios")]
let platform = IOSPlatform::new(None)?;
```

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_command_execution() {
        let platform = PlatformFactory::create(
            PlatformType::Android { device_id: None }
        ).await.unwrap();

        let result = platform.execute(
            Command::GetScreenSize
        ).await;

        assert!(result.is_ok());
    }
}
```

## 依赖项

在 `Cargo.toml` 中添加：

```toml
[dependencies]
flowx-core = { path = "crates/flowx-core" }
flowx-android = { path = "crates/flowx-android" }
tokio = { version = "1.35", features = ["full"] }
```

## 编译和运行

```bash
# 检查编译
cargo check

# 运行测试
cargo test

# 运行 CLI 示例
cargo run -p flowx-cli

# 构建 release 版本
cargo build --release
```

---

**参考文档**：
- ARCHITECTURE.md - 完整项目结构
- ARCH_ACCEPTANCE.md - 验收清单
- docs/architecture/ARCH-001-系统架构设计.md
- docs/architecture/ARCH-002-统一指令集设计.md
- docs/architecture/ARCH-003-平台抽象层设计.md
