# FlowX 自然语言驱动快速指南

**日期**: 2026-06-12  
**功能**: 使用自然语言控制设备自动化

---

## 快速开始

### 1. 启动本地 LLM

```bash
./scripts/start_llm.sh
```

模型: `mlx-community/Qwen2.5-3B-Instruct-4bit`  
端口: `8080`

### 2. 使用自然语言

```bash
python3 scripts/nlp_engine_v2.py "打开 Safari 浏览器"
python3 scripts/nlp_engine_v2.py "点击坐标 (100, 200)"
python3 scripts/nlp_engine_v2.py "输入文字 hello world"
```

### 3. Dry Run 模式（只生成代码不执行）

```bash
python3 scripts/nlp_engine_v2.py --dry-run "从 (540, 1500) 滑动到 (540, 800)"
```

---

## 工作原理

```
自然语言指令
    ↓
本地 LLM (Qwen2.5-3B)
    ↓
Python 代码生成
    ↓
FlowX API 执行
    ↓
设备操作
```

---

## 示例

### 示例 1: 打开应用

```bash
$ python3 scripts/nlp_engine_v2.py --dry-run "打开 Safari"

生成代码:
import flowx
flowx.Device.connect("macos").open_app("Safari")
```

### 示例 2: 点击操作

```bash
$ python3 scripts/nlp_engine_v2.py --dry-run "点击屏幕坐标 (100, 200)"

生成代码:
import flowx
device = flowx.Device.connect("macos")
device.click(100, 200)
```

### 示例 3: 滑动操作

```bash
$ python3 scripts/nlp_engine_v2.py --dry-run "从 (540, 1500) 滑动到 (540, 800)"

生成代码:
import flowx
device = flowx.Device.connect("android")
device.swipe(540, 1500, 540, 800, 300)
```

---

## 测试

```bash
python3 scripts/test_nlp_final.py
```

**测试结果**: 4/4 通过 ✅

---

## 技术栈

- **LLM**: MLX-LM (Apple Silicon 优化)
- **模型**: Qwen2.5-3B-Instruct-4bit
- **框架**: FlowX 自动化框架
- **语言**: Python 3.11+

---

## 配置

编辑 `scripts/start_llm.sh`:
```bash
MLX_MODEL="mlx-community/Qwen2.5-3B-Instruct-4bit"
MLX_PORT="8080"
MLX_HOST="127.0.0.1"
```

---

## 注意事项

1. **仅支持 macOS Apple Silicon** (MLX 要求)
2. **首次启动需下载模型** (~2GB)
3. **Dry run 模式推荐用于测试**
4. **生成的代码需人工审核后再执行**

---

## 扩展

### 自定义提示词

编辑 `scripts/nlp_engine_v2.py` 中的 `prompt` 变量添加更多示例。

### 支持更多操作

在提示词中添加更多 FlowX API 示例即可自动学习。

---

**FlowX - 自然语言驱动的跨平台自动化！** 🚀
