# FlowX Desktop

现代化的 Android/iOS 自动化脚本开发桌面应用。

## 🎉 新版本特性

- ✨ **现代化 UI**: 基于 React + Tailwind CSS 的全新界面
- 🎨 **主题支持**: 暗色/亮色主题一键切换
- 📝 **智能编辑器**: 集成代码编辑和实时预览
- 📱 **设备预览**: 实时截图和交互式坐标选择
- ⚡ **高性能**: 使用 Tauri 2 + Rust 构建

## 🚀 快速开始

### 前置要求

- Node.js 18+ 
- pnpm 或 npm
- Rust (如需完整构建)

### 安装依赖

```bash
cd crates/flowx-desktop
pnpm install
```

### 开发模式

#### 方式 1: 仅前端开发 (推荐)
```bash
pnpm run dev
```
然后访问 http://localhost:1420

#### 方式 2: 完整应用 (需要 Rust 编译)
```bash
pnpm run tauri dev
```

### 生产构建

```bash
pnpm run build              # 构建前端
pnpm run tauri build        # 构建完整应用
```

## 📖 使用说明

### 1. 连接设备
- 点击左下角的 "连接设备" 按钮
- 输入设备 URI (例如: `android://localhost:5555`)
- 设备连接后会显示在设备列表中

### 2. 创建脚本
- 点击左上角的 "新建" 按钮
- 在中间的编辑器中编写 Python 脚本
- 点击 "保存" 保存脚本

### 3. 获取坐标
- 点击右侧的 "刷新" 或 "截图" 按钮获取设备截图
- 鼠标悬停在截图上查看坐标
- 点击截图自动插入 `device.click(x, y)` 代码

### 4. 运行脚本
- 点击 "运行" 按钮执行当前脚本
- 在下方的控制台查看输出结果

## 🎨 UI 布局

```
┌─────────────────────────────────────────────────────────┐
│  FlowX Desktop                              🌙 主题切换  │
├──────────┬────────────────────────┬─────────────────────┤
│  脚本列表  │    代码编辑器             │    设备预览          │
│  --------│    ▶ 运行  💾 保存      │    🔄 刷新  📷 截图   │
│  📄 script│                        │                     │
│  📄 test  │    [编辑区域]           │    [设备截图]        │
│          │                        │                     │
│  设备列表  │                        │    坐标显示          │
│  --------│    ────────────────    │    X: 100 Y: 200    │
│  📱 设备1 │    输出 (Console)      │                     │
│          │    [控制台输出]         │                     │
└──────────┴────────────────────────┴─────────────────────┘
```

## 📦 技术栈

- **前端**: React 18 + TypeScript
- **样式**: Tailwind CSS 3
- **构建**: Vite 6
- **图标**: Lucide React
- **后端**: Tauri 2 + Rust
- **核心**: flowx-core

## ⚠️ 已知问题

### Rust 编译错误
当前使用 Rust 1.95 可能遇到依赖冲突问题。

**临时解决方案**:
- 仅运行前端: `pnpm run dev`
- 等待 Tauri 更新
- 或使用 Rust 1.83: `rustup override set 1.83.0`

详见: [TAURI_MIGRATION_REPORT.md](./TAURI_MIGRATION_REPORT.md)

## 📝 脚本示例

```python
# 连接设备
device = flowx.PyDevice.connect("android://localhost:5555")

# 点击屏幕
device.click(500, 600)

# 滑动
device.swipe(100, 500, 900, 500, 300)

# 输入文本
device.input_text("Hello World")

# 截图
screenshot = device.screenshot()

# 获取屏幕尺寸
width, height = device.get_screen_size()
```

## 🔗 相关文档

- [迁移指南](./MIGRATION.md)
- [完整报告](./TAURI_MIGRATION_REPORT.md)
- [API 文档](../../docs/API.md)

## 📄 License

MIT

## 🙏 致谢

- 原 Slint 版本的代码已备份至 `../flowx-desktop-slint-backup/`
- 感谢 Tauri、React 和所有开源社区的贡献
