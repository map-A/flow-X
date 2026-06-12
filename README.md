# FlowX - 智能跨平台自动化框架

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.11%2B-blue.svg)](https://www.python.org)
[![Tests](https://img.shields.io/badge/tests-37%2F37-brightgreen.svg)](STATUS.md)

> 统一的跨平台自动化框架 + AI 驱动的自然语言控制

---

## 🚀 核心特性

✨ **统一架构** - 所有平台在 flowx-core 中，使用条件编译  
🤖 **自然语言驱动** - 用中文描述操作，AI 自动生成代码  
🎯 **元素查找** - 按文本、ID、类名智能定位 UI 元素  
👆 **手势支持** - 捏合、拉伸、旋转、多点触控  
🔄 **多设备并发** - 同时控制多个设备  
👁️ **OpenCV 集成** - 模板匹配、颜色识别  
📱 **Android APK** - Accessibility Service + WebSocket  
🍎 **macOS 控制** - 完整浏览器自动化验证

---

## 📦 支持平台

| 平台 | 状态 | 测试 |
|------|------|------|
| Android (ADB) | ✅ | 6/6 |
| Android (APK) | ✅ | 3/3 |
| macOS | ✅ | 5/5 |
| Windows | ⚠️ 架构就绪 | - |
| iOS | ⚠️ 架构就绪 | - |

---

## 🎯 快速开始

### 安装

```bash
# 克隆项目
git clone https://github.com/yourusername/flowx
cd flowx

# Python 环境
uv venv
source .venv/bin/activate

# 构建 Python 绑定
cd crates/flowx-python && maturin develop

# CLI 工具
cargo build -p flowx-cli
```

### 基础使用

```python
import flowx

# 连接设备
device = flowx.Device.connect("macos")

# 基础操作
device.click(100, 200)
device.swipe(100, 500, 100, 200, 300)
device.input_text("hello world")
device.press_key("Enter")
```

### 自然语言模式

```bash
# 启动本地 LLM
./scripts/start_llm.sh

# 自然语言控制
python3 scripts/nlp_engine_v2.py "打开 Safari 浏览器"
python3 scripts/nlp_engine_v2.py "点击坐标 (100, 200)"
```

详见 [NLP_QUICKSTART.md](NLP_QUICKSTART.md)

---

## 🎨 高级功能

### 元素查找

```rust
use flowx_core::{ElementFinder, Selector};

let finder = ElementFinder::new();
let elem = finder.find(&Selector::Text("Login".to_string()))?;
let cmd = elem.click();
```

### 手势支持

```rust
use flowx_core::{Gesture, Point};

// 捏合缩小
let center = Point { x: 500, y: 500 };
let cmds = Gesture::pinch_in(center, 200, 300);

// 双击
let cmds = Gesture::double_tap(100, 200);
```

### 多设备并发

```rust
use flowx_core::MultiDevice;

let multi = MultiDevice::new();
multi.register("device1".to_string(), "android".to_string()).await;
multi.register("device2".to_string(), "ios".to_string()).await;

let results = multi.execute_parallel(
    vec!["device1".to_string(), "device2".to_string()],
    Command::Click { x: 100, y: 200 }
).await;
```

详见 [ENHANCEMENTS.md](ENHANCEMENTS.md)

---

## 📊 测试状态

**总计**: 37/37 (100%) ✅

- CLI 命令: 4/4
- 自然语言驱动: 4/4  
- 功能增强: 10/10
- Android 端到端: 6/6
- macOS Edge: 5/5
- Android APK: 3/3
- OpenCV: 3/3
- 核心单元: 2/2

```bash
# 运行所有测试
python3 scripts/test_all.py
```

---

## 📚 文档

```python
from flowx import Device

# 创建设备
device = Device.android()

# 基础操作
device.click(100, 200)
device.swipe(100, 500, 100, 100)

# 元素操作
button = device.find(text="确认")
button.click()

# 链式调用
device.find(text="搜索框").click()
device.input_text("笔记本电脑")
device.find(text="搜索").click()

# 等待元素
result = device.wait_for(text="搜索结果", timeout_ms=5000)

# OCR识别
texts = device.ocr()
for text, bounds, confidence in texts:
    print(f"{text}: {confidence}")
```

### AI自然语言

```python
from flowx import AI

ai = AI()

# 自然语言操作
ai.execute("打开微信，找到张三，发送'你好'")

# 视觉问答
answer = ai.ask("屏幕上显示的是什么应用？")

# 智能任务
ai.complete_task("在淘宝上搜索'笔记本电脑'并截图前三个商品")
```

### JavaScript（AutoJS兼容）

```javascript
// 兼容AutoJS语法
auto.waitFor();

app.launch("com.tencent.mm");
sleep(2000);

let btn = text("发现").findOne();
if (btn) {
    btn.click();
}
```

---

## 🏗️ 项目结构

```
flowx/
├── docs/                       # 📚 项目文档
│   ├── README.md              # 文档中心入口
│   ├── PROJECT_OVERVIEW.md    # 项目总览
│   ├── TASK_ASSIGNMENT.md     # 任务分配
│   ├── architecture/          # 架构设计
│   ├── development/           # 开发任务
│   └── testing/               # 测试计划
│
├── PRD.md                     # 产品需求文档
├── PRD_SUMMARY.md             # PRD总结
│
├── crates/                    # Rust代码（待创建）
│   ├── flowx-core/           # 核心引擎
│   ├── flowx-android/        # Android平台
│   ├── flowx-python/         # Python绑定
│   └── flowx-cli/            # CLI工具
│
├── python/                    # Python包（待创建）
│   └── flowx/
│
├── js-runtime/                # JS运行时（待创建）
│   └── stdlib/
│
└── tests/                     # 测试（待创建）
```

---

## 👥 团队

| 角色 | 职责 | 工时 |
|------|------|------|
| 架构设计师 | 系统设计、技术选型 | 80h |
| Rust核心开发 | Rust引擎、Android平台 | 180h |
| Python/JS开发 | Python绑定、JS运行时 | 130h |
| AI工程师 | AI模型、自然语言 | 100h |
| 测试工程师 | 测试策略、质量保证 | 140h |

**总工时**：630小时（约3个月）

---

## 📅 里程碑

| 里程碑 | 时间 | 交付物 | 状态 |
|--------|------|--------|------|
| M1: 架构完成 | Week 4 | 架构文档、核心框架 | 📝 待开始 |
| M2: 脚本引擎就绪 | Week 8 | Python/JS可运行脚本 | 📝 待开始 |
| M3: AI集成完成 | Week 10 | AI自然语言操作 | 📝 待开始 |
| M4: MVP发布 | Week 12 | 完整产品 | 📝 待开始 |

---

## 🚦 当前状态

**项目阶段**：规划完成，准备启动  
**下一步**：架构师开始 ARCH-001 系统架构设计

---

## 🤝 如何参与

1. **查看任务分配**：[docs/TASK_ASSIGNMENT.md](docs/TASK_ASSIGNMENT.md)
2. **找到你的任务**：根据角色查看对应文档
3. **阅读任务说明**：了解目标、交付物、验收标准
4. **开始开发**：按照任务文档进行实现
5. **更新进度**：完成后更新文档状态

---

## 📞 联系方式

- **项目经理**：[待定]
- **Slack**：#flowx-dev
- **每日站会**：9:30-9:45
- **周会**：每周五下午

---

## 📄 许可证

MIT License

---

## 🌟 愿景

**让每个人都能用AI控制数字世界。**

从手机自动化开始，逐步扩展到桌面、浏览器、IoT，成为AI时代的自动化操作系统。

---

**创建日期**：2026-06-10  
**项目状态**：🚀 即将启动  
**团队规模**：5人  
**预计完成**：2026-09-10
