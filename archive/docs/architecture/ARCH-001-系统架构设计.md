# ARCH-001: 系统架构设计

**任务ID**：ARCH-001  
**负责角色**：架构设计师  
**优先级**：P0（最高）  
**预计工时**：40小时  
**依赖**：无  
**阶段**：Phase 1 (Week 1-2)

---

## 任务目标

设计FlowX的整体系统架构，定义各模块职责、接口规范和技术选型，为后续开发提供架构蓝图。

---

## 背景

FlowX是一个**双模式输入 + Rust统一引擎**的自动化平台：
- **输入模式**：AI自然语言 + 编程脚本（Python/JS）
- **执行引擎**：所有操作由Rust核心统一执行
- **目标平台**：Android（MVP）、iOS（后续）、Desktop（后续）

**核心设计原则**：
1. Rust是唯一的执行引擎
2. Python/JS只是描述层，不参与执行
3. 统一指令集，所有输入最终翻译成Rust指令
4. 性能关键路径全在Rust

---

## 交付物

### 1. 系统架构图

需要输出以下架构图：

#### 1.1 总体架构图
```
┌──────────────────┐  ┌──────────────────┐
│   AI模式输入      │  │  脚本模式输入     │
│  (自然语言)       │  │  (Python/JS)     │
└────────┬─────────┘  └────────┬─────────┘
         │                     │
         └──────────┬──────────┘
                    ↓
        ┌───────────────────────┐
        │  指令层 (Command API)  │
        │  - click(x, y)         │
        │  - find_element(...)   │
        │  - swipe(...)          │
        └───────────────────────┘
                    ↓
        ┌───────────────────────┐
        │   Rust核心引擎         │
        │  - 设备控制            │
        │  - UI自动化            │
        │  - 视觉能力            │
        └───────────────────────┘
                    ↓
        ┌───────────────────────┐
        │   平台抽象层           │
        │  Android/iOS/Desktop   │
        └───────────────────────┘
```

#### 1.2 Rust核心引擎内部架构

定义Rust内部模块结构：
- `flowx-core/engine` - 指令执行器
- `flowx-core/device` - 设备控制
- `flowx-core/vision` - 视觉能力
- `flowx-core/platform` - 平台抽象
- `flowx-core/scripting` - 脚本运行时接口

#### 1.3 数据流架构

说明数据如何在各层之间流动：
- AI输入 → 指令序列
- Python脚本 → FFI调用 → Rust指令
- JS脚本 → QuickJS → Rust指令
- 指令 → 平台实现 → 设备操作

### 2. 模块职责定义

| 模块 | 职责 | 输入 | 输出 | 依赖 |
|------|------|------|------|------|
| AI理解层 | 自然语言→指令序列 | 文本+截图 | Command序列 | GLM-4V |
| Python FFI | Python API → Rust调用 | Python函数调用 | Rust指令 | PyO3 |
| JS运行时 | JS脚本执行 | JS代码 | Rust指令 | QuickJS |
| 指令执行器 | 统一指令调度 | Command | 平台调用 | - |
| 设备控制 | 触摸/按键/截图 | 控制指令 | 设备操作 | 平台层 |
| 视觉能力 | OCR/图像识别 | 图像 | 识别结果 | OpenCV |
| 平台抽象 | 跨平台统一接口 | 平台无关指令 | 平台特定调用 | Android SDK |

### 3. 接口规范文档

定义核心接口：

#### 3.1 统一指令集（Command API）

Rust枚举定义：
```rust
pub enum Command {
    Click { x: i32, y: i32 },
    Swipe { from: Point, to: Point, duration_ms: u64 },
    Input { text: String },
    FindElement { selector: Selector },
    Screenshot,
    OCR { region: Option<Rect> },
    // ... 其他指令
}

pub enum Selector {
    Text(String),
    Id(String),
    XPath(String),
    Image(Vec<u8>),
}
```

#### 3.2 Python FFI接口

Python侧API设计：
```python
class Device:
    def click(self, x: int, y: int) -> None
    def swipe(self, x1: int, y1: int, x2: int, y2: int) -> None
    def find(self, text: str) -> Element
    def screenshot(self) -> Image
    def ocr(self, region: Optional[Rect] = None) -> List[Text]
```

#### 3.3 平台抽象接口

Rust trait定义：
```rust
pub trait Platform {
    fn click(&self, x: i32, y: i32) -> Result<()>;
    fn swipe(&self, from: Point, to: Point, duration: Duration) -> Result<()>;
    fn screenshot(&self) -> Result<Image>;
    fn find_elements(&self, selector: &Selector) -> Result<Vec<Element>>;
}
```

### 4. 技术选型文档

| 组件 | 技术方案 | 理由 | 备选方案 |
|------|---------|------|---------|
| 核心语言 | Rust 1.75+ | 性能、安全、跨平台 | - |
| Python绑定 | PyO3 | 零开销FFI | ctypes |
| JS运行时 | QuickJS | 轻量（600KB） | Deno Core |
| 异步运行时 | tokio | Rust生态标准 | async-std |
| 图像处理 | opencv-rust | 成熟、功能全 | image crate |
| OCR引擎 | PaddleOCR | 中文支持好 | Tesseract |
| Android通信 | ADB + JNI | 官方支持 | - |

### 5. 项目目录结构

```
flowx/
├── Cargo.toml                    # Rust workspace
├── crates/
│   ├── flowx-core/               # 核心引擎
│   │   ├── src/
│   │   │   ├── engine/           # 指令执行器
│   │   │   ├── device/           # 设备控制
│   │   │   ├── vision/           # 视觉能力
│   │   │   ├── platform/         # 平台抽象
│   │   │   └── scripting/        # 脚本接口
│   │   └── Cargo.toml
│   │
│   ├── flowx-android/            # Android平台实现
│   │   └── src/
│   │       ├── jni_bridge.rs
│   │       └── accessibility.rs
│   │
│   ├── flowx-python/             # Python绑定
│   │   └── src/
│   │       └── lib.rs            # PyO3绑定
│   │
│   └── flowx-cli/                # CLI工具
│       └── src/
│           └── main.rs
│
├── python/
│   └── flowx/                    # Python包
│       ├── __init__.py
│       ├── device.py
│       ├── ai.py
│       └── _core.pyi             # 类型提示
│
├── js-runtime/
│   └── stdlib/                   # AutoJS兼容API
│       ├── app.js
│       └── ui.js
│
├── android/
│   └── app/                      # Android宿主App
│       └── src/main/java/
│
├── docs/                         # 文档
└── tests/                        # 测试
```

---

## 验收标准

- [ ] 提供完整的系统架构图（总体、内部、数据流）
- [ ] 提供模块职责定义表
- [ ] 提供核心接口规范（Command、Python、Platform trait）
- [ ] 提供技术选型文档并说明理由
- [ ] 提供项目目录结构和模块划分
- [ ] 通过技术评审（需要团队全员参与）

---

## 技术风险

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| PyO3性能不足 | 高 | 提前做性能测试，准备备选方案 |
| Android权限限制 | 中 | 调研Accessibility Service API |
| 跨平台抽象复杂 | 中 | MVP先专注Android，后续再扩展 |

---

## 参考资料

- [PyO3 官方文档](https://pyo3.rs/)
- [QuickJS GitHub](https://github.com/bellard/quickjs)
- [Android Accessibility Service](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService)
- [Tokio 异步编程](https://tokio.rs/)

---

## 下游任务

完成后会解锁：
- DEV-001: Rust核心引擎开发
- DEV-002: Android平台实现
- DEV-004: Python FFI绑定
- DEV-005: JavaScript运行时

---

**创建日期**：2026-06-10  
**最后更新**：2026-06-10  
**状态**：待开始
