# FlowX - AI 驱动的跨平台自动化框架

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.11%2B-blue.svg)](https://www.python.org)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Android-lightgrey.svg)]()

> **说出你的需求，FlowX 帮你自动执行。**
>
> AI 自然语言 + Python 脚本 + Rust 高性能引擎 = 下一代跨平台自动化

---

## 特性

- **AI 自然语言驱动** — 用中文描述操作，AI 自动生成并执行
- **Python 脚本** — 简洁的 Python API，快速编写自动化脚本
- **Rust 核心引擎** — 高性能、跨平台、类型安全
- **可视化桌面应用** — FlowX Desktop 提供脚本编辑、设备预览、坐标定位
- **Android APK** — 无需 ADB、无需 Root，WiFi 连接即用
- **多设备并发** — 同时控制多台设备执行相同操作

---

## 支持平台

| 平台 | 组件 | 状态 | 说明 |
|------|------|------|------|
| **macOS** | CLI + Desktop | ✅ 完整支持 | 本机自动化 + Safari/Edge 控制 |
| **Windows** | CLI + Desktop | ✅ 支持 | 需安装 Rust 和 Python |
| **Android** | APK (Accessibility) | ✅ 完整支持 | 无需 Root，WiFi 连接 |
| **Android** | ADB 模式 | ✅ 支持 | USB/WiFi ADB 连接 |

---

## 快速开始（下载即用）

前往 [Releases](https://github.com/map-A/flow-X/releases) 页面下载对应平台的安装包：

| 平台 | 下载文件 | 说明 |
|------|---------|------|
| macOS (Apple Silicon) | `FlowX-Desktop-aarch64.dmg` | M1/M2/M3/M4 芯片 |
| macOS (Intel) | `FlowX-Desktop-x86_64.dmg` | Intel 芯片 |
| Windows | `FlowX-Desktop-x64.msi` | Windows 10/11 |
| Android | `FlowX.apk` | Android 8.0+ |

---

## macOS 使用指南

### 1. 安装 FlowX Desktop

下载 `FlowX-Desktop-aarch64.dmg`，打开并将 FlowX 拖入 Applications 文件夹。

### 2. 安装 Python 环境

```bash
# 安装 Python 3.11+（推荐使用 Homebrew）
brew install python@3.11

# 安装 uv（Python 包管理工具）
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 3. 连接 Android 设备

**方式 A：使用 FlowX APK（推荐）**

1. 在 Android 手机上安装 `FlowX.apk`（下载地址见上方）
2. 打开手机 设置 → 无障碍 → 已安装的服务 → FlowX → 启用
3. 确保手机和电脑在同一 WiFi 下
4. 在 FlowX Desktop 中点击"连接设备"，输入：`android://手机IP:6789`

**方式 B：使用 ADB**

```bash
# 安装 ADB
brew install android-platform-tools

# 连接设备（USB 或 WiFi）
adb devices
# 在 FlowX Desktop 中输入设备 ID，如 emulator-5554
```

### 4. 编写并运行脚本

在 FlowX Desktop 的编辑器中输入：

```python
import flowx

# 连接 Android 设备
device = flowx.Device.connect("android://192.168.1.100:6789")

# 点击屏幕坐标
device.click(540, 1170)

# 滑动操作
device.swipe(540, 1500, 540, 500, 300)

# 输入文字
device.input_text("Hello FlowX")

# 截图
screenshot = device.screenshot()
```

### 5. macOS 本机自动化

```python
import flowx

device = flowx.Device.connect("macos")

# 打开应用
device.open_app("Safari")

# 键盘操作
device.press_key("CommandT")  # Cmd+T
device.input_text("anthropic.com")
device.press_key("Enter")

# 坐标点击
device.click(500, 300)
```

### 6. 自然语言驱动（AI 模式）

```bash
# 设置 AI API Key（使用智谱 GLM-4V）
export FLOWX_API_KEY="your-api-key"

# 用自然语言控制设备
flowx ai "打开 Safari 浏览器"
flowx ai "点击坐标 (100, 200)"
```

---

## Windows 使用指南

### 1. 安装前置依赖

```powershell
# 安装 Rust（https://rustup.rs）
winget install Rustlang.Rustup

# 安装 Python 3.11+
winget install Python.Python.3.11

# 安装 Node.js（Desktop 开发需要）
winget install OpenJS.NodeJS.LTS
```

### 2. 安装 FlowX Desktop

下载 `FlowX-Desktop-x64.msi` 并运行安装程序。

### 3. 连接 Android 设备

与 macOS 相同，推荐使用 FlowX APK：

1. 安装 `FlowX.apk` 到 Android 手机
2. 启用无障碍服务
3. 在 FlowX Desktop 中连接：`android://手机IP:6789`

### 4. Windows 本机自动化

```python
import flowx

device = flowx.Device.connect("windows")

# 打开应用
device.open_app("notepad")

# 点击坐标
device.click(500, 300)

# 输入文字
device.input_text("Hello from FlowX")

# 按键
device.press_key("Enter")
```

### 5. 从源码构建（Windows）

```powershell
git clone https://github.com/map-A/flow-X.git
cd flow-X

# 构建 Rust 核心
cargo build --release

# 构建 Python 绑定
cd crates/flowx-python
pip install maturin
maturin develop --release

# 构建 Desktop 应用
cd ../flowx-desktop
npm install
npm run tauri build
```

---

## Android APK 使用指南

### 安装

1. 下载 `FlowX.apk`（从 [Releases](https://github.com/map-A/flow-X/releases) 页面）
2. 在 Android 手机上打开 APK 文件安装（需允许"未知来源"）

### 启用无障碍服务

```
设置 → 无障碍（或辅助功能）→ 已安装的服务 → FlowX → 开启
```

> ⚠️ **重要**：必须启用无障碍服务，否则无法控制设备。

### 连接方式

FlowX APK 通过 **WebSocket** 通信（端口 6789），手机和电脑需在同一 WiFi 网络。

查看手机 IP：设置 → 关于手机 → 状态 → IP 地址

```python
# 通过 Python 连接
import flowx
device = flowx.Device.connect("android://手机IP:6789")
device.click(540, 1170)
```

### 支持的功能

| 功能 | 说明 | Android 版本要求 |
|------|------|-----------------|
| 点击 | `device.click(x, y)` | Android 7.0+ |
| 滑动 | `device.swipe(x1, y1, x2, y2, ms)` | Android 7.0+ |
| 输入文字 | `device.input_text("text")` | Android 7.0+ |
| 按键 | `device.press_key("BACK")` | Android 7.0+ |
| 查找元素 | `device.find(text="登录")` | Android 7.0+ |
| 截图 | `device.screenshot()` | **Android 11+** |
| 获取屏幕尺寸 | `device.get_screen_size()` | Android 7.0+ |

---

## Python API 速查

### 基础操作

```python
import flowx

# 连接设备
device = flowx.Device.connect("macos")                        # macOS 本机
device = flowx.Device.connect("windows")                      # Windows 本机
device = flowx.Device.connect("emulator-5554")                # ADB 设备
device = flowx.Device.connect("android://192.168.1.100:6789") # APK 设备

# 基础操作
device.click(100, 200)              # 点击坐标
device.swipe(100, 500, 100, 200, 300)  # 滑动（起x, 起y, 止x, 止y, 毫秒）
device.input_text("Hello World")    # 输入文字
device.press_key("Enter")           # 按键
device.screenshot()                 # 截图
device.get_screen_size()            # 获取屏幕尺寸
```

### 元素查找

```python
# 按文本查找
button = device.find(text="确认")
button.click()

# 链式调用
device.find(text="搜索框").click()
device.input_text("笔记本电脑")

# 等待元素出现
result = device.wait_for(text="搜索结果", timeout_ms=5000)
```

### AI 自然语言

```python
from flowx import AI

ai = AI()

# 自然语言操作
ai.execute("打开微信，找到张三，发送'你好'")

# 视觉问答
answer = ai.ask("屏幕上显示的是什么应用？")
```

---

## CLI 工具

```bash
# 点击操作
flowx-cli --device emulator-5554 click 540 1170

# 滑动操作
flowx-cli --device emulator-5554 swipe 540 1500 540 500 300

# 输入文字
flowx-cli --device emulator-5554 input "Hello"

# 获取设备信息
flowx-cli --device emulator-5554 info

# 自然语言驱动
flowx-cli ai "打开微信"
```

---

## FlowX Desktop

FlowX Desktop 是可视化脚本开发环境：

```
┌──────────┬────────────────────────┬─────────────────────┐
│  脚本列表  │    代码编辑器             │    设备预览          │
│  --------│    ▶ 运行  💾 保存      │    🔄 刷新  📷 截图   │
│  📄 script│                        │                     │
│  📄 test  │    [Monaco 编辑器]      │    [设备截图]        │
│          │                        │                     │
│  设备列表  │                        │    坐标显示          │
│  --------│    ────────────────    │    X: 100 Y: 200    │
│  📱 设备1 │    输出 (Console)      │                     │
│          │    [控制台输出]         │                     │
└──────────┴────────────────────────┴─────────────────────┘
```

**功能**：
- 脚本管理（新建、编辑、保存）
- Monaco Editor 代码编辑器（语法高亮、智能提示）
- 实时设备屏幕预览
- 坐标模式：鼠标悬停显示坐标，点击自动插入代码
- 脚本运行和控制台输出

---

## 从源码构建

### 前置要求

| 工具 | 版本 | 用途 |
|------|------|------|
| Rust | 1.95+ | 核心引擎 |
| Python | 3.11+ | Python 绑定 |
| Node.js | 18+ | Desktop 前端 |
| pnpm | 8+ | Desktop 依赖管理 |

### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/map-A/flow-X.git
cd flow-X

# 构建 Rust 核心
cargo build --release

# 构建 Python 绑定
cd crates/flowx-python
pip install maturin
maturin develop --release
cd ../..

# 构建 Desktop 应用
cd crates/flowx-desktop
pnpm install
pnpm run tauri build
cd ../..

# 构建 Android APK
cd android-app
./gradlew assembleDebug
cd ..
```

### 构建产物

| 组件 | 产物路径 |
|------|---------|
| CLI | `target/release/flowx-cli` |
| Python 绑定 | 已安装到 Python 环境 |
| Desktop (macOS) | `crates/flowx-desktop/src-tauri/target/release/bundle/dmg/` |
| Desktop (Windows) | `crates/flowx-desktop/src-tauri/target/release/bundle/msi/` |
| Android APK | `android-app/build/outputs/apk/debug/FlowX-debug.apk` |

---

## 项目结构

```
flow-X/
├── crates/
│   ├── flowx-core/         # Rust 核心引擎（所有平台逻辑）
│   ├── flowx-python/       # Python 绑定（PyO3）
│   ├── flowx-cli/          # 命令行工具
│   └── flowx-desktop/      # Tauri + React 桌面应用
├── android-app/            # Android Accessibility Service APK
├── scripts/                # 测试脚本和工具
├── docs/                   # API 文档和贡献指南
├── examples/               # 示例代码
└── archive/                # 历史文档存档
```

---

## 常见问题

**Q: Android APK 连接失败？**
- 确保手机和电脑在同一 WiFi
- 确保已启用无障碍服务
- 检查防火墙是否阻止了 6789 端口

**Q: macOS 无法控制应用？**
- 需要在 系统设置 → 隐私与安全 → 辅助功能 中授权终端或 FlowX Desktop

**Q: Python 绑定安装失败？**
- 确保安装了 Rust 工具链：`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- 确保安装了 maturin：`pip install maturin`

**Q: Desktop 应用启动慢？**
- 首次启动需要编译 Rust 代码（约 2-3 分钟），后续启动秒开

---

## 许可证

[MIT License](LICENSE)

---

**FlowX** — 让每个人都能用 AI 控制数字世界。
