# FlowX Desktop - 前端应用 PRD

**版本**: v1.0  
**创建日期**: 2026-06-12  
**产品类型**: 跨平台桌面应用（Rust）  
**目标用户**: 自动化脚本开发者、测试工程师

---

## 1. 产品概述

### 1.1 产品定位

FlowX Desktop 是一个可视化的自动化脚本开发环境，提供：
- Python 脚本管理（新建、编辑、运行）
- 实时设备屏幕预览
- 可视化坐标定位
- FlowX API 智能提示
- 多设备并行控制

### 1.2 核心价值

1. **所见即所得** - 在设备屏幕上直接定位元素坐标
2. **提升效率** - 代码提示 + 模板，减少查文档时间
3. **降低门槛** - 可视化界面，新手也能快速上手
4. **多设备支持** - 同时管理多个 Android/iOS/macOS 设备

---

## 2. 技术选型

### 2.1 UI 框架

**首选: Slint**

| 方案 | 优势 | 劣势 |
|------|------|------|
| **Tauri + Web** | 开发快、UI 美观、生态丰富 | 体积较大、性能一般 |
| **Slint** | 纯 Rust、性能好、跨平台 | 生态较新、组件较少 |
| **iced** | 纯 Rust、Elm 架构、稳定 | 学习曲线、组件较少 |
| **egui** | 即时模式、开发快 | UI 不够现代 |

**推荐**: **Tauri** (快速 MVP) 或 **Slint** (长期可维护性)

### 2.2 技术栈

```
Rust (Backend)
├── Slint (UI Framework)
├── flowx-core (核心引擎)
├── tokio (异步运行时)
└── serde (序列化)

Frontend (Tauri Web 方案)
├── React/Vue/Svelte
├── Monaco Editor (代码编辑器)
└── Canvas (屏幕预览 + 坐标标注)
```

---

## 3. 功能需求

### 3.1 主界面布局

```
┌─────────────────────────────────────────────────────────┐
│  FlowX Desktop                        [设置] [帮助]     │
├──────────┬──────────────────────────────────────────────┤
│          │                                              │
│  脚本列表 │            代码编辑器                         │
│          │         (Monaco Editor)                      │
│  [+新建]  │                                              │
│  [刷新]   │  1  import flowx                            │
│          │  2                                           │
│  📄 test1│  3  device = flowx.PyDevice.connect("xxx")  │
│  📄 test2│  4  device.click(500, 600)                  │
│  📄 demo │  5                                           │
│          │                                              │
│          │  [▶ 运行] [💾 保存] [📋 复制]               │
├──────────┼──────────────────────────────────────────────┤
│          │                                              │
│  设备列表 │            设备屏幕预览                       │
│          │         (实时截图 + 坐标蒙版)                 │
│ 📱 Pixel │                                              │
│   5 (已连接)│        ┌─────────────────┐                │
│           │        │                 │ X: 500          │
│ 📱 iPhone │        │   [点击定位]     │ Y: 600          │
│   13      │        │                 │                 │
│ 💻 MacBook│        └─────────────────┘                │
│           │                                              │
│ [+连接设备]│  [刷新] [截图] [坐标模式: ON]              │
└──────────┴──────────────────────────────────────────────┘
```

### 3.2 核心功能详述

#### 功能 1: 脚本管理

**脚本列表**
- 显示所有 Python 脚本（`~/.flowx/scripts/` 目录）
- 支持搜索、排序（按名称/修改时间）
- 右键菜单：重命名、删除、复制、导出

**新建脚本**
- 点击 `[+新建]` 按钮
- 弹出对话框输入脚本名称
- 自动生成默认模板：

```python
#!/usr/bin/env python3
"""
FlowX 自动化脚本
创建时间: {timestamp}
目标设备: {current_device}
"""

import flowx

# 连接设备
device = flowx.PyDevice.connect("{current_device_uri}")

# 获取屏幕信息
width, height = device.get_screen_size()
print(f"屏幕尺寸: {width}x{height}")

# TODO: 在此处编写你的自动化脚本
device.click(500, 600)
```

**编辑脚本**
- 使用 Monaco Editor（VS Code 同款编辑器）
- 语法高亮（Python）
- 代码折叠、行号
- FlowX API 智能提示（见 3.3）

**保存脚本**
- 自动保存（3秒无输入后）
- 手动保存快捷键：`Ctrl/Cmd + S`
- 状态指示：已保存 / 未保存*

#### 功能 2: 代码智能提示

**FlowX API 自动完成**

触发条件：输入 `device.` 或 `flowx.`

提示内容：
```
device.
  ├─ click(x: int, y: int)            # 点击坐标
  ├─ swipe(from_x, from_y, to_x, to_y, duration)
  ├─ input_text(text: str)            # 输入文本
  ├─ screenshot() -> bytes            # 截图
  ├─ get_screen_size() -> (int, int)  # 获取屏幕尺寸
  └─ find_element(selector) -> Element
```

**文档悬浮提示**

鼠标悬停在 API 上时显示：
```
device.click(x, y)
───────────────────
点击屏幕坐标

参数:
  x (int): X 坐标
  y (int): Y 坐标

示例:
  device.click(500, 600)
```

**代码片段（Snippets）**

输入缩写自动展开：
- `click` → `device.click(x, y)`
- `swipe` → `device.swipe(from_x, from_y, to_x, to_y, duration_ms)`
- `wait` → `device.wait_for_element(selector, timeout_ms)`

#### 功能 3: 设备管理

**设备列表**
- 显示所有已连接设备
  - Android 设备（通过 ADB）
  - iOS 设备（通过 libimobiledevice）
  - macOS 本机
- 设备信息：名称、状态、分辨率
- 选中设备高亮显示

**连接设备**
- 点击 `[+连接设备]` 按钮
- 弹出对话框选择：
  - Android (ADB)
  - iOS (USB)
  - macOS (本机)
  - Windows (远程)
- 输入设备 URI（如 `android://192.168.1.100:5555`）

**断开设备**
- 右键菜单 → 断开连接
- 设备状态变为 "未连接"

#### 功能 4: 设备屏幕预览

**实时预览**
- 每 500ms 自动刷新截图（可配置间隔）
- 自适应窗口大小（保持宽高比）
- 支持缩放：50% / 100% / 150%

**坐标蒙版**

开启 "坐标模式" 后：
- 在屏幕上显示半透明网格（可选）
- 鼠标移动时实时显示坐标（右上角）
- 点击屏幕时：
  1. 复制坐标到剪贴板
  2. 在编辑器当前光标位置插入坐标
  3. 坐标点显示红色标记（3秒后淡出）

**坐标标注示例**：
```
用户在屏幕点击 (500, 600) →
编辑器自动插入: device.click(500, 600)
```

**交互增强**
- 右键菜单：
  - 复制坐标
  - 插入 click()
  - 插入 swipe()（需要两个点）
  - 截图保存
- 滑动操作：
  1. 按住鼠标左键拖动
  2. 自动生成 `device.swipe(x1, y1, x2, y2, 300)`

#### 功能 5: 脚本运行

**运行模式**
- 点击 `[▶ 运行]` 按钮
- 在底部打开输出面板（Console）
- 实时显示：
  - 脚本输出（print 语句）
  - 错误信息（异常堆栈）
  - 执行时间

**输出面板**
```
┌─────────────────────────────────────┐
│ 输出 (Console)          [清空] [停止]│
├─────────────────────────────────────┤
│ [14:30:15] 开始执行 test1.py        │
│ [14:30:15] 连接设备: android://xxx  │
│ [14:30:16] 屏幕尺寸: 1080x1920      │
│ [14:30:16] 点击坐标: (500, 600)     │
│ [14:30:17] ✅ 执行成功 (1.2s)       │
└─────────────────────────────────────┘
```

**停止运行**
- 点击 `[停止]` 按钮
- 强制终止 Python 进程
- 显示 "已中止" 状态

---

## 4. 用户流程

### 4.1 首次使用流程

```
1. 启动 FlowX Desktop
   ↓
2. [+连接设备] → 选择 Android → 输入设备 URI
   ↓
3. 设备列表显示 "Pixel 5 (已连接)"
   ↓
4. [+新建] → 输入 "测试脚本"
   ↓
5. 编辑器自动填充默认模板（已连接设备信息）
   ↓
6. 开启 "坐标模式" → 点击屏幕 (500, 600)
   ↓
7. 编辑器自动插入: device.click(500, 600)
   ↓
8. [▶ 运行] → 查看输出
```

### 4.2 日常使用流程

```
1. 启动应用 → 自动重连上次设备
   ↓
2. 从脚本列表选择已有脚本
   ↓
3. 修改代码 → 坐标模式定位
   ↓
4. [▶ 运行] → 调试 → 修改 → 运行（循环）
   ↓
5. [💾 保存] → 完成
```


### 6.1 核心组件

| 组件 | 说明 | 技术 |
|------|------|------|
| ScriptList | 脚本列表 | 树形组件 |
| CodeEditor | 代码编辑器 | Monaco Editor |
| DeviceList | 设备列表 | 列表组件 |
| ScreenPreview | 屏幕预览 | Canvas / Image |
| CoordinateOverlay | 坐标蒙版 | 透明 Canvas 叠加 |
| Console | 输出面板 | 文本区域 + 自动滚动 |

### 6.2 布局策略

**主界面**: 三栏布局（可调整大小）
- 左侧：脚本列表 + 设备列表（20%）
- 中间：代码编辑器（50%）
- 右侧：屏幕预览（30%）

**响应式**:
- 最小宽度：1280px
- 推荐分辨率：1920x1080
- 支持 HiDPI 显示器

---

## 7. 性能要求

### 7.1 响应速度

| 操作 | 目标延迟 |
|------|---------|
| 打开脚本 | < 100ms |
| 保存脚本 | < 50ms |
| 代码提示 | < 50ms |
| 屏幕刷新 | 500ms (2 FPS) |
| 坐标显示 | < 16ms (60 FPS) |
| 运行脚本 | 启动 < 1s |

### 7.2 资源占用

- 内存：< 200MB（空闲）
- CPU：< 5%（空闲）
- 磁盘：< 100MB（安装包）

---

## 8. 安全性

### 8.1 文件权限

- 脚本文件：用户目录 `~/.flowx/scripts/`
- 配置文件：`~/.flowx/config.toml`
- 仅当前用户可读写

### 8.2 设备连接

- ADB 连接：仅本地网络
- iOS 连接：需用户授权
- 不收集任何设备数据

### 8.3 代码执行

- 脚本权限：仅能调用 FlowX API
- 不允许访问系统文件（可配置）

---

## 9. 配置管理

### 9.1 应用配置 (`~/.flowx/config.toml`)

```toml
[editor]
font_size = 14
theme = "dark"  # dark, light
tab_size = 4
auto_save = true
auto_save_interval = 3  # 秒

[preview]
refresh_interval = 500  # ms
default_scale = 1.0
show_grid = false
coordinate_mode = true

[general]
last_script = "test1.py"
last_device = "android://192.168.1.100:5555"
```

---

## 10. 未来扩展

### Phase 2 功

1. **录制模式**
   - 在设备上操作
   - 自动生成 Python 代码

2. **AI 辅助**
   - 自然语言生成脚本
   - "点击登录按钮" → `device.click(x, y)

3. **插件系统**
   - 自定义代码提示
   - 自定义主题
   - 扩展 API

---

## 11. 交付清单

### 11.1 技术实现

- [ ] Rust 后端架构
- [ ] UI 框架集成（Tauri/Slint）
- [ ] 脚本管理模块
- [ ] 设备管理模块
- [ ] 屏幕预览模块
- [ ] 坐标标注模块
- [ ] 代码编辑器集成
- [ ] Python 运行时

### 11.2 文档

- [ ] 用户手册
- [ ] API 文档
- [ ] 开发者指南
- [ ] 键盘快捷键列表

### 11.3 测试

- [ ] 单元测试（Rust 模块）
- [ ] UI 测试（E2E）
- [ ] 跨平台测试（macOS, Windows, Linux）
- [ ] 性能测试

---

## 12. 开发计划

### Week 1-2: 基础架构
- 选定 UI 框架（Tauri 快速 MVP）
- 搭建项目结构
- 实现脚本管理（CRUD）

### Week 3-4: 核心功能
- 设备连接和列表
- 屏幕预览（实时截图）
- 代码编辑器集成

### Week 5-6: 交互增强
- 坐标蒙版和标注
- 代码智能提示
- 脚本运行和输出

### Week 7-8: 打磨和测试
- UI 美化
- 性能优化
- 跨平台测试
- 文档编写

**总工时**: 320 小时（2 人 x 4 周）

---

## 13. 成功指标

### 13.1 功能完整性
- ✅ 所有核心功能实现
- ✅ 跨平台运行（macOS, Windows, Linux）
- ✅ 稳定性测试通过

### 13.2 用户体验
- 新用户 5 分钟内完成首个脚本
- 代码提示响应 < 50ms
- 屏幕预览流畅（无卡顿）

### 13.3 技术指标
- 内存占用 < 200MB
- 启动时间 < 3s
- 打包体积 < 50MB

---

**PRD 版本**: v1.0  
**批准人**: [待定]  
**开始日期**: 2026-06-13

---

## 附录 A: 架构说明

### Android 设备连接方式

FlowX Desktop **不使用 ADB**，而是通过 Accessibility Service 实现设备控制。

#### 架构设计

```
┌─────────────────────┐
│  FlowX Desktop      │
│  (Rust/Slint)       │
└──────────┬──────────┘
           │ HTTP/WebSocket
           │
┌──────────▼──────────┐
│  Android App        │
│  (Java/Kotlin)      │
│                     │
│  ┌───────────────┐  │
│  │ HTTP Server   │  │
│  └───────┬───────┘  │
│          │          │
│  ┌───────▼───────┐  │
│  │ Accessibility │  │
│  │   Service     │  │
│  └───────────────┘  │
└─────────────────────┘
```

#### 通信协议

**设备 URI 格式**: `android://host:port`  
示例: `android://localhost:8080`

**API 端点**:
- `GET  /api/screen_size` - 获取屏幕尺寸
- `GET  /api/screenshot` - 获取屏幕截图 (PNG)
- `POST /api/tap` - 点击坐标 `{"x": 100, "y": 200}`
- `POST /api/swipe` - 滑动手势 `{"from_x": 100, "from_y": 500, "to_x": 900, "to_y": 500, "duration_ms": 300}`
- `POST /api/input_text` - 输入文本 `{"text": "Hello"}`
- `POST /api/press_key` - 按键 `{"key": "BACK"}`
- `POST /api/long_press` - 长按 `{"x": 100, "y": 200, "duration_ms": 1000}`
- `GET  /api/current_package` - 获取当前包名
- `GET  /api/current_activity` - 获取当前 Activity

#### 优势

1. **无需 USB 连接** - 通过 WiFi 网络通信
2. **更高权限** - Accessibility Service 可访问 UI 树
3. **更稳定** - 不依赖 ADB 调试模式
4. **更安全** - 用户主动安装授权
5. **跨网络** - 支持远程设备控制

#### Android App 实现要点

1. **Accessibility Service**:
   - 实现 `AccessibilityService` 类
   - 在 `AndroidManifest.xml` 中声明
   - 用户手动开启无障碍权限

2. **HTTP Server**:
   - 使用 NanoHTTPD 或 Ktor
   - 监听端口 8080
   - 处理 REST API 请求

3. **手势执行**:
   - `AccessibilityService.dispatchGesture()`
   - 构造 `GestureDescription`
   - 支持点击、滑动、长按

4. **截图功能**:
   - `MediaProjection` API
   - 需要用户授权屏幕录制权限
   - 返回 PNG 格式数据

