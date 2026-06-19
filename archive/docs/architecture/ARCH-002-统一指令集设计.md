# ARCH-002: 统一指令集设计

**任务ID**：ARCH-002  
**负责角色**：架构设计师  
**优先级**：P0（最高）  
**预计工时**：24小时  
**依赖**：ARCH-001  
**阶段**：Phase 1 (Week 2-3)

---

## 任务目标

设计FlowX的统一指令集（Command API），这是连接所有输入模式（AI、Python、JS）和Rust执行引擎的核心接口。

---

## 背景

**为什么需要统一指令集？**

```
AI输入: "点击屏幕上的确认按钮"
         ↓
Python输入: device.find(text="确认").click()
         ↓
JS输入: text("确认").findOne().click()
         ↓
    【统一指令集】
         ↓
Command::FindElement → Command::Click
         ↓
     Rust执行引擎
```

所有输入最终都转换成统一的Command，由Rust引擎执行。

---

## 交付物

### 1. 指令分类和定义

#### 1.1 基础操作指令

```rust
pub enum Command {
    // 触摸操作
    Click { x: i32, y: i32 },
    LongClick { x: i32, y: i32, duration_ms: u64 },
    Swipe { from: Point, to: Point, duration_ms: u64 },
    Drag { from: Point, to: Point },
    
    // 输入操作
    InputText { text: String },
    PressKey { key: Key },
    
    // 元素查找
    FindElement { selector: Selector },
    FindElements { selector: Selector },
    WaitForElement { selector: Selector, timeout_ms: u64 },
    
    // 屏幕操作
    Screenshot { region: Option<Rect> },
    GetScreenSize,
    ScrollTo { direction: Direction, distance: i32 },
    
    // 应用管理
    OpenApp { package: String },
    CloseApp { package: String },
    GetCurrentApp,
    
    // 视觉识别
    OCR { region: Option<Rect> },
    FindImage { template: Image, threshold: f32 },
    FindColor { color: Color, region: Option<Rect> },
}
```

#### 1.2 选择器定义

```rust
pub enum Selector {
    // 文本匹配
    Text(String),
    TextContains(String),
    TextMatches(Regex),
    
    // ID匹配
    Id(String),
    
    // 类名匹配
    ClassName(String),
    
    // XPath
    XPath(String),
    
    // 图像匹配
    Image { template: Vec<u8>, threshold: f32 },
    
    // 组合选择器
    And(Box<Selector>, Box<Selector>),
    Or(Box<Selector>, Box<Selector>),
    Parent(Box<Selector>),
    Child(Box<Selector>, usize),
}
```

#### 1.3 辅助类型定义

```rust
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Key {
    Back,
    Home,
    Menu,
    Enter,
    Delete,
    VolumeUp,
    VolumeDown,
}

pub struct Element {
    pub id: String,
    pub text: Option<String>,
    pub bounds: Rect,
    pub class_name: Option<String>,
    pub clickable: bool,
}

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

pub enum ImageFormat {
    RGB,
    RGBA,
    Gray,
}
```

### 2. 指令执行流程

```rust
// 指令执行器接口
pub trait CommandExecutor {
    fn execute(&self, command: Command) -> Result<CommandResult>;
}

pub enum CommandResult {
    Success,
    Element(Element),
    Elements(Vec<Element>),
    Image(Image),
    Text(String),
    Texts(Vec<Text>),
    Boolean(bool),
    Number(i32),
}

pub struct Text {
    pub content: String,
    pub bounds: Rect,
    pub confidence: f32,
}
```

### 3. 从各输入模式到指令的映射

#### 3.1 Python → Command

```python
# Python代码
device.click(100, 200)
device.find(text="确认").click()

# 映射到
Command::Click { x: 100, y: 200 }
Command::FindElement { selector: Selector::Text("确认") }
→ Command::Click { x: <element.x>, y: <element.y> }
```

#### 3.2 JavaScript → Command

```javascript
// JS代码
click(100, 200);
text("确认").findOne().click();

// 映射到
Command::Click { x: 100, y: 200 }
Command::FindElement { selector: Selector::Text("确认") }
→ Command::Click { x: <element.x>, y: <element.y> }
```

#### 3.3 AI → Command

```
AI输入: "点击屏幕中间的确认按钮"

AI理解:
1. 获取屏幕截图: Command::Screenshot
2. 识别"确认按钮": 视觉+文本识别
3. 生成点击指令: Command::Click { x, y }
```

### 4. 错误处理

```rust
pub enum CommandError {
    // 元素未找到
    ElementNotFound { selector: Selector },
    
    // 超时
    Timeout { operation: String, timeout_ms: u64 },
    
    // 平台错误
    PlatformError { message: String },
    
    // 权限错误
    PermissionDenied { permission: String },
    
    // 无效参数
    InvalidArgument { param: String, reason: String },
}

pub type Result<T> = std::result::Result<T, CommandError>;
```

### 5. 异步执行支持

```rust
use tokio;

// 异步执行器
#[async_trait]
pub trait AsyncCommandExecutor {
    async fn execute_async(&self, command: Command) -> Result<CommandResult>;
    
    // 批量执行
    async fn execute_batch(&self, commands: Vec<Command>) -> Vec<Result<CommandResult>>;
}
```

---

## 设计原则

1. **完备性**：覆盖AutoJS 80%的核心API
2. **扩展性**：易于添加新指令
3. **类型安全**：利用Rust的类型系统
4. **性能**：指令结构零开销
5. **序列化**：支持序列化（用于网络传输、日志）

---

## API对比表

与AutoJS API的映射关系：

| AutoJS API | FlowX Command | 说明 |
|------------|---------------|------|
| `click(x, y)` | `Command::Click` | 点击坐标 |
| `text("确认").findOne()` | `Command::FindElement(Selector::Text)` | 查找文本 |
| `id("btn").findOne()` | `Command::FindElement(Selector::Id)` | 查找ID |
| `swipe(x1,y1,x2,y2,500)` | `Command::Swipe` | 滑动 |
| `longClick(x, y)` | `Command::LongClick` | 长按 |
| `input("text")` | `Command::InputText` | 输入文本 |
| `captureScreen()` | `Command::Screenshot` | 截图 |
| `app.launch(pkg)` | `Command::OpenApp` | 打开应用 |

---

## 验收标准

- [ ] 完整的指令枚举定义（覆盖MVP所需功能）
- [ ] 选择器和辅助类型定义
- [ ] 指令执行器trait定义
- [ ] 错误处理枚举
- [ ] AutoJS API映射表
- [ ] 提供Rust单元测试示例
- [ ] 通过代码评审

---

## 示例代码

```rust
// 示例：执行指令
use flowx_core::engine::{Command, Selector, CommandExecutor};

let executor = AndroidExecutor::new()?;

// 点击坐标
executor.execute(Command::Click { x: 100, y: 200 })?;

// 查找并点击元素
let result = executor.execute(Command::FindElement {
    selector: Selector::Text("确认".to_string())
})?;

if let CommandResult::Element(element) = result {
    executor.execute(Command::Click {
        x: element.bounds.x + element.bounds.width as i32 / 2,
        y: element.bounds.y + element.bounds.height as i32 / 2,
    })?;
}
```

---

## 下游任务

完成后会被以下任务使用：
- DEV-001: Rust核心引擎（实现CommandExecutor）
- DEV-004: Python FFI（Python API → Command转换）
- DEV-005: JS运行时（JS API → Command转换）
- DEV-007: AI集成（AI理解 → Command生成）

---

**创建日期**：2026-06-10  
**最后更新**：2026-06-10  
**状态**：待开始
