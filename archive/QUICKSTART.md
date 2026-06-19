# FlowX Desktop - 快速使用指南

## ✅ 项目已完成

### 升级内容
- ✅ Rust 1.95.0 + Edition 2024
- ✅ React 19.2.7 + TypeScript 6.0.3 + Vite 8.0.16
- ✅ UI 从 Slint 迁移到 Tauri + React
- ✅ ADB 模拟器连接功能

---

## 🚀 快速开始

### 1. 启动应用

```bash
cd /Users/mm/Documents/compile_accelarate/crates/flowx-desktop
cargo tauri dev
```

首次启动需要 2-3 分钟编译，之后会自动打开应用窗口。

### 2. 连接模拟器

**步骤:**
1. 确认模拟器运行: `adb devices`（应显示 emulator-5554）
2. 点击"连接设备"按钮
3. 输入: `emulator-5554`
4. 点击确认

### 3. 编写和运行脚本

点击"新建"，输入脚本：

```python
# 点击测试
device.click(540, 1170)
```

点击"运行"执行。

---

## 📝 脚本示例

```python
import time

# 返回主屏幕
device.press_key("KEYCODE_HOME")
time.sleep(1)

# 点击
device.click(540, 1170)

# 滑动
device.swipe(540, 1500, 540, 500, 300)

# 输入文字
device.type_text("Hello")

# 截图
screenshot = device.screenshot()
```

---

## 🔧 API 参考

| 操作 | 语法 |
|------|------|
| 点击 | `device.click(x, y)` |
| 输入 | `device.type_text("text")` |
| 滑动 | `device.swipe(x1, y1, x2, y2, ms)` |
| 按键 | `device.press_key("KEYCODE_BACK")` |
| 截图 | `device.screenshot()` |

---

## 📚 完整文档

- `SIMULATOR_GUIDE.md` - 详细使用指南
- `TEST_REPORT.md` - 测试报告
- `FINAL_REPORT.md` - 项目总结

---

**🎉 开始使用！**
