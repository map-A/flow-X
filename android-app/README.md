# FlowX Android App - Accessibility Service

FlowX Android 端 APK，基于 Accessibility Service 实现无需 ADB 的自动化。

## 特性

- ✅ 无需 ADB
- ✅ 无需 root
- ✅ 只需授权辅助功能
- ✅ 通过 WebSocket 通信 (端口 6789)
- ✅ 支持点击、滑动、输入、截图、查找元素
- ✅ Android 11+ 支持原生截图 API

## 环境要求

- **Android Studio**: Arctic Fox (2020.3.1) 或更高版本
- **Android SDK**: API 30+ (Android 11+)
- **Kotlin**: 1.9+
- **Gradle**: 8.0+
- **JDK**: 11 或更高版本

## Java 环境配置

### macOS 安装 JDK

本项目需要 JDK 11 或更高版本。推荐使用 Homebrew 安装：

```bash
# 安装 OpenJDK 17 (推荐)
brew install openjdk@17

# 配置环境变量
echo 'export PATH="/opt/homebrew/opt/openjdk@17/bin:$PATH"' >> ~/.zshrc
echo 'export JAVA_HOME="/opt/homebrew/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home"' >> ~/.zshrc

# 重新加载配置
source ~/.zshrc

# 验证安装
java -version
```

预期输出：
```
openjdk version "17.0.x" 2024-xx-xx
OpenJDK Runtime Environment Homebrew (build 17.0.x)
OpenJDK 64-Bit Server VM Homebrew (build 17.0.x, mixed mode, sharing)
```

### 配置 Android Studio 使用 JDK

1. 打开 Android Studio
2. 进入 `Preferences/Settings → Build, Execution, Deployment → Build Tools → Gradle`
3. 设置 **Gradle JDK** 为 `/opt/homebrew/opt/openjdk@17`

或者使用命令行验证：
```bash
# 查看已安装的 Java 版本
/usr/libexec/java_home -V

# 设置 JAVA_HOME（如果有多个 JDK）
export JAVA_HOME=$(/usr/libexec/java_home -v 17)
```

### Windows 安装 JDK

```powershell
# 使用 Scoop
scoop install openjdk17

# 或者从 Oracle 官网下载
# https://www.oracle.com/java/technologies/downloads/#java17
```

### Linux 安装 JDK

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install openjdk-17-jdk

# Fedora/RHEL
sudo dnf install java-17-openjdk-devel

# 验证
java -version
```

## 项目导入

### 方式 1: 使用 Android Studio

1. 打开 Android Studio
2. 选择 `File → Open`
3. 导航到 `android-app` 目录并选择
4. 等待 Gradle 同步完成（首次导入会自动下载依赖）

### 方式 2: 命令行

```bash
cd android-app

# 同步依赖
./gradlew --refresh-dependencies

# 检查项目配置
./gradlew tasks
```

## 编译命令

### Debug 版本

```bash
cd android-app
./gradlew assembleDebug
```

生成的 APK 位于：`app/build/outputs/apk/debug/app-debug.apk`

### Release 版本

```bash
./gradlew assembleRelease
```

生成的 APK 位于：`app/build/outputs/apk/release/app-release.apk`

### 清理构建

```bash
./gradlew clean
```

### 完整重新构建

```bash
./gradlew clean assembleDebug
```

## 安装到设备

### 通过 ADB 安装

```bash
# 安装 Debug 版本
adb install app/build/outputs/apk/debug/app-debug.apk

# 覆盖安装（保留数据）
adb install -r app/build/outputs/apk/debug/app-debug.apk

# 安装 Release 版本
adb install app/build/outputs/apk/release/app-release.apk
```

### 通过 Android Studio 安装

1. 连接 Android 设备（确保 USB 调试已开启）
2. 点击工具栏的 "Run" 按钮（绿色三角形）或按 `Shift + F10`
3. 在弹出的设备列表中选择目标设备

### 检查设备连接

```bash
# 查看已连接设备
adb devices

# 通过 WiFi 连接设备（需先通过 USB 连接）
adb tcpip 5555
adb connect <设备IP>:5555
```

## 使用说明

### 1. 启用无障碍服务

安装 APK 后，必须在系统设置中启用无障碍服务：

```
设置 → 无障碍 → 已安装的服务 → FlowX → 启用
```

验证服务是否启用：
```bash
adb shell settings get secure enabled_accessibility_services
```

应包含：`com.flowx.automation/.FlowXService`

### 2. WebSocket 服务

服务启动后，会在 **端口 6789** 上启动 WebSocket 服务器。

- 本地连接：`ws://localhost:6789`
- WiFi 连接：`ws://<设备IP>:6789`

### 3. 使用 FlowX CLI

```bash
# 获取屏幕信息
flowx-cli --device android://localhost:6789 info

# 点击操作
flowx-cli --device android://localhost:6789 click 500 1000

# 截图
flowx-cli --device android://localhost:6789 screenshot --output screen.png
```

### 4. 使用 Python API

```python
from flowx import Device

# 连接设备
device = Device.connect("android://192.168.1.100:6789")

# 点击
device.click(500, 1000)

# 滑动
device.swipe(500, 1000, 500, 500, duration=300)

# 输入文本
device.input_text("Hello FlowX")

# 截图
image = device.screenshot()
```

## 支持的功能

| 功能 | 命令类型 | Android 版本要求 |
|------|----------|------------------|
| 点击 | `click` | API 24+ |
| 滑动 | `swipe` | API 24+ |
| 输入文本 | `input_text` | API 24+ |
| 按键 | `press_key` | API 24+ |
| 查找元素 | `find` | API 24+ |
| 截图 | `screenshot` | **API 30+ (Android 11+)** |
| 屏幕尺寸 | `screen_size` | API 24+ |
| 当前包名 | `current_package` | API 24+ |
| 当前Activity | `current_activity` | API 24+ |

## WebSocket 协议示例

### 点击操作

请求：
```json
{
  "type": "click",
  "x": 500,
  "y": 1000
}
```

响应：
```json
{
  "status": "success"
}
```

### 截图操作

请求：
```json
{
  "type": "screenshot"
}
```

响应：
```json
{
  "status": "success",
  "data": "iVBORw0KGgoAAAANSUhEUgAA..." 
}
```

`data` 字段为 Base64 编码的 PNG 图片。

### 查找元素

请求：
```json
{
  "type": "find",
  "text": "登录"
}
```

响应：
```json
{
  "status": "success",
  "data": {
    "x": 540,
    "y": 1200,
    "text": "登录"
  }
}
```

## 查看日志

### 实时日志

```bash
# 查看 FlowX 相关日志
adb logcat | grep FlowX

# 只看错误日志
adb logcat *:E | grep FlowX

# 清空日志缓冲区
adb logcat -c
```

### 日志输出示例

```
FlowX: WebSocket server started on port 6789
FlowX: Client connected from /192.168.1.100:54321
FlowX: Received command: click
FlowX: Sent response: success
```

## 故障排除

### 1. WebSocket 连接失败

**检查服务状态：**
```bash
adb shell settings get secure enabled_accessibility_services
```

应包含：`com.flowx.automation/.FlowXService`

**重启服务：**
在设置中关闭再重新开启 FlowX 无障碍服务。

### 2. 截图返回 null 或失败

- ✅ 确认 Android 版本 >= 11 (API 30+)
- ✅ 检查 `accessibility_service_config.xml` 中 `canTakeScreenshot="true"`
- ✅ 重启无障碍服务

查看截图错误日志：
```bash
adb logcat *:E | grep -i screenshot
```

### 3. 输入文本不生效

- ✅ 确保目标输入框已获得焦点
- ✅ 先调用 `click` 点击输入框，再调用 `input_text`
- ✅ 某些应用可能拦截无障碍输入

### 4. Gradle 同步失败

```bash
# 清理 Gradle 缓存
./gradlew clean --refresh-dependencies

# 删除本地缓存重新下载
rm -rf ~/.gradle/caches/
./gradlew build
```

### 5. 编译错误：Kotlin 版本不匹配

在 `build.gradle` 中检查 Kotlin 版本，确保 >= 1.9.0

## 权限说明

应用需要以下权限：

- **无障碍服务权限** (Accessibility Service) - 核心功能，用于控制和读取 UI
- **截图权限** (canTakeScreenshot) - 用于截图功能 (Android 11+)

**注意**：无需申请存储、网络等其他危险权限。

## 系统要求

| 项目 | 最低版本 | 推荐版本 |
|------|---------|---------|
| Android 系统 | 8.0 (API 26) | 11+ (API 30+) |
| 屏幕截图 | ❌ | ✅ API 30+ |

## 架构

```
Python/Rust Client (flowx-cli / flowx-python)
       ↓ WebSocket (port 6789)
FlowX Accessibility Service (APK)
       ↓ Android Accessibility API
       ↓ GestureDescription / AccessibilityNodeInfo
Android System
```

## 依赖库

- **Kotlin**: 1.9.x
- **Java-WebSocket**: WebSocket 服务器实现
- **Gson**: JSON 序列化
- **Android Accessibility API**: 核心自动化能力

## 开发者

FlowX Team
