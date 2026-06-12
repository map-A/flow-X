# FlowX Android App - Accessibility Service

FlowX Android 端 APK，基于 Accessibility Service 实现无需 ADB 的自动化。

## 特性

- ✅ 无需 ADB
- ✅ 无需 root
- ✅ 只需授权辅助功能
- ✅ 通过 WebSocket 通信
- ✅ 支持点击、滑动、查找元素

## 构建

```bash
cd android-app
./gradlew build
```

## 安装

```bash
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

## 使用

1. 安装 APK
2. 打开"设置 → 辅助功能 → FlowX"
3. 开启服务
4. Python 代码连接:

```python
from flowx import Device

# 通过 WiFi 连接
device = Device.connect("android://192.168.1.100")
device.click(500, 1000)
```

## 架构

```
Python/Rust Client
       ↓ WebSocket (port 6789)
FlowX Accessibility Service (APK)
       ↓ Android Accessibility API
Android System
```
