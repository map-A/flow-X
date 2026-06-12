# FlowX 全平台自动化架构方案

**目标**: 统一的跨平台自动化框架 (Android, iOS, Windows, macOS)

---

## 🎯 当前问题

### 1. Android 实现的局限性
- ❌ 依赖 ADB（需要 USB 调试或网络 ADB）
- ❌ 需要开发者模式
- ❌ 权限限制（某些操作需要 root）
- ❌ 无法直接在设备上运行

### 2. 缺少其他平台支持
- ❌ iOS 未实现
- ❌ Windows 未实现
- ❌ macOS 未实现

---

## 🚀 全平台统一方案

### 方案对比

| 平台 | 方案 1: 系统 API | 方案 2: Accessibility | 方案 3: 图像识别 | 推荐 |
|------|-----------------|---------------------|----------------|------|
| **Android** | ADB (外部) | Accessibility Service | OpenCV + OCR | ✅ Accessibility |
| **iOS** | XCUITest (受限) | Accessibility API | OpenCV + OCR | ✅ Accessibility |
| **Windows** | Win32 API | UI Automation | OpenCV + OCR | ✅ UI Automation |
| **macOS** | Quartz Events | Accessibility API | OpenCV + OCR | ✅ Accessibility |

---

## ✅ 推荐架构：混合方案

### 1. Android 改进方案

#### 方案 A: Accessibility Service (推荐)
**优点**:
- ✅ 无需 ADB
- ✅ 可以打包成 APK 在设备上运行
- ✅ 官方支持，稳定性好
- ✅ 可以获取 UI 树结构
- ✅ 支持点击、滑动、文本输入

**实现**:
```
flowx-android-service/  (Android APK)
├── AccessibilityService  # 监听和执行操作
├── WebSocket Server      # 与 Rust 核心通信
└── UI 控制 API
```

**工作流程**:
```
FlowX Rust Core
    ↓ WebSocket/HTTP
Android Accessibility Service (APK)
    ↓ Android Accessibility API
Android System
```

#### 方案 B: uiautomator2 (备选)
- 基于 Android UIAutomator
- 需要 ATX-Agent 守护进程
- Python 生态成熟

#### 方案 C: Appium (通用但重)
- 支持多平台
- 基于 WebDriver 协议
- 较重，适合测试场景

---

### 2. iOS 方案

#### 方案 A: XCUITest + WebDriverAgent (推荐)
**特点**:
- 官方 XCUITest 框架
- WebDriverAgent 提供 HTTP 接口
- 需要 Mac 开发环境

**实现**:
```
FlowX Rust Core
    ↓ HTTP/WebDriver
WebDriverAgent (iOS)
    ↓ XCUITest
iOS System
```

#### 方案 B: libimobiledevice (越狱/有限)
- 开源工具
- 功能受限
- 需要越狱才能完整功能

---

### 3. Windows 方案

#### 方案 A: UI Automation (推荐)
**Windows 官方 API**:
```rust
// 使用 windows-rs crate
use windows::UI::UIAutomation::*;

// 查找元素
let automation = UIAutomation::new()?;
let element = automation.find_element(...)?;

// 点击
element.invoke()?;
```

**优点**:
- ✅ 官方支持
- ✅ 稳定可靠
- ✅ 无需额外依赖

---

### 4. macOS 方案

#### 方案 A: Accessibility API (推荐)
**macOS 官方 API**:
```rust
// 使用 core-foundation 和 accessibility-sys
use accessibility::*;

// 查找元素
let app = AXUIElementCreateApplication(pid);
let element = find_element(&app, role, title);

// 点击
perform_action(element, "AXPress");
```

**优点**:
- ✅ 官方支持
- ✅ 功能完整
- ✅ 需要授权但稳定

---

## 🏗️ FlowX 统一架构设计

### 核心架构

```
┌─────────────────────────────────────┐
│   FlowX Core (Rust)                 │
│   - 统一 Command API                 │
│   - 跨平台抽象层                     │
└─────────────────┬───────────────────┘
                  │
        ┌─────────┼─────────┐
        │         │         │         │
┌───────▼──┐ ┌───▼────┐ ┌──▼─────┐ ┌─▼────────┐
│ Android  │ │  iOS   │ │Windows │ │  macOS   │
│ Platform │ │Platform│ │Platform│ │ Platform │
└─────┬────┘ └────┬───┘ └───┬────┘ └────┬─────┘
      │           │         │            │
┌─────▼────┐ ┌───▼────┐ ┌──▼────┐ ┌────▼─────┐
│Accessib. │ │XCUITest│ │UI Auto│ │ Accessib.│
│ Service  │ │  +WDA  │ │ -mation│ │   API    │
└──────────┘ └────────┘ └────────┘ └──────────┘
```

---

## 📋 实施计划

### Phase 1: Android 改进 (1-2周)
```rust
// crates/flowx-android-service/
mod accessibility_service;  // Android Accessibility Service
mod websocket_server;       // 通信层
mod ui_controller;          // UI 操作

// 打包成 APK
// 安装到设备后运行
```

**优势**:
- 无需 ADB
- 直接在设备运行
- 获取完整 UI 树

### Phase 2: iOS 支持 (2-3周)
```rust
// crates/flowx-ios/
mod webdriver_client;  // WebDriverAgent 客户端
mod xcuitest_bridge;   // XCUITest 桥接
```

### Phase 3: Windows 支持 (1-2周)
```rust
// crates/flowx-windows/
mod ui_automation;  // Windows UI Automation
```

### Phase 4: macOS 支持 (1-2周)
```rust
// crates/flowx-macos/
mod accessibility;  // macOS Accessibility API
```

---

## 🎯 推荐的最佳实践

### 统一 API 设计
```python
from flowx import Device

# 所有平台统一接口
device = Device.connect("android://emulator-5554")
device = Device.connect("ios://iPhone-12")
device = Device.connect("windows://localhost")
device = Device.connect("macos://localhost")

# 统一操作
device.click(x, y)
device.swipe(x1, y1, x2, y2)
device.input_text("text")

# 高级功能（基于 UI 树）
device.find_element(text="登录").click()
device.wait_for(text="首页")
```

---

## 💡 Android 最佳方案详解

### 选择: Accessibility Service

**为什么不用 ADB?**
1. ❌ 需要开发者模式
2. ❌ 需要 USB 连接或网络 ADB
3. ❌ 无法分发给普通用户
4. ❌ 性能较差（IPC 开销）

**为什么用 Accessibility Service?**
1. ✅ 可打包成 APK 分发
2. ✅ 用户只需授权辅助功能
3. ✅ 完整的 UI 树访问
4. ✅ 官方支持，稳定可靠
5. ✅ 性能好（直接系统 API）

**实现示例**:
```kotlin
// Android Accessibility Service
class FlowXService : AccessibilityService() {
    
    override fun onAccessibilityEvent(event: AccessibilityEvent) {
        // 监听 UI 变化
    }
    
    // HTTP/WebSocket 服务器
    private val server = WebSocketServer(port = 6789)
    
    // 执行操作
    fun performClick(x: Int, y: Int) {
        val path = Path().apply {
            moveTo(x.toFloat(), y.toFloat())
        }
        dispatchGesture(
            GestureDescription.Builder()
                .addStroke(GestureDescription.StrokeDescription(path, 0, 100))
                .build(),
            null, null
        )
    }
    
    fun findElement(selector: String): AccessibilityNodeInfo? {
        // 遍历 UI 树查找元素
        return rootInActiveWindow?.findBySelector(selector)
    }
}
```

**Rust 客户端**:
```rust
// crates/flowx-android/src/accessibility_client.rs
pub struct AccessibilityClient {
    endpoint: String,  // ws://device-ip:6789
}

impl Platform for AccessibilityClient {
    async fn execute(&self, command: Command) -> Result<CommandResult> {
        // 通过 WebSocket 发送命令到设备上的 Service
        let response = self.send_command(command).await?;
        Ok(response)
    }
}
```

---

## 🔄 迁移路径

### Step 1: 保持 ADB 兼容
```rust
// 同时支持两种模式
pub enum AndroidMode {
    Adb(AdbClient),              // 当前方案
    Accessibility(AccessClient), // 新方案
}
```

### Step 2: 逐步迁移
```python
# 自动选择最佳方案
device = Device.connect("android://auto")  
# 优先 Accessibility，fallback 到 ADB
```

---

## 📊 方案对比总结

| 特性 | ADB | Accessibility Service | UI Automation |
|------|-----|---------------------|---------------|
| 无需 root | ✅ | ✅ | ✅ |
| 无需开发者模式 | ❌ | ✅ | ✅ |
| 可打包分发 | ❌ | ✅ | ✅ |
| UI 树访问 | ⚠️ (需解析) | ✅ | ✅ |
| 性能 | ⚠️ 中 | ✅ 高 | ✅ 高 |
| 跨平台 | ❌ | ❌ | ✅ (Windows) |

---

## 🎯 下一步建议

### 立即可做
1. **设计统一 Platform trait** - 支持多种后端
2. **实现 Android Accessibility Service** - 替代 ADB
3. **添加 Windows UI Automation** - 最简单的桌面平台

### 中期目标
4. iOS 支持 (需要 Mac + WebDriverAgent)
5. macOS 支持

### 长期目标
6. 图像识别兜底方案 (OCR + 模板匹配)
7. AI 驱动的智能识别

---

**结论**: 
- Android → **Accessibility Service** (替代 ADB)
- iOS → **XCUITest + WebDriverAgent**
- Windows → **UI Automation**
- macOS → **Accessibility API**

这样 FlowX 就是真正的全平台统一自动化框架！
