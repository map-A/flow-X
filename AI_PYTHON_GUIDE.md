# FlowX AI 模块 - Python 使用指南

## 安装

```bash
cd crates/flowx-python
maturin develop --features ai
```

## 快速开始

### 1. 设置 API Key

```bash
export FLOWX_API_KEY="your-glm-4v-api-key"
```

或者在 Python 代码中直接传入。

### 2. 基本使用

```python
import flowx

# 初始化 AI Agent
agent = flowx.PyAIAgent.from_env()

# 创建设备连接
device = flowx.PyDevice.connect("macos")

# 获取屏幕截图
screenshot = device.screenshot()

# 执行自然语言指令
result = agent.execute("打开 Safari 浏览器", screenshot)

if result.success:
    print(f"成功！执行了 {result.steps} 个步骤")
else:
    print(f"失败：{result.error}")
```

## API 参考

### PyAIAgent

#### 初始化

```python
# 从环境变量初始化（推荐）
agent = flowx.PyAIAgent.from_env()

# 或直接传入 API key
agent = flowx.PyAIAgent("your-api-key")
```

#### 方法

**execute(instruction: str, screenshot: bytes) -> PyTaskResult**

执行自然语言指令。

- `instruction`: 自然语言描述的操作指令
- `screenshot`: PNG 格式的屏幕截图
- 返回: `PyTaskResult` 对象

```python
result = agent.execute("打开微信，找到张三", screenshot)
```

**ask(question: str, screenshot: bytes) -> str**

视觉问答。

- `question`: 关于屏幕内容的问题
- `screenshot`: PNG 格式的屏幕截图
- 返回: 问题的答案（字符串）

```python
answer = agent.ask("屏幕上显示的是什么应用？", screenshot)
```

**set_max_steps(max_steps: int)**

设置最大执行步骤数（默认 20）。

```python
agent.set_max_steps(30)
```

### PyTaskResult

执行结果对象。

#### 属性

- `success: bool` - 是否成功
- `steps: int` - 执行的步骤数
- `error: Optional[str]` - 错误信息（如果失败）

```python
result = agent.execute("...", screenshot)
print(f"成功：{result.success}")
print(f"步骤：{result.steps}")
if result.error:
    print(f"错误：{result.error}")
```

## 支持的操作类型

AI Agent 可以理解并生成以下操作：

1. **open_app** - 打开应用
2. **click** - 点击元素
3. **input** - 输入文本
4. **swipe** - 滑动
5. **wait** - 等待元素出现
6. **back** - 返回上一级
7. **home** - 回到主屏幕

## 示例

### 示例 1: 简单操作

```python
import flowx

agent = flowx.PyAIAgent.from_env()
device = flowx.PyDevice.connect("macos")
screenshot = device.screenshot()

# 打开应用
result = agent.execute("打开 Safari 浏览器", screenshot)
print(result.success)
```

### 示例 2: 复杂任务

```python
# 多步骤任务
result = agent.execute(
    "打开微信，搜索张三，发送消息'你好'",
    screenshot
)
```

### 示例 3: 视觉问答

```python
# 询问屏幕内容
answer = agent.ask("当前是什么界面？有哪些按钮？", screenshot)
print(answer)
```

### 示例 4: 错误处理

```python
try:
    result = agent.execute("打开设置", screenshot)
    if not result.success:
        print(f"执行失败：{result.error}")
except Exception as e:
    print(f"异常：{e}")
```

## 注意事项

1. **API Key**: 必须设置 `FLOWX_API_KEY` 环境变量或在初始化时传入
2. **截图格式**: 必须是 PNG 格式的字节数据
3. **网络连接**: 需要访问 GLM-4V API（https://open.bigmodel.cn）
4. **成本**: GLM-4V API 按调用次数收费，注意使用成本

## 性能指标

- **响应延迟**: 通常 < 5s（取决于网络和 API 响应）
- **成功率**: 简单指令 > 80%，复杂指令 > 60%
- **最大步骤数**: 默认 20，可通过 `set_max_steps()` 调整

## 故障排查

### 问题: `API key not found`
**解决**: 确保设置了 `FLOWX_API_KEY` 环境变量

### 问题: `HTTP request failed`
**解决**: 检查网络连接，确认可以访问 GLM-4V API

### 问题: `Action parse error`
**解决**: 模型返回的格式不正确，可能需要调整 prompt 或重试

## 完整示例

参见 `examples/ai_demo.py`

```bash
python3 examples/ai_demo.py
```
