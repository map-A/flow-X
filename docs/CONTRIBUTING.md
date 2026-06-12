# 贡献指南

欢迎为 FlowX 贡献代码！

---

## 开发环境

### 必需工具
- Rust 1.70+
- Python 3.8+
- uv (Python 包管理)

### 平台特定
- **Android**: Android SDK, ADB
- **macOS**: Xcode Command Line Tools
- **Windows**: Visual Studio Build Tools
- **iOS**: Xcode, WebDriverAgent

---

## 项目结构

```
flowx/
├── crates/
│   ├── flowx-core/      # 核心 + 所有平台
│   ├── flowx-vision/    # OpenCV
│   ├── flowx-python/    # Python 绑定
│   └── flowx-cli/       # CLI 工具
├── scripts/             # 测试脚本
├── android-app/         # Android APK
└── docs/                # 文档
```

---

## 开发流程

### 1. 克隆仓库
```bash
git clone <repo-url>
cd flowx
```

### 2. 设置 Python 环境
```bash
uv venv
source .venv/bin/activate  # Linux/Mac
# .venv\Scripts\activate   # Windows
```

### 3. 构建 Rust 项目
```bash
cargo build
cargo test
```

### 4. 构建 Python 绑定
```bash
cd crates/flowx-python
maturin develop
```

---

## 添加新功能

### 1. 添加新命令

**Step 1**: 在 `flowx-core/src/engine/command.rs` 添加命令
```rust
pub enum Command {
    // ... existing commands
    NewCommand { param: String },
}
```

**Step 2**: 在平台实现中处理
```rust
// flowx-core/src/platforms/macos.rs
match command {
    Command::NewCommand { param } => {
        // 实现逻辑
        Ok(CommandResult::Success)
    }
    // ...
}
```

**Step 3**: 在 Python 绑定中暴露
```rust
// flowx-python/src/device.rs
#[pymethods]
impl Device {
    fn new_command(&self, param: &str) -> PyResult<()> {
        // 调用核心 API
    }
}
```

### 2. 添加新平台

**Step 1**: 创建平台模块
```rust
// flowx-core/src/platforms/newos.rs
pub struct NewOSClient;

#[async_trait]
impl AsyncCommandExecutor for NewOSClient {
    async fn execute_async(&self, command: Command)
        -> Result<CommandResult, CommandError> {
        // 实现
    }
}
```

**Step 2**: 注册平台
```rust
// flowx-core/src/platforms/mod.rs
#[cfg(target_os = "newos")]
pub mod newos;
```

**Step 3**: 添加 Python 后端
```rust
// flowx-python/src/executor.rs
enum MockExecutor {
    // ...
    NewOS(NewOSClient),
}
```

---

## 代码规范

### Rust
- 遵循 `rustfmt` 格式
- 所有公共 API 需要文档注释
- 添加单元测试

```rust
/// 执行点击操作
/// 
/// # Arguments
/// * `x` - X 坐标
/// * `y` - Y 坐标
/// 
/// # Example
/// ```
/// device.click(100, 200).await?;
/// ```
pub async fn click(&self, x: i32, y: i32) -> Result<()> {
    // ...
}
```

### Python
- 遵循 PEP 8
- 添加类型注解
- 添加 docstrings

```python
def click(self, x: int, y: int) -> None:
    """
    点击指定坐标
    
    Args:
        x: X 坐标
        y: Y 坐标
    """
    pass
```

---

## 测试

### 单元测试 (Rust)
```bash
cargo test -p flowx-core
cargo test -p flowx-vision
```

### 集成测试 (Python)
```bash
python3 scripts/test_e2e.py
python3 scripts/test_opencv.py
python3 scripts/test_macos_edge.py
```

### 添加测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // 测试代码
    }

    #[tokio::test]
    async fn test_async_feature() {
        // 异步测试
    }
}
```

---

## 提交规范

### Commit Message
```
<type>: <subject>

<body>
```

**类型**:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `test`: 测试相关
- `refactor`: 重构
- `perf`: 性能优化

**示例**:
```
feat: add Windows platform support

- Implement WindowsClient
- Add PowerShell command execution
- Update Python bindings
```

---

## Pull Request

1. Fork 仓库
2. 创建功能分支: `git checkout -b feat/my-feature`
3. 提交更改: `git commit -m "feat: add feature"`
4. 推送分支: `git push origin feat/my-feature`
5. 创建 Pull Request

### PR 检查清单
- [ ] 代码通过 `cargo fmt` 和 `cargo clippy`
- [ ] 添加单元测试
- [ ] 更新文档
- [ ] 通过所有测试

---

## 文档

### 生成 API 文档
```bash
cargo doc --no-deps --open
```

### 更新文档
- `docs/API.md` - API 使用文档
- `docs/ARCHITECTURE.md` - 架构说明
- `README.md` - 项目介绍

---

## 常见问题

### Q: 如何调试 Python 绑定？
```python
import flowx
print(flowx.__file__)  # 查看模块位置
```

### Q: 如何测试特定平台？
```bash
# 只编译 macOS 平台
cargo build --target aarch64-apple-darwin
```

### Q: OpenCV 依赖问题？
```bash
brew install opencv  # macOS
apt-get install libopencv-dev  # Linux
```

---

## 联系方式

- Issues: GitHub Issues
- Discussions: GitHub Discussions

---

**感谢您的贡献！** 🎉
